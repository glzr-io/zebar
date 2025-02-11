import {
  Card,
  CardHeader,
  CardTitle,
  Badge,
  CardDescription,
  Tooltip,
  TooltipTrigger,
  Button,
  TooltipContent,
  AlertDialog,
  AlertDialogTrigger,
  AlertDialogContent,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogClose,
  AlertDialogAction,
  CardContent,
} from '@glzr/components';
import {
  IconPackage,
  IconEye,
  IconSettings,
  IconTrash,
} from '@tabler/icons-solidjs';

import { WidgetPack } from '~/common';
import { DeleteWidgetPackDialog } from './dialogs/DeleteWidgetPackDialog';

export interface WidgetPackCardProps {
  pack: WidgetPack;
  isLocal: boolean;
  onDelete: (packId: string) => void;
}

export function WidgetPackCard(props: WidgetPackCardProps) {
  return (
    <Card class="mb-4">
      <CardHeader>
        <div class="flex justify-between items-start">
          <div>
            <CardTitle class="flex items-center gap-2">
              <IconPackage class="h-5 w-5" />
              {props.pack.name}
              <Badge variant="outline" class="ml-2">
                {props.pack.version}
              </Badge>
            </CardTitle>
            <CardDescription class="mt-1">
              by {props.pack.author}
            </CardDescription>
          </div>
          <div class="flex gap-2">
            <Tooltip>
              <TooltipTrigger>
                <Button variant="outline" size="icon">
                  <IconEye class="h-4 w-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Preview Widgets</TooltipContent>
            </Tooltip>

            <Tooltip>
              <TooltipTrigger>
                <Button variant="outline" size="icon">
                  <IconSettings class="h-4 w-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Configure Pack</TooltipContent>
            </Tooltip>

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
        <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
          {props.pack.description}
        </p>
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
