import {
  Card,
  CardContent,
  TextField,
  ChipField,
  TextAreaField,
  FormField,
  toaster,
} from '@glzr/components';
import { join, sep } from '@tauri-apps/api/path';
import { createForm, Field, FormState } from 'smorf';
import { createEffect, createResource, on } from 'solid-js';
import { configSchemas } from 'zebar';
import * as z from 'zod';

import { ImageSelector, WidgetPack } from '~/common';

export type WidgetPackFormData = Omit<
  z.infer<typeof configSchemas.widgetPack>,
  'widgetPaths'
>;

export interface WidgetPackFormProps {
  pack: WidgetPack;
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
    { schema: configSchemas.widgetPack },
  );

  const [imagePaths] = createResource(
    () => form.getFieldValue('previewImages'),
    async images => {
      return Promise.all(
        images.map(image => join(props.pack.directoryPath, image)),
      );
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

    // Filter out images that are not within the pack directory.
    const validImages = images
      .filter(image => image.startsWith(pathPrefix))
      .map(image => image.replace(pathPrefix, ''));

    // Check if any images were outside the pack directory.
    if (validImages.length !== images.length) {
      toaster.show({
        title:
          'Some images were outside the pack directory and were ignored.',
        description: `${images.length - validImages.length} image(s) were ignored.`,
        variant: 'destructive',
      });
    }

    form.setFieldValue('previewImages', validImages);
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
