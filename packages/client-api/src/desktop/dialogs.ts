import { message as messageDialog } from '@tauri-apps/plugin-dialog';

/**
 * Whether a dialog is currently open.
 *
 * Used to prevent multiple dialogs from opening at the same time.
 */
let hasOpenDialog = false;

export interface ErrorDialogOptions {
  title: string;
  error: unknown;
}

export async function showErrorDialog(args: ErrorDialogOptions) {
  const { title, error } = args;

  if (!hasOpenDialog) {
    hasOpenDialog = true;

    await messageDialog((error as Error)?.message ?? 'Unknown reason.', {
      title,
      kind: 'error',
    });

    hasOpenDialog = false;
  }
}
