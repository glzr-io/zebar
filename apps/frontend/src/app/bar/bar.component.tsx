import { For, createMemo } from 'solid-js';

import { BarGroup } from './bar-group.component';
import { useProviders } from '~/shared/providers';
import { BarConfig, GroupConfig } from '~/shared/user-config';

export interface BarProps {
  config: BarConfig;
}

export function Bar(props: BarProps) {
  const providers = useProviders(props.config.providers);

  // Get group configs by filtering 'group/**' keys.
  const groupConfigs = createMemo(
    () =>
      Object.entries(props.config)
        .filter(([key]) => key.startsWith('group/'))
        .map(([_, value]) => value) as GroupConfig[],
  );

  return (
    <div id={props.config.id} class={props.config.class_name}>
      <For each={groupConfigs()}>
        {groupConfig => <BarGroup config={groupConfig} />}
      </For>
    </div>
  );
}
