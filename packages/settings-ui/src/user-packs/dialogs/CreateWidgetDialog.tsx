import {
  Button,
  DialogFooter,
  DialogHeader,
  DialogContent,
  Dialog,
  DialogTitle,
  DialogDescription,
} from '@glzr/components';
import { FormState } from 'smorf';
import { createSignal } from 'solid-js';

import { CreateWidgetArgs } from '~/common';
import { CreateWidgetForm } from '../CreateWidgetForm';

export type CreateWidgetDialogProps = {
  packName: string;
  onSubmit: (widget: CreateWidgetArgs) => void;
};

export function CreateWidgetDialog(props: CreateWidgetDialogProps) {
  const [form, setForm] = createSignal<FormState<CreateWidgetArgs> | null>(
    null,
  );

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>Add new widget</DialogTitle>
        <DialogDescription>
          Create a new widget in this pack.
        </DialogDescription>
      </DialogHeader>

      <div class="py-4">
        <CreateWidgetForm onChange={setForm} />
      </div>

      <DialogFooter>
        <Dialog.CloseButton>
          <Button variant="outline">Cancel</Button>
        </Dialog.CloseButton>

        <Dialog.CloseButton
          onClick={() =>
            props.onSubmit({ ...form()?.value, packName: props.packName })
          }
        >
          <Button disabled={!form()?.value.name.trim()}>
            Create widget
          </Button>
        </Dialog.CloseButton>
      </DialogFooter>
    </DialogContent>
  );
}
