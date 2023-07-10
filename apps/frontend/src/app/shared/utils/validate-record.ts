import {
  ValidationArguments,
  ValidationOptions,
  registerDecorator,
  validateSync,
} from 'class-validator';

export function ValidateRecord(opts?: ValidationOptions) {
  return function (object: Object, propertyName: string) {
    registerDecorator({
      name: 'validateRecord',
      target: object.constructor,
      propertyName: propertyName,
      options: opts,
      validator: {
        validate(value: Record<string, unknown>, args: ValidationArguments) {
          const errors = Object.values(value).flatMap(key =>
            validateSync(key as object, opts),
          );

          return errors.length === 0;
        },
      },
    });
  };
}
