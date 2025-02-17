import {
  SelectField,
  Button,
  TextField,
  DialogFooter,
} from '@glzr/components';
import { createForm, Field } from 'smorf';

import { CreateWidgetForm } from '~/common';

export type CreateWidgetDialogProps = {
  onSubmit: (widget: CreateWidgetForm) => void;
};

export function CreateWidgetDialog(props: CreateWidgetDialogProps) {
  const form = createForm<CreateWidgetForm>({
    name: '',
    template: 'react-buildless',
  });

  return (
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Field of={form} path="name">
          {inputProps => (
            <TextField
              label="Widget Name"
              placeholder="My Cool Widget"
              {...inputProps}
            />
          )}
        </Field>
      </div>

      <div class="grid gap-2">
        <Field of={form} path="template">
          {inputProps => (
            <SelectField
              label="Template"
              placeholder="Select a template"
              options={[
                { label: 'React Buildless', value: 'react-buildless' },
                { label: 'Solid TypeScript', value: 'solid-ts' },
              ]}
              {...inputProps}
            />
          )}
        </Field>
      </div>

      <DialogFooter>
        <Button
          type="submit"
          onClick={() => props.onSubmit(form.value)}
          disabled={!form.value.name}
        >
          Create Widget
        </Button>
      </DialogFooter>
    </div>
  );
}
