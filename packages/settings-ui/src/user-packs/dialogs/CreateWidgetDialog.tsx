import {
  Button,
  DialogFooter,
  DialogHeader,
  DialogContent,
  Dialog,
  DialogTitle,
  DialogDescription,
} from '@glzr/components';
import { createForm } from 'smorf';

import { CreateWidgetArgs } from '~/common';
import { CreateWidgetForm } from '../CreateWidgetForm';

export type CreateWidgetDialogProps = {
  onSubmit: (widget: CreateWidgetArgs) => void;
};

export function CreateWidgetDialog(props: CreateWidgetDialogProps) {
  const form = createForm<CreateWidgetArgs>({
    name: '',
    packId: '',
    template: 'react_buildless',
  });

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>Add new widget</DialogTitle>
        <DialogDescription>
          Create a new widget in this pack.
        </DialogDescription>
      </DialogHeader>

      <div class="py-4">
        <CreateWidgetForm form={form} />
      </div>

      <DialogFooter>
        <Dialog.CloseButton>
          <Button variant="outline">Cancel</Button>
        </Dialog.CloseButton>

        <Dialog.CloseButton onClick={() => props.onSubmit(form.value)}>
          <Button disabled={!form.value.name.trim()}>Create widget</Button>
        </Dialog.CloseButton>
      </DialogFooter>
    </DialogContent>
  );
}
