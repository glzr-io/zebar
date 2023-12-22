import { ElementContext } from '~/element-context.model';

export type SelfProvider = Omit<
  ElementContext,
  'parsedConfig' | 'providers'
>;

export async function createSelfProvider(
  elementContext: Omit<ElementContext, 'parsedConfig' | 'providers'>,
): Promise<SelfProvider> {
  return elementContext;
}
