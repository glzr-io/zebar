import {
  Button,
  Card,
  CardContent,
  TextField,
  ChipField,
  FormField,
  toaster,
} from '@glzr/components';
import { join, sep } from '@tauri-apps/api/path';
import { createForm, Field, FormState } from 'smorf';
import { createEffect, createResource, on } from 'solid-js';
import { configSchemas, type WidgetPack } from 'zebar';
import * as z from 'zod';

import { ImageSelector } from '~/common';

export type WidgetPackFormData = Omit<
  z.infer<typeof configSchemas.widgetPack>,
  'widgets'
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
      version: '0.0.0',
      tags: [],
      previewImages: [],
      repositoryUrl: '',
      mimeTypes: [],
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
          form.unsetDirty();
          form.unsetTouched();
          form.setValue({
            name: props.pack.name,
            version: props.pack.version,
            description: props.pack.description,
            tags: props.pack.tags,
            previewImages: props.pack.previewImages,
            repositoryUrl: props.pack.repositoryUrl,
            mimeTypes: props.pack.mimeTypes ?? [],
          });
        }
      },
    ),
  );

  // Broadcast the form changes to the parent.
  createEffect(
    on(
      () => form.value,
      () => props.onChange?.(form),
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

  function addMimeType() {
    form.setFieldValue('mimeTypes', mimeTypes => [
      ...mimeTypes,
      { extension: '', contentType: '' },
    ]);
  }

  function deleteMimeType(targetIndex: number) {
    form.setFieldValue('mimeTypes', mimeTypes =>
      mimeTypes.filter((_, index) => index !== targetIndex),
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
                disabled={props.disabled}
                error={field.error()}
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={form} path="version">
            {(inputProps, field) => (
              <TextField
                label="Version"
                placeholder="0.0.0"
                description="Version number when published to the marketplace."
                disabled={props.disabled}
                error={field.error()}
                {...inputProps()}
              />
            )}
          </Field>

          <Field of={form} path="description">
            {(inputProps, field) => (
              <TextField
                label="Description (optional)"
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
                label="Tags (optional)"
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

          <Field of={form} path="repositoryUrl">
            {(inputProps, field) => (
              <TextField
                label="Repository URL (optional)"
                description="The URL of the repository containing the widget pack."
                disabled={props.disabled}
                error={field.error()}
                {...inputProps()}
              />
            )}
          </Field>

          <div class="space-y-3">
            <div>
              <h3 class="font-medium">Custom asset MIME types</h3>
              <p class="text-sm text-muted-foreground">
                Override the content type used when serving files with
                these extensions. Extensions are case-insensitive and omit
                the dot.
              </p>
            </div>

            {form.value.mimeTypes.map((_, index) => (
              <div class="grid grid-cols-1 md:grid-cols-[1fr_1fr_auto] gap-2 items-end">
                <Field of={form} path={`mimeTypes.${index}.extension`}>
                  {(inputProps, field) => (
                    <TextField
                      label="Extension"
                      placeholder="tsx"
                      disabled={props.disabled}
                      error={field.error()}
                      {...inputProps()}
                    />
                  )}
                </Field>

                <Field of={form} path={`mimeTypes.${index}.contentType`}>
                  {(inputProps, field) => (
                    <TextField
                      label="Content type"
                      placeholder="text/javascript"
                      disabled={props.disabled}
                      error={field.error()}
                      {...inputProps()}
                    />
                  )}
                </Field>

                <Button
                  type="button"
                  variant="secondary"
                  disabled={props.disabled}
                  onClick={() => deleteMimeType(index)}
                >
                  Remove
                </Button>
              </div>
            ))}

            <Button
              type="button"
              variant="outline"
              disabled={props.disabled}
              onClick={addMimeType}
            >
              Add MIME type
            </Button>
          </div>
        </CardContent>
      </Card>
    </form>
  );
}
