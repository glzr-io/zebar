import { Button } from '@glzr/components';
import { IconPlayerPause } from '@tabler/icons-solidjs';

import { WidgetPack } from '~/common';

export type PreviewBarProps = {
  pack: WidgetPack;
  onStop: () => void;
};

export function PreviewBar(props: PreviewBarProps) {
  return (
    <div class="fixed bottom-4 left-1/2 -translate-x-1/2 bg-black rounded-lg px-4 py-3 shadow-lg flex items-center gap-4 text-white">
      <p>
        Previewing <span class="font-medium">{props.pack.name}</span>
      </p>

      <Button variant="ghost" size="sm" onClick={() => props.onStop()}>
        <IconPlayerPause class="h-4 w-4 mr-2" />
        Stop Preview
      </Button>
    </div>
  );
}
