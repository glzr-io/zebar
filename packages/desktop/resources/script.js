export function focusWorkspace(event, context) {
  console.log('Focus button clicked!', event, context);
  const id = event.target.id;
  context.providers.glazewm.focusWorkspace(id);
}

export function toggleTilingDirection(event, context) {
  console.log('Tiling direction toggled!', event, context);
  context.providers.glazewm.toggleTilingDirection();
}
