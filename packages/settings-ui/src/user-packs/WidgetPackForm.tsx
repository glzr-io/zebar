import {
  Card,
  CardContent,
  TextField,
  ChipField,
  TextAreaField,
  FormField,
} from '@glzr/components';
import { createForm, Field } from 'smorf';
import { createEffect, createResource, on } from 'solid-js';
import { join, sep } from '@tauri-apps/api/path';
import * as z from 'zod';

import { ImageSelector, WidgetPack } from '~/common';

const formSchema = z.object({
  name: z.string().min(2, {
    message: 'Name must be at least 2 characters.',
  }),
  description: z.string().min(10, {
    message: 'Description must be at least 10 characters.',
  }),
  tags: z.array(z.string()).min(1, {
    message: 'At least one tag is required.',
  }),
  previewImages: z.array(z.string()).min(1, {
    message: 'At least one preview image is required.',
  }),
  excludeFiles: z.string(),
});

export type WidgetPackFormData = z.infer<typeof formSchema>;

export interface WidgetPackFormProps {
  pack?: WidgetPack;
  onChange?: (form: WidgetPackFormData) => void;
  disabled?: boolean;
}

export function WidgetPackForm(props: WidgetPackFormProps) {
  const form = createForm<WidgetPackFormData>({
    name: '',
    description: '',
    tags: [],
    previewImages: [],
    excludeFiles: '',
  });

  const [imagePaths] = createResource(
    () =>
      [
        form.getFieldValue('previewImages'),
        props.pack.directoryPath,
      ] as const,
    async ([images, dirPath]) => {
      return Promise.all(images.map(image => join(dirPath, image)));
    },
  );

  createEffect(
    on(
      () => props.pack,
      value => {
        form.setValue({
          name: value.name,
          description: value.description,
          tags: value.tags,
          previewImages: value.previewImages,
          excludeFiles: value.excludeFiles,
        });
      },
    ),
  );

  createEffect(
    on(
      () => form.value,
      value => {
        if (form.isDirty()) {
          props.onChange?.(value);
        }
      },
    ),
  );

  async function onImageChange(images: string[]) {
    const pathPrefix = props.pack.directoryPath + sep();

    form.setFieldValue(
      'previewImages',
      images.map(image => image.replace(pathPrefix, '')),
    );
  }

  return (
    <form class="space-y-8 mb-4">
      <Card>
        <CardContent class="pt-6">
          <Field of={form} path="name">
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

          <Field of={form} path="description">
            {inputProps => (
              <TextField
                label="Description"
                placeholder="A collection of beautiful widgets..."
                disabled={props.disabled}
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={form} path="tags">
            {inputProps => (
              <ChipField
                label="Tags"
                placeholder="Press enter to add tags..."
                disabled={props.disabled}
                {...inputProps()}
              />
            )}
          </Field>

          <FormField label="Preview images" disabled={props.disabled}>
            <ImageSelector
              images={imagePaths() ?? []}
              cwd={props.pack.directoryPath}
              onChange={onImageChange}
              disabled={props.disabled}
            />
          </FormField>

          <Field of={form} path="excludeFiles">
            {inputProps => (
              <TextAreaField
                label="Exclude files"
                description="A list of file patterns to exclude from the pack separated by new lines."
                disabled={props.disabled}
                {...inputProps()}
              />
            )}
          </Field>
        </CardContent>
      </Card>
    </form>
  );
}
