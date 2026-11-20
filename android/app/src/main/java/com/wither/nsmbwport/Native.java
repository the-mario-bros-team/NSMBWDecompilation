package com.wither.nsmbwport;

import android.view.Surface;

/** libnsmbw.so (crates/android/src/lib.rs): the game, its renderer and its sound. */
final class Native {
    static {
        System.loadLibrary("nsmbw");
    }

    private Native() {}

    /**
     * Once per process. Saves, settings and caches go under filesDir; gameDir holds the extracted
     * disc (sys/, files/). language is the Wii's (1 English, 3 French, 4 Spanish, 5 Italian). The game
     * starts with the first surface. Returns null, or why it cannot start.
     */
    static native String start(String filesDir, String gameDir, int scale, boolean widescreen, boolean vsync, int language);

    /** The game's surface, or null while it is being destroyed (returns once it is let go of). */
    static native void setSurface(Surface surface);

    static native void surfaceResized();

    /** Wii Remote player (0..3): WiiRemote button bits, shake, tilt -1..1. */
    static native void setRemote(int player, boolean connected, int buttons, boolean shake, float tilt);

    /** The app left the screen (true) or came back: the game stops and nothing plays. */
    static native void setPaused(boolean paused);

    /** Whether Wii Remote 1's rumble motor is on. */
    static native boolean rumble();

    /** Why the game stopped, once; else null. */
    static native String takeError();
}
