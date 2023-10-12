import { TemplateNodeType } from './template-node-type.model';
import { TemplateNode } from './template-node.model';

export interface SwitchStatementBranch {
  expression: string;
  children: TemplateNode[];
}

export interface SwitchStatementNode {
  type: TemplateNodeType.SWITCH_STATEMENT;
  expression: string;
  branches: SwitchStatementBranch[];
}
