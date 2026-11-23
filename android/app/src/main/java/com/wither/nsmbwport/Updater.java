package com.wither.nsmbwport;

import android.app.PendingIntent;
import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageInstaller;
import android.content.pm.PackageManager;
import android.os.Build;

import org.json.JSONArray;
import org.json.JSONException;
import org.json.JSONObject;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.HttpURLConnection;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

/**
 * Check for Updates and Auto Update: the release source crates/port-update uses (GitHub's latest
 * release of the project), its Android asset (*-android-arm64.apk), downloaded straight into a
 * PackageInstaller session while its SHA-256 is checked against the digest GitHub lists for it, then
 * committed. Android installs it over this app only when the same key signed it; UpdateReceiver hears
 * how it went. Network work: never on the UI thread.
 */
final class Updater {
    /** port_update::DEFAULT_SOURCE. */
    private static final String SOURCE = "https://api.github.com/repos/the-mario-bros-team/NSMBWDecompilation/releases/latest";
    private static final String ASSET_SUFFIX = "-android-arm64.apk";
    private static final int TIMEOUT_MS = 15000;

    private Updater() {}

    static final class Release {
        /** "0.0.3", the tag without its "v". */
        String version;
        String url;
        long size = -1;
        /** Lowercase hex, or null when GitHub lists no digest. */
        String sha256;
    }

    interface Progress { void update(long done, long total); }

    /**
     * The latest release, or null when it has no Android download. For tests, a file named update-source
     * in the app's files holds another address answering the same JSON.
     */
    static Release latest(Context context) throws IOException {
        final String override = readText(new File(context.getFilesDir(), "update-source"));
        final HttpURLConnection http = open(override != null && !override.isEmpty() ? override : SOURCE);
        http.setRequestProperty("Accept", "application/vnd.github+json");
        try {
            if (http.getResponseCode() != HttpURLConnection.HTTP_OK) throw new IOException(Lang.f("GitHub answered %d", http.getResponseCode()));
            final JSONObject json = new JSONObject(readAll(http.getInputStream()));
            final JSONArray assets = json.getJSONArray("assets");
            for (int i = 0; i < assets.length(); ++i) {
                final JSONObject asset = assets.getJSONObject(i);
                if (!asset.getString("name").endsWith(ASSET_SUFFIX)) continue;
                final Release release = new Release();
                release.version = json.getString("tag_name").replaceFirst("^[vV]", "");
                release.url = asset.getString("browser_download_url");
                release.size = asset.optLong("size", -1);
                final String digest = asset.optString("digest", "");
                release.sha256 = digest.startsWith("sha256:") ? digest.substring(7).toLowerCase(java.util.Locale.ROOT) : null;
                return release;
            }
            return null;
        } catch (JSONException e) {
            throw new IOException("unreadable answer: " + e.getMessage());
        } finally {
            http.disconnect();
        }
    }

    /** This app's version ("0.0.2"); for tests, a file named update-test-version in its files stands in. */
    static String installedVersion(Context context) {
        final String test = readText(new File(context.getFilesDir(), "update-test-version"));
        if (test != null && !test.isEmpty()) return test;
        try {
            return context.getPackageManager().getPackageInfo(context.getPackageName(), 0).versionName;
        } catch (PackageManager.NameNotFoundException e) {
            return "0";
        }
    }

    /** Whether version `remote` ("0.1.0") comes after `local`, number by number. */
    static boolean isNewer(String remote, String local) {
        final String[] a = remote.split("\\."), b = local.split("\\.");
        for (int i = 0; i < Math.max(a.length, b.length); ++i) {
            final int x = i < a.length ? leadingNumber(a[i]) : 0, y = i < b.length ? leadingNumber(b[i]) : 0;
            if (x != y) return x > y;
        }
        return false;
    }

    /** Whether the player has let this app install apps (else system settings asks them). */
    static boolean canInstall(Context context) {
        return context.getPackageManager().canRequestPackageInstalls();
    }

    /**
     * Downloads `release` into an install session, checks it and commits it; returns once the system has
     * it (UpdateReceiver hears the rest). Throws when the download or the check fails: nothing changes.
     */
    static void downloadAndInstall(Context context, Release release, Progress progress) throws IOException {
        final PackageInstaller installer = context.getPackageManager().getPackageInstaller();
        final PackageInstaller.SessionParams params = new PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL);
        params.setAppPackageName(context.getPackageName());
        if (release.size > 0) params.setSize(release.size);
        if (Build.VERSION.SDK_INT >= 31) {
            // Android 12+: an app that installed itself may update itself without asking again.
            params.setRequireUserAction(PackageInstaller.SessionParams.USER_ACTION_NOT_REQUIRED);
        }
        final int id = installer.createSession(params);
        boolean committed = false;
        try (PackageInstaller.Session session = installer.openSession(id)) {
            final MessageDigest sha256;
            try {
                sha256 = MessageDigest.getInstance("SHA-256");
            } catch (NoSuchAlgorithmException e) {
                throw new IOException("no SHA-256");
            }
            final HttpURLConnection http = open(release.url);
            long done = 0;
            try {
                if (http.getResponseCode() != HttpURLConnection.HTTP_OK) throw new IOException(Lang.f("GitHub answered %d", http.getResponseCode()));
                final long total = release.size > 0 ? release.size : http.getContentLengthLong();
                try (InputStream in = http.getInputStream();
                     OutputStream out = session.openWrite("base.apk", 0, total > 0 ? total : -1)) {
                    final byte[] buffer = new byte[1 << 16];
                    for (int n; (n = in.read(buffer)) > 0; ) {
                        out.write(buffer, 0, n);
                        sha256.update(buffer, 0, n);
                        done += n;
                        progress.update(done, total);
                    }
                    session.fsync(out);
                }
            } finally {
                http.disconnect();
            }
            if (release.size > 0 && done != release.size) {
                throw new IOException("the download is " + done + " bytes, not " + release.size);
            }
            if (release.sha256 != null && !release.sha256.equals(hex(sha256.digest()))) {
                throw new IOException("the download doesn't match the release's SHA-256");
            }
            final PendingIntent result = PendingIntent.getBroadcast(context, id, new Intent(context, UpdateReceiver.class),
                    PendingIntent.FLAG_UPDATE_CURRENT | PendingIntent.FLAG_MUTABLE);
            session.commit(result.getIntentSender());
            committed = true;
        } finally {
            if (!committed) installer.abandonSession(id);
        }
    }

    private static HttpURLConnection open(String url) throws IOException {
        final HttpURLConnection http = (HttpURLConnection) new URL(url).openConnection();
        http.setConnectTimeout(TIMEOUT_MS);
        http.setReadTimeout(TIMEOUT_MS);
        http.setInstanceFollowRedirects(true);  // release downloads redirect to GitHub's file host
        http.setRequestProperty("User-Agent", "nsmbw-port-android-updater");
        return http;
    }

    private static String readAll(InputStream in) throws IOException {
        try (InputStream stream = in) {
            final ByteArrayOutputStream out = new ByteArrayOutputStream();
            GameFiles.copy(stream, out);
            return out.toString("UTF-8");
        }
    }

    private static String readText(File file) {
        try {
            return new String(Files.readAllBytes(file.toPath()), StandardCharsets.UTF_8).trim();
        } catch (IOException e) {
            return null;
        }
    }

    private static int leadingNumber(String part) {
        int value = 0;
        for (int i = 0; i < part.length() && Character.isDigit(part.charAt(i)); ++i) value = value * 10 + (part.charAt(i) - '0');
        return value;
    }

    private static String hex(byte[] bytes) {
        final StringBuilder s = new StringBuilder(bytes.length * 2);
        for (byte b : bytes) s.append(Character.forDigit((b >> 4) & 0xF, 16)).append(Character.forDigit(b & 0xF, 16));
        return s.toString();
    }
}
