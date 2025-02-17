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
  toaster,
  TextField,
  ChipField,
} from '@glzr/components';
import { IconPlus, IconCopy } from '@tabler/icons-solidjs';
import { createForm, Field } from 'smorf';
import { createSignal } from 'solid-js';
import * as z from 'zod';
import { Widget } from 'zebar';

import { AppBreadcrumbs, CreateWidgetArgs } from '~/common';
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
  const fileInputRef = createSignal<HTMLInputElement | null>(null);
  const [widgets, setWidgets] = createSignal<Widget[]>([]);

  const form = createForm<z.infer<typeof formSchema>>({
    name: '',
    description: '',
    tags: [],
    previewImages: [],
  });

  function handleAddWidget(widget: CreateWidgetArgs) {
    // setWidgets([...widgets, widget]);
    toaster.show({
      title: 'Widget added',
      description: `${widget.name} has been added to the widget pack.`,
    });
  }

  function handleDeleteWidget(id: string) {
    // setWidgets(widgets.filter(widget => widget.id !== id));
    toaster.show({
      title: 'Widget deleted',
      description: 'The widget has been removed from the pack.',
    });
  }

  function handleDuplicatePack() {
    toaster.show({
      title: 'Widget pack duplicated',
      description: 'A copy of this widget pack has been created.',
    });
  }

  // function handleImageUpload(event: ChangeEvent<HTMLInputElement>) {
  //   const files = event.target.files;
  //   if (files) {
  //     const newImages = Array.from(files).map(file =>
  //       URL.createObjectURL(file),
  //     );
  //     setPreviewImages([...previewImages, ...newImages]);
  //     form.setValue('previewImages', [...previewImages, ...newImages]);
  //   }
  // }

  // const removeImage = (index: number) => {
  //   const newImages = previewImages.filter((_, i) => i !== index);
  //   setPreviewImages(newImages);
  //   form.setValue('previewImages', newImages);
  // };

  return (
    <div class="container mx-auto py-6 max-w-4xl">
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

      <div class="flex justify-between items-center mb-6">
        <h1 class="text-3xl font-bold">Edit Widget Pack</h1>
        <Button onClick={handleDuplicatePack}>
          <IconCopy class="mr-2 h-4 w-4" />
          Duplicate Pack
        </Button>
      </div>

      <form class="space-y-8">
        <Card>
          <CardContent class="pt-6">
            <div class="grid gap-6">
              <Field of={form} path="name">
                {inputProps => (
                  <TextField
                    placeholder="My Awesome Widget Pack"
                    description="This will be used as the directory name (as a slug)"
                    {...inputProps()}
                  />
                )}
              </Field>

              <Field of={form} path="description">
                {inputProps => (
                  <TextField
                    placeholder="A collection of beautiful widgets..."
                    {...inputProps()}
                  />
                )}
              </Field>

              <Field of={form} path="tags">
                {inputProps => (
                  <ChipField
                    placeholder="Press enter to add tags..."
                    {...inputProps()}
                  />
                )}
              </Field>

              {/* <FormField
                control={form.control}
                name="previewImages"
                render={() => (
                  <FormItem>
                    <FormLabel>Preview Images</FormLabel>
                    <FormControl>
                      <div class="space-y-4">
                        <div class="flex items-center gap-4">
                          <Button
                            type="button"
                            variant="outline"
                            onClick={() => fileInputRef.current?.click()}
                          >
                            <Upload class="mr-2 h-4 w-4" />
                            Upload Images
                          </Button>
                          <input
                            type="file"
                            ref={fileInputRef}
                            class="hidden"
                            multiple
                            accept="image/*"
                            onChange={handleImageUpload}
                          />
                        </div>
                        <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
                          {previewImages.map((image, index) => (
                            <div
                              key={index}
                              class="relative aspect-video group"
                            >
                              <img
                                src={image || '/placeholder.svg'}
                                alt={`Preview ${index + 1}`}
                                class="rounded-lg object-cover w-full h-full"
                              />
                              <Button
                                type="button"
                                variant="destructive"
                                size="icon"
                                class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity"
                                onClick={() => removeImage(index)}
                              >
                                <X class="h-4 w-4" />
                              </Button>
                            </div>
                          ))}
                        </div>
                      </div>
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              /> */}
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
                <CreateWidgetDialog onSubmit={handleAddWidget} />
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
