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
  onDelete: (widgetName: string) => void;
}

export function DeleteWidgetDialog(props: DeleteWidgetDialogProps) {
  return (
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>Delete Widget</AlertDialogTitle>
        <AlertDialogDescription>
          Are you sure you want to delete "{props.widget.name}"? This
          action cannot be undone.
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogClose>Cancel</AlertDialogClose>
        <AlertDialogAction
          onClick={() => props.onDelete(props.widget.name)}
          class="bg-red-500 hover:bg-red-600"
        >
          Delete
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  );
}
