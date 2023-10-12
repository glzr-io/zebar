import { TemplateNodeType } from './template-node-type.model';

export interface InterpolationNode {
  type: TemplateNodeType.INTERPOLATION;
  expression: string;
}
