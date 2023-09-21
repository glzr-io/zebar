import { For, createMemo } from 'solid-js';
import { BarComponent } from '~/bar-component/bar-component';

import { GroupConfig } from '~/shared/user-config';
import { clsx } from '~/shared/utils';

export interface GroupProps {
  config: GroupConfig;
}

export function Group(props: GroupProps) {
  const componentConfigs = createMemo(() => props.config.components);

  return (
    <div class={clsx(props.config.id, props.config.class_name)}>
      <For each={componentConfigs()}>
        {componentConfig => <BarComponent config={componentConfig} />}
      </For>
    </div>
  );
}
