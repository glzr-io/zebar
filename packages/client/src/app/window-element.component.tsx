import { Index, Show, createMemo, createSignal } from 'solid-js';
import {
  WindowConfig,
  WindowContext,
  getChildConfigs,
  initWindow,
  toCssSelector,
} from 'zebar';

import { ChildElement } from './child-element.component';

export function WindowElement() {
  const [context, setContext] = createSignal<WindowContext | null>(null);

  const childIds = createMemo(() =>
    !context()
      ? []
      : getChildConfigs(context()!.rawConfig as WindowConfig).map(
          ([key]) => key,
        ),
  );

  function getChildIds(context: WindowContext) {
    return getChildConfigs(context.rawConfig as any).map(([key]) => key);
  }

  initWindow(context => setContext(context));

  return (
    <Show when={context()}>
      {context => (
        <div
          id={toCssSelector(context().parsedConfig.id)}
          class={context().parsedConfig.class_name}
        >
          {/* <Index each={childIds()}> */}
          <Index each={getChildIds(context())}>
            {childId => (
              <ChildElement
                childId={childId()}
                parentContext={context()}
              />
            )}
          </Index>
        </div>
      )}
    </Show>
  );
}
