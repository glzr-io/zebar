export interface ProviderNode {
  id: string;
  variables: Record<string, unknown>;
  functions: Record<string, (...args: unknown[]) => unknown>;
  slots: Record<string, string>; // TODO: Unsure here.
  parent: ProviderNode | null;
  children: ProviderNode[];
}
