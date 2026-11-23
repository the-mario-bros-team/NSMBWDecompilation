package com.wither.nsmbwport;

import android.app.Activity;
import android.app.AlertDialog;
import android.content.Context;
import android.content.Intent;
import android.content.res.ColorStateList;
import android.graphics.Color;
import android.graphics.Typeface;
import android.graphics.drawable.GradientDrawable;
import android.graphics.drawable.RippleDrawable;
import android.net.wifi.WifiManager;
import android.os.Build;
import android.os.Handler;
import android.os.Looper;
import android.provider.Settings;
import android.text.InputType;
import android.util.TypedValue;
import android.view.Gravity;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

/**
 * LAN Play on the start menu: host a room, join one found on the Wi-Fi or over Tailscale, or join by
 * address, with phones and PCs alike (crates/netplay). The game then starts in its own process as the
 * host or a joiner (beforeStart); anyone joins at any time and plays from where the host is. Also the
 * devices asked for rooms over Tailscale, as NSMB-Wii-Recompilation's Tailscale & LAN Devices.
 */
final class LanPlay {
    private static final String EXTRA_MODE = "lanMode";
    private static final String EXTRA_ADDRESS = "lanAddress";
    private static final int HOST = 1, JOIN = 2;

    // The start menu's colors (LauncherActivity).
    private static final int ACCENT = Color.rgb(229, 57, 53);
    private static final int SURFACE = Color.rgb(24, 31, 38);
    private static final int RAISED = Color.rgb(38, 48, 58);
    private static final int MUTED = Color.rgb(170, 178, 186);
    private static final int FAINT = Color.rgb(120, 128, 136);
    private static final Typeface MEDIUM = Typeface.create("sans-serif-medium", Typeface.NORMAL);

    /** Held while looking for rooms, and in a game hosting one: some phones drop broadcasts without it. */
    private static WifiManager.MulticastLock multicast;

    private LanPlay() {}

    /** In the game's process, before Native.start: the session it was started for (none for Play). */
    static void beforeStart(Activity game) {
        final Intent intent = game.getIntent();
        final int mode = intent == null ? 0 : intent.getIntExtra(EXTRA_MODE, 0);
        if (mode == 0) return;
        if (mode == HOST) holdMulticast(game, true);
        Native.setNet(mode, intent.getStringExtra(EXTRA_ADDRESS), deviceName(game));
    }

    /** The phone's name as rooms are called: its name in the phone's settings, else its model. */
    private static String deviceName(Context context) {
        String name = Build.VERSION.SDK_INT >= 25 ? Settings.Global.getString(context.getContentResolver(), Settings.Global.DEVICE_NAME) : null;
        if (name == null || name.trim().isEmpty()) name = Build.MODEL;
        return name;
    }

    private static void holdMulticast(Context context, boolean hold) {
        if (hold && multicast == null) {
            final WifiManager wifi = (WifiManager) context.getApplicationContext().getSystemService(Context.WIFI_SERVICE);
            if (wifi == null) return;
            multicast = wifi.createMulticastLock("nsmbw-lan");
            multicast.setReferenceCounted(false);
            multicast.acquire();
        } else if (!hold && multicast != null) {
            multicast.release();
            multicast = null;
        }
    }

    private static void startGame(Activity activity, int mode, String address) {
        activity.startActivity(new Intent(activity, GameActivity.class).putExtra(EXTRA_MODE, mode).putExtra(EXTRA_ADDRESS, address));
    }

    /** The LAN Play window: rooms refresh every second while it is open. */
    static void show(Activity activity) {
        final String problem = Native.lanOpen(GameFiles.dataDir(activity).getAbsolutePath());
        if (problem != null) {
            new AlertDialog.Builder(activity).setMessage(Lang.f("Can't look for rooms: %s", problem))
                    .setPositiveButton(Lang.t("OK"), null).show();
            return;
        }
        holdMulticast(activity, true);
        final LinearLayout column = new LinearLayout(activity);
        column.setOrientation(LinearLayout.VERTICAL);
        column.setPadding(dp(activity, 20), dp(activity, 8), dp(activity, 20), dp(activity, 8));
        column.addView(label(activity, Lang.t("Play with other phones and PCs on your Wi-Fi, or over Tailscale. "
                + "Every player needs their own game and the same version of NSMBW."), 13, MUTED, false));

        final AlertDialog[] dialog = new AlertDialog[1];
        final Button host = button(activity, Lang.t("Create Room"), true);
        host.setOnClickListener(v -> {
            dialog[0].dismiss();
            startGame(activity, HOST, "");
        });
        column.addView(host, matchWidth(activity, 14));

        column.addView(heading(activity, Lang.t("Rooms")));
        final LinearLayout rooms = new LinearLayout(activity);
        rooms.setOrientation(LinearLayout.VERTICAL);
        column.addView(rooms, matchWidth(activity, 0));
        final Button direct = button(activity, Lang.t("Join by Address…"), false);
        direct.setOnClickListener(v -> askAddress(activity, dialog[0]));
        column.addView(direct, matchWidth(activity, 10));

        column.addView(heading(activity, Lang.t("This phone")));
        final StringBuilder own = new StringBuilder();
        for (String address : Native.lanAddresses()) {
            if (own.length() > 0) own.append('\n');
            own.append(address).append(isTailscale(address) ? "  (Tailscale)" : "  (Wi-Fi)");
        }
        column.addView(label(activity, own.length() > 0 ? own.toString() : Lang.t("No network"), 15, Color.WHITE, false));
        final Button devices = button(activity, Lang.t("Tailscale & LAN Devices"), false);
        devices.setOnClickListener(v -> showDevices(activity));
        column.addView(devices, matchWidth(activity, 12));

        final Handler main = new Handler(Looper.getMainLooper());
        final String[] shown = {null};
        final Runnable poll = new Runnable() {
            @Override
            public void run() {
                showRooms(activity, rooms, Native.lanRooms(), shown, dialog[0]);
                main.postDelayed(this, 1000);
            }
        };
        final ScrollView scroll = new ScrollView(activity);
        scroll.addView(column);
        dialog[0] = new AlertDialog.Builder(activity)
                .setTitle(Lang.t("LAN Play"))
                .setView(scroll)
                .setNegativeButton(Lang.t("Done"), null)
                .create();
        dialog[0].setOnDismissListener(d -> {
            main.removeCallbacks(poll);
            Native.lanClose();
            holdMulticast(activity, false);
        });
        dialog[0].show();
        main.post(poll);
    }

    /** The rooms found (name, address, "1" over Tailscale), redrawn when they change. */
    private static void showRooms(Activity activity, LinearLayout list, String[] found, String[] shown, AlertDialog dialog) {
        final String key = String.join("\n", found);
        if (key.equals(shown[0])) return;
        shown[0] = key;
        list.removeAllViews();
        if (found.length == 0) {
            list.addView(label(activity, Lang.t("Looking for rooms…"), 15, MUTED, false));
            return;
        }
        for (int i = 0; i + 2 < found.length; i += 3) {
            final String address = found[i + 1];
            final String where = found[i + 2].equals("1") ? address + "  ·  Tailscale" : address;
            final Button room = button(activity, found[i] + "\n" + where, false);
            room.setOnClickListener(v -> {
                dialog.dismiss();
                startGame(activity, JOIN, address);
            });
            list.addView(room, matchWidth(activity, 8));
        }
    }

    private static void askAddress(Activity activity, AlertDialog lan) {
        final EditText input = new EditText(activity);
        input.setHint(Lang.t("e.g. s26-ultra or 100.84.140.32"));
        input.setInputType(InputType.TYPE_CLASS_TEXT | InputType.TYPE_TEXT_VARIATION_URI);
        input.setSingleLine(true);
        final LinearLayout box = new LinearLayout(activity);
        box.setPadding(dp(activity, 20), dp(activity, 8), dp(activity, 20), 0);
        box.addView(input, matchWidth(activity, 0));
        new AlertDialog.Builder(activity)
                .setTitle(Lang.t("Join a Room"))
                .setMessage(Lang.t("The host's address, as its LAN Play shows it, or its Tailscale name."))
                .setView(box)
                .setPositiveButton(Lang.t("Join"), (d, w) -> {
                    final String text = input.getText().toString().replaceAll("\\s+", "");
                    if (text.isEmpty()) return;
                    lan.dismiss();
                    startGame(activity, JOIN, text);
                })
                .setNegativeButton(Lang.t("Cancel"), null)
                .show();
    }

    /** The devices asked for rooms directly (crates/netplay's lan-devices.txt). */
    private static void showDevices(Activity activity) {
        final LinearLayout column = new LinearLayout(activity);
        column.setOrientation(LinearLayout.VERTICAL);
        column.setPadding(dp(activity, 20), dp(activity, 8), dp(activity, 20), dp(activity, 8));
        column.addView(label(activity, Lang.t("Rooms on your Wi-Fi are found by themselves. Over Tailscale, PCs find "
                + "every device, and devices that have played together find each other. To find another phone "
                + "the first time, add its Tailscale name or address here (or it adds yours)."), 13, MUTED, false));
        column.addView(heading(activity, Lang.t("Devices asked for rooms")));
        final String[] devices = Native.lanDevices();
        if (devices.length == 0) column.addView(label(activity, Lang.t("None yet"), 15, Color.WHITE, false));
        final AlertDialog[] dialog = new AlertDialog[1];
        for (String device : devices) {
            final TextView row = label(activity, device + "   ✕", 15, Color.WHITE, false);
            row.setPadding(0, dp(activity, 8), 0, dp(activity, 8));
            row.setOnClickListener(v -> new AlertDialog.Builder(activity)
                    .setMessage(Lang.f("Stop asking %s for rooms?", device))
                    .setPositiveButton(Lang.t("Remove"), (d, w) -> {
                        Native.lanRemoveDevice(device);
                        dialog[0].dismiss();
                        showDevices(activity);
                    })
                    .setNegativeButton(Lang.t("Cancel"), null)
                    .show());
            column.addView(row);
        }
        final ScrollView scroll = new ScrollView(activity);
        scroll.addView(column);
        dialog[0] = new AlertDialog.Builder(activity)
                .setTitle(Lang.t("Tailscale & LAN Devices"))
                .setView(scroll)
                .setPositiveButton(Lang.t("Add Device…"), (d, w) -> askDevice(activity))
                .setNegativeButton(Lang.t("Done"), null)
                .show();
    }

    private static void askDevice(Activity activity) {
        final EditText input = new EditText(activity);
        input.setHint(Lang.t("e.g. s26-ultra or 100.84.140.32"));
        input.setInputType(InputType.TYPE_CLASS_TEXT | InputType.TYPE_TEXT_VARIATION_URI);
        input.setSingleLine(true);
        final LinearLayout box = new LinearLayout(activity);
        box.setPadding(dp(activity, 20), dp(activity, 8), dp(activity, 20), 0);
        box.addView(input, matchWidth(activity, 0));
        new AlertDialog.Builder(activity)
                .setTitle(Lang.t("Add a Device"))
                .setMessage(Lang.t("Its Tailscale name (as in the Tailscale app) or address. It is asked for rooms from now on."))
                .setView(box)
                .setPositiveButton(Lang.t("Add"), (d, w) -> {
                    Native.lanAddDevice(input.getText().toString().replaceAll("\\s+", ""));
                    showDevices(activity);
                })
                .setNegativeButton(Lang.t("Cancel"), (d, w) -> showDevices(activity))
                .show();
    }

    /** 100.64.0.0/10. */
    private static boolean isTailscale(String address) {
        final String[] parts = address.split("\\.");
        if (parts.length != 4) return false;
        try {
            return Integer.parseInt(parts[0]) == 100 && (Integer.parseInt(parts[1]) & 0xC0) == 64;
        } catch (NumberFormatException e) {
            return false;
        }
    }

    // --- views, in the start menu's style ----------------------------------------------------------

    private static TextView label(Context context, String text, int sp, int color, boolean bold) {
        final TextView view = new TextView(context);
        view.setText(text);
        view.setTextSize(TypedValue.COMPLEX_UNIT_SP, sp);
        view.setTextColor(color);
        view.setGravity(Gravity.START);
        if (bold) view.setTypeface(Typeface.DEFAULT_BOLD);
        return view;
    }

    private static TextView heading(Context context, String text) {
        final TextView view = label(context, text, 17, Color.WHITE, true);
        view.setPadding(0, dp(context, 18), 0, dp(context, 6));
        return view;
    }

    /** A pill button as the start menu's: `primary` red. */
    private static Button button(Context context, String text, boolean primary) {
        final Button view = new Button(context);
        view.setText(text);
        view.setAllCaps(false);
        view.setStateListAnimator(null);
        view.setTextSize(TypedValue.COMPLEX_UNIT_SP, primary ? 18 : 15);
        view.setTypeface(primary ? Typeface.DEFAULT_BOLD : MEDIUM);
        view.setTextColor(new ColorStateList(new int[][] {{-android.R.attr.state_enabled}, {}}, new int[] {FAINT, Color.WHITE}));
        final int height = dp(context, primary ? 54 : 50);
        view.setMinHeight(height);
        view.setMinimumHeight(height);
        view.setPadding(dp(context, 20), dp(context, 6), dp(context, 20), dp(context, 6));
        final GradientDrawable shape = new GradientDrawable();
        shape.setCornerRadius(height / 2f);
        shape.setColor(new ColorStateList(new int[][] {{-android.R.attr.state_enabled}, {}}, new int[] {SURFACE, primary ? ACCENT : RAISED}));
        final GradientDrawable mask = new GradientDrawable();
        mask.setCornerRadius(height / 2f);
        mask.setColor(Color.WHITE);
        view.setBackground(new RippleDrawable(ColorStateList.valueOf(Color.argb(56, 255, 255, 255)), shape, mask));
        return view;
    }

    private static LinearLayout.LayoutParams matchWidth(Context context, int topMarginDp) {
        final LinearLayout.LayoutParams params = new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT);
        params.topMargin = dp(context, topMarginDp);
        return params;
    }

    private static int dp(Context context, int value) {
        return Math.round(value * context.getResources().getDisplayMetrics().density);
    }
}
