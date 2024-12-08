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
import { createEffect, createSignal, on, Show } from 'solid-js';
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
              <CacheDurationField
                id="default-duration"
                {...inputProps()}
              />
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
                    <CacheDurationField
                      id={`rule-duration-${index}`}
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

function CacheDurationField(props: {
  id?: string;
  label?: string;
  value: number;
  onChange: (value: number) => void;
}) {
  const [mode, setMode] = createSignal<'preset' | 'custom'>(
    isPresetDuration(props.value) ? 'preset' : 'custom',
  );

  const [customValue, setCustomValue] = createSignal(
    mode() === 'custom' ? props.value : 0,
  );

  // Sync with external value changes
  createEffect(
    on(
      () => props.value,
      newValue => {
        if (isPresetDuration(newValue)) {
          setMode('preset');
        } else {
          setMode('custom');
          setCustomValue(newValue);
        }
      },
    ),
  );

  function isPresetDuration(duration: number) {
    return [
      { value: 60 * 60, label: '1 hour' },
      { value: 24 * 60 * 60, label: '1 day' },
      { value: 7 * 24 * 60 * 60, label: '1 week' },
      { value: 0, label: 'No cache (network-only)' },
      { value: -1, label: 'Custom' },
    ].some(option => option.value === duration && option.value !== -1);
  }

  return (
    <>
      <SelectField
        id={props.id}
        placeholder="Select cache duration"
        options={[
          { value: 60 * 60, label: '1 hour' },
          { value: 24 * 60 * 60, label: '1 day' },
          { value: 7 * 24 * 60 * 60, label: '1 week' },
          { value: 0, label: 'No cache (network-only)' },
          { value: -1, label: 'Custom' },
        ]}
        value={mode() === 'preset' ? props.value : -1}
        onChange={value => {
          if (value === -1) {
            setMode('custom');
            props.onChange(customValue());
          } else {
            setMode('preset');
            props.onChange(value);
          }
        }}
      />

      <Show when={mode() === 'custom'}>
        <NumberField
          id={props.id}
          label={props.label ?? 'Cache duration'}
          placeholder="Enter cache duration (in seconds)"
          class="mt-2"
          value={customValue()}
          onChange={value => {
            setCustomValue(value);
            props.onChange(value);
          }}
        />
      </Show>
    </>
  );
}
