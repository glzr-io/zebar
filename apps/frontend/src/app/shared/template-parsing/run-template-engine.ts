import { renderString } from 'nunjucks';

/**
 * Nunjucks is used to evaluate variables in the template.
 */
export function runTemplateEngine(
  template: string,
  templateContext: Record<string, unknown>,
): string {
  return renderString(template, templateContext);
}
