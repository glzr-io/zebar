import { SwitchStatementNode } from './switch-statement-node.model';
import { ForStatementNode } from './for-statement-node';
import { IfStatementNode } from './if-statement-node.model';
import { InterpolationNode } from './interpolation-node.model';
import { TextNode } from './text-node.model';

export type TemplateNode =
  | TextNode
  | InterpolationNode
  | IfStatementNode
  | ForStatementNode
  | SwitchStatementNode;
