import {
  Button,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@glzr/components';
import { IconBrandGithub, IconFolderPlus } from '@tabler/icons-solidjs';
import { For } from 'solid-js';

import { useUserPacks } from '~/common';
import { WidgetPackCard } from './WidgetPackCard';

export function WidgetPacks() {
  const {
    widgetConfigs,
    widgetStates,
    updateWidgetConfig,
    togglePreset,
    communityPacks,
    localPacks,
  } = useUserPacks();

  function handleDeletePack(packId: string) {
    // TODO
  }

  return (
    <div class="container mx-auto p-6">
      <div class="flex justify-between items-center mb-6">
        <h1 class="text-3xl font-bold">Widget Packs</h1>
        <div class="flex gap-2">
          <Button variant="outline">
            <IconFolderPlus class="mr-2 h-4 w-4" />
            Create New Pack
          </Button>

          <Button variant="outline">
            <IconBrandGithub class="mr-2 h-4 w-4" />
            Submit to Community
          </Button>
        </div>
      </div>

      <Tabs defaultValue="installed" class="w-full">
        <TabsList>
          <TabsTrigger value="installed">
            Installed ({communityPacks.length})
          </TabsTrigger>
          <TabsTrigger value="local">
            Local ({localPacks.length})
          </TabsTrigger>
        </TabsList>

        <TabsContent value="installed" class="mt-6">
          <For each={communityPacks()}>
            {pack => (
              <WidgetPackCard
                pack={pack}
                isLocal={false}
                onDelete={handleDeletePack}
              />
            )}
          </For>
        </TabsContent>

        <TabsContent value="local" class="mt-6">
          <For each={localPacks()}>
            {pack => (
              <WidgetPackCard
                pack={pack}
                isLocal={true}
                onDelete={handleDeletePack}
              />
            )}
          </For>
        </TabsContent>
      </Tabs>
    </div>
  );
}
