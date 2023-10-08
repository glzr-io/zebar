export type Expression = '@if' | '@else' | '@else if' | '@for' | '@switch';

export function useTemplateEngine() {
  function parse() {}

  function parseExpression(expression: Expression) {
    switch (expression) {
      case '@if':
        return parseConditional(expression);
    }
  }

  function expect() {
    throw new Error('Function not implemented.');
  }

  function parseConditional(expression: string) {
    throw new Error('Function not implemented.');
  }
}
