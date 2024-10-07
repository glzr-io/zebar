import { createStore } from 'solid-js/store';

export function App() {
  const [output, setOutput] = createStore({});

  return <div class="app"></div>;
}
