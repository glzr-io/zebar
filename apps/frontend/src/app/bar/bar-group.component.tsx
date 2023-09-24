import { For } from 'solid-js';

import { BarComponent } from './bar-component.component';
import { useProviders } from '~/shared/providers';
import { GroupConfig } from '~/shared/user-config';
import { clsx } from '~/shared/utils';

export interface BarGroupProps {
  config: GroupConfig;
}

export function BarGroup(props: BarGroupProps) {
  const providers = useProviders(props.config.providers);

  return (
    <div class={clsx(props.config.id, props.config.class_name)}>
      <For each={props.config.components}>
        {componentConfig => <BarComponent config={componentConfig} />}
      </For>
    </div>
  );
}
