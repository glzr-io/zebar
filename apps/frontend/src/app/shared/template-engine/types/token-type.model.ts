export enum TokenType {
  /** Start of if tag (ie. `@if`). */
  IF_TAG_START,

  /** Start of else if tag (ie. `@else if`). */
  ELSE_IF_TAG_START,

  /** Start of else tag (ie. `@else`). */
  ELSE_TAG_START,

  /** Start of for tag (ie. `@for`). */
  FOR_TAG_START,

  /** Start of switch tag (ie. `@switch`). */
  SWITCH_TAG_START,

  /** Start of case tag (ie. `@case`). */
  CASE_TAG_START,

  /** Opening curly brace (ie. `{`) after the start of a tag. */
  OPEN_TAG_BLOCK,

  /** Closing curly brace (ie. `}`) marking the end of a tag. */
  CLOSE_TAG_BLOCK,

  /** Opening double curly brace (ie. `{{`) marking the start of an output. */
  OPEN_OUTPUT,

  /** Closing double curly brace (ie. `}}`) marking the end of an output. */
  CLOSE_OUTPUT,

  /** Expression to evaluate (either within an output or start tag). */
  EXPRESSION,

  /** Ordinary text. */
  TEXT,
}
