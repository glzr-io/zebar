import { Button, TextField } from '@glzr/components';
import { A } from '@solidjs/router';
import { IconDownload, IconEye } from '@tabler/icons-solidjs';
import { createForm, Field } from 'smorf';
import { createMemo } from 'solid-js';

import { useCommunityPacks } from '~/common';

type FilterQuery = {
  search: string;
};

export function CommunityPacks() {
  const communityPacks = useCommunityPacks();

  const filterQueryForm = createForm<FilterQuery>({
    search: '',
  });

  const filteredPacks = createMemo(() =>
    communityPacks.allPacks().filter(pack => {
      const searchQuery = filterQueryForm.value.search;

      const matchesSearch =
        pack.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        pack.description
          .toLowerCase()
          .includes(searchQuery.toLowerCase()) ||
        pack.author.toLowerCase().includes(searchQuery.toLowerCase());

      return matchesSearch;
    }),
  );

  return (
    <div class="min-h-screen bg-background">
      <div class="container mx-auto py-6 space-y-6">
        <div class="space-y-1">
          <h1 class="text-3xl font-bold tracking-tight">
            Browse Community
          </h1>

          <p class="text-muted-foreground">
            Discover and install widget packs that other users have
            created.
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

        <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
          {filteredPacks().map(pack => (
            <div class="group relative">
              <A href={`/community/${pack.id}`} class="block">
                <div class="overflow-hidden rounded-lg aspect-[3/2] bg-muted">
                  <img
                    src={pack.galleryUrls[0] || '/placeholder.svg'}
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
                      by {pack.author}
                    </p>
                  </div>
                  <div class="flex items-center gap-2">
                    <Button
                      variant="ghost"
                      size="icon"
                      class="h-8 w-8"
                      onClick={e => {
                        e.preventDefault();
                        communityPacks.install(pack);
                      }}
                    >
                      <IconDownload class="h-4 w-4" />
                      <span class="sr-only">Install</span>
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon"
                      class="h-8 w-8"
                      onClick={e => {
                        e.preventDefault();
                        communityPacks.startPreview(pack);
                      }}
                    >
                      <IconEye class="h-4 w-4" />
                      <span class="sr-only">Preview</span>
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
      </div>
    </div>
  );
}
