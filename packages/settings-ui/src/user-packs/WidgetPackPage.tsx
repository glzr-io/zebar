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
  TextField,
  ChipField,
  FormLabel,
  TextAreaField,
} from '@glzr/components';
import { IconPlus } from '@tabler/icons-solidjs';
import { createForm, Field } from 'smorf';
import { createEffect, createMemo, createSignal, Show } from 'solid-js';
import * as z from 'zod';
import { Widget } from 'zebar';

import { AppBreadcrumbs, useUserPacks, ImageSelector } from '~/common';
import { CreateWidgetDialog } from './dialogs';
import { useParams } from '@solidjs/router';

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

export function WidgetPackPage() {
  const params = useParams();
  const userPacks = useUserPacks();

  const selectedPack = createMemo(() =>
    userPacks.allPacks().find(pack => pack.id === params.packId),
  );

  const form = createForm<z.infer<typeof formSchema>>({
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

        <h1 class="text-3xl font-bold">Widget Pack</h1>

        <form class="space-y-8">
          <Card>
            <CardContent class="pt-6">
              <div class="grid gap-4">
                <Field of={form} path="name">
                  {inputProps => (
                    <TextField
                      label="Name"
                      placeholder="My widget pack"
                      description="This will be used as the directory name (as a slug)."
                      {...inputProps()}
                    />
                  )}
                </Field>

                <Field of={form} path="description">
                  {inputProps => (
                    <TextField
                      label="Description"
                      placeholder="A collection of beautiful widgets..."
                      {...inputProps()}
                    />
                  )}
                </Field>

                <Field of={form} path="tags">
                  {inputProps => (
                    <ChipField
                      label="Tags"
                      placeholder="Press enter to add tags..."
                      {...inputProps()}
                    />
                  )}
                </Field>

                <div>
                  <FormLabel>Preview Images</FormLabel>
                  <ImageSelector
                    images={form.value.previewImages}
                    onChange={images =>
                      form.setFieldValue('previewImages', images)
                    }
                  />
                </div>

                <Field of={form} path="excludeFiles">
                  {inputProps => (
                    <TextAreaField
                      label="Exclude Files"
                      description="A list of file patterns to exclude from the pack separated by new lines."
                      {...inputProps()}
                    />
                  )}
                </Field>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent class="pt-6">
              <div class="flex justify-between items-center mb-4">
                <h2 class="text-xl font-semibold">Widgets</h2>

                <Dialog>
                  <DialogTrigger>
                    <Button variant="outline">
                      <IconPlus class="mr-2 h-4 w-4" />
                      Add Widget
                    </Button>
                  </DialogTrigger>
                  <CreateWidgetDialog onSubmit={userPacks.createWidget} />
                </Dialog>
              </div>

              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Name</TableHead>
                    <TableHead>Template</TableHead>
                    <TableHead class="w-[100px]">Actions</TableHead>
                  </TableRow>
                </TableHeader>

                {/* <TableBody>
                {widgets().map(widget => (
                  <TableRow>
                    <TableCell>{widget.name}</TableCell>
                    <TableCell>{widget.template}</TableCell>
                    <TableCell>
                      <AlertDialog
                        open={deleteWidgetId === widget.id}
                        onOpenChange={open =>
                          setDeleteWidgetId(open ? widget.id : null)
                        }
                      >
                        <Button
                          variant="ghost"
                          size="icon"
                          class="text-destructive"
                          onClick={() => setDeleteWidgetId(widget.id)}
                        >
                          <IconTrash class="h-4 w-4" />
                        </Button>
                        <AlertDialogContent>
                          <AlertDialogHeader>
                            <AlertDialogTitle>
                              Delete Widget: {widget.name}
                            </AlertDialogTitle>
                            <AlertDialogDescription>
                              <div class="flex flex-col gap-4">
                                <div class="flex items-center gap-2 text-destructive">
                                  <IconAlertTriangle class="h-5 w-5" />
                                  <span>
                                    This action cannot be undone. The
                                    following files will be deleted:
                                  </span>
                                </div>
                                <ul class="list-disc list-inside space-y-1 text-muted-foreground">
                                  <li>zebar-widget.json</li>
                                  <li>
                                    /
                                    {widget.name
                                      .toLowerCase()
                                      .replace(/\s+/g, '-')}
                                    /
                                  </li>
                                </ul>
                              </div>
                            </AlertDialogDescription>
                          </AlertDialogHeader>
                          <AlertDialogFooter>
                            <AlertDialogClose>Cancel</AlertDialogClose>
                            <AlertDialogAction
                              onClick={() => handleDeleteWidget(widget.id)}
                              class="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                            >
                              Delete Widget
                            </AlertDialogAction>
                          </AlertDialogFooter>
                        </AlertDialogContent>
                      </AlertDialog>
                    </TableCell>
                  </TableRow>
                ))}

                {widgets().length === 0 && (
                  <TableRow>
                    <TableCell
                      colSpan={3}
                      class="text-center text-muted-foreground"
                    >
                      No widgets added yet
                    </TableCell>
                  </TableRow>
                )}
              </TableBody> */}
              </Table>
            </CardContent>
          </Card>
        </form>
      </Show>
    </div>
  );
}
