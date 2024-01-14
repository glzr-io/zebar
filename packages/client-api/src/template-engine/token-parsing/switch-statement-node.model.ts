import { TemplateNodeType } from './template-node-type.model';
import type { TemplateNode } from './template-node.model';

export interface CaseBranch {
  type: 'case';
  expression: string;
  children: TemplateNode[];
}

export interface DefaultBranch {
  type: 'default';
  children: TemplateNode[];
}

export interface SwitchStatementNode {
  type: TemplateNodeType.SWITCH_STATEMENT;
  expression: string;
  branches: (CaseBranch | DefaultBranch)[];
}
