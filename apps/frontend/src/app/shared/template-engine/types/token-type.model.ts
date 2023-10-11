export enum TokenType {
  /** Start of if statement (ie. `@if`). */
  IF_STATEMENT = 'IF_STATEMENT',

  /** Start of else if statement (ie. `@else if`). */
  ELSE_IF_STATEMENT = 'ELSE_IF_STATEMENT',

  /** Start of else statement (ie. `@else`). */
  ELSE_STATEMENT = 'ELSE_STATEMENT',

  /** Start of for statement (ie. `@for`). */
  FOR_STATEMENT = 'FOR_STATEMENT',

  /** Start of switch statement (ie. `@switch`). */
  SWITCH_STATEMENT = 'SWITCH_STATEMENT',

  /** Start of case statement (ie. `@case`). */
  CASE_STATEMENT = 'CASE_STATEMENT',

  /** Opening curly brace (ie. `{`) after the start of a statement. */
  OPEN_BLOCK = 'OPEN_BLOCK',

  /** Closing curly brace (ie. `}`) marking the end of a statement. */
  CLOSE_BLOCK = 'CLOSE_BLOCK',

  /** Opening double curly brace (ie. `{{`) of an interpolation tag. */
  OPEN_INTERPOLATION = 'OPEN_INTERPOLATION',

  /** Closing double curly brace (ie. `}}`) of an interpolation tag. */
  CLOSE_INTERPOLATION = 'CLOSE_INTERPOLATION',

  /** Expression to evaluate (within tag start or an interpolation tag. */
  EXPRESSION = 'EXPRESSION',

  /** Ordinary text. */
  TEXT = 'TEXT',
}
