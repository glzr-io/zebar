import {
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogClose,
  AlertDialogAction,
} from '@glzr/components';

import { WidgetPack } from '~/common';

export interface DeleteWidgetPackDialogProps {
  pack: WidgetPack;
  onDelete: (packId: string) => void;
}

export function DeleteWidgetPackDialog(
  props: DeleteWidgetPackDialogProps,
) {
  return (
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>Delete Widget Pack</AlertDialogTitle>
        <AlertDialogDescription>
          Are you sure you want to delete "{props.pack.name}"? This action
          cannot be undone.
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogClose>Cancel</AlertDialogClose>
        <AlertDialogAction
          onClick={() => props.onDelete(props.pack.id)}
          class="bg-red-500 hover:bg-red-600"
        >
          Delete
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  );
}
