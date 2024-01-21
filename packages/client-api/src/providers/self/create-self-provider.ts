import type { ElementContext } from '~/element-context.model';
import type { PickPartial } from '~/utils';

export type SelfProvider = PickPartial<
  ElementContext,
  'parsedConfig' | 'providers'
>;

export async function createSelfProvider(
  elementContext: PickPartial<
    ElementContext,
    'parsedConfig' | 'providers'
  >,
): Promise<SelfProvider> {
  return elementContext;
}
