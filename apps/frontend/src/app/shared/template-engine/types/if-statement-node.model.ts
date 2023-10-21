import { TemplateNodeType } from './template-node-type.model';
import { TemplateNode } from './template-node.model';

export interface IfBranch {
  type: 'if' | 'else if';
  expression: string;
  children: TemplateNode[];
}

export interface ElseBranch {
  type: 'else';
  expression: null;
  children: TemplateNode[];
}

export interface IfStatementNode {
  type: TemplateNodeType.IF_STATEMENT;
  branches: (IfBranch | ElseBranch)[];
}
