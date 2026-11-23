package com.wither.nsmbwport;

/**
 * Xiaomi Mode (AppSettings.gameInMainProcess): the same game in the app's main process instead of
 * ":game". Xiaomi's MIUI / HyperOS ends an app's extra processes, even one on screen, which left the
 * game on a black screen. libnsmbw.so starts once per process, so leaving the game replaces the main
 * process with a fresh one (RestartActivity).
 */
public final class MainProcessGameActivity extends GameActivity {
}
