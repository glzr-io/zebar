import {
  Card,
  CardHeader,
  CardTitle,
  Badge,
  CardDescription,
  Button,
  AlertDialog,
  AlertDialogTrigger,
  CardContent,
} from '@glzr/components';
import { useNavigate } from '@solidjs/router';
import { IconPackage, IconTrash } from '@tabler/icons-solidjs';
import { Show } from 'solid-js';

import { WidgetPack } from '~/common';
import { DeleteWidgetPackDialog } from './dialogs';

export interface WidgetPackCardProps {
  pack: WidgetPack;
  onDelete: (packId: string) => void;
}

export function WidgetPackCard(props: WidgetPackCardProps) {
  const navigate = useNavigate();

  return (
    <Card
      class="mb-4 cursor-pointer"
      onClick={() => navigate(`/packs/${props.pack.id}`)}
    >
      <CardHeader>
        <div class="flex justify-between items-start">
          <div>
            <CardTitle class="flex items-center gap-2">
              <IconPackage class="h-5 w-5" />
              {props.pack.name}

              {props.pack.type === 'marketplace' && (
                <Badge variant="outline" class="ml-2">
                  {props.pack.version}
                </Badge>
              )}
            </CardTitle>

            {props.pack.type === 'marketplace' && (
              <CardDescription class="mt-1">
                by {props.pack.author}
              </CardDescription>
            )}
          </div>

          <div class="flex gap-2" onClick={e => e.stopPropagation()}>
            <AlertDialog>
              <AlertDialogTrigger>
                <Button
                  variant="outline"
                  size="icon"
                  class="text-red-500 hover:text-red-600"
                >
                  <IconTrash class="h-4 w-4" />
                </Button>
              </AlertDialogTrigger>
              <DeleteWidgetPackDialog
                pack={props.pack}
                onDelete={props.onDelete}
              />
            </AlertDialog>
          </div>
        </div>
      </CardHeader>

      <CardContent>
        <Show when={props.pack.description !== ''}>
          <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
            {props.pack.description}
          </p>
        </Show>

        <div class="flex flex-wrap gap-2 mb-4">
          {props.pack.tags.map(tag => (
            <Badge key={tag} variant="secondary">
              {tag}
            </Badge>
          ))}
        </div>

        <div class="text-sm">
          <p>
            <strong>Widgets:</strong> {props.pack.widgets.length}
          </p>
        </div>
      </CardContent>
    </Card>
  );
}
