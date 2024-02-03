# Zebar

Zebar is a way to create customizable and cross-platform taskbars, desktop widgets, and popups.

[How to contribute](https://github.com/glzr-io/zebar/blob/main/CONTRIBUTING.md)

## Providers

### Battery

## Date

https://moment.github.io/luxon/#/formatting?id=table-of-tokens

### Host

- `hostname` -
- `os_name` - Name of the operating system. This is 'Darwin' on MacOS, 'Windows' on Windows, or the Linux distro name retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. 'Debian GNU/Linux' on Debian).
- `os_version` - Operating system version. This is the version number on MacOS (eg. '13.2.1'), the major version + build number on Windows (eg. '11 22000'), or the Linux distro version retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. '9' on Debian 9).
- `friendly_os_version` - Friendly name of operating system version (eg. 'MacOS 13.2.1', 'Windows 10 Pro', 'Linux Debian GNU/Linux 9').
- `boot_time` - Time when the system booted since UNIX epoch in milliseconds (eg. `1699452379304`).
- `uptime` - Time in milliseconds since boot.
