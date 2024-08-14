import { z } from 'zod';

export const FnPathSchema = z
  .string()
  .regex(
    /^(.+)#([a-zA-Z_$][a-zA-Z0-9_$]*)$/,
    "Invalid function path. Needs to be in format 'path/to/my-script.js#functionName'.",
  );
