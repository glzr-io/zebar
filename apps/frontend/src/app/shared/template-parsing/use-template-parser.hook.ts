import { memoize } from '../utils';
import { createTemplateElement } from './create-template-element';

export const useTemplateParser = memoize(() => {
  return {
    createElement: createTemplateElement,
  };
});
