package com.wither.nsmbwport;

import android.content.ContentResolver;
import android.content.Context;
import android.database.Cursor;
import android.net.Uri;
import android.provider.DocumentsContract;
import android.provider.OpenableColumns;

import java.io.BufferedInputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.FilterInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.List;
import java.util.zip.ZipEntry;
import java.util.zip.ZipInputStream;

/**
 * The player's own extracted game, imported into the app's storage (files/game): checking it, and
 * importing the folder Dolphin's Extract Entire Disc makes, or a zip of it, through the system's file
 * picker. From NSMB-Wii-Recompilation's app (AppFiles and the start menu's import), for this port,
 * which runs the same disc: the USA one (SMNE01), revision 1.
 */
final class GameFiles {
    private GameFiles() {}

    interface Progress {
        void update(int done, int total, String text);
    }

    /** What an import or the game imported before says when files the game needs are missing. */
    static final String INCOMPLETE = "The folder doesn't include all required files.";

    /** The app's files: the native side's working folder (saves/, cache/). */
    static File dataDir(Context context) { return context.getFilesDir(); }

    static File gameDir(Context context) { return new File(dataDir(context), "game"); }

    private static File stagingDir(Context context) { return new File(dataDir(context), "game.importing"); }

    /** An extracted disc: sys/main.dol and the files/ tree. */
    static boolean isGameRoot(File dir) {
        return new File(dir, "sys/main.dol").isFile() && new File(dir, "files").isDirectory();
    }

    static boolean gameReady(Context context) {
        return isGameRoot(gameDir(context)) && gameProblem(gameDir(context)) == null;
    }

    /** Why the extracted game at `root` can't be played, or null when it can. */
    static String gameProblem(File root) {
        if (!isGameRoot(root)) return Lang.t(INCOMPLETE);
        final byte[] header = new byte[8];
        try (InputStream in = new FileInputStream(new File(root, "sys/boot.bin"))) {
            if (readFully(in, header) < header.length) {
                return Lang.t("Its sys/boot.bin is too short, so the game can't be identified.");
            }
        } catch (IOException e) {
            return Lang.t(INCOMPLETE);
        }
        final String problem = headerProblem(header);
        if (problem != null) return problem;
        // The port also reads the partition's title metadata (tmd.bin, beside sys/ in Dolphin's
        // extraction) for the IOS layer and the boot.
        if (!new File(root, "tmd.bin").isFile() || !GameFileList.missing(root, 1).isEmpty()) return Lang.t(INCOMPLETE);
        return moduleProblem(root);
    }

    /**
     * The game's code modules are checked against the original disc's (this port knows the
     * original's code): each is the original's LZ file or, decompressed, the original's .rel.
     */
    private static final String[] MODULES = {"d_profileNP", "d_basesNP", "d_enemiesNP", "d_en_bossNP"};
    private static final long[] MODULE_LZ_SIZES = {8791, 1190805, 779805, 229984};
    private static final long[] MODULE_REL_SIZES = {24244, 3019336, 2137424, 605552};

    private static String moduleProblem(File root) {
        for (int i = 0; i < MODULES.length; ++i) {
            final File lz = new File(root, "files/rels/" + MODULES[i] + ".rel.LZ");
            final File rel = new File(root, "files/rels/" + MODULES[i] + ".rel");
            if (lz.length() == MODULE_LZ_SIZES[i] || rel.length() == MODULE_REL_SIZES[i]) continue;
            if (!lz.isFile() && !rel.isFile()) {
                return Lang.f("It has no files/rels/%s.rel.LZ: this port needs the whole disc, as Dolphin's "
                        + "Extract Entire Disc makes it.", MODULES[i]);
            }
            return Lang.f("Its files/rels/%s is changed: it isn't the original disc (a modded or repacked "
                    + "image?). This port runs only the unchanged disc.", (lz.isFile() ? lz : rel).getName());
        }
        return null;
    }

    /** The first 8 bytes of sys/boot.bin name the game (6 characters) and its revision (byte 7). */
    static String headerProblem(byte[] header) {
        final String id = new String(header, 0, 6, StandardCharsets.US_ASCII);
        final int revision = header[7] & 0xFF;
        if (!id.equals("SMNE01")) {
            return Lang.f("That is %s, not New Super Mario Bros. Wii for the USA (SMNE01). "
                    + "This port runs only the USA disc, revision 1.", id.trim());
        }
        if (revision != 1) {
            return Lang.f("That is New Super Mario Bros. Wii revision %d. This port runs only "
                    + "revision 1 (Dolphin shows it under Properties > Info).", revision);
        }
        return null;
    }

    // --- import ------------------------------------------------------------------------------------

    /** The checked game at `root` (inside the staging folder) becomes the game; saves stay. */
    private static String install(Context context, File root, int fileCount) throws IOException {
        final File staging = stagingDir(context);
        final String problem = gameProblem(root);
        if (problem != null) {
            deleteRecursively(staging);
            return problem.equals(Lang.t(INCOMPLETE)) ? problem : Lang.f("%s\n\nNothing was changed.", problem);
        }
        final File game = gameDir(context);
        deleteRecursively(game);
        if (!root.renameTo(game)) throw new IOException("cannot move the imported data into place");
        deleteRecursively(staging);
        return Lang.f("Imported %d files. Tap Play to start.", fileCount);
    }

    static void abandonImport(Context context) { deleteRecursively(stagingDir(context)); }

    private static final class Entry {
        final String id, name;
        final boolean directory;
        Entry(String id, String name, boolean directory) { this.id = id; this.name = name; this.directory = directory; }
    }

    /** A folder from the document picker: copied into the staging folder, checked, then swapped in. */
    static String copyFolder(Context context, Uri tree, Progress progress) throws IOException {
        final ContentResolver resolver = context.getContentResolver();
        String rootId = DocumentsContract.getTreeDocumentId(tree);
        if (!looksLikeGame(resolver, tree, rootId)) {
            String nested = null;
            for (Entry e : list(resolver, tree, rootId)) {
                if (e.directory && e.name.equalsIgnoreCase("DATA") && looksLikeGame(resolver, tree, e.id)) nested = e.id;
            }
            if (nested == null) return Lang.t(INCOMPLETE);
            rootId = nested;
        }
        final String problem = treeProblem(resolver, tree, rootId);
        if (problem != null) return Lang.f("%s\n\nNothing was changed.", problem);
        final List<String[]> files = new ArrayList<>();  // {documentId, relative path}
        collect(resolver, tree, rootId, "", files);

        final File staging = stagingDir(context);
        deleteRecursively(staging);
        for (int i = 0; i < files.size(); ++i) {
            final File target = new File(staging, files.get(i)[1]);
            final File parent = target.getParentFile();
            if (parent != null && !parent.isDirectory() && !parent.mkdirs()) throw new IOException("cannot create " + parent);
            final Uri document = DocumentsContract.buildDocumentUriUsingTree(tree, files.get(i)[0]);
            try (InputStream in = resolver.openInputStream(document); OutputStream out = new FileOutputStream(target)) {
                if (in == null) throw new IOException("cannot read " + files.get(i)[1]);
                copy(in, out);
            }
            progress.update(i + 1, files.size(), Lang.f("%d of %d files", i + 1, files.size()));
        }
        return install(context, staging, files.size());
    }

    /** sys/boot.bin of the picked folder names the game and its revision. */
    private static String treeProblem(ContentResolver resolver, Uri tree, String rootId) throws IOException {
        String sysId = null;
        for (Entry e : list(resolver, tree, rootId)) {
            if (e.directory && e.name.equals("sys")) sysId = e.id;
        }
        String bootId = null;
        if (sysId != null) {
            for (Entry e : list(resolver, tree, sysId)) {
                if (!e.directory && e.name.equals("boot.bin")) bootId = e.id;
            }
        }
        if (bootId == null) return Lang.t(INCOMPLETE);
        final byte[] header = new byte[8];
        try (InputStream in = resolver.openInputStream(DocumentsContract.buildDocumentUriUsingTree(tree, bootId))) {
            if (in == null || readFully(in, header) < header.length) {
                return Lang.t("That folder's sys/boot.bin is too short, so the game can't be identified.");
            }
        }
        return headerProblem(header);
    }

    /** A zip of the folder Dolphin extracted: unpacked into the staging folder, checked, swapped in. */
    static String copyZip(Context context, Uri zip, Progress progress) throws IOException {
        final File staging = stagingDir(context);
        final int count = unzip(context, zip, staging, progress);
        final File root = findGameRoot(staging, 3);
        if (root == null) {
            deleteRecursively(staging);
            return Lang.t(INCOMPLETE);
        }
        return install(context, root, count);
    }

    /** Unpacks `zip` into a fresh `staging` folder; returns how many files it held. */
    private static int unzip(Context context, Uri zip, File staging, Progress progress) throws IOException {
        final long size = documentSize(context, zip);
        deleteRecursively(staging);
        if (!staging.mkdirs()) throw new IOException("cannot create " + staging);
        final String stagingPath = staging.getCanonicalPath() + File.separator;
        int count = 0;
        try (InputStream raw = context.getContentResolver().openInputStream(zip)) {
            if (raw == null) throw new IOException("cannot read the zip");
            final CountingInputStream counted = new CountingInputStream(new BufferedInputStream(raw, 1 << 16));
            try (ZipInputStream in = new ZipInputStream(counted)) {
                ZipEntry entry;
                while ((entry = in.getNextEntry()) != null) {
                    // Windows PowerShell's zips separate (and end folder entries) with backslashes.
                    final String name = entry.getName().replace('\\', '/');
                    final File target = new File(staging, name);
                    if (!target.getCanonicalPath().startsWith(stagingPath)) {
                        throw new IOException("the zip holds a path outside its folder: " + name);
                    }
                    if (entry.isDirectory() || name.endsWith("/")) {
                        if (!target.isDirectory() && !target.mkdirs()) throw new IOException("cannot create " + target);
                        continue;
                    }
                    final File parent = target.getParentFile();
                    if (parent != null && !parent.isDirectory() && !parent.mkdirs()) throw new IOException("cannot create " + parent);
                    try (OutputStream out = new FileOutputStream(target)) {
                        copy(in, out);
                    }
                    ++count;
                    if (size > 0) {
                        final int permille = (int) Math.min(1000, counted.count * 1000 / size);
                        progress.update(permille, 1000, Lang.f("%d of %d MB", counted.count >> 20, size >> 20));
                    } else {
                        progress.update(0, 0, Lang.f("%d files", count));
                    }
                }
            }
        }
        return count;
    }

    /** The extracted game inside an unpacked zip: its top, DATA, or a folder holding either. */
    private static File findGameRoot(File dir, int depth) {
        if (isGameRoot(dir)) return dir;
        if (depth == 0) return null;
        final File[] children = dir.listFiles(File::isDirectory);
        if (children == null) return null;
        for (File child : children) {
            if (child.getName().equals("__MACOSX")) continue;
            final File found = findGameRoot(child, depth - 1);
            if (found != null) return found;
        }
        return null;
    }

    private static long documentSize(Context context, Uri uri) {
        try (Cursor c = context.getContentResolver().query(uri, new String[] {OpenableColumns.SIZE}, null, null, null)) {
            if (c != null && c.moveToFirst() && !c.isNull(0)) return c.getLong(0);
        } catch (RuntimeException ignored) {
            // no size: progress counts files instead
        }
        return -1;
    }

    private static final class CountingInputStream extends FilterInputStream {
        long count;
        CountingInputStream(InputStream in) { super(in); }
        @Override public int read() throws IOException {
            final int b = super.read();
            if (b >= 0) ++count;
            return b;
        }
        @Override public int read(byte[] buffer, int offset, int length) throws IOException {
            final int read = super.read(buffer, offset, length);
            if (read > 0) count += read;
            return read;
        }
        @Override public long skip(long n) throws IOException {
            final long skipped = super.skip(n);
            count += skipped;
            return skipped;
        }
    }

    private static boolean looksLikeGame(ContentResolver resolver, Uri tree, String id) {
        boolean sys = false, files = false;
        for (Entry e : list(resolver, tree, id)) {
            if (e.directory && e.name.equals("sys")) sys = true;
            if (e.directory && e.name.equals("files")) files = true;
        }
        return sys && files;
    }

    private static void collect(ContentResolver resolver, Uri tree, String id, String prefix, List<String[]> out) {
        for (Entry e : list(resolver, tree, id)) {
            final String path = prefix.isEmpty() ? e.name : prefix + "/" + e.name;
            if (e.directory) collect(resolver, tree, e.id, path, out);
            else out.add(new String[] {e.id, path});
        }
    }

    private static List<Entry> list(ContentResolver resolver, Uri tree, String id) {
        final List<Entry> entries = new ArrayList<>();
        final Uri children = DocumentsContract.buildChildDocumentsUriUsingTree(tree, id);
        final String[] columns = {
            DocumentsContract.Document.COLUMN_DOCUMENT_ID,
            DocumentsContract.Document.COLUMN_DISPLAY_NAME,
            DocumentsContract.Document.COLUMN_MIME_TYPE,
        };
        try (Cursor c = resolver.query(children, columns, null, null, null)) {
            while (c != null && c.moveToNext()) {
                entries.add(new Entry(c.getString(0), c.getString(1),
                        DocumentsContract.Document.MIME_TYPE_DIR.equals(c.getString(2))));
            }
        }
        return entries;
    }

    // --- files -------------------------------------------------------------------------------------

    static int readFully(InputStream in, byte[] buffer) throws IOException {
        int total = 0;
        while (total < buffer.length) {
            final int read = in.read(buffer, total, buffer.length - total);
            if (read <= 0) break;
            total += read;
        }
        return total;
    }

    static void copy(InputStream in, OutputStream out) throws IOException {
        final byte[] buffer = new byte[1 << 16];
        int read;
        while ((read = in.read(buffer)) > 0) {
            out.write(buffer, 0, read);
        }
    }

    static void deleteRecursively(File file) {
        final File[] children = file.listFiles();
        if (children != null) {
            for (File child : children) {
                deleteRecursively(child);
            }
        }
        //noinspection ResultOfMethodCallIgnored
        file.delete();
    }
}
