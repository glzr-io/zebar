import type { SwitchStatementNode } from './switch-statement-node.model';
import type { ForStatementNode } from './for-statement-node';
import type { IfStatementNode } from './if-statement-node.model';
import type { InterpolationNode } from './interpolation-node.model';
import type { TextNode } from './text-node.model';

export type TemplateNode =
  | TextNode
  | InterpolationNode
  | IfStatementNode
  | ForStatementNode
  | SwitchStatementNode;
