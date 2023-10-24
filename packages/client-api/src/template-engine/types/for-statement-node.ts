import { TemplateNode } from './template-node.model';
import { TemplateNodeType } from './template-node-type.model';

export interface ForStatementNode {
  type: TemplateNodeType.FOR_STATEMENT;
  expression: string;
  children: TemplateNode[];
}
