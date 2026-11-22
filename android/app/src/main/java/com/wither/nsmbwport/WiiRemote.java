package com.wither.nsmbwport;

/**
 * Wii Remote core buttons as the native side takes them (hle_ios::wiimote::button), and the
 * on-screen remote's buttons mapped onto them. The remote is held sideways, D-pad on the left, so a
 * direction on screen is a different one on the remote's own D-pad (crates/input's `sideways`):
 * screen up is the D-pad's right, screen right its down.
 */
final class WiiRemote {
    static final int LEFT = 0x0100, RIGHT = 0x0200, DOWN = 0x0400, UP = 0x0800, PLUS = 0x1000;
    static final int TWO = 0x0001, ONE = 0x0002, B = 0x0004, A = 0x0008, MINUS = 0x0010, HOME = 0x0080;

    private WiiRemote() {}

    /** Screen directions (the remote held sideways) to its own D-pad. */
    static int screenUp() { return RIGHT; }
    static int screenDown() { return LEFT; }
    static int screenLeft() { return UP; }
    static int screenRight() { return DOWN; }

    /** TouchControlsView's bits (screen directions, its button bits) as Wii Remote buttons. */
    static int fromTouch(int t) {
        int b = 0;
        if ((t & TouchControlsView.LEFT) != 0) b |= screenLeft();
        if ((t & TouchControlsView.RIGHT) != 0) b |= screenRight();
        if ((t & TouchControlsView.UP) != 0) b |= screenUp();
        if ((t & TouchControlsView.DOWN) != 0) b |= screenDown();
        if ((t & TouchControlsView.ONE) != 0) b |= ONE;
        if ((t & TouchControlsView.TWO) != 0) b |= TWO;
        if ((t & TouchControlsView.A) != 0) b |= A;
        if ((t & TouchControlsView.B) != 0) b |= B;
        if ((t & TouchControlsView.PLUS) != 0) b |= PLUS;
        if ((t & TouchControlsView.MINUS) != 0) b |= MINUS;
        if ((t & TouchControlsView.HOME) != 0) b |= HOME;
        return b;
    }
}
