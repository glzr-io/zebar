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
import { configSchemas } from 'zebar';
import { z } from 'zod';

import { CreateWidgetPackArgs } from '~/common';

const formSchema = z.object({
  name: configSchemas.name,
  description: z.string(),
});

export type CreateWidgetPackDialogFormData = z.infer<typeof formSchema>;

export interface CreateWidgetPackDialogProps {
  onSubmit: (args: CreateWidgetPackArgs) => void;
}

export function CreateWidgetPackDialog(
  props: CreateWidgetPackDialogProps,
) {
  const packForm = createForm<CreateWidgetPackDialogFormData>({
    name: '',
    description: '',
  });

  function onSubmit(e: Event) {
    if (!packForm.isDirty() || packForm.hasError()) {
      e.preventDefault();
      return;
    }

    props.onSubmit({
      ...packForm.value,
      version: '0.0.0',
      tags: [],
      repositoryUrl: '',
    });
  }

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>Create Widget Pack</DialogTitle>
        <DialogDescription>
          Enter a name for your new widget pack.
        </DialogDescription>
      </DialogHeader>

      <Field of={packForm} path="name">
        {(inputProps, field) => (
          <TextField
            id="name"
            label="Pack Name"
            placeholder="my-widget-pack"
            error={field.error()}
            {...inputProps()}
          />
        )}
      </Field>

      <Field of={packForm} path="description">
        {(inputProps, field) => (
          <TextField
            id="description"
            label="Description (optional)"
            placeholder="A collection of beautiful widgets..."
            error={field.error()}
            {...inputProps()}
          />
        )}
      </Field>

      <DialogFooter>
        <Dialog.CloseButton>
          <Button variant="outline">Cancel</Button>
        </Dialog.CloseButton>

        <Dialog.CloseButton onClick={onSubmit}>
          <Button>Create</Button>
        </Dialog.CloseButton>
      </DialogFooter>
    </DialogContent>
  );
}
