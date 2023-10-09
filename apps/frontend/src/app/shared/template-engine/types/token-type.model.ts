export enum TokenType {
  /** Start of if tag (ie. `@if`). */
  IF_TAG = 'IF_TAG',

  /** Start of else if tag (ie. `@else if`). */
  ELSE_IF_TAG = 'ELSE_IF_TAG',

  /** Start of else tag (ie. `@else`). */
  ELSE_TAG = 'ELSE_TAG',

  /** Start of for tag (ie. `@for`). */
  FOR_TAG = 'FOR_TAG',

  /** Start of switch tag (ie. `@switch`). */
  SWITCH_TAG = 'SWITCH_TAG',

  /** Start of case tag (ie. `@case`). */
  CASE_TAG = 'CASE_TAG',

  /** Opening curly brace (ie. `{`) after the start of a tag. */
  OPEN_TAG_BLOCK = 'OPEN_TAG_BLOCK',

  /** Closing curly brace (ie. `}`) marking the end of a tag. */
  CLOSE_TAG_BLOCK = 'CLOSE_TAG_BLOCK',

  /** Opening double curly brace (ie. `{{`) of an interpolation tag. */
  OPEN_INTERPOLATION_TAG = 'OPEN_INTERPOLATION_TAG',

  /** Closing double curly brace (ie. `}}`) of an interpolation tag. */
  CLOSE_INTERPOLATION_TAG = 'CLOSE_INTERPOLATION_TAG',

  /** Expression to evaluate (within tag start or an interpolation tag. */
  EXPRESSION = 'EXPRESSION',

  /** Ordinary text. */
  TEXT = 'TEXT',
}
