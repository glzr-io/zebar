import {
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogClose,
  AlertDialogAction,
} from '@glzr/components';
import { WidgetConfig } from 'zebar';

export interface DeleteWidgetDialogProps {
  widget: WidgetConfig;
  onDelete: () => void;
}

export function DeleteWidgetDialog(props: DeleteWidgetDialogProps) {
  return (
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>Delete Widget</AlertDialogTitle>
        <AlertDialogDescription class="space-y-2">
          <p>Are you sure you want to delete "{props.widget.name}"?</p>
          <p>
            This will remove the widget from the <code>zpack.json</code>{' '}
            file. Other files will not be affected.
          </p>
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogClose>Cancel</AlertDialogClose>
        <AlertDialogAction
          onClick={() => props.onDelete()}
          class="bg-red-500 hover:bg-red-600"
        >
          Delete
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  );
}
