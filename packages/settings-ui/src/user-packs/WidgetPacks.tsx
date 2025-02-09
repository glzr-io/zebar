import {
  Button,
  Dialog,
  DialogTrigger,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@glzr/components';
import { IconBrandGithub, IconFolderPlus } from '@tabler/icons-solidjs';
import { For } from 'solid-js';

import { useUserPacks } from '~/common';
import { WidgetPackCard } from './WidgetPackCard';
import { CreateWidgetPackDialog } from './dialogs/CreateWidgetPackDialog';

export function WidgetPacks() {
  const userPacks = useUserPacks();

  return (
    <div class="container mx-auto p-6">
      <div class="flex justify-between items-center mb-6">
        <h1 class="text-3xl font-bold">Widget Packs</h1>
        <div class="flex gap-2">
          <Dialog>
            <DialogTrigger>
              <Button variant="outline">
                <IconFolderPlus class="mr-2 h-4 w-4" />
                Create New Pack
              </Button>
            </DialogTrigger>
            <CreateWidgetPackDialog onSubmit={userPacks.createPack} />
          </Dialog>

          <Button variant="outline">
            <IconBrandGithub class="mr-2 h-4 w-4" />
            Submit to Community
          </Button>
        </div>
      </div>

      <Tabs defaultValue="installed" class="w-full">
        <TabsList>
          <TabsTrigger value="installed">
            Installed ({userPacks.communityPacks()?.length})
          </TabsTrigger>
          <TabsTrigger value="local">
            Local ({userPacks.localPacks()?.length})
          </TabsTrigger>
        </TabsList>

        <TabsContent value="installed" class="mt-6">
          <For each={userPacks.communityPacks()}>
            {pack => (
              <WidgetPackCard
                pack={pack}
                isLocal={false}
                onDelete={userPacks.deletePack}
              />
            )}
          </For>
        </TabsContent>

        <TabsContent value="local" class="mt-6">
          <For each={userPacks.localPacks()}>
            {pack => (
              <WidgetPackCard
                pack={pack}
                isLocal={true}
                onDelete={userPacks.deletePack}
              />
            )}
          </For>
        </TabsContent>
      </Tabs>
    </div>
  );
}
