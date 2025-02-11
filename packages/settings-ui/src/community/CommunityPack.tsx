import {
  Badge,
  Button,
  Separator,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  Breadcrumb,
  BreadcrumbList,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbSeparator,
  BreadcrumbPage,
  Card,
  CardContent,
} from '@glzr/components';
import { A, useParams } from '@solidjs/router';
import {
  IconDownload,
  IconEye,
  IconHeart,
  IconShare2,
  IconArrowLeft,
  IconChevronLeft,
  IconChevronRight,
  IconBrandGithub,
} from '@tabler/icons-solidjs';

import { createEffect, createSignal, Show } from 'solid-js';

import { useCommunityPacks, WidgetPack } from '~/common';

export function CommunityPack() {
  const params = useParams();
  const communityPacks = useCommunityPacks();

  const [currentImageIndex, setCurrentImageIndex] = createSignal(0);

  createEffect(() => communityPacks.selectPack(params.id));

  function nextImage(selectedPack: WidgetPack) {
    setCurrentImageIndex(prev =>
      prev === selectedPack.galleryUrls.length - 1 ? 0 : prev + 1,
    );
  }

  function previousImage(selectedPack: WidgetPack) {
    setCurrentImageIndex(prev =>
      prev === 0 ? selectedPack.galleryUrls.length - 1 : prev - 1,
    );
  }

  return (
    <div class="container mx-auto pt-6 pb-40">
      <Show when={communityPacks.selectedPack()}>
        {selectedPack => (
          <div class="space-y-8">
            {/* Header */}
            <div class="space-y-3">
              <div class="flex items-center gap-2">
                <Breadcrumb>
                  <BreadcrumbList>
                    <BreadcrumbItem>
                      <BreadcrumbLink href="/community">
                        Browse Community
                      </BreadcrumbLink>
                    </BreadcrumbItem>
                    <BreadcrumbSeparator />
                    <BreadcrumbItem>
                      <BreadcrumbPage>
                        {selectedPack().name}
                      </BreadcrumbPage>
                    </BreadcrumbItem>
                  </BreadcrumbList>
                </Breadcrumb>
              </div>

              <div class="flex items-center gap-4">
                <h1 class="text-3xl font-bold tracking-tight">
                  {selectedPack().name}
                </h1>

                <Badge variant="secondary" class="h-6">
                  v{selectedPack().version}
                </Badge>
              </div>

              <div class="flex items-center gap-4 text-sm text-muted-foreground">
                <div class="flex items-center gap-2">
                  <img
                    src="https://placehold.co/200x200"
                    alt={selectedPack().author}
                    width={24}
                    height={24}
                    class="rounded-full"
                  />
                  <span>by {selectedPack().author}</span>
                </div>
                <span>
                  Published{' '}
                  {new Date(
                    selectedPack().versions?.[0].publishDate,
                  ).toLocaleDateString()}
                </span>
              </div>
            </div>

            {/* Gallery */}
            <div class="relative aspect-[2/1] w-full overflow-hidden rounded-lg bg-muted">
              <img
                src={
                  selectedPack().galleryUrls[currentImageIndex()] ||
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
                {selectedPack().galleryUrls.map((_, index) => (
                  <button
                    class={`h-1.5 w-1.5 rounded-full ${index === currentImageIndex() ? 'bg-white' : 'bg-white/50'}`}
                    onClick={() => setCurrentImageIndex(index)}
                  />
                ))}
              </div>
            </div>

            {/* Header */}
            {/* <div class="flex items-start justify-between">
              <div class="flex items-center gap-2">
                <Button
                  variant="outline"
                  onClick={() =>
                    communityPacks.startPreview(selectedPack())
                  }
                >
                  <IconEye class="mr-2 h-4 w-4" />
                  Preview
                </Button>
                <Button
                  onClick={() => communityPacks.install(selectedPack())}
                >
                  <IconDownload class="mr-2 h-4 w-4" />
                  Install
                </Button>
              </div>
            </div> */}

            <div class="grid grid-cols-1 md:grid-cols-[1fr_300px] gap-8">
              {/* Action buttons and repo - full width on mobile, sidebar on desktop */}
              <div class="space-y-4 md:order-2 md:col-start-2">
                <div class="flex flex-col gap-2">
                  <Button
                    class="w-full"
                    onClick={() => communityPacks.install(selectedPack())}
                  >
                    <IconDownload class="mr-2 h-4 w-4" />
                    Install
                  </Button>
                  <Button
                    variant="outline"
                    class="w-full"
                    onClick={() =>
                      communityPacks.startPreview(selectedPack())
                    }
                  >
                    <IconEye class="mr-2 h-4 w-4" />
                    Preview
                  </Button>
                </div>

                <Card>
                  <CardContent>
                    <div class="space-y-2 mt-3">
                      <h3 class="font-medium">Repository</h3>
                      <a
                        href={selectedPack().versions?.[0].repoUrl}
                        class="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground"
                        target="_blank"
                      >
                        <IconBrandGithub class="h-4 w-4" />
                        {new URL(
                          selectedPack().versions?.[0].repoUrl,
                        ).pathname.slice(1)}
                      </a>
                    </div>

                    <div class="space-y-2 mt-1">
                      <h3 class="font-medium">Tags</h3>
                      <div class="flex flex-wrap gap-2">
                        {selectedPack().tags.map(tag => (
                          <Badge key={tag} variant="secondary">
                            {tag}
                          </Badge>
                        ))}
                      </div>
                    </div>
                  </CardContent>
                </Card>
              </div>

              {/* Left side with TabsList */}
              <div class="md:row-span-2">
                {/* Content */}
                <Tabs defaultValue="readme" class="space-y-6">
                  <TabsList>
                    <TabsTrigger value="readme">Readme</TabsTrigger>
                    <TabsTrigger value="widgets">
                      Included widgets
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
                        {selectedPack().description}
                      </div>
                    </div>
                  </TabsContent>

                  <TabsContent value="widgets" class="space-y-6">
                    <div class="grid gap-6 sm:grid-cols-2">
                      {selectedPack().widgets.map(widget => (
                        <div class="group relative space-y-3">
                          <div>
                            <h3 class="font-medium">{widget.name}</h3>
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
                                v{version.versionNumber}
                              </h3>
                              <span class="text-sm text-muted-foreground">
                                •{' '}
                                {new Date(
                                  version.publishDate,
                                ).toLocaleDateString()}
                              </span>
                            </div>
                            <p class="text-sm text-muted-foreground">
                              {version.releaseNotes}
                            </p>
                          </div>
                          <div class="text-sm text-muted-foreground">
                            <code class="px-2 py-1 rounded-md bg-muted">
                              {version.commitHash}
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
