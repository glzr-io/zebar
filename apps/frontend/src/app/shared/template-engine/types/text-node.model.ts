import { TemplateNodeType } from './template-node-type.model';

export interface TextNode {
  type: TemplateNodeType.TEXT;
  text: string;
}
