import {
  createContext,
  splitProps,
  useContext,
  type JSXElement,
} from 'solid-js';

/**
 * Creates a context provider setup with a custom hook for accessing the
 * context.
 *
 * @param createState - Function that returns the state to be provided by
 * the context.
 * @returns Object containing Provider component, Context, and a hook to
 * access the context.
 *
 * @example
 * ```tsx
 * // Define your state creation function.
 * function createCounterState(args: { initialCount?: number } = {}) {
 *   const [count, setCount] = createSignal(args.initialCount ?? 0);
 *
 *   return {
 *     count,
 *     increment: () => setCount(c => c + 1),
 *     decrement: () => setCount(c => c - 1)
 *   };
 * }
 *
 * // Create provider, context and hook.
 * const [CounterProvider, CounterContext, useCounter] = makeProvider(createCounterState);
 *
 * // Use in your app.
 * function App() {
 *   return (
 *     <CounterProvider initialCount={10}>
 *       <Counter />
 *     </CounterProvider>
 *   );
 * }
 *
 * function Counter() {
 *   const { count, increment } = useCounter();
 *   return <button onClick={increment}>{count()}</button>;
 * }
 * ```
 */
export function makeProvider<T, P extends object = {}>(
  createState: (props: P) => T,
) {
  const Context = createContext<T>();

  // Create the provider component with the args to `createState` as props.
  function Provider(props: P & { children: JSXElement }) {
    const [_, others] = splitProps(props, ['children']);
    const state = createState(others as P);

    return (
      <Context.Provider value={state}>{props.children}</Context.Provider>
    );
  }

  // Create the hook to access the context.
  function useContextValue() {
    const context = useContext(Context);

    if (!context) {
      throw new Error('Context used outside of provider.');
    }

    return context;
  }

  return [Provider, Context, useContextValue] as const;
}
