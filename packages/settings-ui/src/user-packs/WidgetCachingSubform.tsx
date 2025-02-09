import {
  Button,
  TextField,
  SelectField,
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
      // Default to one week (in seconds).
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
    <div>
      <h3 class="text-lg font-semibold">Cache settings</h3>
      <p class="text-sm text-muted-foreground mb-3">
        Any web requests made by the widget will be cached for offline
        availability & faster loading.
      </p>

      <div class="space-y-2 mb-3">
        <Field of={cachingForm} path="defaultDuration">
          {inputProps => (
            <CacheDurationField
              id="default-duration"
              label="Default cache duration"
              {...inputProps()}
            />
          )}
        </Field>
      </div>

      <h3 class="text-sm font-medium">Cache rules</h3>
      <div class="space-y-2">
        {cachingForm.value.rules.map((_, index) => (
          <div class="flex items-center space-x-2">
            <Field of={cachingForm} path={`rules.${index}.urlRegex`}>
              {inputProps => (
                <TextField
                  id="rule-url"
                  placeholder="URL regex"
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
          Add cache rule +
        </Button>
      </div>
    </div>
  );
}

interface CacheDurationFieldProps {
  id?: string;
  label?: string;
  value: number;
  onChange: (value: number) => void;
  onBlur: () => void;
}

enum SelectOptions {
  OneHour,
  OneDay,
  OneWeek,
  NoCache,
  Custom,
}

function CacheDurationField(props: CacheDurationFieldProps) {
  const [selectValue, setSelectValue] = createSignal<SelectOptions>(
    toSelectValue(props.value),
  );

  const [customValue, setCustomValue] = createSignal<number | undefined>(
    selectValue() === SelectOptions.Custom ? props.value : undefined,
  );

  // Update the select value when the props value changes.
  createEffect(
    on(
      () => props.value,
      newValue => {
        const selectValue = toSelectValue(newValue);
        setSelectValue(selectValue);
        setCustomValue(
          selectValue === SelectOptions.Custom ? newValue : undefined,
        );
      },
      { defer: true },
    ),
  );

  function toSelectValue(value: number) {
    switch (value) {
      case 60 * 60:
        return SelectOptions.OneHour;
      case 24 * 60 * 60:
        return SelectOptions.OneDay;
      case 7 * 24 * 60 * 60:
        return SelectOptions.OneWeek;
      case 0:
        return SelectOptions.NoCache;
      default:
        return SelectOptions.Custom;
    }
  }

  function toDuration(value: SelectOptions) {
    switch (value) {
      case SelectOptions.OneHour:
        return 60 * 60;
      case SelectOptions.OneDay:
        return 24 * 60 * 60;
      case SelectOptions.OneWeek:
        return 7 * 24 * 60 * 60;
      case SelectOptions.NoCache:
        return 0;
      case SelectOptions.Custom:
        return customValue();
    }
  }

  return (
    <>
      <SelectField
        id={props.id}
        placeholder="Select cache duration"
        label={props.label}
        options={
          [
            { value: SelectOptions.OneHour, label: '1 hour' },
            { value: SelectOptions.OneDay, label: '1 day' },
            { value: SelectOptions.OneWeek, label: '1 week' },
            { value: SelectOptions.Custom, label: 'Custom' },
            {
              value: SelectOptions.NoCache,
              label: 'No cache',
            },
          ] as const
        }
        value={selectValue()}
        onChange={(value: SelectOptions) => {
          setSelectValue(value);

          if (value !== SelectOptions.Custom) {
            props.onChange(toDuration(value));
          }
        }}
        onBlur={props.onBlur}
      />

      <Show when={selectValue() === SelectOptions.Custom}>
        <NumberField
          id={props.id}
          placeholder="Cache duration (seconds)"
          value={customValue()}
          onChange={value => {
            setCustomValue(value);
            props.onChange(value);
          }}
          onBlur={props.onBlur}
        />
      </Show>
    </>
  );
}
