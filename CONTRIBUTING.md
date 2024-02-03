# Contributing to Zebar

Zebar uses the Tauri desktop framework and requires Node.js + Rust.

Rust **nightly** and Node.js **version 20** are currently used.

### Installing Rust

[`rustup`](https://rustup.rs/) is the recommended way to set up the Rust toolchain.

### Installing Node.js

Install Node.js via the [official download](https://nodejs.org/en/download) or a version manager like NVM ([unix](https://github.com/nvm-sh/nvm#installing-and-updating), [windows](https://github.com/coreybutler/nvm-windows?tab=readme-ov-file#overview)), or [pnpm via its standalone script](https://pnpm.io/installation#using-a-standalone-script) and then switching version via `pnpm env use -g 20`.

### Development

To start the project in development mode:

```shell
# Install pnpm (package manager) if not already installed.
npm i -g pnpm

# Install dependencies.
pnpm i

# Start in development mode.
pnpm dev
```

## Architecture

Zebar is split into 3 packages:

- `desktop` - a Tauri app which is a CLI that can spawn windows.
- `client` - a SolidJS frontend which is spawned by Tauri on `zebar open <window_name>`.
- `client-api` - business logic for communicating with Tauri.

### How to create a new provider?

1. **Create a new config schema for your provider.**

   1. Add a schema for the config under [`packages/client-api/src/user-config/window/providers`](https://github.com/glzr-io/zebar/tree/main/packages/client-api/src/user-config/window/providers).
   2. Add the new provider type to the [`ProviderType`](https://github.com/glzr-io/zebar/blob/main/packages/client-api/src/user-config/window/provider-type.model.ts) enum.
   3. Add the schema to the [`ProviderConfigSchema`](https://github.com/glzr-io/zebar/blob/main/packages/client-api/src/user-config/window/provider-config.model.ts) array.

2. **Add the client-side logic for the provider.** Most providers aren't client-side heavy, and simply subscribe to some variables sent from the Tauri backend (eg. [`create-ip-provider.ts`](https://github.com/glzr-io/zebar/tree/main/packages/client-api/src/providers/ip/create-ip-provider.ts)).

   1. Add a new provider under [`packages/client-api/src/providers`](https://github.com/glzr-io/zebar/tree/main/packages/client-api/src/providers).
   2. Add the provider to the switch statement in [`createProvider(...)`](https://github.com/glzr-io/zebar/blob/main/packages/client-api/src/providers/create-provider.ts#L28).

3. **Add the backend logic for the provider.**

   1. Add the logic for the provider under [`packages/desktop/src/providers`](https://github.com/glzr-io/zebar/tree/main/packages/desktop/src/providers).
   2. Add the provider's config to the [`ProviderConfig`](https://github.com/glzr-io/zebar/blob/main/packages/desktop/src/providers/config.rs) enum.
   3. Add the provider's variables to the [`ProviderVariables`](https://github.com/glzr-io/zebar/blob/main/packages/desktop/src/providers/variables.rs) enum.
   4. Add the provider to the switch statement in [`createProvider(...)`](https://github.com/glzr-io/zebar/blob/main/packages/desktop/src/providers/manager.rs#L155).
