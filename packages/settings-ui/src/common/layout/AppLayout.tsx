import {
  Resizable,
  ResizableHandle,
  ResizablePanel,
} from '@glzr/components';
import { createSignal, Show, type JSX } from 'solid-js';
import { RouteSectionProps } from '@solidjs/router';

import { Sidebar } from './Sidebar';
import { PreviewBar } from './PreviewBar';
import { useMarketplacePacks } from '~/common';

export interface AppLayoutProps {
  children: JSX.Element;
}

export function AppLayout(props: AppLayoutProps & RouteSectionProps) {
  const communityPacks = useMarketplacePacks();
  const [sizes, setSizes] = createSignal<number[]>([0.2, 0.8]);

  return (
    <>
      <Resizable sizes={sizes()} onSizesChange={setSizes}>
        <Sidebar
          initialSize={sizes()[0]}
          onCollapseClick={() => setSizes([0, 1])}
        />

        <ResizableHandle withHandle />

        <ResizablePanel initialSize={sizes()[1]} class="overflow-auto">
          {props.children}
        </ResizablePanel>
      </Resizable>

      <Show when={communityPacks.previewPack()}>
        {pack => (
          <PreviewBar pack={pack()} onStop={communityPacks.stopPreview} />
        )}
      </Show>
    </>
  );
}
