import { z } from 'zod';

export function delimitedKey<const T extends string>(prefix: T) {
  return z.custom<`${T}/${string}`>(val => {
    return (val as string).startsWith(`${prefix}/`);
  });
}
