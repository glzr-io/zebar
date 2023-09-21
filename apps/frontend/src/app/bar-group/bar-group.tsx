import { For, createMemo } from 'solid-js';
import { BarComponent } from '~/bar-component/bar-component';

import { GroupConfig } from '~/shared/user-config';
import { clsx } from '~/shared/utils';

export interface BarGroupProps {
  config: GroupConfig;
}

export function BarGroup(props: BarGroupProps) {
  const componentConfigs = createMemo(() => props.config.components ?? []);

  return (
    <div class={clsx(props.config.id, props.config.class_name)}>
      <For each={componentConfigs()}>
        {componentConfig => <BarComponent config={componentConfig} />}
      </For>
    </div>
  );
}
