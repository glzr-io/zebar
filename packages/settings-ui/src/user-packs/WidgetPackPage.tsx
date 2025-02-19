import {
  Button,
  Dialog,
  DialogTrigger,
  Card,
  CardContent,
  Table,
  TableHead,
  TableHeader,
  TableRow,
  TableBody,
  TableCell,
  AlertDialogTrigger,
  AlertDialog,
} from '@glzr/components';
import { useNavigate, useParams } from '@solidjs/router';
import { IconPlus, IconTrash } from '@tabler/icons-solidjs';
import { createForm } from 'smorf';
import { createEffect, createMemo, Show } from 'solid-js';
import * as z from 'zod';

import { AppBreadcrumbs, useUserPacks } from '~/common';
import { CreateWidgetDialog, DeleteWidgetDialog } from './dialogs';
import { WidgetPackForm } from './WidgetPackForm';

const formSchema = z.object({
  name: z.string().min(2, {
    message: 'Name must be at least 2 characters.',
  }),
  description: z.string().min(10, {
    message: 'Description must be at least 10 characters.',
  }),
  tags: z.array(z.string()).min(1, {
    message: 'At least one tag is required.',
  }),
  previewImages: z.array(z.string()).min(1, {
    message: 'At least one preview image is required.',
  }),
  excludeFiles: z.string(),
});

export type WidgetPackFormData = z.infer<typeof formSchema>;

export function WidgetPackPage() {
  const params = useParams();
  const navigate = useNavigate();
  const userPacks = useUserPacks();

  const selectedPack = createMemo(() =>
    userPacks.allPacks().find(pack => pack.id === params.packId),
  );

  const isMarketplacePack = createMemo(
    () => selectedPack()?.type === 'marketplace',
  );

  const form = createForm<WidgetPackFormData>({
    name: '',
    description: '',
    tags: [],
    previewImages: [],
    excludeFiles: '',
  });

  createEffect(() => {
    if (selectedPack()) {
      form.setValue({
        name: selectedPack().name,
        description: selectedPack().description,
        tags: selectedPack().tags,
        previewImages: selectedPack().previewUrls,
        excludeFiles: selectedPack().excludeFiles,
      });
    }
  });

  return (
    <div class="container mx-auto pt-3.5 pb-32">
      <Show when={selectedPack()}>
        <AppBreadcrumbs
          entries={[
            {
              href: `/packs/${selectedPack().id}`,
              content: selectedPack().id,
            },
          ]}
        />

        <h1 class="text-3xl font-bold mb-4">Widget Pack</h1>

        <WidgetPackForm form={form} disabled={isMarketplacePack()} />

        <Card>
          <CardContent class="pt-6">
            <div class="flex justify-between items-center mb-4">
              <h2 class="text-xl font-semibold">Widgets</h2>

              <Dialog>
                <DialogTrigger>
                  <Button variant="outline" disabled={isMarketplacePack()}>
                    <IconPlus class="mr-2 h-4 w-4" />
                    Add widget
                  </Button>
                </DialogTrigger>
                <CreateWidgetDialog onSubmit={userPacks.createWidget} />
              </Dialog>
            </div>

            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Name</TableHead>
                  <TableHead>HTML Path</TableHead>
                  <TableHead class="w-[100px]">Actions</TableHead>
                </TableRow>
              </TableHeader>

              <TableBody>
                {selectedPack()?.widgets.map(widget => (
                  <TableRow
                    class="cursor-pointer"
                    onClick={() =>
                      navigate(
                        `/packs/${selectedPack().id}/${widget.name}`,
                      )
                    }
                  >
                    <TableCell>{widget.name}</TableCell>
                    <TableCell>{widget.htmlPath}</TableCell>
                    <TableCell>
                      <AlertDialog>
                        <AlertDialogTrigger disabled={isMarketplacePack()}>
                          <Button
                            variant="outline"
                            size="icon"
                            class="text-red-500 hover:text-red-600"
                            disabled={isMarketplacePack()}
                          >
                            <IconTrash class="h-4 w-4" />
                          </Button>
                        </AlertDialogTrigger>
                        <DeleteWidgetDialog
                          widget={widget}
                          onDelete={userPacks.deleteWidget}
                        />
                      </AlertDialog>
                    </TableCell>
                  </TableRow>
                ))}

                {selectedPack()?.widgets.length === 0 && (
                  <TableRow>
                    <TableCell
                      colSpan={3}
                      class="text-center text-muted-foreground"
                    >
                      No widgets added yet
                    </TableCell>
                  </TableRow>
                )}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      </Show>
    </div>
  );
}
