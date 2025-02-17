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
  FileField,
} from '@glzr/components';
import { IconPhoto, IconPlus, IconX } from '@tabler/icons-solidjs';
import { convertFileSrc } from '@tauri-apps/api/core';
import { open as openFileDialog } from '@tauri-apps/plugin-dialog';
import { createForm, Field } from 'smorf';
import { createSignal } from 'solid-js';
import * as z from 'zod';
import { Widget } from 'zebar';

import { AppBreadcrumbs, useUserPacks } from '~/common';
import { CreateWidgetDialog } from './dialogs';

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
});

export function WidgetPackPage() {
  const userPacks = useUserPacks();

  const fileInputRef = createSignal<HTMLInputElement | null>(null);
  const [widgets, setWidgets] = createSignal<Widget[]>([]);

  const form = createForm<z.infer<typeof formSchema>>({
    name: '',
    description: '',
    tags: [],
    previewImages: [],
  });

  function onFileSelect(value: File[]) {
    console.log('files', value);

    // setSelectedFiles(files);

    // Update form with file paths.
    const paths = value.map(file => `./resources/${file.name}`);
    form.setFieldValue('previewImages', paths);
  }

  function removeImage(index: number) {
    // setSelectedFiles(prev => prev.filter((_, i) => i !== index));
    // packForm.setFieldValue('previewImages', prev =>
    //   prev.filter((_, i) => i !== index),
    // );
  }

  return (
    <div class="container mx-auto pt-3.5 pb-32">
      <AppBreadcrumbs
        entries={[
          {
            href: 'TODO',
            content: 'TODO',
            // href: `/packs/${selectedPack().id}`,
            // content: selectedPack().name,
          },
        ]}
      />

      <h1 class="text-3xl font-bold">Edit Widget Pack</h1>

      <form class="space-y-8">
        <Card>
          <CardContent class="pt-6">
            <div class="grid gap-4">
              <Field of={form} path="name">
                {inputProps => (
                  <TextField
                    label="Name"
                    placeholder="My Awesome Widget Pack"
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

              <div class="space-y-4">
                <div class="flex items-center justify-between">
                  <label class="text-sm font-medium">Preview Images</label>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={async e => {
                      e.preventDefault();
                      const selected = await openFileDialog({
                        multiple: true,
                        filters: [
                          {
                            name: 'Images',
                            extensions: ['png', 'jpg', 'jpeg'],
                          },
                        ],
                      });
                      if (selected) {
                        form.setFieldValue('previewImages', selected);
                      }
                    }}
                  >
                    <IconPlus class="h-4 w-4 mr-2" />
                    Add Images
                  </Button>
                </div>

                <div class="grid grid-cols-2 gap-4">
                  {form.value.previewImages.length > 0 ? (
                    form.value.previewImages.map((path, index) => (
                      <div class="relative group aspect-video">
                        <img
                          src={convertFileSrc(path)}
                          alt={`Preview ${index + 1}`}
                          class="w-full h-full object-cover rounded-md"
                        />
                        <Button
                          variant="secondary"
                          size="icon"
                          class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity"
                          onClick={() => removeImage(index)}
                        >
                          <IconX class="h-4 w-4" />
                        </Button>
                      </div>
                    ))
                  ) : (
                    <div class="col-span-2 border-2 border-dashed rounded-md p-8 text-center">
                      <div class="text-muted-foreground">
                        <IconPhoto class="h-8 w-8 mx-auto mb-2" />
                        <p>No preview images selected</p>
                        <p class="text-sm">
                          Click "Add Images" to select preview images
                        </p>
                      </div>
                    </div>
                  )}
                </div>
              </div>
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
    </div>
  );
}
