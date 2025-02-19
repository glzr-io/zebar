import { SelectField, TextField } from '@glzr/components';
import { createForm, Field, FormState } from 'smorf';
import { createEffect, on } from 'solid-js';

import { CreateWidgetArgs } from '~/common';

export type CreateWidgetFormProps = {
  onChange: (form: FormState<CreateWidgetArgs>) => void;
};

export function CreateWidgetForm(props: CreateWidgetFormProps) {
  const form = createForm<CreateWidgetArgs>({
    name: '',
    packName: '',
    template: 'react_buildless',
  });

  createEffect(
    on(
      () => form.value,
      () => props.onChange(form),
    ),
  );

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
                { label: 'React Buildless', value: 'react_buildless' },
                { label: 'Solid TypeScript', value: 'solid_typescript' },
              ]}
              {...inputProps}
            />
          )}
        </Field>
      </div>
    </div>
  );
}
