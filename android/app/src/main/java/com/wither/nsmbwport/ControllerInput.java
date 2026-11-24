package com.wither.nsmbwport;

import android.util.SparseArray;
import android.view.InputDevice;
import android.view.KeyEvent;
import android.view.MotionEvent;

/**
 * Controllers as sideways Wii Remotes, with the desktop port's Xbox-layout defaults (crates/input):
 * A jumps (2), B and X run (1), Y and the bumpers shake, Start is +, Select is -, the guide button
 * HOME; the D-pad and the left stick are the D-pad; the triggers and the right stick tilt. As on the
 * PC, the right stick's click is Fast Forward while held and both sticks' clicks Restart Course.
 * Controllers take players 1-4 in the order they are first used; player 1 shares the on-screen remote.
 */
final class ControllerInput {
    static final int PLAYERS = 4;
    private static final float STICK_PRESS = 0.5f;
    private static final float TILT_DEAD_ZONE = 0.15f;

    private static final class Pad {
        final int player;
        int keys;          // Wii Remote buttons held by keys
        boolean keyShake;
        boolean thumbL, thumbR;  // the sticks' clicks (Practice)
        float x, y, hatX, hatY, rx, lt, rt;
        Pad(int player) { this.player = player; }

        int buttons() {
            int b = keys;
            final float sx = Math.abs(hatX) > Math.abs(x) ? hatX : x, sy = Math.abs(hatY) > Math.abs(y) ? hatY : y;
            if (sy < -STICK_PRESS) b |= WiiRemote.screenUp();
            if (sy > STICK_PRESS) b |= WiiRemote.screenDown();
            if (sx < -STICK_PRESS) b |= WiiRemote.screenLeft();
            if (sx > STICK_PRESS) b |= WiiRemote.screenRight();
            return b;
        }

        float tilt() {
            final float t = rx + rt - lt;
            return Math.abs(t) > TILT_DEAD_ZONE ? Math.max(-1f, Math.min(1f, t)) : 0f;
        }
    }

    private final SparseArray<Pad> pads = new SparseArray<>();
    private final boolean[] on = new boolean[PLAYERS];

    /** A key event from a controller: whether it was one (and is now taken). */
    boolean onKey(KeyEvent e) {
        if (!fromController(e)) return false;
        final int code = e.getKeyCode();
        if (code == KeyEvent.KEYCODE_BUTTON_THUMBL || code == KeyEvent.KEYCODE_BUTTON_THUMBR) {
            final Pad pad = pad(e.getDeviceId());
            if (pad == null) return true;
            final boolean down = e.getAction() == KeyEvent.ACTION_DOWN;
            if (code == KeyEvent.KEYCODE_BUTTON_THUMBL) pad.thumbL = down;
            else pad.thumbR = down;
            return true;
        }
        final int bits = keyButtons(e.getKeyCode());
        final boolean shake = keyShakes(e.getKeyCode());
        if (bits == 0 && !shake) return false;
        final Pad pad = pad(e.getDeviceId());
        if (pad == null) return true;
        final boolean down = e.getAction() == KeyEvent.ACTION_DOWN;
        if (bits != 0) pad.keys = down ? pad.keys | bits : pad.keys & ~bits;
        if (shake) pad.keyShake = down;
        return true;
    }

    /** Sticks, the hat and the triggers; whether the event was a controller's. */
    boolean onMotion(MotionEvent e) {
        if (!fromController(e) || e.getAction() != MotionEvent.ACTION_MOVE) return false;
        final Pad pad = pad(e.getDeviceId());
        if (pad == null) return true;
        pad.x = e.getAxisValue(MotionEvent.AXIS_X);
        pad.y = e.getAxisValue(MotionEvent.AXIS_Y);
        pad.hatX = e.getAxisValue(MotionEvent.AXIS_HAT_X);
        pad.hatY = e.getAxisValue(MotionEvent.AXIS_HAT_Y);
        pad.rx = e.getAxisValue(MotionEvent.AXIS_Z);
        pad.lt = Math.max(e.getAxisValue(MotionEvent.AXIS_LTRIGGER), e.getAxisValue(MotionEvent.AXIS_BRAKE));
        pad.rt = Math.max(e.getAxisValue(MotionEvent.AXIS_RTRIGGER), e.getAxisValue(MotionEvent.AXIS_GAS));
        return true;
    }

    /** A controller was unplugged: it lets go of everything (its player's remote stays on). */
    void removed(int deviceId) {
        final Pad pad = pads.get(deviceId);
        if (pad != null) {
            pads.remove(deviceId);
            pads.put(-1 - pad.player, pad);  // keeps the player taken for when it comes back
            pad.keys = 0;
            pad.keyShake = false;
            pad.thumbL = pad.thumbR = false;
            pad.x = pad.y = pad.hatX = pad.hatY = pad.rx = pad.lt = pad.rt = 0f;
        }
    }

    /** Lets go of every controller's buttons (the app lost focus). */
    void release() {
        for (int i = 0; i < pads.size(); ++i) {
            final Pad p = pads.valueAt(i);
            p.keys = 0;
            p.keyShake = false;
            p.thumbL = p.thumbR = false;
            p.x = p.y = p.hatX = p.hatY = p.rx = p.lt = p.rt = 0f;
        }
    }

    boolean connected(int player) { return on[player]; }

    /** A controller's right stick is clicked (not both sticks): Fast Forward. */
    boolean fastForward() {
        for (int i = 0; i < pads.size(); ++i) if (pads.valueAt(i).thumbR && !pads.valueAt(i).thumbL) return true;
        return false;
    }

    /** A controller's sticks are both clicked: Restart Course. */
    boolean restart() {
        for (int i = 0; i < pads.size(); ++i) if (pads.valueAt(i).thumbR && pads.valueAt(i).thumbL) return true;
        return false;
    }

    int buttons(int player) {
        int b = 0;
        for (int i = 0; i < pads.size(); ++i) if (pads.valueAt(i).player == player) b |= pads.valueAt(i).buttons();
        return b;
    }

    boolean shake(int player) {
        for (int i = 0; i < pads.size(); ++i) if (pads.valueAt(i).player == player && pads.valueAt(i).keyShake) return true;
        return false;
    }

    float tilt(int player) {
        float t = 0f;
        for (int i = 0; i < pads.size(); ++i) if (pads.valueAt(i).player == player) t += pads.valueAt(i).tilt();
        return Math.max(-1f, Math.min(1f, t));
    }

    /** The controller's pad, taking the next player for one not seen before (null with all four taken). */
    private Pad pad(int deviceId) {
        Pad pad = pads.get(deviceId);
        if (pad != null) return pad;
        // A controller that comes back takes its old player (its id may be new: the first free slot kept).
        for (int i = 0; i < pads.size(); ++i) {
            if (pads.keyAt(i) < 0) {
                pad = pads.valueAt(i);
                pads.removeAt(i);
                pads.put(deviceId, pad);
                return pad;
            }
        }
        int player = 0;
        while (player < PLAYERS && taken(player)) ++player;
        if (player >= PLAYERS) return null;
        pad = new Pad(player);
        on[player] = true;
        pads.put(deviceId, pad);
        return pad;
    }

    private boolean taken(int player) {
        for (int i = 0; i < pads.size(); ++i) if (pads.valueAt(i).player == player) return true;
        return false;
    }

    private static int keyButtons(int code) {
        switch (code) {
            case KeyEvent.KEYCODE_DPAD_UP: return WiiRemote.screenUp();
            case KeyEvent.KEYCODE_DPAD_DOWN: return WiiRemote.screenDown();
            case KeyEvent.KEYCODE_DPAD_LEFT: return WiiRemote.screenLeft();
            case KeyEvent.KEYCODE_DPAD_RIGHT: return WiiRemote.screenRight();
            case KeyEvent.KEYCODE_BUTTON_A: return WiiRemote.TWO;
            case KeyEvent.KEYCODE_BUTTON_B:
            case KeyEvent.KEYCODE_BUTTON_X: return WiiRemote.ONE;
            case KeyEvent.KEYCODE_BUTTON_START: return WiiRemote.PLUS;
            case KeyEvent.KEYCODE_BUTTON_SELECT: return WiiRemote.MINUS;
            case KeyEvent.KEYCODE_BUTTON_MODE: return WiiRemote.HOME;
            default: return 0;
        }
    }

    private static boolean keyShakes(int code) {
        return code == KeyEvent.KEYCODE_BUTTON_Y || code == KeyEvent.KEYCODE_BUTTON_L1 || code == KeyEvent.KEYCODE_BUTTON_R1;
    }

    /** A real controller's event: gamepad or joystick sources (some controllers say they're keyboards
     *  but send gamepad buttons). */
    static boolean fromController(android.view.InputEvent event) {
        if (event.getDeviceId() < 0) return false;
        final int sources = event.getSource();
        if ((sources & InputDevice.SOURCE_GAMEPAD) == InputDevice.SOURCE_GAMEPAD
                || (sources & InputDevice.SOURCE_JOYSTICK) == InputDevice.SOURCE_JOYSTICK) {
            return true;
        }
        return event instanceof KeyEvent && KeyEvent.isGamepadButton(((KeyEvent) event).getKeyCode());
    }

    /** A real controller: sticks or a joystick pad (not a stickless TV remote). */
    static boolean isController(InputDevice device) {
        return device != null && !device.isVirtual()
                && (device.getSources() & InputDevice.SOURCE_JOYSTICK) == InputDevice.SOURCE_JOYSTICK
                && device.getMotionRange(MotionEvent.AXIS_X, InputDevice.SOURCE_JOYSTICK) != null;
    }

    static boolean anyController() {
        for (int id : InputDevice.getDeviceIds()) {
            if (isController(InputDevice.getDevice(id))) return true;
        }
        return false;
    }
}
