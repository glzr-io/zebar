export enum TokenType {
  /** Start of if tag (ie. `@if`). */
  START_IF_TAG,

  /** Start of else if tag (ie. `@else if`). */
  START_ELSE_IF_TAG,

  /** Start of else tag (ie. `@else`). */
  START_ELSE_TAG,

  /** Start of for tag (ie. `@for`). */
  START_FOR_TAG,

  /** Start of switch tag (ie. `@switch`). */
  START_SWITCH_TAG,

  /** Start of case tag (ie. `@case`). */
  START_CASE_TAG,

  /** Opening curly brace (ie. `{`) after the start of a tag. */
  OPEN_TAG,

  /** Closing curly brace (ie. `}`) marking the end of a tag. */
  CLOSE_TAG,

  /** Opening double curly brace (ie. `{{`) marking the start of an output. */
  OPEN_OUTPUT,

  /** Closing double curly brace (ie. `}}`) marking the end of an output. */
  CLOSE_OUTPUT,

  /** Expression to evaluate (either within an output or start tag). */
  EXPRESSION,

  /** Ordinary text. */
  TEXT,
}
