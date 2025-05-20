import { Button, TextField, toaster } from '@glzr/components';
import { A } from '@solidjs/router';
import {
  IconAlertCircle,
  IconDownload,
  IconEye,
} from '@tabler/icons-solidjs';
import { createForm, Field } from 'smorf';
import { createMemo } from 'solid-js';

import {
  AppBreadcrumbs,
  useMarketplacePacks,
  useWidgetPreview,
} from '~/common';

type FilterQuery = {
  search: string;
};

export function MarketplacePage() {
  const marketplacePacks = useMarketplacePacks();
  const widgetPreview = useWidgetPreview();

  const filterQueryForm = createForm<FilterQuery>({
    search: '',
  });

  const filteredPacks = createMemo(() =>
    (marketplacePacks.allPacks() ?? []).filter(pack => {
      const searchQuery = filterQueryForm.value.search;

      const matchesSearch =
        pack.publishedId
          .toLowerCase()
          .includes(searchQuery.toLowerCase()) ||
        pack.description.toLowerCase().includes(searchQuery.toLowerCase());

      return matchesSearch;
    }),
  );

  return (
    <div class="container mx-auto pt-3.5 pb-32 space-y-6">
      <div class="space-y-3">
        <AppBreadcrumbs
          entries={[{ href: '/marketplace', content: 'Marketplace' }]}
        />

        <h1 class="text-3xl font-bold tracking-tight">Marketplace</h1>

        <p class="text-muted-foreground">
          Discover and install widget packs that other users have created.
        </p>
      </div>

      <Field of={filterQueryForm} path="search">
        {inputProps => (
          <TextField
            id="search"
            placeholder="Search widget packs..."
            class="w-full sm:w-[300px]"
            {...inputProps()}
          />
        )}
      </Field>

      {marketplacePacks.allPacks.error ? (
        <div class="text-center py-16 px-4">
          <div class="max-w-md mx-auto">
            <IconAlertCircle class="h-12 w-12 text-destructive mx-auto mb-4" />

            <h3 class="text-xl font-semibold mb-2">
              Unable to load marketplace.
            </h3>

            <p class="text-muted-foreground mb-6">
              {marketplacePacks.allPacks.error.message ||
                'An unexpected error occurred while loading widget packs.'}
            </p>
          </div>
        </div>
      ) : (
        <>
          <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
            {filteredPacks().map(pack => (
              <div class="group relative">
                <A
                  href={`/marketplace/packs/${pack.publishedId}`}
                  class="block"
                >
                  <div class="overflow-hidden rounded-lg aspect-[3/2] bg-muted">
                    <img
                      src={
                        pack.previewImageUrls?.[0] || '/placeholder.svg'
                      }
                      alt={`Preview of ${pack.name}`}
                      width={600}
                      height={400}
                      class="object-cover w-full h-full transition-transform group-hover:scale-105"
                    />
                  </div>
                  <div class="mt-3 flex items-start justify-between">
                    <div class="space-y-1">
                      <h3 class="font-medium leading-none">{pack.name}</h3>
                      <p class="text-sm text-muted-foreground">
                        by {pack.publishedId.split('.')[0]}
                      </p>
                    </div>
                    <div class="flex items-center gap-2">
                      <Button
                        variant="ghost"
                        size="icon"
                        class="h-8 w-8"
                        onClick={e => {
                          e.preventDefault();
                          widgetPreview.startPreview(pack);
                        }}
                      >
                        <IconEye class="h-4 w-4" />
                        <span class="sr-only">Preview</span>
                      </Button>
                      <Button
                        variant="ghost"
                        size="icon"
                        class="h-8 w-8"
                        onClick={async e => {
                          e.preventDefault();
                          await marketplacePacks.install(pack);
                          toaster.show({
                            title: 'Widget pack installed!',
                            description: `Widget pack ${pack.name} v${pack.latestVersion} installed successfully.`,
                            variant: 'default',
                          });
                        }}
                      >
                        <IconDownload class="h-4 w-4" />
                        <span class="sr-only">Install</span>
                      </Button>
                    </div>
                  </div>
                </A>
              </div>
            ))}
          </div>

          {filteredPacks().length === 0 && (
            <div class="text-center py-12">
              <p class="text-lg text-muted-foreground">
                No widget packs found matching your criteria.
              </p>
            </div>
          )}
        </>
      )}
    </div>
  );
}
