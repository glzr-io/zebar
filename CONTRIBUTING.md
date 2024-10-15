# Contributing guide

PRs are always a huge help 💛. Check out issues marked as [good first issue](https://github.com/glzr-io/zebar/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) or [help wanted](https://github.com/glzr-io/zebar/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22) to get started.

First fork, then clone the repo:

```shell
git clone git@github.com:your-username/zebar.git
```

If not already installed, [install Rust](#installing-rust) and [Node.js v20](#installing-nodejs), then run:

```shell
# Install pnpm (package manager).
npm i -g pnpm

# Install dependencies.
pnpm i

# Start in development mode.
pnpm dev
```

After making your changes, push to your fork and [submit a pull request](https://github.com/glzr-io/zebar/pulls). Please try to address only a single feature or fix in the PR so that it's easy to review.

### Installing Rust

[rustup](https://rustup.rs/) is the de-facto way to set up the Rust toolchain.

### Installing Node.js

Install Node.js via the [official download](https://nodejs.org/en/download) or a version manager like NVM ([download - works on Unix and WSL/Git Bash on Windows](https://github.com/nvm-sh/nvm#installing-and-updating)).

## Architecture

Zebar is split into 2 packages:

- `desktop`

A Tauri desktop application which acts as the backend for spawning and communicating with windows.

- `client-api`

JS package for communicating with the Tauri backend. Published to npm as [`zebar`](https://www.npmjs.com/package/zebar).

### How to create a new provider?

1. **Add the client-side logic for the provider.** Most providers aren't client-side heavy, and simply subscribe to some outputs sent from the Tauri backend (eg. [`create-ip-provider.ts`](https://github.com/glzr-io/zebar/tree/main/packages/client-api/src/providers/ip/create-ip-provider.ts)).

   1. Add a new provider under [`client-api/src/providers/<YOUR_PROVIDER>`](https://github.com/glzr-io/zebar/tree/main/packages/client-api/src/providers).
   2. Modify [`create-provider.ts`](https://github.com/glzr-io/zebar/blob/main/packages/client-api/src/providers/create-provider.ts) to add the new provider to the `ProviderConfigMap` and `ProviderMap` types, and to create the provider in the switch statement within `createProvider`.
   3. Export the provider's types from [`client-api/src/providers/index.ts`](https://github.com/glzr-io/zebar/blob/main/packages/client-api/src/providers/index.ts).

2. **Add the backend logic for the provider.**

   1. Add the logic for the provider under [`desktop/src/providers/<YOUR_PROVIDER>`](https://github.com/glzr-io/zebar/tree/main/packages/desktop/src/providers).
   2. Add the provider's config to the [`ProviderConfig`](https://github.com/glzr-io/zebar/blob/main/packages/desktop/src/providers/provider_config.rs) enum.
   3. Add the provider's outputs to the [`ProviderOutput`](https://github.com/glzr-io/zebar/blob/main/packages/desktop/src/providers/provider_output.rs) enum.
   4. Add the provider to the switch statement in [`create_provider(...)`](https://github.com/glzr-io/zebar/blob/main/packages/desktop/src/providers/provider_ref.rs#L163).
   5. Add the provider's exports to [`desktop/src/providers/mod.rs`](https://github.com/glzr-io/zebar/blob/main/packages/desktop/src/providers/mod.rs)
