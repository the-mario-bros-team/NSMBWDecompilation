package com.wither.nsmbwport;

import android.app.Activity;
import android.app.AlertDialog;
import android.content.Intent;
import android.content.pm.PackageInfo;
import android.content.pm.PackageManager;
import android.content.res.ColorStateList;
import android.graphics.Color;
import android.graphics.Typeface;
import android.graphics.drawable.GradientDrawable;
import android.graphics.drawable.RippleDrawable;
import android.net.Uri;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.util.TypedValue;
import android.view.Gravity;
import android.view.View;
import android.widget.Button;
import android.widget.LinearLayout;
import android.widget.ProgressBar;
import android.widget.ScrollView;
import android.widget.TextView;

import java.util.Locale;

/**
 * Start screen (NSMB-Wii-Recompilation's, trimmed to what this port has): Play, the language, Help,
 * Check for Updates and Auto Update, Settings, Edit Touch Controls, Skip Start Menu, Xiaomi Mode, GPU
 * Driver, importing the player's own game the way Dolphin extracts it (the folder, or a zip of it),
 * and Download Logs.
 */
public final class LauncherActivity extends Activity {
    private static final int REQUEST_IMPORT = 1;
    private static final int REQUEST_ZIP = 2;
    private static final int REQUEST_IMAGE = 3;
    private static final int REQUEST_LOGS = 4;
    private static final int REQUEST_GPU_DRIVER = 5;
    /** Set by the game's "Start Menu": show this screen even with Skip Start Menu on. */
    static final String EXTRA_SHOW_START_MENU = "showStartMenu";

    private AppSettings settings;
    private TextView skip;
    private TextView status;
    private Button play;
    private Button lanPlay;
    private View steps;
    private boolean incompleteShown;
    private final Handler main = new Handler(Looper.getMainLooper());
    private TextView autoUpdate;
    private TextView mainProcess;
    private TextView gpuDriver;
    /** Skip Start Menu with Auto Update on: the game opens once the check finds nothing to install. */
    private boolean openGameAfterCheck;
    /** An update waiting for the player to allow installs from this app. */
    private Updater.Release waitingUpdate;
    /** An update is being offered or installed: a later check doesn't start another. */
    private boolean updating;
    /** When the app last looked for an update (elapsedRealtime), or -1: Auto Update looks again when the
     *  player comes back after a while. */
    private static long lastCheck = -1;
    private static final long CHECK_AGAIN_MS = 5 * 60 * 1000;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        settings = new AppSettings(this);
        final Intent launch = getIntent();
        final boolean asked = launch != null && launch.getBooleanExtra(EXTRA_SHOW_START_MENU, false);
        if (savedInstanceState == null && !asked && settings.skipStartMenu() && GameFiles.gameReady(this)) {
            if (!settings.autoUpdate()) {
                startActivity(GameActivity.intent(this));
                overridePendingTransition(0, 0);
                finish();
                return;
            }
            openGameAfterCheck = true;  // Auto Update first; the game opens once there is nothing to install
        }
        Lang.load(this);
        final int pad = dp(20);
        final int screenWidth = getResources().getDisplayMetrics().widthPixels;
        // Landscape (or a tablet): title, status and Play on the left, the rest beside them.
        final boolean twoPanes = screenWidth >= dp(720);

        final LinearLayout intro = new LinearLayout(this);
        intro.setOrientation(LinearLayout.VERTICAL);
        final LinearLayout header = new LinearLayout(this);
        header.setOrientation(LinearLayout.HORIZONTAL);
        header.setGravity(Gravity.CENTER_VERTICAL);
        final TextView title = text("NSMBW", 34);
        title.setTypeface(Typeface.DEFAULT_BOLD);
        title.setGravity(Gravity.START);
        header.addView(title, new LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f));
        final TextView language = text(Lang.NAMES[Lang.index()], 15);
        language.setTypeface(MEDIUM);
        language.setPadding(dp(14), dp(8), dp(18), dp(8));
        language.setBackground(rounded(Color.TRANSPARENT, Color.TRANSPARENT, dp(18), STROKE));
        language.setCompoundDrawablesRelativeWithIntrinsicBounds(R.drawable.ic_language, 0, 0, 0);
        language.setCompoundDrawablePadding(dp(6));
        language.setContentDescription(Lang.t("Language"));
        language.setOnClickListener(v -> chooseLanguage());
        final LinearLayout.LayoutParams languageParams = new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.WRAP_CONTENT, LinearLayout.LayoutParams.WRAP_CONTENT);
        languageParams.rightMargin = dp(8);
        header.addView(language, languageParams);
        final TextView help = text(Lang.t("Help"), 15);
        help.setTypeface(MEDIUM);
        help.setPadding(dp(18), dp(8), dp(18), dp(8));
        help.setBackground(rounded(Color.TRANSPARENT, Color.TRANSPARENT, dp(18), STROKE));
        help.setOnClickListener(v -> showHelp());
        header.addView(help);
        intro.addView(header, matchWidth(0));
        final TextView subtitle = text(Lang.t("New Super Mario Bros. Wii, the Rust port"), 15);
        subtitle.setGravity(Gravity.START);
        subtitle.setTextColor(MUTED);
        intro.addView(subtitle, matchWidth(dp(2)));

        status = text("", 15);
        status.setGravity(Gravity.START | Gravity.CENTER_VERTICAL);
        status.setCompoundDrawablePadding(dp(10));
        status.setPadding(dp(16), dp(12), dp(16), dp(12));
        status.setBackground(rounded(SURFACE, SURFACE, dp(16), 0));
        intro.addView(status, matchWidth(dp(20)));

        play = button(Lang.t("Play"), true);
        play.setOnClickListener(v -> startActivity(GameActivity.intent(this)));
        intro.addView(play, matchWidth(dp(14)));
        lanPlay = button(Lang.t("LAN Play"), false);
        lanPlay.setOnClickListener(v -> LanPlay.show(this));
        intro.addView(lanPlay, matchWidth(dp(10)));

        final LinearLayout updates = card(Lang.t("Updates"));
        final Button check = button(Lang.t("Check for Updates"), false);
        check.setOnClickListener(v -> checkForUpdates(false));
        updates.addView(check, matchWidth(dp(10)));
        autoUpdate = toggle(updates, Lang.t("Auto Update"), Lang.t("On: opening the app installs the latest release from "
                + "GitHub when there is one. Your saves, settings and game files are kept."), v -> {
                    settings.setAutoUpdate(!settings.autoUpdate());
                    showToggle(autoUpdate, settings.autoUpdate());
                });
        showToggle(autoUpdate, settings.autoUpdate());

        final LinearLayout options = card(Lang.t("Options"));
        final Button settingsButton = button(Lang.t("Settings"), false);
        settingsButton.setOnClickListener(v -> SettingsActivity.open(this));
        options.addView(settingsButton, matchWidth(dp(10)));
        final Button touchLayout = button(Lang.t("Edit Touch Controls"), false);
        touchLayout.setOnClickListener(v -> startActivity(new Intent(this, TouchEditorActivity.class)));
        options.addView(touchLayout, matchWidth(dp(10)));
        skip = toggle(options, Lang.t("Skip Start Menu"), Lang.t("On: opening the app goes straight into the game. "
                + "In the game, the phone's Back gesture and Start Menu bring you back here."), v -> {
                    settings.setSkipStartMenu(!settings.skipStartMenu());
                    showToggle(skip, settings.skipStartMenu());
                });
        showToggle(skip, settings.skipStartMenu());
        mainProcess = toggle(options, Lang.t("Xiaomi Mode"), Lang.t("On: the game runs in the app's main process. For "
                + "Xiaomi, Redmi and POCO phones (on by default there), whose system closes the game's own "
                + "process and leaves a black screen."), v -> {
                    settings.setGameInMainProcess(!settings.gameInMainProcess());
                    showToggle(mainProcess, settings.gameInMainProcess());
                });
        showToggle(mainProcess, settings.gameInMainProcess());
        final boolean snapdragon = GpuDrivers.supported();
        gpuDriver = toggle(options, Lang.t("GPU Driver"), Lang.t(snapdragon
                ? "Run the game on a driver from a zip, such as Turnip, in place of the phone's own. If a new "
                + "driver stops the game from starting, Default comes back by itself."
                : "Only for phones with a Snapdragon chip."), v -> chooseGpuDriver());
        gpuDriver.setSingleLine(true);
        gpuDriver.setEllipsize(android.text.TextUtils.TruncateAt.END);
        gpuDriver.setMaxWidth(dp(180));
        if (!snapdragon) {
            final View row = (View) gpuDriver.getParent();
            row.setOnClickListener(null);
            row.setClickable(false);
            row.setAlpha(0.4f);
        }
        showGpuDriver();

        final LinearLayout gameData = card(Lang.t("Game Data"));
        final LinearLayout stepsBox = new LinearLayout(this);
        stepsBox.setOrientation(LinearLayout.VERTICAL);
        final TextView stepsTitle = text(Lang.t("Get the game files (once)"), 18);
        stepsTitle.setTypeface(Typeface.DEFAULT_BOLD);
        stepsTitle.setGravity(Gravity.START);
        stepsBox.addView(stepsTitle);
        final TextView stepsText = text(Lang.t(
                "1. In Dolphin on a computer, right-click New Super Mario Bros. Wii and choose Properties.\n"
                + "2. Open Filesystem, right-click the disc at the top and choose Extract Entire Disc.\n"
                + "3. Copy that folder (or a zip of it) to this phone.\n"
                + "4. Tap Import from Extracted Game Data Folder… or Import Game Data Zip… below and choose it.\n\n"
                + "Use the USA disc, revision 1 (SMNE01): Dolphin shows the revision under Properties > Info."), 15);
        stepsText.setGravity(Gravity.START);
        stepsText.setPadding(0, dp(6), 0, dp(14));
        stepsText.setLineSpacing(dp(3), 1f);
        stepsBox.addView(stepsText);
        steps = stepsBox;
        gameData.addView(stepsBox, matchWidth(dp(10)));
        final TextView importHelp = text(Lang.t("Import your own New Super Mario Bros. Wii "
                + "(USA, SMNE01) once. Easiest: the folder Dolphin's Extract Entire Disc makes (DATA, or the folder "
                + "that holds it), or a zip of that folder. Your saves are kept."), 13);
        importHelp.setGravity(Gravity.START);
        importHelp.setTextColor(MUTED);
        gameData.addView(importHelp, matchWidth(dp(8)));
        final Button importFolder = button(Lang.t("Import from Extracted Game Data Folder…"), false);
        importFolder.setOnClickListener(v -> startActivityForResult(new Intent(Intent.ACTION_OPEN_DOCUMENT_TREE), REQUEST_IMPORT));
        gameData.addView(importFolder, matchWidth(dp(12)));
        final Button importZip = button(Lang.t("Import Game Data Zip…"), false);
        importZip.setOnClickListener(v -> pickZip());
        gameData.addView(importZip, matchWidth(dp(10)));
        final Button importImage = button(Lang.t("Import Disc Image (ISO, WBFS, RVZ)…"), false);
        importImage.setOnClickListener(v -> pickImage());
        gameData.addView(importImage, matchWidth(dp(10)));
        final TextView imageHelp = text(Lang.t("Or import your disc image straight away (.iso, .wbfs, .rvz or .wia): "
                + "it is extracted once."), 13);
        imageHelp.setGravity(Gravity.START);
        imageHelp.setTextColor(MUTED);
        gameData.addView(imageHelp, matchWidth(dp(6)));

        final Button downloadLogs = button(Lang.t("Download Logs (For Github Reports)"), false);
        downloadLogs.setOnClickListener(v -> LogExport.pickDestination(this, REQUEST_LOGS));

        final TextView version = text("NSMBW " + versionText(), 13);
        version.setTextColor(FAINT);

        final LinearLayout page = new LinearLayout(this);
        page.setPadding(pad, dp(16), pad, pad);
        if (twoPanes) {
            page.setOrientation(LinearLayout.HORIZONTAL);
            page.setGravity(Gravity.CENTER_HORIZONTAL);
            final int left = dp(340);
            final int right = Math.min(dp(500), screenWidth - 2 * pad - left - dp(24));
            version.setGravity(Gravity.START);
            intro.addView(version, matchWidth(dp(16)));
            page.addView(intro, new LinearLayout.LayoutParams(left, LinearLayout.LayoutParams.WRAP_CONTENT));
            final LinearLayout cards = new LinearLayout(this);
            cards.setOrientation(LinearLayout.VERTICAL);
            cards.addView(updates, matchWidth(0));
            cards.addView(options, matchWidth(dp(14)));
            cards.addView(gameData, matchWidth(dp(14)));
            cards.addView(downloadLogs, matchWidth(dp(14)));
            final LinearLayout.LayoutParams cardsParams = new LinearLayout.LayoutParams(right, LinearLayout.LayoutParams.WRAP_CONTENT);
            cardsParams.leftMargin = dp(24);
            page.addView(cards, cardsParams);
        } else {
            page.setOrientation(LinearLayout.VERTICAL);
            page.setGravity(Gravity.CENTER_HORIZONTAL);
            final int width = Math.min(dp(520), screenWidth - 2 * pad);
            final LinearLayout column = new LinearLayout(this);
            column.setOrientation(LinearLayout.VERTICAL);
            column.addView(intro, matchWidth(0));
            column.addView(updates, matchWidth(dp(20)));
            column.addView(options, matchWidth(dp(14)));
            column.addView(gameData, matchWidth(dp(14)));
            column.addView(downloadLogs, matchWidth(dp(14)));
            column.addView(version, matchWidth(dp(18)));
            page.addView(column, new LinearLayout.LayoutParams(width, LinearLayout.LayoutParams.WRAP_CONTENT));
        }
        final ScrollView scroll = new ScrollView(this);
        scroll.setFillViewport(true);
        scroll.addView(page);
        setContentView(scroll);
        if (settings.autoUpdate() && savedInstanceState == null) checkForUpdates(true);
    }

    @Override
    protected void onResume() {
        super.onResume();
        if (status == null) return;  // skipped straight into the game
        // The settings' Language tab may have changed the language since this screen was drawn.
        final int shown = Lang.index();
        Lang.load(this);
        if (Lang.index() != shown) {
            recreate();
            return;
        }
        refresh();
        showGpuDriver();
        if (GpuDrivers.takeFailure(this)) {
            new AlertDialog.Builder(this).setTitle(Lang.t("GPU Driver"))
                    .setMessage(Lang.t("The game closed right after starting on the new GPU driver, so it's back on "
                            + "Default. You can pick that driver again under GPU Driver."))
                    .setPositiveButton(Lang.t("OK"), null).show();
        }
        // Back from "Allow from this source": carry on with the update that waited for it.
        if (waitingUpdate != null) {
            final Updater.Release release = waitingUpdate;
            waitingUpdate = null;
            if (Updater.canInstall(this)) {
                install(release);
            } else {
                updating = false;
            }
        } else if (settings.autoUpdate() && !updating && lastCheck >= 0
                && android.os.SystemClock.elapsedRealtime() - lastCheck >= CHECK_AGAIN_MS) {
            // Android often keeps the app in memory: coming back looks again after a while.
            checkForUpdates(true);
        }
    }

    // --- updates -------------------------------------------------------------------------------

    /** Check for Updates (automatic: Auto Update, which says nothing unless there is one to install). */
    private void checkForUpdates(boolean automatic) {
        if (updating) return;
        lastCheck = android.os.SystemClock.elapsedRealtime();
        final AlertDialog checking = automatic ? null : new AlertDialog.Builder(this)
                .setMessage(Lang.t("Checking for updates…")).setCancelable(false).show();
        new Thread(() -> {
            Updater.Release release = null;
            String problem = null;
            try {
                release = Updater.latest(this);
            } catch (Exception e) {
                problem = e.getMessage();
            }
            final Updater.Release found = release;
            final String error = problem;
            main.post(() -> {
                if (checking != null) checking.dismiss();
                if (isFinishing()) return;
                final String installed = Updater.installedVersion(this);
                if (found != null && Updater.isNewer(found.version, installed)) {
                    offerUpdate(found, automatic);
                    return;
                }
                if (openGameAfterCheck) {
                    openGameAfterCheck = false;
                    startActivity(GameActivity.intent(this));
                    finish();
                    return;
                }
                if (automatic) return;
                final String message = error != null ? Lang.f("Couldn't check for updates: %s", error)
                        : found == null ? Lang.t("The latest release has no Android download.")
                        : Lang.f("You have the latest version (%s).", installed);
                new AlertDialog.Builder(this).setMessage(message).setPositiveButton(Lang.t("OK"), null).show();
            });
        }, "update-check").start();
    }

    /** A newer release: Auto Update installs it, Check for Updates asks first. */
    private void offerUpdate(Updater.Release release, boolean automatic) {
        updating = true;
        openGameAfterCheck = false;
        if (automatic) {
            install(release);
            return;
        }
        new AlertDialog.Builder(this)
                .setTitle(Lang.f("NSMBW %s is available", release.version))
                .setMessage(Lang.t("Your saves, settings and game files are kept."))
                .setPositiveButton(Lang.t("Update"), (d, w) -> install(release))
                .setNegativeButton(Lang.t("Not Now"), (d, w) -> updating = false)
                .setOnCancelListener(d -> updating = false)
                .show();
    }

    /** Downloads, checks and hands `release` to the system installer, after the install permission. */
    private void install(Updater.Release release) {
        if (!Updater.canInstall(this)) {
            new AlertDialog.Builder(this)
                    .setMessage(Lang.t("To install updates, let NSMBW install apps: turn on Allow from this source on "
                            + "the next screen, then come back here."))
                    .setPositiveButton(Lang.t("Open Settings"), (d, w) -> {
                        waitingUpdate = release;
                        startActivity(new Intent(android.provider.Settings.ACTION_MANAGE_UNKNOWN_APP_SOURCES,
                                Uri.parse("package:" + getPackageName())));
                    })
                    .setNegativeButton(Lang.t("Not Now"), (d, w) -> updating = false)
                    .setOnCancelListener(d -> updating = false)
                    .show();
            return;
        }
        final ProgressBar bar = new ProgressBar(this, null, android.R.attr.progressBarStyleHorizontal);
        final TextView label = text(Lang.t("Downloading…"), 14);
        final LinearLayout box = new LinearLayout(this);
        box.setOrientation(LinearLayout.VERTICAL);
        box.setPadding(dp(24), dp(16), dp(24), dp(8));
        box.addView(bar);
        box.addView(label);
        final AlertDialog dialog = new AlertDialog.Builder(this)
                .setTitle(Lang.f("Updating to NSMBW %s", release.version)).setView(box).setCancelable(false).show();
        new Thread(() -> {
            String error = null;
            try {
                Updater.downloadAndInstall(getApplicationContext(), release, (done, total) -> main.post(() -> {
                    bar.setIndeterminate(total <= 0);
                    if (total > 0) {
                        bar.setMax(1000);
                        bar.setProgress((int) (done * 1000 / total));
                        label.setText(Lang.f("Downloading… %d of %d MB", done >> 20, total >> 20));
                    }
                }));
            } catch (Exception e) {
                error = e.getMessage();
            }
            final String failed = error;
            main.post(() -> {
                if (failed == null) {
                    // The system takes over (its confirmation, then the new version).
                    bar.setIndeterminate(true);
                    label.setText(Lang.f("Installing NSMBW %s…", release.version));
                    main.postDelayed(() -> {
                        if (dialog.isShowing()) dialog.dismiss();
                        updating = false;
                    }, 4000);
                    return;
                }
                dialog.dismiss();
                updating = false;
                new AlertDialog.Builder(this).setMessage(Lang.f("The update failed: %s", failed))
                        .setPositiveButton(Lang.t("OK"), null).show();
            });
        }, "update-download").start();
    }

    // --- GPU driver ----------------------------------------------------------------------------

    private void showGpuDriver() {
        if (gpuDriver == null) return;
        final GpuDrivers.Driver picked = GpuDrivers.supported() ? GpuDrivers.picked(this) : null;
        gpuDriver.setText(picked != null ? picked.name : Lang.t("Default"));
        gpuDriver.setTextColor(picked != null ? Color.WHITE : MUTED);
        final GradientDrawable shape = new GradientDrawable();
        shape.setCornerRadius(dp(17));
        shape.setColor(picked != null ? ACCENT : SURFACE);
        gpuDriver.setBackground(shape);
    }

    /** Default, the imported drivers, Import… and Delete…. */
    private void chooseGpuDriver() {
        final java.util.List<GpuDrivers.Driver> drivers = GpuDrivers.list(this);
        final GpuDrivers.Driver picked = GpuDrivers.picked(this);
        final java.util.List<String> labels = new java.util.ArrayList<>();
        labels.add(Lang.t("Default"));
        int checked = 0;
        for (GpuDrivers.Driver d : drivers) {
            if (picked != null && picked.id.equals(d.id)) checked = labels.size();
            labels.add(d.name);
        }
        labels.add(Lang.t("Import…"));
        if (!drivers.isEmpty()) labels.add(Lang.t("Delete…"));
        new AlertDialog.Builder(this)
                .setTitle(Lang.t("GPU Driver"))
                .setSingleChoiceItems(labels.toArray(new String[0]), checked, (dialog, which) -> {
                    dialog.dismiss();
                    if (which == drivers.size() + 1) {
                        final Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT).addCategory(Intent.CATEGORY_OPENABLE).setType("*/*")
                                .putExtra(Intent.EXTRA_MIME_TYPES, new String[] {"application/zip", "application/x-zip-compressed", "application/x-zip"});
                        startActivityForResult(intent, REQUEST_GPU_DRIVER);
                    } else if (which == drivers.size() + 2) {
                        deleteGpuDriver(drivers);
                    } else {
                        try {
                            GpuDrivers.pick(this, which == 0 ? null : drivers.get(which - 1));
                        } catch (java.io.IOException e) {
                            new AlertDialog.Builder(this).setMessage(e.getMessage()).setPositiveButton(Lang.t("OK"), null).show();
                        }
                        showGpuDriver();
                    }
                })
                .setNegativeButton(Lang.t("Cancel"), null)
                .show();
    }

    private void deleteGpuDriver(java.util.List<GpuDrivers.Driver> drivers) {
        final String[] names = new String[drivers.size()];
        for (int i = 0; i < names.length; ++i) names[i] = drivers.get(i).name;
        new AlertDialog.Builder(this)
                .setTitle(Lang.t("Delete a GPU driver"))
                .setItems(names, (dialog, which) -> {
                    try {
                        GpuDrivers.delete(this, drivers.get(which));
                    } catch (java.io.IOException e) {
                        new AlertDialog.Builder(this).setMessage(e.getMessage()).setPositiveButton(Lang.t("OK"), null).show();
                    }
                    showGpuDriver();
                })
                .setNegativeButton(Lang.t("Cancel"), null)
                .show();
    }

    /** `job` on a worker thread under a spinner titled `title`; then its message, or its error's. */
    private void runInBackground(String title, java.util.concurrent.Callable<String> job) {
        final ProgressBar spinner = new ProgressBar(this);
        spinner.setPadding(0, dp(16), 0, dp(16));
        final AlertDialog dialog = new AlertDialog.Builder(this).setTitle(title).setView(spinner).setCancelable(false).show();
        new Thread(() -> {
            String message;
            try {
                message = job.call();
            } catch (Exception e) {
                message = e.getMessage();
            }
            final String shown = message;
            main.post(() -> {
                dialog.dismiss();
                showGpuDriver();
                new AlertDialog.Builder(this).setMessage(shown).setPositiveButton(Lang.t("OK"), null).show();
            });
        }, "background").start();
    }

    private void refresh() {
        final boolean present = GameFiles.gameDir(this).isDirectory();
        final String problem = present ? GameFiles.gameProblem(GameFiles.gameDir(this)) : null;
        if (problem != null && problem.equals(Lang.t(GameFiles.INCOMPLETE)) && !incompleteShown) {
            incompleteShown = true;
            new AlertDialog.Builder(this).setMessage(problem).setPositiveButton(Lang.t("OK"), null).show();
        }
        if (!present) {
            showStatus(Lang.t("No game data yet. Import the folder Dolphin made, or a zip of it."), AMBER);
        } else if (problem != null) {
            showStatus(Lang.f("The imported game can't be played. %s", problem), ACCENT);
        } else {
            showStatus(Lang.t("Game data is installed and checked."), GREEN);
        }
        play.setEnabled(present && problem == null);
        lanPlay.setEnabled(present && problem == null);
        steps.setVisibility(present && problem == null ? View.GONE : View.VISIBLE);
    }

    /** Getting Started. */
    private void showHelp() {
        final LinearLayout column = new LinearLayout(this);
        column.setOrientation(LinearLayout.VERTICAL);
        column.setPadding(dp(24), dp(16), dp(24), dp(16));
        section(column, Lang.t("1. Import New Super Mario Bros. Wii"), Lang.t("Use your own USA New Super Mario Bros. Wii "
                + "(SMNE01, revision 1). Easiest: in Dolphin on a computer, right-click the game, choose Properties, "
                + "Filesystem, then right-click the disc and choose Extract Entire Disc. Copy that folder (or a zip of "
                + "it) to this phone and tap Import. Dolphin shows the revision under Properties > Info."));
        section(column, Lang.t("2. Play"), Lang.t("Tap Play. The on-screen Wii Remote is held sideways; turn the "
                + "phone to tilt and jolt it to shake. The Back gesture opens the game's menu."));
        final TextView version = text("NSMBW " + versionText(), 13);
        version.setGravity(Gravity.START);
        version.setTextColor(MUTED);
        version.setPadding(0, dp(16), 0, 0);
        column.addView(version);
        final ScrollView scroll = new ScrollView(this);
        scroll.addView(column);
        new AlertDialog.Builder(this).setTitle(Lang.t("Getting Started")).setView(scroll)
                .setPositiveButton(Lang.t("Done"), null).show();
    }

    /** The Language button: sets the game's language and redraws this screen in it. */
    private void chooseLanguage() {
        new AlertDialog.Builder(this)
                .setTitle(Lang.t("Language"))
                .setSingleChoiceItems(Lang.NAMES, Lang.index(), (dialog, which) -> {
                    dialog.dismiss();
                    if (which != Lang.index()) {
                        Lang.set(this, which);
                        recreate();
                    }
                })
                .setNegativeButton(Lang.t("Cancel"), null)
                .show();
    }

    private void section(LinearLayout column, String title, String body) {
        final TextView heading = text(title, 17);
        heading.setTypeface(Typeface.DEFAULT_BOLD);
        heading.setGravity(Gravity.START);
        heading.setPadding(0, dp(12), 0, dp(4));
        column.addView(heading);
        final TextView text = text(body, 15);
        text.setGravity(Gravity.START);
        text.setTextColor(MUTED);
        column.addView(text);
    }

    private String versionText() {
        try {
            final PackageInfo info = getPackageManager().getPackageInfo(getPackageName(), 0);
            return info.versionName;
        } catch (PackageManager.NameNotFoundException e) {
            return "";
        }
    }

    private void pickZip() {
        final Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT);
        intent.addCategory(Intent.CATEGORY_OPENABLE);
        intent.setType("*/*");
        intent.putExtra(Intent.EXTRA_MIME_TYPES, new String[] {"application/zip", "application/x-zip-compressed", "application/x-zip"});
        startActivityForResult(intent, REQUEST_ZIP);
    }

    private void pickImage() {
        final Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT);
        intent.addCategory(Intent.CATEGORY_OPENABLE);
        // .wbfs, .rvz and .wia have no type of their own: any file, checked by its header.
        intent.setType("*/*");
        startActivityForResult(intent, REQUEST_IMAGE);
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (resultCode != RESULT_OK || data == null || data.getData() == null) return;
        final Uri uri = data.getData();
        if (requestCode == REQUEST_LOGS) {
            final String version = versionText();
            runInBackground(Lang.t("Download Logs (For Github Reports)"), () -> {
                try {
                    final int runs = LogExport.write(this, uri, version);
                    return runs == 0 ? Lang.t("Saved, but there are no game logs yet: play once, then download them again.")
                            : runs == 1 ? Lang.t("Saved the logs of the last game session. Attach the zip to your GitHub issue.")
                            : Lang.f("Saved the logs of the last %d game sessions. Attach the zip to your GitHub issue.", runs);
                } catch (java.io.IOException e) {
                    return Lang.f("Could not save the logs: %s", e.getMessage());
                }
            });
        } else if (requestCode == REQUEST_GPU_DRIVER) {
            runInBackground(Lang.t("Importing the GPU driver"),
                    () -> Lang.f("%s is ready. The game uses it from its next start.", GpuDrivers.importZip(this, uri).name));
        } else if (requestCode == REQUEST_IMPORT) {
            runImport(progress -> GameFiles.copyFolder(this, uri, progress));
        } else if (requestCode == REQUEST_ZIP) {
            runImport(progress -> GameFiles.copyZip(this, uri, progress));
        } else if (requestCode == REQUEST_IMAGE) {
            runImport(progress -> GameFiles.copyImage(this, uri, progress));
        }
    }

    private interface ImportJob {
        String run(GameFiles.Progress progress) throws Exception;
    }

    /** `job` on a worker thread under a progress dialog, then its message. */
    private void runImport(ImportJob job) {
        final ProgressBar bar = new ProgressBar(this, null, android.R.attr.progressBarStyleHorizontal);
        final TextView label = text(Lang.t("Looking at the game files…"), 14);
        final LinearLayout box = new LinearLayout(this);
        box.setOrientation(LinearLayout.VERTICAL);
        box.setPadding(dp(24), dp(16), dp(24), dp(8));
        box.addView(bar);
        box.addView(label);
        final AlertDialog dialog = new AlertDialog.Builder(this)
                .setTitle(Lang.t("Importing game data")).setView(box).setCancelable(false).show();
        new Thread(() -> {
            String result;
            try {
                result = job.run((done, total, text) -> main.post(() -> {
                    bar.setIndeterminate(total <= 0);
                    if (total > 0) {
                        bar.setMax(total);
                        bar.setProgress(done);
                    }
                    label.setText(text);
                }));
            } catch (Exception e) {
                GameFiles.abandonImport(this);
                result = Lang.f("%s\n\nNothing was changed.", Lang.f("Import failed: %s", e.getMessage()));
            }
            final String message = result;
            main.post(() -> {
                dialog.dismiss();
                if (message.equals(Lang.t(GameFiles.INCOMPLETE))) incompleteShown = true;
                refresh();
                new AlertDialog.Builder(this).setMessage(message).setPositiveButton(Lang.t("OK"), null).show();
            });
        }, "game-import").start();
    }

    // --- views ---------------------------------------------------------------------------------

    private TextView text(String value, int sp) {
        final TextView view = new TextView(this);
        view.setText(value);
        view.setTextSize(TypedValue.COMPLEX_UNIT_SP, sp);
        view.setTextColor(Color.WHITE);
        view.setGravity(Gravity.CENTER_HORIZONTAL);
        return view;
    }

    // The start menu's colors, on the theme's #101418 background.
    private static final int ACCENT = Color.rgb(229, 57, 53);
    private static final int SURFACE = Color.rgb(24, 31, 38);
    private static final int RAISED = Color.rgb(38, 48, 58);
    private static final int STROKE = Color.rgb(48, 60, 72);
    private static final int MUTED = Color.rgb(170, 178, 186);
    private static final int FAINT = Color.rgb(120, 128, 136);
    private static final int GREEN = Color.rgb(102, 187, 106);
    private static final int AMBER = Color.rgb(255, 183, 77);
    private static final Typeface MEDIUM = Typeface.create("sans-serif-medium", Typeface.NORMAL);

    /** A rounded shape that ripples when touched; `disabled` is its color while disabled. */
    private RippleDrawable rounded(int color, int disabled, int radius, int stroke) {
        final GradientDrawable shape = new GradientDrawable();
        shape.setCornerRadius(radius);
        shape.setColor(new ColorStateList(new int[][] {{-android.R.attr.state_enabled}, {}}, new int[] {disabled, color}));
        if (stroke != 0) shape.setStroke(dp(1), stroke);
        final GradientDrawable mask = new GradientDrawable();
        mask.setCornerRadius(radius);
        mask.setColor(Color.WHITE);
        return new RippleDrawable(ColorStateList.valueOf(Color.argb(56, 255, 255, 255)), shape, mask);
    }

    /** A pill button: `primary` is the big red Play. */
    private Button button(String label, boolean primary) {
        final Button view = new Button(this);
        view.setText(label);
        view.setAllCaps(false);
        view.setStateListAnimator(null);
        view.setTextSize(TypedValue.COMPLEX_UNIT_SP, primary ? 20 : 16);
        view.setTypeface(primary ? Typeface.DEFAULT_BOLD : MEDIUM);
        view.setTextColor(new ColorStateList(new int[][] {{-android.R.attr.state_enabled}, {}}, new int[] {FAINT, Color.WHITE}));
        final int height = dp(primary ? 60 : 50);
        view.setMinHeight(height);
        view.setMinimumHeight(height);
        view.setPadding(dp(20), 0, dp(20), 0);
        view.setBackground(primary ? rounded(ACCENT, RAISED, height / 2, 0) : rounded(RAISED, SURFACE, height / 2, 0));
        return view;
    }

    /** A rounded section of the start menu, with a small heading. */
    private LinearLayout card(String heading) {
        final LinearLayout card = new LinearLayout(this);
        card.setOrientation(LinearLayout.VERTICAL);
        card.setPadding(dp(16), dp(14), dp(16), dp(16));
        final GradientDrawable shape = new GradientDrawable();
        shape.setCornerRadius(dp(22));
        shape.setColor(SURFACE);
        shape.setStroke(dp(1), Color.rgb(34, 44, 53));
        card.setBackground(shape);
        final TextView title = text(heading.toUpperCase(Locale.ROOT), 12);
        title.setGravity(Gravity.START);
        title.setTypeface(Typeface.DEFAULT_BOLD);
        title.setLetterSpacing(0.12f);
        title.setTextColor(MUTED);
        card.addView(title, matchWidth(0));
        return card;
    }

    /** A setting row in `card`: its label, an On/Off pill (returned) and what On does underneath. */
    private TextView toggle(LinearLayout card, String label, String help, View.OnClickListener onClick) {
        final LinearLayout row = new LinearLayout(this);
        row.setOrientation(LinearLayout.HORIZONTAL);
        row.setGravity(Gravity.CENTER_VERTICAL);
        row.setMinimumHeight(dp(50));
        row.setPadding(dp(20), dp(6), dp(8), dp(6));
        row.setBackground(rounded(RAISED, RAISED, dp(25), 0));
        row.setClickable(true);
        row.setOnClickListener(onClick);
        final TextView name = text(label, 16);
        name.setTypeface(MEDIUM);
        name.setGravity(Gravity.START);
        row.addView(name, new LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f));
        final TextView pill = text("", 14);
        pill.setTypeface(Typeface.DEFAULT_BOLD);
        pill.setGravity(Gravity.CENTER);
        pill.setMinWidth(dp(64));
        pill.setPadding(dp(14), dp(7), dp(14), dp(7));
        row.addView(pill);
        card.addView(row, matchWidth(dp(10)));
        final TextView about = text(help, 13);
        about.setGravity(Gravity.START);
        about.setTextColor(MUTED);
        about.setPadding(dp(4), dp(6), dp(4), 0);
        card.addView(about, matchWidth(0));
        return pill;
    }

    private void showToggle(TextView pill, boolean on) {
        pill.setText(Lang.t(on ? "On" : "Off"));
        pill.setTextColor(on ? Color.WHITE : MUTED);
        final GradientDrawable shape = new GradientDrawable();
        shape.setCornerRadius(dp(17));
        shape.setColor(on ? ACCENT : SURFACE);
        pill.setBackground(shape);
    }

    private void showStatus(String text, int color) {
        status.setText(text);
        final GradientDrawable dot = new GradientDrawable();
        dot.setShape(GradientDrawable.OVAL);
        dot.setColor(color);
        dot.setSize(dp(10), dp(10));
        status.setCompoundDrawablesRelativeWithIntrinsicBounds(dot, null, null, null);
    }

    private LinearLayout.LayoutParams matchWidth(int topMargin) {
        final LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT);
        params.topMargin = topMargin;
        return params;
    }

    private int dp(int value) {
        return Math.round(value * getResources().getDisplayMetrics().density);
    }
}
