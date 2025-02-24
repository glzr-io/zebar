import {
  Card,
  CardContent,
  TextField,
  ChipField,
  TextAreaField,
  FormField,
} from '@glzr/components';
import { join, sep } from '@tauri-apps/api/path';
import { createForm, Field, FormState } from 'smorf';
import { createEffect, createResource, on } from 'solid-js';
import * as z from 'zod';

import { ImageSelector, WidgetPack } from '~/common';

const formSchema = z.object({
  name: z
    .string()
    .min(2, 'Name must be at least 2 characters.')
    .max(24, 'Name cannot exceed 24 characters.')
    .regex(
      /^[a-z0-9][a-z0-9-_]*$/,
      'Only lowercase letters, numbers, and the characters - and _ are allowed.',
    ),
  description: z
    .string()
    .max(1000, 'Description cannot exceed 1000 characters.'),
  tags: z.array(z.string()).max(10, 'At most 10 tags are allowed.'),
  previewImages: z
    .array(z.string())
    .min(1, 'At least one preview image is required.')
    .max(6, 'At most 6 preview images are allowed.'),
  excludeFiles: z
    .string()
    .max(1000, 'File exclusion list cannot exceed 1000 characters.'),
});

export type WidgetPackFormData = z.infer<typeof formSchema>;

export interface WidgetPackFormProps {
  pack?: WidgetPack;
  disabled?: boolean;
  onChange?: (form: FormState<WidgetPackFormData>) => void;
}

export function WidgetPackForm(props: WidgetPackFormProps) {
  const form = createForm<WidgetPackFormData>(
    {
      name: '',
      description: '',
      tags: [],
      previewImages: [],
      excludeFiles: '',
    },
    { schema: formSchema },
  );

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

  // Update the form values when the pack is different.
  createEffect(
    on(
      () => props.pack.id,
      (id, prevId) => {
        if (id !== prevId) {
          form.setValue({
            name: props.pack.name,
            description: props.pack.description,
            tags: props.pack.tags,
            previewImages: props.pack.previewImages,
            excludeFiles: props.pack.excludeFiles,
          });
        }
      },
    ),
  );

  // Broadcast the form changes to the parent.
  createEffect(
    on(
      () => form.value,
      () => {
        props.onChange?.(form);
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
            {(inputProps, field) => (
              <TextField
                label="Name"
                placeholder="My widget pack"
                description="This is used as the directory name."
                disabled={props.disabled}
                error={field.error()}
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={form} path="description">
            {(inputProps, field) => (
              <TextField
                label="Description"
                placeholder="A collection of beautiful widgets..."
                disabled={props.disabled}
                error={field.error()}
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={form} path="tags">
            {(inputProps, field) => (
              <ChipField
                label="Tags"
                placeholder="Press enter to add tags..."
                disabled={props.disabled}
                error={field.error()}
                {...inputProps()}
              />
            )}
          </Field>

          <FormField
            label="Preview images"
            disabled={props.disabled}
            error={form.getFieldError('previewImages')}
          >
            <ImageSelector
              images={imagePaths() ?? []}
              cwd={props.pack.directoryPath}
              onChange={onImageChange}
              disabled={props.disabled}
            />
          </FormField>

          <Field of={form} path="excludeFiles">
            {(inputProps, field) => (
              <TextAreaField
                label="Exclude files"
                description="A list of file patterns to exclude from the pack separated by new lines."
                disabled={props.disabled}
                error={field.error()}
                {...inputProps()}
              />
            )}
          </Field>
        </CardContent>
      </Card>
    </form>
  );
}
