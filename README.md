# Zebar

Zebar is a way to create customizable and cross-platform taskbars, desktop widgets, and popups.

## Contributing

Zebar uses the Tauri desktop framework and requires Node.js + Rust.

### Installing Rust

[`rustup`](https://rustup.rs/) is the recommended way to set up the Rust toolchain. It can be installed via:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing Node.js

Install Node.js via [NVM](https://github.com/nvm-sh/nvm#installing-and-updating) (recommended for easily switching between Node.js versions) or the [official download](https://nodejs.org/en/download).

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
