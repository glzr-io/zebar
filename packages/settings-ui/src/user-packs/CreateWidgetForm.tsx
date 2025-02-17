import { SelectField, TextField } from '@glzr/components';
import { Field, FormState } from 'smorf';

import { CreateWidgetArgs } from '~/common';

export type CreateWidgetFormProps = {
  form: FormState<CreateWidgetArgs>;
};

export function CreateWidgetForm(props: CreateWidgetFormProps) {
  return (
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Field of={props.form} path="name">
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
        <Field of={props.form} path="template">
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
    </div>
  );
}
