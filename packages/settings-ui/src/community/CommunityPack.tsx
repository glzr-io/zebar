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
    <div class="container mx-auto py-6">
      <Show when={communityPacks.selectedPack()}>
        {selectedPack => (
          <div class="space-y-8">
            {/* Header */}
            <div class="flex items-center gap-2">
              <Breadcrumb>
                <BreadcrumbList>
                  <BreadcrumbItem>
                    <BreadcrumbLink href="/community">
                      Marketplace
                    </BreadcrumbLink>
                  </BreadcrumbItem>
                  <BreadcrumbSeparator />
                  <BreadcrumbItem>
                    <BreadcrumbPage>{selectedPack().name}</BreadcrumbPage>
                  </BreadcrumbItem>
                </BreadcrumbList>
              </Breadcrumb>
            </div>

            <h1 class="text-3xl font-bold tracking-tight">
              {selectedPack().name}
            </h1>

            {/* Gallery */}
            <div class="relative aspect-[2/1] w-full overflow-hidden rounded-lg bg-muted">
              <img
                src={
                  selectedPack().galleryUrls[currentImageIndex()] ||
                  '/placeholder.svg'
                }
                alt={`${selectedPack().name} preview ${currentImageIndex() + 1}`}
                class="object-cover"
              />
              <div class="absolute inset-0 flex items-center justify-between p-4">
                <Button
                  variant="outline"
                  size="icon"
                  onClick={previousImage}
                  class="h-8 w-8 bg-background/50 backdrop-blur-sm"
                >
                  <IconChevronLeft class="h-4 w-4" />
                </Button>
                <Button
                  variant="outline"
                  size="icon"
                  onClick={nextImage}
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
            <div class="flex items-start justify-between">
              <div class="space-y-2">
                <h1 class="text-3xl font-bold tracking-tight">
                  {selectedPack().name}
                </h1>
                <div class="flex items-center gap-4">
                  <div class="flex items-center gap-2">
                    <img
                      src="/placeholder.svg"
                      alt={selectedPack().author}
                      width={24}
                      height={24}
                      class="rounded-full"
                    />
                    <span class="text-sm text-muted-foreground">
                      by {selectedPack().author}
                    </span>
                  </div>
                  <Separator orientation="vertical" class="h-4" />
                  {/* <div class="flex items-center gap-4 text-sm text-muted-foreground">
                  <span class="flex items-center gap-1">
                    <IconDownload class="h-4 w-4" />
                    {selectedPack().downloads.toLocaleString()}
                  </span>
                  <span class="flex items-center gap-1">
                    <IconUsers class="h-4 w-4" />
                    {selectedPack().views.toLocaleString()}
                  </span>
                  <span class="flex items-center gap-1">
                    <IconHeart class="h-4 w-4" />
                    {selectedPack().likes.toLocaleString()}
                  </span>
                  <span class="flex items-center gap-1">
                    <IconClock class="h-4 w-4" />
                    Last updated{' '}
                    {new Date(
                      selectedPack().lastUpdated,
                    ).toLocaleDateString()}
                  </span>
                </div> */}
                </div>
              </div>
              <div class="flex items-center gap-2">
                <Button variant="outline" size="icon">
                  <IconShare2 class="h-4 w-4" />
                </Button>
                <Button variant="outline" size="icon">
                  <IconHeart class="h-4 w-4" />
                </Button>
                <Button variant="outline">
                  <IconEye class="mr-2 h-4 w-4" />
                  Preview
                </Button>
                <Button>
                  <IconDownload class="mr-2 h-4 w-4" />
                  Install
                </Button>
              </div>
            </div>

            {/* Content */}
            <Tabs defaultValue="about" class="space-y-6">
              <TabsList>
                <TabsTrigger value="about">About</TabsTrigger>
                <TabsTrigger value="widgets">Included Widgets</TabsTrigger>
              </TabsList>

              <TabsContent value="about" class="space-y-6">
                <div class="prose prose-sm dark:prose-invert max-w-none">
                  <div class="whitespace-pre-line">
                    {selectedPack().description}
                  </div>
                </div>
                <div class="flex flex-wrap gap-2">
                  {selectedPack().tags.map(tag => (
                    <Badge key={tag} variant="secondary">
                      {tag}
                    </Badge>
                  ))}
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
            </Tabs>
          </div>
        )}
      </Show>
    </div>
  );
}
