Zebar is split into 3 packages:

- `desktop` - a Tauri app which is a CLI that can spawn windows.
- `client` - a SolidJS frontend which is spawned by Tauri on `zebar open <window_name>`.
- `client-api` - business logic for communicating with Tauri.

# How to create a new provider?

1. Create a new config schema for your provider.
   1a. Add a schema for the config under [`packages/client-api/src/user-config/window/providers`](https://github.com/glazerdesktop/zebar/tree/main/packages/client-api/src/user-config/window/providers).
   1b. Add the new provider type to [`ProviderType`](https://github.com/glazerdesktop/zebar/blob/main/packages/client-api/src/user-config/window/provider-type.model.ts).
   1c. Add the schema to [`ProviderConfigSchema`](https://github.com/glazerdesktop/zebar/blob/main/packages/client-api/src/user-config/window/provider-config.model.ts).

2. Add the client-side logic for the provider. Most providers aren't client-side heavy, and simply subscribe to some variables sent from the Tauri backend (eg. [`create-ip-provider.ts`](https://github.com/glazerdesktop/zebar/tree/main/packages/client-api/src/providers/ip/create-ip-provider.ts)).
   2a. Add a new provider under [`packages/client-api/src/providers`](https://github.com/glazerdesktop/zebar/tree/main/packages/client-api/src/providers).

3. Add the backend logic for the provider.
   3a.
