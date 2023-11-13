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

## Providers

### Battery

### Host

- `hostname` -
- `os_name` - Name of the operating system. This is 'Darwin' on MacOS, 'Windows' on Windows, or the Linux distro name retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. 'Debian GNU/Linux' on Debian).
- `os_version` - Operating system version. This is the version number on MacOS (eg. '13.2.1'), the major version + build number on Windows (eg. '11 22000'), or the Linux distro version retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. '9' on Debian 9).
- `friendly_os_version` - Friendly name of operating system version (eg. 'MacOS 13.2.1', 'Windows 10 Pro', 'Linux Debian GNU/Linux 9').
- `boot_time` - Time when the system booted since UNIX epoch in milliseconds (eg. `1699452379304`).
- `uptime` - Time in milliseconds since boot.
