import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  TextField,
  SelectField,
  CardDescription,
  NumberField,
} from '@glzr/components';
import { IconX } from '@tabler/icons-solidjs';
import { createForm, Field } from 'smorf';
import { createEffect, on, Show } from 'solid-js';
import type { WidgetCaching } from 'zebar';

export interface WidgetCachingSubformProps {
  value?: WidgetCaching;
  onChange: (caching: WidgetCaching) => void;
}

export function WidgetCachingSubform(props: WidgetCachingSubformProps) {
  const cachingForm = createForm<WidgetCaching>(
    props.value ?? {
      defaultDuration: 7 * 24 * 60 * 60,
      rules: [],
    },
  );

  createEffect(
    on(
      () => props.value,
      value => cachingForm.setValue(value),
    ),
  );

  createEffect(
    on(
      () => cachingForm.value,
      value => {
        if (cachingForm.isDirty()) {
          props.onChange(value);
        }
      },
    ),
  );

  return (
    <Card class="w-full max-w-3xl">
      <CardHeader>
        <CardTitle>Cache Settings</CardTitle>
        <CardDescription>
          Configure your caching preferences and rules
        </CardDescription>
      </CardHeader>

      <CardContent class="space-y-6">
        <div class="space-y-2">
          <Field of={cachingForm} path="defaultDuration">
            {inputProps => (
              <>
                <SelectField
                  id="default-duration-select"
                  label="Default cache duration"
                  placeholder="Select default cache duration"
                  options={[
                    {
                      value: 60 * 60,
                      label: '1 hour',
                    },
                    {
                      value: 24 * 60 * 60,
                      label: '1 day',
                    },
                    {
                      value: 7 * 24 * 60 * 60,
                      label: '1 week',
                    },
                    {
                      value: 1,
                      label: 'Custom',
                    },
                    {
                      value: 0,
                      label: 'No cache (network-only)',
                    },
                  ]}
                  {...inputProps()}
                />

                <Show
                  when={
                    inputProps().value !== 0 &&
                    inputProps().value !== 60 * 60 &&
                    inputProps().value !== 24 * 60 * 60 &&
                    inputProps().value !== 7 * 24 * 60 * 60
                  }
                >
                  <NumberField
                    id="default-duration-number"
                    label="Default cache duration"
                    placeholder="Enter default cache duration (in seconds)"
                    class="mt-2"
                    {...inputProps()}
                  />
                </Show>
              </>
            )}
          </Field>
        </div>

        <div class="space-y-2">
          <h3 class="text-sm font-medium">Cache rules</h3>
          <div class="space-y-2">
            {cachingForm.value.rules.map((_, index) => (
              <div class="flex items-center space-x-2">
                <Field of={cachingForm} path={`rules.${index}.urlRegex`}>
                  {inputProps => (
                    <TextField
                      id="rule-url"
                      placeholder="URL pattern"
                      class="flex-grow"
                      {...inputProps()}
                    />
                  )}
                </Field>

                <Field of={cachingForm} path={`rules.${index}.duration`}>
                  {inputProps => (
                    <SelectField
                      id="default-duration-select"
                      placeholder="Select default cache duration"
                      options={[
                        {
                          value: 60 * 60,
                          label: '1 hour',
                        },
                        {
                          value: 24 * 60 * 60,
                          label: '1 day',
                        },
                        {
                          value: 7 * 24 * 60 * 60,
                          label: '1 week',
                        },
                        {
                          value: 0,
                          label: 'No cache',
                        },
                      ]}
                      {...inputProps()}
                    />
                  )}
                </Field>

                <Button
                  type="button"
                  variant="outline"
                  size="icon"
                  onClick={() =>
                    cachingForm.setFieldValue('rules', rules =>
                      rules.filter((_, i) => i !== index),
                    )
                  }
                  aria-label="Remove rule"
                >
                  <IconX class="h-4 w-4" />
                </Button>
              </div>
            ))}

            <Button
              type="button"
              variant="outline"
              onClick={() =>
                cachingForm.setFieldValue('rules', rules => [
                  ...rules,
                  { urlRegex: '', duration: 0 },
                ])
              }
              class="w-full"
            >
              Add cache rule
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
