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

    /** The app left the screen (true) or came back: the game stops and nothing plays. Coming back
     *  removes the backup backup() wrote. */
    static native void setPaused(boolean paused);

    /**
     * setPaused(true), after writing the game to path (a save state: the backup for Android ending the
     * app while it is away). Waits up to timeoutMs; returns null once it is written, else why not. The
     * file appears whole or not at all.
     */
    static native String backup(String path, int timeoutMs);

    /** Before start: a backup to start the game from instead of power-on (removed once the game shows
     *  a field), or null. */
    static native void setLoadState(String path);

    /**
     * Before start: Graphics Compatibility (AppSettings.GPU_COMPAT_*) and, with hookDir, the GPU driver
     * driverLib in driverDir opened through libadrenotools in hookDir (tmpDir: Android 9's patched
     * libraries). Returns null, or why the driver can't be used (the phone's own is used then).
     */
    static native String setGraphics(int compat, String hookDir, String driverDir, String driverLib, String tmpDir);

    /** Before start: the Gameplay options (both true: the game's own rules; net games keep those). */
    static native void setGameplay(boolean powerUpFreeze, boolean playerCollision);

    /** The game was tapped off the on-screen remote's buttons: skips the opening cutscene or a hint movie. */
    static native void tap();

    /** Whether Wii Remote 1's rumble motor is on. */
    static native boolean rumble();

    /** Why the game stopped, once; else null. */
    static native String takeError();

    /**
     * The disc image open as fd (detached: the library closes it): its game ID and revision as the
     * first 8 bytes of sys/boot.bin hold them, or null when it isn't a Wii disc image it can read
     * (ISO, WBFS, RVZ, WIA, CISO, GCZ).
     */
    static native byte[] discHeader(int fd);

    /**
     * Extracts the disc image open as fd (detached) into dest, as Dolphin's Extract Entire Disc
     * lays it out, telling progress as it goes. Returns null when done, or why not.
     */
    static native String extractDisc(int fd, String dest, GameFiles.Progress progress);

    // --- LAN Play (LanPlay; crates/android/src/lobby.rs) ---------------------------------------------

    /**
     * In the game's process, before start: 0 a game of one's own, 1 host a room called name, 2 join the
     * room at address (ip, ip:port or a Tailscale name).
     */
    static native void setNet(int mode, String address, String name);

    /** Starts looking for rooms (the known devices are kept under filesDir); null, or why not. */
    static native String lanOpen(String filesDir);

    static native void lanClose();

    /** The rooms heard from lately: name, ip:port, and "1" when found over Tailscale, for each. */
    static native String[] lanRooms();

    /** This phone's addresses, Tailscale first. */
    static native String[] lanAddresses();

    /** The devices asked for rooms directly (over Tailscale, which carries no broadcasts). */
    static native String[] lanDevices();

    /** False when it is known already or isn't a name or address. */
    static native boolean lanAddDevice(String nameOrAddress);

    static native void lanRemoveDevice(String nameOrAddress);
}
