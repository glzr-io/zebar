Zebar is split into 3 "packages"

- `desktop` - a Tauri app
- `client` - a SolidJS frontend which is opened by Tauri on `zebar open <window_name>`
- `client-api` - business logic for communicating with Tauri

### What happens when a hook is `memoize`'d?

A common pattern throughout `client-api` is to memoize hooks. This is a way to create "singleton" hooks. This comes with the caveat that the SolidJS hooks `onMount` and `onCleanup` run in the context of where they were first initialized. `onCleanup` can therefore run when there are still references to the hook and should be avoided.
