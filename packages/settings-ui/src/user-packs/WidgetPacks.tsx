import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Badge,
  Button,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  AlertDialog,
  AlertDialogAction,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  AlertDialogClose,
} from '@glzr/components';
import {
  IconBrandGithub,
  IconDownload,
  IconEye,
  IconFolderPlus,
  IconPackage,
  IconSettings,
  IconTrash,
} from '@tabler/icons-solidjs';
import { For } from 'solid-js';

import { useUserPacks } from '~/common';

export function WidgetPacks() {
  const {
    widgetConfigs,
    widgetStates,
    updateWidgetConfig,
    togglePreset,
    communityPacks,
    localPacks,
  } = useUserPacks();

  const handleDeletePack = (packName, isLocal) => {
    // TODO
  };

  const WidgetPackCard = ({ pack, isLocal }) => (
    <Card class="mb-4">
      <CardHeader>
        <div class="flex justify-between items-start">
          <div>
            <CardTitle class="flex items-center gap-2">
              <IconPackage class="h-5 w-5" />
              {pack.name}
              <Badge variant="outline" class="ml-2">
                {pack.version}
              </Badge>
            </CardTitle>
            <CardDescription class="mt-1">
              by {pack.author}
            </CardDescription>
          </div>
          <div class="flex gap-2">
            <Tooltip>
              <TooltipTrigger>
                <Button variant="outline" size="icon">
                  <IconEye class="h-4 w-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Preview Widgets</TooltipContent>
            </Tooltip>

            <Tooltip>
              <TooltipTrigger>
                <Button variant="outline" size="icon">
                  <IconSettings class="h-4 w-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Configure Pack</TooltipContent>
            </Tooltip>

            <AlertDialog>
              <AlertDialogTrigger>
                <Button
                  variant="outline"
                  size="icon"
                  class="text-red-500 hover:text-red-600"
                >
                  <IconTrash class="h-4 w-4" />
                </Button>
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>Delete Widget Pack</AlertDialogTitle>
                  <AlertDialogDescription>
                    Are you sure you want to delete "{pack.name}"? This
                    action cannot be undone.
                  </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                  <AlertDialogClose>Cancel</AlertDialogClose>
                  <AlertDialogAction
                    onClick={() => handleDeletePack(pack.name, isLocal)}
                    class="bg-red-500 hover:bg-red-600"
                  >
                    Delete
                  </AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
          </div>
        </div>
      </CardHeader>

      <CardContent>
        <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
          {pack.description}
        </p>
        <div class="flex flex-wrap gap-2 mb-4">
          {pack.tags.map(tag => (
            <Badge key={tag} variant="secondary">
              {tag}
            </Badge>
          ))}
        </div>
        <div class="text-sm">
          <p>
            <strong>Widgets:</strong> {pack.widgets.length}
          </p>
          <p>
            <strong>License:</strong> {pack.license}
          </p>
        </div>
      </CardContent>
    </Card>
  );

  return (
    <div class="container mx-auto p-6">
      <div class="flex justify-between items-center mb-6">
        <h1 class="text-3xl font-bold">Widget Packs</h1>
        <div class="flex gap-2">
          <Dialog>
            <DialogTrigger>
              <Button>
                <IconDownload class="mr-2 h-4 w-4" />
                Browse Community Packs
              </Button>
            </DialogTrigger>
            <DialogContent class="max-w-2xl">
              <DialogHeader>
                <DialogTitle>Community Widget Packs</DialogTitle>
                <DialogDescription>
                  Browse and install widget packs created by the community
                </DialogDescription>
              </DialogHeader>
              {/* Community marketplace content would go here */}
            </DialogContent>
          </Dialog>

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
            {pack => <WidgetPackCard pack={pack} isLocal={false} />}
          </For>
        </TabsContent>

        <TabsContent value="local" class="mt-6">
          <For each={localPacks()}>
            {pack => <WidgetPackCard pack={pack} isLocal={true} />}
          </For>
        </TabsContent>
      </Tabs>
    </div>
  );
}
