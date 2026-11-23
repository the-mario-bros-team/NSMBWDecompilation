package com.wither.nsmbwport;

import android.content.Context;
import android.net.Uri;
import android.os.Build;

import org.json.JSONException;
import org.json.JSONObject;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import java.util.zip.ZipEntry;
import java.util.zip.ZipInputStream;

/**
 * GPU Driver: drivers for Snapdragon (Adreno) phones the player imports as zips in the usual layout (a
 * meta.json naming the library, as Turnip's releases are), and which one the game runs on. libnsmbw.so
 * opens the picked one through libadrenotools (crates/android/src/gpu_driver.rs). Android loads
 * libraries only from the app's internal storage, so drivers are unpacked under files/gpu_drivers; so
 * are libadrenotools and its two hook libraries (APK assets, gpu_hooks/), and only once a driver is
 * picked: on the phone's own driver they are never loaded.
 *
 * A newly picked driver is "on trial" until the game has run on it for TRIAL_MS. A start on trial leaves
 * a mark that the game clears when the trial ends; a mark still there at the next start means the game
 * died on the driver, so it goes back to Default and the start menu says so (once).
 */
final class GpuDrivers {
    static final long TRIAL_MS = 20000;
    private static final String[] HOOKS = {"libadrenotools.so", "libhook_impl.so", "libmain_hook.so"};

    private GpuDrivers() {}

    static final class Driver {
        /** Its folder under installed/. */
        final String id;
        final String name;
        /** The library file meta.json names. */
        final String library;

        Driver(String id, String name, String library) {
            this.id = id;
            this.name = name;
            this.library = library;
        }
    }

    /** Adreno GPUs, that is Snapdragon chips: their kernel driver is /dev/kgsl-3d0. */
    static boolean supported() {
        if (Build.VERSION.SDK_INT >= 31) {
            final String soc = Build.SOC_MANUFACTURER.toLowerCase(Locale.ROOT);
            if (soc.contains("qualcomm") || soc.contains("qti")) return true;
        }
        return new File("/dev/kgsl-3d0").exists() || Build.HARDWARE.toLowerCase(Locale.ROOT).contains("qcom");
    }

    private static File root(Context c) { return new File(c.getFilesDir(), "gpu_drivers"); }
    private static File installed(Context c) { return new File(root(c), "installed"); }
    private static File pickFile(Context c) { return new File(root(c), "picked"); }
    private static File trialFile(Context c) { return new File(root(c), "on-trial"); }
    private static File startedFile(Context c) { return new File(root(c), "trial-started"); }
    private static File failedFile(Context c) { return new File(root(c), "trial-failed"); }

    static List<Driver> list(Context context) {
        final List<Driver> drivers = new ArrayList<>();
        final File[] folders = installed(context).listFiles(File::isDirectory);
        if (folders != null) {
            for (File folder : folders) {
                final Driver d = load(folder);
                if (d != null) drivers.add(d);
            }
        }
        drivers.sort((a, b) -> a.name.compareToIgnoreCase(b.name));
        return drivers;
    }

    /** The picked driver, or null for the phone's own (Default). */
    static Driver picked(Context context) {
        final String id = read(pickFile(context));
        return id == null || id.isEmpty() ? null : load(new File(installed(context), id));
    }

    /** Picks `driver` (null: Default). A driver starts on trial. */
    static void pick(Context context, Driver driver) throws IOException {
        root(context).mkdirs();
        startedFile(context).delete();
        if (driver == null) {
            pickFile(context).delete();
            trialFile(context).delete();
            return;
        }
        Files.write(pickFile(context).toPath(), driver.id.getBytes(StandardCharsets.UTF_8));
        trialFile(context).createNewFile();
    }

    static void delete(Context context, Driver driver) throws IOException {
        final Driver current = picked(context);
        if (current != null && current.id.equals(driver.id)) pick(context, null);
        GameFiles.deleteRecursively(new File(installed(context), driver.id));
    }

    /** Unpacks a driver zip and picks it; throws with the reason to show when it isn't a usable one. */
    static Driver importZip(Context context, Uri zip) throws IOException {
        final File staging = new File(root(context), "importing");
        GameFiles.deleteRecursively(staging);
        staging.mkdirs();
        try {
            final String inside = staging.getCanonicalPath() + File.separator;
            try (InputStream in = context.getContentResolver().openInputStream(zip);
                 ZipInputStream entries = new ZipInputStream(in)) {
                for (ZipEntry e; (e = entries.getNextEntry()) != null; ) {
                    final File out = new File(staging, e.getName());
                    if (!out.getCanonicalPath().startsWith(inside)) continue;  // a name leading out of the folder
                    if (e.isDirectory()) {
                        out.mkdirs();
                    } else {
                        out.getParentFile().mkdirs();
                        try (OutputStream file = new FileOutputStream(out)) {
                            GameFiles.copy(entries, file);
                        }
                    }
                }
            }
            final String meta = read(new File(staging, "meta.json"));
            if (meta == null) throw new IOException(Lang.t("This zip isn't a GPU driver: it has no meta.json."));
            final int minApi;
            try {
                minApi = new JSONObject(meta).optInt("minApi", 0);
            } catch (JSONException e) {
                throw new IOException(Lang.t("This zip isn't a GPU driver: its meta.json can't be read."));
            }
            if (minApi > Build.VERSION.SDK_INT) throw new IOException(Lang.f("This driver needs Android API %s or newer.", minApi));
            final Driver found = load(staging);
            if (found == null) {
                throw new IOException(Lang.t("This zip isn't a GPU driver: the library its meta.json names isn't in it."));
            }
            // A folder name from the driver's name: letters, digits and . _ - only (the linker's search path
            // is built from it), never "." or "..".
            String id = "driver-" + found.name.replaceAll("[^A-Za-z0-9._-]", "_");
            if (id.length() > 72) id = id.substring(0, 72);
            final File destination = new File(installed(context), id);
            final Driver current = picked(context);
            if (current != null && current.id.equals(id)) pick(context, null);
            GameFiles.deleteRecursively(destination);
            installed(context).mkdirs();
            if (!staging.renameTo(destination)) throw new IOException("could not move the driver into place");
            final Driver driver = new Driver(id, found.name, found.library);
            pick(context, driver);
            return driver;
        } finally {
            GameFiles.deleteRecursively(staging);
        }
    }

    /**
     * Just before the game starts: Graphics Compatibility, and the picked driver if any (unpacking
     * libadrenotools and its hooks for it), handed to libnsmbw.so. A driver on trial whose last start
     * never ended its trial is dropped for Default here.
     */
    static void applyToGame(Context context, int compat) {
        Driver driver = supported() ? picked(context) : null;
        if (driver != null && trialFile(context).exists()) {
            if (startedFile(context).exists()) {
                fail(context);
                driver = null;
            } else {
                try {
                    startedFile(context).createNewFile();
                } catch (IOException ignored) {
                }
            }
        }
        if (driver == null) {
            Native.setGraphics(compat, null, null, null, null);
            return;
        }
        final File hooks = new File(root(context), "hooks");
        final File tmp = new File(root(context), "tmp");
        try {
            hooks.mkdirs();
            tmp.mkdirs();
            for (String hook : HOOKS) {
                final File part = new File(hooks, hook + ".part");
                try (InputStream in = context.getAssets().open("gpu_hooks/" + hook);
                     OutputStream out = new FileOutputStream(part)) {
                    GameFiles.copy(in, out);
                }
                if (!part.renameTo(new File(hooks, hook))) throw new IOException("could not unpack " + hook);
            }
        } catch (IOException e) {
            android.util.Log.e("NSMBW", "GPU driver: " + e.getMessage());
            Native.setGraphics(compat, null, null, null, null);
            return;
        }
        final String error = Native.setGraphics(compat, hooks.getPath(),
                new File(installed(context), driver.id).getPath(), driver.library, tmp.getPath());
        // libadrenotools couldn't open it: the game runs on the phone's own driver, so the pick goes too.
        if (error != null) fail(context);
    }

    private static void fail(Context context) {
        try {
            pick(context, null);
            failedFile(context).createNewFile();
        } catch (IOException ignored) {
        }
    }

    /** The game has run on the picked driver for TRIAL_MS: it stays picked. */
    static void endTrial(Context context) {
        trialFile(context).delete();
        startedFile(context).delete();
    }

    /** Whether a driver was just put back to Default for stopping the game (true once). */
    static boolean takeFailure(Context context) {
        return failedFile(context).delete();
    }

    private static Driver load(File folder) {
        final String meta = read(new File(folder, "meta.json"));
        if (meta == null) return null;
        try {
            final JSONObject json = new JSONObject(meta);
            final String library = json.getString("libraryName");
            if (library.contains("/") || !new File(folder, library).isFile()) return null;
            return new Driver(folder.getName(), json.optString("name", library), library);
        } catch (JSONException e) {
            return null;
        }
    }

    private static String read(File file) {
        try {
            return new String(Files.readAllBytes(file.toPath()), StandardCharsets.UTF_8).trim();
        } catch (IOException e) {
            return null;
        }
    }
}
