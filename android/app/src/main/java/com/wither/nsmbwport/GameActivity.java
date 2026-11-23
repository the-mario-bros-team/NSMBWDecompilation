package com.wither.nsmbwport;

import android.app.Activity;
import android.app.AlertDialog;
import android.content.Context;
import android.content.Intent;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.os.Process;
import android.util.Log;
import android.view.KeyEvent;
import android.view.MotionEvent;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
import android.view.View;
import android.view.ViewGroup;
import android.view.WindowInsets;
import android.view.WindowInsetsController;
import android.view.WindowManager;
import android.widget.FrameLayout;

/**
 * The game: libnsmbw.so drawing with Vulkan into a SurfaceView, with NSMB-Wii-Recompilation's
 * on-screen Wii Remote and the phone's motion on top, controllers as Wii Remotes, the Back gesture's
 * menu, and the game paused while the app is away (after writing a backup: if Android ends the app
 * meanwhile, the next start carries on from it).
 */
public class GameActivity extends Activity
        implements TouchControlsView.Listener, MotionInput.Listener, SurfaceHolder.Callback {
    private static final String TAG = "NSMBW";

    private AppSettings settings;
    private TouchControlsView touch;
    private MotionInput motion;
    private final ControllerInput controllers = new ControllerInput();
    private boolean started;
    private int touchButtons;
    private boolean touchShake, motionShake;
    private float motionTilt;
    private final Handler main = new Handler(Looper.getMainLooper());
    private AlertDialog errorDialog;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        Lang.load(this);
        settings = new AppSettings(this);
        getWindow().addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);

        final FrameLayout root = new FrameLayout(this);
        final SurfaceView surface = new SurfaceView(this);
        surface.getHolder().addCallback(this);
        root.addView(surface, new FrameLayout.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT));
        touch = new TouchControlsView(this);
        touch.setListener(this);
        applyControlSettings();
        root.addView(touch, new FrameLayout.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.MATCH_PARENT));
        setContentView(root);
        motion = new MotionInput(this, this);
        watchControllers();

        if (!GameFiles.gameReady(this)) {
            openStartMenu();
            return;
        }
        GpuDrivers.applyToGame(this, settings.gpuCompat());
        final java.io.File restore = takeBackup();
        Native.setLoadState(restore != null ? restore.getAbsolutePath() : null);
        final String error = Native.start(GameFiles.dataDir(this).getAbsolutePath(), GameFiles.gameDir(this).getAbsolutePath(),
                settings.nativeScale(), settings.widescreen(), settings.vsync(), Lang.wiiLanguage());
        if (error != null) {
            showError(error);
            return;
        }
        started = true;
        pushInput();
        main.post(pollErrors);
        if (restore != null) {
            Log.i(TAG, "starting from the backup of a run Android ended");
            android.widget.Toast.makeText(this, Lang.t("Continuing where you left off."), android.widget.Toast.LENGTH_LONG).show();
        }
        // A GPU driver on trial has kept the game running (no error): it stays picked.
        final Context app = getApplicationContext();
        main.postDelayed(() -> {
            if (errorDialog == null) GpuDrivers.endTrial(app);
        }, GpuDrivers.TRIAL_MS);
    }

    // --- the backup ----------------------------------------------------------------------------------

    /** Where leaving the app writes the backup: named for this build, so another version's isn't loaded. */
    private java.io.File backupFile() {
        long build = 0;
        try {
            build = getPackageManager().getPackageInfo(getPackageName(), 0).getLongVersionCode();
        } catch (android.content.pm.PackageManager.NameNotFoundException ignored) {
        }
        return new java.io.File(GameFiles.dataDir(this), "backup-" + build + ".state");
    }

    /**
     * At the game's start: the backup a run left when Android ended it while away, renamed so it is
     * tried once (libnsmbw.so removes it once the game runs from it); null for a normal start. Others'
     * backups (older versions, half-written ones) go.
     */
    private java.io.File takeBackup() {
        final java.io.File dir = GameFiles.dataDir(this), mine = backupFile();
        final java.io.File restore = new java.io.File(dir, "restore.state");
        restore.delete();
        final boolean found = mine.isFile() && mine.renameTo(restore);
        final java.io.File[] others = dir.listFiles((d, name) -> name.startsWith("backup-"));
        if (others != null) {
            for (java.io.File f : others) f.delete();
        }
        return found ? restore : null;
    }

    // --- the surface -------------------------------------------------------------------------------

    @Override
    public void surfaceCreated(SurfaceHolder holder) {
        if (started) Native.setSurface(holder.getSurface());
    }

    @Override
    public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
        if (started) Native.surfaceResized();
    }

    @Override
    public void surfaceDestroyed(SurfaceHolder holder) {
        if (started) Native.setSurface(null);  // returns once the renderer has let go of it
    }

    /** Why the game stopped (the native side's error), shown once; and the remote's rumble. */
    private final Runnable pollErrors = new Runnable() {
        @Override
        public void run() {
            final String error = Native.takeError();
            if (error != null) {
                showError(error);
                return;
            }
            vibrateForGame(Native.rumble());
            main.postDelayed(this, 50);
        }
    };

    private boolean vibrating;

    /**
     * Wii Remote 1's rumble on the phone (off with the Vibration setting), as NSMB-Wii-Recompilation's
     * app does: at most 2 s at a time, the game stops it.
     */
    @SuppressWarnings("deprecation")
    private void vibrateForGame(boolean on) {
        if (on == vibrating || (on && !settings.haptics())) return;
        vibrating = on;
        final android.os.Vibrator vibrator = android.os.Build.VERSION.SDK_INT >= 31
                ? ((android.os.VibratorManager) getSystemService(VIBRATOR_MANAGER_SERVICE)).getDefaultVibrator()
                : (android.os.Vibrator) getSystemService(VIBRATOR_SERVICE);
        if (vibrator == null || !vibrator.hasVibrator()) return;
        if (on) {
            // As a game's, not touch feedback: the phone's touch vibration setting doesn't mute it.
            vibrator.vibrate(android.os.VibrationEffect.createOneShot(2000, 180),
                    new android.media.AudioAttributes.Builder().setUsage(android.media.AudioAttributes.USAGE_GAME).build());
        } else {
            vibrator.cancel();
        }
    }

    private void showError(String error) {
        Log.e(TAG, "the game stopped: " + error);
        if (errorDialog != null || isFinishing()) return;
        errorDialog = new AlertDialog.Builder(this)
                .setTitle(Lang.t("The game stopped"))
                .setMessage(error)
                .setPositiveButton(Lang.t("Start Menu"), (d, w) -> openStartMenu())
                .setNegativeButton(Lang.t("Quit Game"), (d, w) -> finish())
                .setCancelable(false)
                .show();
    }

    // --- the app's life ------------------------------------------------------------------------------

    @Override
    protected void onResume() {
        super.onResume();
        if (started) Native.setPaused(false);  // the game carries on where it stopped
        hideSystemBars();
        motion.start(settings.motionTilt(), settings.motionShake());
    }

    @Override
    protected void onPause() {
        // Leaving the app pauses the game: it waits in memory, nothing runs or plays. Leaving for another
        // app writes a backup first, in case Android ends this one meanwhile; the screen turning off and
        // quitting don't.
        if (started) {
            final android.os.PowerManager power = (android.os.PowerManager) getSystemService(POWER_SERVICE);
            if (!isFinishing() && (power == null || power.isInteractive())) {
                final String why = Native.backup(backupFile().getPath(), BACKUP_WAIT_MS);
                if (why != null) Log.i(TAG, "no backup: " + why);
            } else {
                Native.setPaused(true);
            }
        }
        vibrateForGame(false);
        motion.stop();
        touch.release();
        controllers.release();
        pushInput();
        super.onPause();
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (hasFocus) {
            hideSystemBars();
        } else {
            touch.release();  // a dialog or the shade took the screen: let go of every button
            controllers.release();
            pushInput();
        }
    }

    /** How long leaving the app waits for the backup (it carries on being written after that). */
    private static final int BACKUP_WAIT_MS = 2000;

    @Override
    protected void onDestroy() {
        main.removeCallbacksAndMessages(null);
        super.onDestroy();
        if (isFinishing()) {
            // The native side starts once per process; the next start must get a fresh one.
            Process.killProcess(Process.myPid());
        }
    }

    // --- controllers -------------------------------------------------------------------------------

    /** A controller hides the on-screen remote; a tap shows it again; it comes back when the last goes. */
    private void watchControllers() {
        final android.hardware.input.InputManager input =
                (android.hardware.input.InputManager) getSystemService(Context.INPUT_SERVICE);
        if (input == null) return;
        input.registerInputDeviceListener(new android.hardware.input.InputManager.InputDeviceListener() {
            @Override public void onInputDeviceAdded(int id) {
                if (ControllerInput.isController(android.view.InputDevice.getDevice(id))) touch.setControllerHidden(true);
            }
            @Override public void onInputDeviceRemoved(int id) {
                controllers.removed(id);
                pushInput();
                if (!ControllerInput.anyController()) touch.setControllerHidden(false);
            }
            @Override public void onInputDeviceChanged(int id) {}
        }, main);
        if (ControllerInput.anyController()) touch.setControllerHidden(true);
    }

    @Override
    public boolean dispatchGenericMotionEvent(MotionEvent event) {
        if (controllers.onMotion(event)) {
            final int[] axes = {MotionEvent.AXIS_X, MotionEvent.AXIS_Y, MotionEvent.AXIS_Z, MotionEvent.AXIS_RZ,
                MotionEvent.AXIS_HAT_X, MotionEvent.AXIS_HAT_Y, MotionEvent.AXIS_LTRIGGER, MotionEvent.AXIS_RTRIGGER};
            for (int axis : axes) {
                if (Math.abs(event.getAxisValue(axis)) > 0.5f) {
                    touch.setControllerHidden(true);
                    break;
                }
            }
            pushInput();
            return true;
        }
        return super.dispatchGenericMotionEvent(event);
    }

    @Override
    public boolean dispatchKeyEvent(KeyEvent event) {
        if (event.getKeyCode() == KeyEvent.KEYCODE_BACK) {
            if (event.getAction() == KeyEvent.ACTION_UP && !event.isCanceled()) onMenu();
            return true;
        }
        if (controllers.onKey(event)) {
            if (event.getAction() == KeyEvent.ACTION_DOWN) touch.setControllerHidden(true);
            pushInput();
            return true;
        }
        return super.dispatchKeyEvent(event);
    }

    // --- input ---------------------------------------------------------------------------------------

    @Override
    public void onTouchInput(int buttons, boolean shake) {
        touchButtons = buttons;
        touchShake = shake;
        pushInput();
    }

    @Override
    public void onMotion(float tilt, boolean shake) {
        motionTilt = tilt;
        motionShake = shake;
        pushInput();
    }

    /** Player 1: the on-screen remote, the phone's motion and the first controller; players 2-4 the
     *  other controllers. Reverse Tilt mirrors the tilt, wherever it comes from. */
    private void pushInput() {
        if (!started) return;
        final float sign = settings.reverseTilt() ? -1f : 1f;
        for (int p = 0; p < ControllerInput.PLAYERS; ++p) {
            int buttons = controllers.buttons(p);
            boolean shake = controllers.shake(p);
            float tilt = controllers.tilt(p);
            if (p == 0) {
                buttons |= WiiRemote.fromTouch(touchButtons);
                shake |= touchShake || motionShake;
                if (tilt == 0f) tilt = motionTilt;
            }
            Native.setRemote(p, p == 0 || controllers.connected(p), buttons, shake, sign * tilt);
        }
    }

    private void applyControlSettings() {
        touch.setControlsVisible(settings.touchControls());
        touch.setOpacity(settings.touchOpacity());
        touch.setHaptics(settings.haptics());
        touch.setLayout(TouchLayoutEditor.startingLayout(this, settings));
        touch.setCheatButton(false);
    }

    @Override
    public void onCheat() {}

    /** Tapping the game's menus needs the recomp's hooks; this port takes the remote's buttons there. */
    @Override
    public void onTap(float x, float y) {}

    @Override
    public void onPointer(float x, float y, boolean held) {}

    @Override
    public boolean canRevealControls() { return true; }

    // --- the Back menu -------------------------------------------------------------------------------

    @Override
    @SuppressWarnings("deprecation")
    public void onBackPressed() {
        onMenu();
    }

    /** The phone's Back gesture: back to the start menu (where the settings are), or quit. */
    @Override
    public void onMenu() {
        touch.release();
        pushInput();
        final java.util.List<String> items = new java.util.ArrayList<>();
        items.add("Start Menu");
        if (settings.touchControls()) items.add("Edit Touch Controls");
        items.add("Quit Game");
        final String[] labels = new String[items.size()];
        for (int i = 0; i < labels.length; ++i) labels[i] = Lang.t(items.get(i));
        new AlertDialog.Builder(this)
                .setTitle("New Super Mario Bros. Wii")
                .setItems(labels, (dialog, which) -> {
                    final String item = items.get(which);
                    if (item.equals("Start Menu")) openStartMenu();
                    else if (item.equals("Edit Touch Controls")) editTouchControls();
                    else finish();
                })
                .setNegativeButton(Lang.t("Back to the Game"), null)
                .show();
    }

    /** The game waits (as with the app away) while the remote is edited. */
    private void editTouchControls() {
        if (started) Native.setPaused(true);
        TouchLayoutEditor.open(this, touch, settings, () -> {
            if (started) Native.setPaused(false);
        });
    }

    /** Back to the start menu (its settings, Skip Start Menu); the game's process ends. */
    private void openStartMenu() {
        final Intent next = new Intent(this, LauncherActivity.class)
                .putExtra(LauncherActivity.EXTRA_SHOW_START_MENU, true)
                .addFlags(Intent.FLAG_ACTIVITY_NEW_TASK | Intent.FLAG_ACTIVITY_CLEAR_TOP);
        // In the main process (Xiaomi Mode) the start menu needs a fresh one: the game's ends with it.
        if (this instanceof MainProcessGameActivity) {
            RestartActivity.replaceProcess(this, next);
        } else {
            startActivity(next);
        }
        finish();
    }

    /** The game, in the process this phone needs (Xiaomi Mode: the main one). */
    static Intent intent(Context context) {
        return new Intent(context, new AppSettings(context).gameInMainProcess() ? MainProcessGameActivity.class : GameActivity.class);
    }

    private void hideSystemBars() {
        if (android.os.Build.VERSION.SDK_INT < 30) {
            getWindow().getDecorView().setSystemUiVisibility(View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                    | View.SYSTEM_UI_FLAG_FULLSCREEN | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                    | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                    | View.SYSTEM_UI_FLAG_LAYOUT_STABLE);
            return;
        }
        getWindow().setDecorFitsSystemWindows(false);
        final WindowInsetsController controller = getWindow().getInsetsController();
        if (controller != null) {
            controller.hide(WindowInsets.Type.systemBars());
            controller.setSystemBarsBehavior(WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
        }
    }
}
