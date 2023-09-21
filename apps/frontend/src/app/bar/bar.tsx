import { For, createMemo } from 'solid-js';

import { BarConfig, GroupConfig } from '~/shared/user-config';
import { BarGroup } from '~/bar-group/bar-group';
import { clsx } from '~/shared/utils';

export interface BarProps {
  config: BarConfig;
}

export function Bar(props: BarProps) {
  // Get group configs by filtering 'group/**' keys.
  const groupConfigs = createMemo(
    () =>
      Object.entries(props.config)
        .filter(([key, value]) => key.startsWith('group') && !!value)
        .map(([_, value]) => value) as GroupConfig[],
  );

  return (
    <div class={clsx(props.config.id, props.config.class_name)}>
      <For each={groupConfigs()}>
        {groupConfig => <BarGroup config={groupConfig} />}
      </For>
    </div>
  );
}
