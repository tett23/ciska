import { createRoot } from 'react-dom/client';
import { createElement } from 'react';
import { App } from './components';

(() => {
  const el = document.querySelector('#app');
  if (el == null) {
    return;
  }
  const root = createRoot(el);

  root.render(createElement(App));
})();
