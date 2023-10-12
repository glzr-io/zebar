export function evalWithContext(
  expression: string,
  ...context: Record<string, unknown>[]
) {
  // TODO: Pass context to eval.
  return eval(expression);
}
