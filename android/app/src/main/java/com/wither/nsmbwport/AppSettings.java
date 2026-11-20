package com.wither.nsmbwport;

import android.content.Context;
import android.content.SharedPreferences;

/**
 * Player settings, kept in one SharedPreferences file shared by the start menu and the game (from
 * NSMB-Wii-Recompilation's app, same keys and defaults where the setting exists here too). The
 * game reads them when it starts.
 */
final class AppSettings {
    static final int ASPECT_4_3 = 0;
    static final int ASPECT_16_9 = 1;
    /** The recomp's Fill Screen (a wider view); this port shows 16:9 for it until it can widen the view. */
    static final int ASPECT_FILL = 2;

    /** Render scales offered: 0 the screen's own height, else that many times 480 lines. */
    static final float[] RESOLUTION_SCALES = {0f, 1f, 2f, 3f, 4f};

    private static final String FILE = "settings";
    private final Context context;

    AppSettings(Context context) {
        this.context = context.getApplicationContext() != null ? context.getApplicationContext() : context;
    }

    /** Both processes (start menu and :game) read and write it; commit() since the game's process
     *  ends right after it closes. */
    @SuppressWarnings("deprecation")
    private SharedPreferences prefs() {
        return context.getSharedPreferences(FILE, Context.MODE_MULTI_PROCESS);
    }

    int aspectMode() { return prefs().getInt("aspect_mode", ASPECT_FILL); }
    void setAspectMode(int mode) { prefs().edit().putInt("aspect_mode", mode).commit(); }

    /** The game boots in 4:3 only for Original 4:3. */
    boolean widescreen() { return aspectMode() != ASPECT_4_3; }

    float resolutionScale() { return prefs().getFloat("resolution_scale", 1f); }
    void setResolutionScale(float scale) { prefs().edit().putFloat("resolution_scale", scale).commit(); }

    /** The EFB scale the native side takes: 0 (the screen's own), else 1..8. */
    int nativeScale() {
        final float s = resolutionScale();
        return s <= 0f ? 0 : Math.max(1, Math.min(8, Math.round(s)));
    }

    /** V-Sync: frames go out in step with the display. */
    boolean vsync() { return prefs().getBoolean("vsync", true); }
    void setVSync(boolean on) { prefs().edit().putBoolean("vsync", on).commit(); }

    /** Where the player moved the on-screen remote's controls and how big (TouchControlsView.layout). */
    String touchLayout() { return prefs().getString("touch_layout", ""); }
    void setTouchLayout(String layout) { prefs().edit().putString("touch_layout", layout).commit(); }

    boolean touchControls() { return prefs().getBoolean("touch_controls", true); }
    void setTouchControls(boolean on) { prefs().edit().putBoolean("touch_controls", on).commit(); }

    float touchOpacity() { return prefs().getFloat("touch_opacity", 0.6f); }
    void setTouchOpacity(float opacity) { prefs().edit().putFloat("touch_opacity", opacity).commit(); }

    boolean motionTilt() { return prefs().getBoolean("motion_tilt", true); }
    void setMotionTilt(boolean on) { prefs().edit().putBoolean("motion_tilt", on).commit(); }

    boolean motionShake() { return prefs().getBoolean("motion_shake", true); }
    void setMotionShake(boolean on) { prefs().edit().putBoolean("motion_shake", on).commit(); }

    /** Reverse Tilt (on by default, as in the recomp): whatever the tilt comes from, mirrored, so
     *  tilting platforms go the way you tilt. */
    boolean reverseTilt() { return prefs().getBoolean("reverse_tilt", true); }
    void setReverseTilt(boolean on) { prefs().edit().putBoolean("reverse_tilt", on).commit(); }

    boolean haptics() { return prefs().getBoolean("haptics", true); }
    void setHaptics(boolean on) { prefs().edit().putBoolean("haptics", on).commit(); }

    /** Open the game straight away when the app starts; the Back menu leads back to the start menu. */
    boolean skipStartMenu() { return prefs().getBoolean("skip_start_menu", false); }
    void setSkipStartMenu(boolean on) { prefs().edit().putBoolean("skip_start_menu", on).commit(); }

    /** The start menu's and the game's language (Lang.CODES). */
    String language() { return prefs().getString("language", "en"); }
    void setLanguage(String code) { prefs().edit().putString("language", code).commit(); }

    /** "2× (960p)": the scale and the number of lines it renders, as the menu shows it. */
    static String resolutionLabel(float scale) {
        if (scale <= 0f) return Lang.t("Match screen");
        final String factor = scale == Math.rint(scale) ? String.valueOf((int) scale) : String.valueOf(scale);
        return factor + "× (" + Math.round(480 * scale) + "p)";
    }
}
