import {
  Badge,
  Button,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  Card,
  CardContent,
  toaster,
} from '@glzr/components';
import { useParams } from '@solidjs/router';
import {
  IconDownload,
  IconEye,
  IconChevronLeft,
  IconChevronRight,
  IconBrandGithub,
} from '@tabler/icons-solidjs';
import { open as shellOpen } from '@tauri-apps/plugin-shell';
import { createResource, createSignal, Show } from 'solid-js';

import {
  AppBreadcrumbs,
  Markdown,
  MarketplaceWidgetPack,
  useApiClient,
  useMarketplacePacks,
  useWidgetPreview,
} from '~/common';

export function MarketplacePackPage() {
  const params = useParams();
  const apiClient = useApiClient();
  const marketplacePacks = useMarketplacePacks();
  const widgetPreview = useWidgetPreview();

  const [currentImageIndex, setCurrentImageIndex] = createSignal(0);

  const [pack] = createResource(() =>
    apiClient.widgetPack.getByPublishedId.query({ id: params.id }),
  );

  const [readmeFile] = createResource(
    () => pack()?.readmeUrl,
    readmeUrl => fetch(readmeUrl).then(res => res.text()),
  );

  function nextImage(selectedPack: MarketplaceWidgetPack) {
    setCurrentImageIndex(prev =>
      prev === selectedPack.previewImageUrls.length - 1 ? 0 : prev + 1,
    );
  }

  function previousImage(selectedPack: MarketplaceWidgetPack) {
    setCurrentImageIndex(prev =>
      prev === 0 ? selectedPack.previewImageUrls.length - 1 : prev - 1,
    );
  }

  return (
    <div class="container mx-auto pt-3.5 pb-32">
      <Show when={pack()}>
        {selectedPack => (
          <div class="space-y-8">
            <div class="space-y-3">
              <AppBreadcrumbs
                entries={[
                  { href: '/marketplace', content: 'Marketplace' },
                  {
                    href: `/marketplace/${selectedPack().publishedId}`,
                    content: selectedPack().publishedId,
                  },
                ]}
              />

              {/* Header */}
              <div class="flex items-center gap-4">
                <h1 class="text-3xl font-bold tracking-tight">
                  {selectedPack().name}
                </h1>

                <Badge variant="secondary" class="h-6">
                  v{selectedPack().latestVersion}
                </Badge>
              </div>

              <div class="flex items-center gap-4 text-sm text-muted-foreground">
                <div class="flex items-center gap-2">
                  <span>
                    by {selectedPack().publishedId.split('.')[0]}
                  </span>
                </div>
                <span>
                  Published{' '}
                  {new Date(
                    selectedPack().versions?.[0].createdAt,
                  ).toLocaleDateString()}
                </span>
              </div>
            </div>

            {/* Gallery */}
            <div class="relative aspect-[2/1] w-full overflow-hidden rounded-lg bg-muted">
              <img
                src={
                  selectedPack().previewImageUrls[currentImageIndex()] ||
                  '/placeholder.svg'
                }
                alt={`${selectedPack().name} preview ${currentImageIndex() + 1}`}
                class="absolute left-1/2 top-1/2 h-full w-full -translate-x-1/2 -translate-y-1/2 object-contain"
              />
              <div class="absolute inset-0 flex items-center justify-between p-4">
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => previousImage(selectedPack())}
                  class="h-8 w-8 bg-background/50 backdrop-blur-sm"
                >
                  <IconChevronLeft class="h-4 w-4" />
                </Button>
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => nextImage(selectedPack())}
                  class="h-8 w-8 bg-background/50 backdrop-blur-sm"
                >
                  <IconChevronRight class="h-4 w-4" />
                </Button>
              </div>
              <div class="absolute bottom-4 left-1/2 -translate-x-1/2 flex gap-2">
                {selectedPack().previewImageUrls.map((_, index) => (
                  <button
                    class={`h-1.5 w-1.5 rounded-full ${index === currentImageIndex() ? 'bg-white' : 'bg-white/50'}`}
                    onClick={() => setCurrentImageIndex(index)}
                  />
                ))}
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-[1fr_300px] gap-8">
              {/* Action buttons and repo - full width on mobile, sidebar on desktop. */}
              <div class="space-y-4 md:order-2 md:col-start-2">
                <div class="flex flex-col gap-2">
                  <Button
                    class="w-full"
                    onClick={async () => {
                      await marketplacePacks.install(selectedPack());
                      toaster.show({
                        title: 'Widget pack installed!',
                        description: `Widget pack ${selectedPack().name} v${selectedPack().latestVersion} installed successfully.`,
                        variant: 'default',
                      });
                    }}
                  >
                    <IconDownload class="mr-2 h-4 w-4" />
                    Install
                  </Button>
                  <Button
                    variant="outline"
                    class="w-full"
                    onClick={() =>
                      widgetPreview.startPreview(selectedPack())
                    }
                  >
                    <IconEye class="mr-2 h-4 w-4" />
                    Preview
                  </Button>
                </div>

                <Card>
                  <CardContent>
                    <Show when={selectedPack().repositoryUrl}>
                      {repositoryUrl => (
                        <div class="space-y-2 mt-3">
                          <h3 class="font-medium">Repository</h3>
                          <a
                            class="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground cursor-pointer"
                            onClick={e => {
                              e.preventDefault();
                              shellOpen(repositoryUrl());
                            }}
                          >
                            <IconBrandGithub class="h-4 w-4" />
                            {new URL(repositoryUrl()).pathname.slice(1)}
                          </a>
                        </div>
                      )}
                    </Show>

                    <div class="space-y-2 mt-1">
                      <h3 class="font-medium">Tags</h3>
                      <div class="flex flex-wrap gap-2">
                        {selectedPack().tags.map(tag => (
                          <Badge variant="secondary">{tag}</Badge>
                        ))}
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </div>

              {/* Left side with tab list. */}
              <div class="md:row-span-2">
                {/* Content */}
                <Tabs defaultValue="readme" class="space-y-6">
                  <TabsList>
                    <TabsTrigger value="readme">Readme</TabsTrigger>
                    <TabsTrigger value="widgets">
                      Included widgets
                      <Badge class="ml-1" variant="secondary">
                        {selectedPack().widgetNames.length}
                      </Badge>
                    </TabsTrigger>
                    <TabsTrigger value="versions">
                      Versions
                      <Badge class="ml-1" variant="secondary">
                        {selectedPack().versions.length}
                      </Badge>
                    </TabsTrigger>
                  </TabsList>

                  <TabsContent value="readme" class="space-y-6">
                    <div class="prose prose-sm dark:prose-invert max-w-none">
                      <div class="whitespace-pre-line">
                        <Show when={readmeFile()}>
                          {readmeFile => (
                            <Markdown children={readmeFile()} />
                          )}
                        </Show>
                      </div>
                    </div>
                  </TabsContent>

                  <TabsContent value="widgets" class="space-y-6">
                    <div class="grid gap-6 sm:grid-cols-2">
                      {selectedPack().widgetNames.map(widgetName => (
                        <div class="group relative space-y-3">
                          <div>
                            <h3 class="font-medium">{widgetName}</h3>
                          </div>
                        </div>
                      ))}
                    </div>
                  </TabsContent>

                  <TabsContent value="versions" class="space-y-6">
                    <div class="space-y-4">
                      {selectedPack().versions.map(version => (
                        <div class="flex items-start justify-between border-b pb-4">
                          <div class="space-y-1">
                            <div class="flex items-center gap-2">
                              <h3 class="font-medium">
                                v{version.version}
                              </h3>
                              <span class="text-sm text-muted-foreground">
                                •{' '}
                                {new Date(
                                  version.createdAt,
                                ).toLocaleDateString()}
                              </span>
                            </div>
                            <p class="text-sm text-muted-foreground">
                              {version.releaseNotes}
                            </p>
                          </div>
                          <div class="text-sm text-muted-foreground">
                            <code class="px-2 py-1 rounded-md bg-muted">
                              {version.commitSha}
                            </code>
                          </div>
                        </div>
                      ))}
                    </div>
                  </TabsContent>
                </Tabs>
              </div>
            </div>
          </div>
        )}
      </Show>
    </div>
  );
}
