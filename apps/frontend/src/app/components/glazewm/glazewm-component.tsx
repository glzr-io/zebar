import { createMemo } from 'solid-js';

import defaultTemplate from './glazewm-component.njk?raw';
import { createTemplateElement } from '~/shared/template-parsing';
import { GlazeWMComponentConfig } from '~/shared/user-config';

export function GlazeWMComponent(props: { config: GlazeWMComponentConfig }) {
  const socket = new WebSocket('ws://localhost:61423');

  socket.onopen = function (e) {
    console.log('[open] Connection established');
    console.log('Sending to server');
    socket.send('My name is John');
  };

  socket.onmessage = function (event) {
    console.log(`[message] Data received from server: ${event.data}`);
  };

  socket.onclose = function (event) {
    if (event.wasClean) {
      console.log(
        `[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`,
      );
    } else {
      // e.g. server process killed or network down
      // event.code is usually 1006 in this case
      console.log('[close] Connection died');
    }
  };

  socket.onerror = function (error) {
    console.log(`[error]`);
  };

  const bindings = createMemo(() => {
    return {
      variables: {
        binding_mode: '',
        workspaces: [
          { name: '1', state: 'focused' },
          { name: '2', state: 'active' },
          { name: '3', state: 'normal' },
          { name: '4', state: 'normal' },
        ],
      },
      functions: {
        focus_workspace: () => {},
      },
    };
  });

  return createTemplateElement({
    bindings,
    config: () => props.config,
    defaultTemplate: () => defaultTemplate,
  });
}
