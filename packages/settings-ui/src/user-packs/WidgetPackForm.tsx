import {
  Card,
  CardContent,
  TextField,
  ChipField,
  FormLabel,
  TextAreaField,
} from '@glzr/components';
import { Field, FormState } from 'smorf';

import { ImageSelector } from '~/common';
import { WidgetPackFormData } from './WidgetPackPage';

export interface WidgetPackFormProps {
  form: FormState<WidgetPackFormData>;
  disabled?: boolean;
}

export function WidgetPackForm(props: WidgetPackFormProps) {
  return (
    <form class="space-y-8">
      <Card>
        <CardContent class="pt-6">
          <div class="grid gap-4">
            <Field of={props.form} path="name">
              {inputProps => (
                <TextField
                  label="Name"
                  placeholder="My widget pack"
                  description="This will be used as the directory name (as a slug)."
                  disabled={props.disabled}
                  {...inputProps()}
                />
              )}
            </Field>

            <Field of={props.form} path="description">
              {inputProps => (
                <TextField
                  label="Description"
                  placeholder="A collection of beautiful widgets..."
                  disabled={props.disabled}
                  {...inputProps()}
                />
              )}
            </Field>

            <Field of={props.form} path="tags">
              {inputProps => (
                <ChipField
                  label="Tags"
                  placeholder="Press enter to add tags..."
                  disabled={props.disabled}
                  {...inputProps()}
                />
              )}
            </Field>

            <div>
              <FormLabel>Preview images</FormLabel>
              <ImageSelector
                images={props.form.value.previewImages}
                onChange={images =>
                  props.form.setFieldValue('previewImages', images)
                }
                disabled={props.disabled}
              />
            </div>

            <Field of={props.form} path="excludeFiles">
              {inputProps => (
                <TextAreaField
                  label="Exclude files"
                  description="A list of file patterns to exclude from the pack separated by new lines."
                  disabled={props.disabled}
                  {...inputProps()}
                />
              )}
            </Field>
          </div>
        </CardContent>
      </Card>
    </form>
  );
}
