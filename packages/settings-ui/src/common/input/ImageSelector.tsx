import { Button } from '@glzr/components';
import { IconPhoto, IconPlus, IconX } from '@tabler/icons-solidjs';
import { convertFileSrc } from '@tauri-apps/api/core';
import { open as openFileDialog } from '@tauri-apps/plugin-dialog';
import { Show } from 'solid-js';

export interface ImageSelectorProps {
  images: string[];
  onChange: (images: string[]) => void;
}

export function ImageSelector(props: ImageSelectorProps) {
  async function addImages() {
    const selectedPaths = await openFileDialog({
      multiple: true,
      filters: [
        {
          name: 'Images',
          extensions: ['png', 'jpg', 'jpeg'],
        },
      ],
    });

    if (selectedPaths) {
      props.onChange([...props.images, ...selectedPaths]);
    }
  }

  function removeImage(index: number) {
    props.onChange(props.images.filter((_, i) => i !== index));
  }

  return (
    <Show
      when={props.images.length > 0}
      fallback={
        <div class="border-2 border-dashed rounded-md p-8 text-center">
          <div class="text-muted-foreground">
            <IconPhoto class="h-8 w-8 mx-auto mb-2" />

            <p class="mb-3">No preview images selected</p>

            <Button variant="outline" size="sm" onClick={addImages}>
              <IconPlus class="h-4 w-4 mr-2" />
              Add images
            </Button>
          </div>
        </div>
      }
    >
      <div class="grid grid-cols-4 gap-2">
        {props.images.map((path, index) => (
          <div class="relative group aspect-square border rounded-md overflow-hidden">
            <img
              src={convertFileSrc(path)}
              alt={`Preview ${index + 1}`}
              class="w-full h-full object-cover rounded-md"
            />

            <Button
              variant="secondary"
              size="icon"
              class="absolute top-1 right-1 opacity-0 group-hover:opacity-100 transition-opacity"
              onClick={() => removeImage(index)}
            >
              <IconX class="h-3 w-3" />
            </Button>
          </div>
        ))}

        <Button
          variant="outline"
          size="icon"
          class="h-8 w-8 self-center ml-2"
          onClick={addImages}
        >
          <IconPlus class="h-4 w-4" />
        </Button>
      </div>
    </Show>
  );
}
