package com.wither.nsmbwport;

import android.app.Activity;
import android.content.Context;
import android.content.Intent;
import android.net.Uri;
import android.os.Build;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.text.SimpleDateFormat;
import java.util.Arrays;
import java.util.Date;
import java.util.Locale;
import java.util.zip.ZipEntry;
import java.util.zip.ZipOutputStream;

/**
 * Download Logs (For Github Reports): a zip of the newest runs' logs (files/logs, written by
 * libnsmbw.so's logcat module), what logcat still holds of this app, the settings and the phone's model,
 * saved where the player picks. No saves, no game files.
 */
final class LogExport {
    /** Bigger files keep their end: a long run's log can be large. */
    private static final long MAX_BYTES = 8L << 20;

    private LogExport() {}

    static void pickDestination(Activity activity, int requestCode) {
        final String stamp = new SimpleDateFormat("yyyy-MM-dd_HH-mm", Locale.ROOT).format(new Date());
        activity.startActivityForResult(new Intent(Intent.ACTION_CREATE_DOCUMENT)
                .addCategory(Intent.CATEGORY_OPENABLE)
                .setType("application/zip")
                .putExtra(Intent.EXTRA_TITLE, "nsmbw-port-logs-" + stamp + ".zip"), requestCode);
    }

    /** Writes the zip to `destination`; returns how many runs' logs went in. Off the UI thread. */
    static int write(Context context, Uri destination, String version) throws IOException {
        File[] runs = new File(context.getFilesDir(), "logs").listFiles((dir, name) -> name.endsWith(".log"));
        if (runs == null) runs = new File[0];
        Arrays.sort(runs, (a, b) -> b.getName().compareTo(a.getName()));  // newest first (named by time)
        try (OutputStream raw = context.getContentResolver().openOutputStream(destination)) {
            if (raw == null) throw new IOException("could not open the chosen file");
            try (ZipOutputStream zip = new ZipOutputStream(raw)) {
                put(zip, "device.txt", device(context, version, runs.length).getBytes(StandardCharsets.UTF_8));
                putFile(zip, "settings.xml", new File(context.getDataDir(), "shared_prefs/settings.xml"));
                for (File run : runs) putFile(zip, "logs/" + run.getName(), run);
                put(zip, "logcat.txt", logcat());
            }
        }
        return runs.length;
    }

    private static String device(Context context, String version, int runs) {
        return "NSMBW port " + version + " (com.wither.nsmbwport)\n"
                + "Device: " + Build.MANUFACTURER + " " + Build.MODEL + " (" + Build.DEVICE + ")\n"
                + "Android " + Build.VERSION.RELEASE + " (API " + Build.VERSION.SDK_INT + ")\n"
                + "Chip: " + Build.HARDWARE + (Build.VERSION.SDK_INT >= 31 ? " (" + Build.SOC_MANUFACTURER + " " + Build.SOC_MODEL + ")" : "")
                + ", ABIs " + String.join(" ", Build.SUPPORTED_ABIS) + "\n"
                + "GPU driver: " + (GpuDrivers.picked(context) != null ? GpuDrivers.picked(context).name : "Default") + "\n"
                + "Runs included: " + runs + " (newest first)\n";
    }

    /** What logcat still holds of this app (Android shows an app only its own lines). */
    private static byte[] logcat() {
        try {
            final Process p = new ProcessBuilder("logcat", "-d", "-v", "threadtime", "-t", "20000").redirectErrorStream(true).start();
            final java.io.ByteArrayOutputStream out = new java.io.ByteArrayOutputStream();
            try (InputStream in = p.getInputStream()) {
                GameFiles.copy(in, out);
            }
            return out.toByteArray();
        } catch (IOException e) {
            return ("logcat: " + e.getMessage() + "\n").getBytes(StandardCharsets.UTF_8);
        }
    }

    private static void put(ZipOutputStream zip, String name, byte[] bytes) throws IOException {
        zip.putNextEntry(new ZipEntry(name));
        zip.write(bytes);
        zip.closeEntry();
    }

    private static void putFile(ZipOutputStream zip, String name, File file) throws IOException {
        if (!file.isFile()) return;
        zip.putNextEntry(new ZipEntry(name));
        try (InputStream in = new FileInputStream(file)) {
            final long skip = file.length() - MAX_BYTES;
            if (skip > 0) {
                zip.write(("[... the first " + skip + " bytes are left out ...]\n").getBytes(StandardCharsets.UTF_8));
                long left = skip;
                while (left > 0) {
                    final long n = in.skip(left);
                    if (n <= 0) break;
                    left -= n;
                }
            }
            GameFiles.copy(in, zip);
        }
        zip.closeEntry();
    }
}
