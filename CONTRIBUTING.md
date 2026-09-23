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

# Start in development mode. (config: ~/.glzr/zebar/)
pnpm dev
```

After making your changes, push to your fork and [submit a pull request](https://github.com/glzr-io/zebar/pulls). Please try to address only a single feature or fix in the PR so that it's easy to review.

## Linting & formatting

Run these from the repo root:

```shell
pnpm format:check        # cargo + prettier formatting check
pnpm lint:check          # clippy on the host platform (warnings only, no changes)
pnpm lint:fix            # apply clippy autofixes until a fixed point is reached
pnpm lint:check:windows  # clippy on the Windows view (cross-target)
pnpm lint:fix:windows    # apply clippy autofixes on the Windows view
```

- `lint:<action>` runs clippy against the current host, `lint:<action>:windows` against the Windows view. The CI fails if `lint:fix` or `lint:fix:windows` would leave any diff, so run them locally before pushing.
- Desktop uses `x86_64-pc-windows-msvc` for its Windows view; the other crates use `x86_64-pc-windows-gnu`, which cross-builds from a Linux host without extra toolchains.
- `systray-util` only builds for Windows, so it defines only the `lint:*:windows` scripts and is skipped by `lint:*`. Its Windows view is linted as part of the `:windows` pass.
- `pnpm lint:fix` / `cargo clippy --fix` refuse to run while the working tree has uncommitted changes (as the CI check runs on a clean tree).

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

### Using local `zebar` NPM package

Just like in production, when running `pnpm dev`, Zebar will use the widgets found within your config directory (i.e. `~/.glzr/zebar/*`). However, when developing locally, you may want to use a local `zebar` NPM package. To do so, run:

```shell
cd path/to/your/widget

# If using pnpm:
pnpm add --link ../../path/to/zebar/packages/client-api

# If using npm:
npm install ../../path/to/zebar/packages/client-api
```

This will create a symlink to the local `zebar` NPM package and results in a `package.json` similar to this:

```json
{
  "dependencies": {
    "zebar": "link:../../repos/zebar/packages/client-api"
  }
}
```

### Troubleshooting

#### MacOS: Settings UI using outdated dependencies

On MacOS, the settings UI may use cached versions of dependencies. To resolve this:

1. Delete the `packages/settings-ui/.vite` directory
2. Run `pnpm dev` again
