import { z } from 'zod';

const length = z
  .string()
  .regex(
    /^([+-]?\d+)(%|px)?$/,
    "Not a valid length value. Must be of format '10px' or '10%'.",
  );

const name = z
  .string()
  .min(2, 'Name must be at least 2 characters.')
  .max(24, 'Name cannot exceed 24 characters.')
  .regex(
    /^[a-z0-9][a-z0-9-_]*$/,
    'Only lowercase letters, numbers, and the characters - and _ are allowed.',
  );

const widget = z.object({
  name: name,
  htmlPath: z.string().refine(path => path.endsWith('.html'), {
    message:
      'Must be a valid HTML file path (e.g. "path/to/widget.html").',
  }),
  zOrder: z.enum(['normal', 'top_most', 'bottom_most']),
  shownInTaskbar: z.boolean(),
  focused: z.boolean(),
  resizable: z.boolean(),
  transparent: z.boolean(),
  caching: z.object({
    defaultDuration: z.number(),
    rules: z.array(
      z.object({
        urlRegex: z.string(),
        duration: z.number(),
      }),
    ),
  }),
  privileges: z.object({
    shellCommands: z.array(
      z.object({
        program: z.string(),
        argsRegex: z.string(),
      }),
    ),
  }),
  presets: z.array(
    z.object({
      name: z.string(),
      anchor: z.enum([
        'top_left',
        'top_center',
        'top_right',
        'center_left',
        'center',
        'center_right',
        'bottom_left',
        'bottom_center',
        'bottom_right',
      ]),
      offsetX: length,
      offsetY: length,
      width: length,
      height: length,
      monitorSelection: z.union([
        z.object({
          type: z.literal('all'),
        }),
        z.object({
          type: z.literal('primary'),
        }),
        z.object({
          type: z.literal('secondary'),
        }),
        z.object({
          type: z.literal('index'),
          match: z.number(),
        }),
        z.object({
          type: z.literal('name'),
          match: z.string(),
        }),
      ]),
      dockToEdge: z.object({
        enabled: z.boolean(),
        edge: z.enum(['top', 'right', 'bottom', 'left']).nullable(),
        windowMargin: z.string(),
      }),
    }),
  ),
});

const widgetPack = z.object({
  name,
  description: z
    .string()
    .max(1000, 'Description cannot exceed 1000 characters.'),
  tags: z.array(z.string()).max(10, 'At most 10 tags are allowed.'),
  previewImages: z
    .array(z.string())
    .min(1, 'At least one preview image is required.')
    .max(6, 'At most 6 preview images are allowed.'),
  excludeFiles: z
    .string()
    .max(1000, 'File exclusion list cannot exceed 1000 characters.'),
  widgets: z.array(widget),
});

export const configSchemas = {
  length,
  name,
  widget,
  widgetPack,
};
