import { For } from 'solid-js';

import { BarGroup } from './bar-group.component';
import { useProviders } from '~/shared/providers';
import { BarConfig, getGroupConfigs } from '~/shared/user-config';

export interface BarProps {
  config: BarConfig;
}

export function Bar(props: BarProps) {
  const providers = useProviders(props.config.providers);

  return (
    <div id={props.config.id} class={props.config.class_name}>
      <For each={getGroupConfigs(props.config)}>
        {groupConfig => <BarGroup config={groupConfig} />}
      </For>
    </div>
  );
}
