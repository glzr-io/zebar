import { createForm, Field } from 'smorf';
import { createEffect, For, on } from 'solid-js';
import { createStore } from 'solid-js/store';

export function App() {
  const [output, setOutput] = createStore({});

  return (
    <div class="app">
      <MyForm2 />
      <p>fjidsa</p>
    </div>
  );
}

export function MyForm() {
  const fruitForm = createForm({
    fruits: [
      {
        name: 'banana',
        isTasty: true,
      },
      {
        name: 'kiwi',
        isTasty: false,
      },
    ],
  });

  createEffect(() => {
    console.log(fruitForm.value);
  });

  createEffect(
    on(
      () => fruitForm.value,
      () => {
        console.log(fruitForm.value);
      },
    ),
  );

  createEffect(
    on(
      () => fruitForm.value.fruits,
      () => {
        console.log(fruitForm.value);
      },
    ),
  );

  return (
    <form>
      <For each={fruitForm.value.fruits}>
        {(_, index) => (
          <Field of={fruitForm} path={`fruits.${index()}.name`}>
            {(field, props) => <input {...props()} />}
            {/* {(field, props) => <p>jfdsoafjsad</p>} */}
          </Field>
        )}
      </For>
    </form>
  );
}

export function MyForm2() {
  const fruitForm = createForm({
    name: 'banana',
    isTasty: true,
  });

  createEffect(() => {
    console.log('aaa', fruitForm.value);
  });

  createEffect(
    on(
      () => fruitForm,
      () => {
        console.log('bbb', fruitForm);
        console.log('bbb', fruitForm.value);
      },
    ),
  );

  createEffect(
    on(
      () => fruitForm.value.name,
      () => {
        console.log('ccc', fruitForm.value);
      },
    ),
  );

  return (
    <form>
      <Field of={fruitForm} path={`name`}>
        {(field, props) => <input {...props()} />}
        {/* {(field, props) => <p>jfdsoafjsad</p>} */}
      </Field>
      <p>{fruitForm.value.name}</p>
    </form>
  );
}

// type xx = ChangeEventHandlerUnion<HTMLInputElement, Event>;
