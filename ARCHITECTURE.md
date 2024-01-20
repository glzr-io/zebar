Zebar is split into 3 packages:

- `desktop` - a Tauri app which is a CLI that can spawn windows.
- `client` - a SolidJS frontend which is spawned by Tauri on `zebar open <window_name>`.
- `client-api` - business logic for communicating with Tauri.

# How to create a new provider?

1. Create a new config schema for your provider.
   1a. Add a schema for the config under [`packages/client-api/src/user-config/window/providers`](https://github.com/glzr-io/zebar/tree/main/packages/client-api/src/user-config/window/providers).
   1b. Add the new provider type to the [`ProviderType`](https://github.com/glzr-io/zebar/blob/main/packages/client-api/src/user-config/window/provider-type.model.ts) enum.
   1c. Add the schema to the [`ProviderConfigSchema`](https://github.com/glzr-io/zebar/blob/main/packages/client-api/src/user-config/window/provider-config.model.ts) array.

2. Add the client-side logic for the provider. Most providers aren't client-side heavy, and simply subscribe to some variables sent from the Tauri backend (eg. [`create-ip-provider.ts`](https://github.com/glzr-io/zebar/tree/main/packages/client-api/src/providers/ip/create-ip-provider.ts)).
   2a. Add a new provider under [`packages/client-api/src/providers`](https://github.com/glzr-io/zebar/tree/main/packages/client-api/src/providers).
   2b. Add the provider to the switch statement in [`createProvider(...)`](https://github.com/glzr-io/zebar/blob/main/packages/client-api/src/providers/create-provider.ts#L28).

3. Add the backend logic for the provider.
   3a. Add the logic for the provider under [`packages/desktop/src/providers`](https://github.com/glzr-io/zebar/tree/main/packages/desktop/src/providers).
   3b. Add the provider's config to the [`ProviderConfig`](https://github.com/glzr-io/zebar/blob/main/packages/desktop/src/providers/config.rs) enum.
   3c. Add the provider's variables to the [`ProviderVariables`](https://github.com/glzr-io/zebar/blob/main/packages/desktop/src/providers/variables.rs) enum.
   3d. Add the provider to the switch statement in [`createProvider(...)`](https://github.com/glzr-io/zebar/blob/main/packages/desktop/src/providers/manager.rs#L155).
