import { For } from 'solid-js';

import { BarGroup } from './bar-group.component';
import { BarConfig, getGroupConfigs } from '~/shared/user-config';
import { ProviderNode } from '~/shared/providers';

export interface BarProps {
  config: BarConfig;
  provider: ProviderNode;
}

export function Bar(props: BarProps) {
  return (
    <div id={props.config.id} class={props.config.class_name}>
      <For each={getGroupConfigs(props.config)}>
        {groupConfig => (
          <BarGroup config={groupConfig} provider={props.provider} />
        )}
      </For>
    </div>
  );
}
