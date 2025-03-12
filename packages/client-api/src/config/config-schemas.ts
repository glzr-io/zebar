import { z } from 'zod';

const widgetPack = z.object({
  name: z
    .string()
    .min(2, 'Name must be at least 2 characters.')
    .max(24, 'Name cannot exceed 24 characters.')
    .regex(
      /^[a-z0-9][a-z0-9-_]*$/,
      'Only lowercase letters, numbers, and the characters - and _ are allowed.',
    ),
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
  widgetPaths: z.array(z.string()),
});

export const configSchemas = {
  widgetPack,
};
