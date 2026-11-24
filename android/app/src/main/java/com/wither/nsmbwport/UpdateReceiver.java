package com.wither.nsmbwport;

import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageInstaller;
import android.widget.Toast;

/** What the system did with an update Updater committed (in the app's main process). */
public final class UpdateReceiver extends BroadcastReceiver {
    @Override
    public void onReceive(Context context, Intent intent) {
        final int status = intent.getIntExtra(PackageInstaller.EXTRA_STATUS, PackageInstaller.STATUS_FAILURE);
        if (status == PackageInstaller.STATUS_PENDING_USER_ACTION) {
            // The system's confirmation (the first time, after "Allow from this source").
            final Intent confirm = intent.getParcelableExtra(Intent.EXTRA_INTENT);
            if (confirm != null) context.startActivity(confirm.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));
        } else if (status != PackageInstaller.STATUS_SUCCESS && status != PackageInstaller.STATUS_FAILURE_ABORTED) {
            // Success is heard by the new version, if at all; a cancel needs no word.
            final String message = intent.getStringExtra(PackageInstaller.EXTRA_STATUS_MESSAGE);
            Lang.load(context);
            Toast.makeText(context, Lang.f("The update failed: %s", message != null ? message : "status " + status),
                    Toast.LENGTH_LONG).show();
        }
    }
}
