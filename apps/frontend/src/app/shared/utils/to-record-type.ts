import { TransformFnParams, plainToInstance } from 'class-transformer';

/**
 * Utility for use with "class-transformer" that converts a record with
 * values of a given class type to instances of that class.
 */
export function toRecordType<T = unknown>(classType: new () => T) {
  return ({ obj, key }: TransformFnParams) => {
    const record = obj[key];

    const transformedRecord = Object.entries(record).map(([key, value]) => {
      return [key, plainToInstance(classType, value)];
    });

    return Object.fromEntries(transformedRecord);
  };
}
