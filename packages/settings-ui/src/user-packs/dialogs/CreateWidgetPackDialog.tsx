import {
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
  Dialog,
  TextField,
  Button,
} from '@glzr/components';
import { createForm, Field } from 'smorf';

import { CreateWidgetPackArgs } from '~/common';

export interface CreateWidgetPackDialogProps {
  onSubmit: (args: CreateWidgetPackArgs) => void;
}

export function CreateWidgetPackDialog(
  props: CreateWidgetPackDialogProps,
) {
  const packForm = createForm<CreateWidgetPackArgs>({
    name: '',
    description: '',
    tags: [],
    previewImages: [],
    excludeFiles: '',
    widgets: [],
  });

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>Create Widget Pack</DialogTitle>
        <DialogDescription>
          Enter a name for your new widget pack.
        </DialogDescription>
      </DialogHeader>

      <div class="p-4">
        <Field of={packForm} path="name">
          {inputProps => (
            <TextField
              id="name"
              label="Pack Name"
              placeholder="Enter pack name..."
              {...inputProps()}
            />
          )}
        </Field>
      </div>

      <DialogFooter>
        <Dialog.CloseButton>
          <Button variant="outline">Cancel</Button>
        </Dialog.CloseButton>

        <Dialog.CloseButton onClick={() => props.onSubmit(packForm.value)}>
          <Button disabled={!packForm.value.name.trim()}>Create</Button>
        </Dialog.CloseButton>
      </DialogFooter>
    </DialogContent>
  );
}
