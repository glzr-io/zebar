import { TemplateNodeType } from './template-node-type.model';
import { TemplateNode } from './template-node.model';

export interface IfStatementBranch {
  type: 'if' | 'else if' | 'else';
  expression: string;
  children: TemplateNode[];
}

export interface IfStatementNode {
  type: TemplateNodeType.IF_STATEMENT;
  branches: IfStatementBranch[];
}
