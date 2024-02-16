# Zebar

**Zebar lets you create customizable and cross-platform taskbars, desktop widgets, and popups.**

[🔧 Development & how to contribute](https://github.com/glzr-io/zebar/blob/main/CONTRIBUTING.md)

[[Screenshot of sample config]]

## Installation

**Downloads for Windows, MacOS, and Linux are available in the [latest release](https://github.com/glzr-io/zebar/releases)**. After installing, you can run the default start script located at `%userprofile%/.glzr/zebar/start.bat` (Windows) or `$HOME/.glzr/zebar/start.sh` (MacOS/Linux).

The config file is located `%userprofile%/.glzr/zebar/config.yaml`. A default config is created on startup.

## Migration from GlazeWM bar

Modify the following GlazeWM config options (at `%userprofile%/.glaze-wm/config.yaml`):

```yaml
gaps:
  # Add more spacing at the top.
  outer_gap: '45px 20px 20px 20px'

bar:
  # Disable the built-in GlazeWM bar.
  enabled: false
```

## Intro to Zebar

There's 3 big differences that set Zebar apart from other similar projects:

- Styled with HTML + CSS
- Reactive "providers" for modifying the bar on the fly
- Templating language

## Providers

- [battery](#Battery)
- [cpu](#CPU)
- [date](#Date)
- [glazewm](#GlazeWM)
- [host](#Host)
- [ip](#IP)
- [memory](#Memory)
- [monitors](#Monitors)
- [network](#Network)
- [self](#Self)
- [weather](#Weather)

### Battery

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

### Variables

| Variable           | Description                                                                                                          | Return type                                                        | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------ | -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chargePercent`    | Battery charge as a percentage of maximum capacity (aka. 'state of charge'). Returned value is between `0` to `100`. | `number`                                                           | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `healthPercent`    | Condition of the battery as a percentage of perfect health. Returned value is between `0` to `100`.                  | `number`                                                           | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `cycleCount`       | Number of charge/discharge cycles.                                                                                   | `number`                                                           | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `state`            | State of the battery.                                                                                                | `'discharging' \| 'charging'   \| 'full'  \| 'empty' \| 'unknown'` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `isCharging`       | Whether the battery is in a `charging` state.                                                                        | `boolean`                                                          | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `timeTillEmpty`    | Approximate time in milliseconds till battery is empty.                                                              | `number \| null`                                                   | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `timeTillFull`     | Approximate time in milliseconds till battery is fully charged.                                                      | `number \| null`                                                   | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `powerConsumption` | Battery power consumption in watts.                                                                                  | `number`                                                           | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `voltage`          | Battery voltage.                                                                                                     | `number \| null`                                                   | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### CPU

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

### Variables

| Variable            | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `frequency`         | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `usage`             | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `logicalCoreCount`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `physicalCoreCount` | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `vendor`            | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

## Date

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `1000`        |

### Variables

| Variable | Description                                                                                                              | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| -------- | ------------------------------------------------------------------------------------------------------------------------ | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `new`    | Current date/time as a JavaScript `Date` object. Uses `new Date()` under the hood.                                       | `Date`      | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `now`    | Current date/time as milliseconds since epoch. Uses `Date.now()` under the hood.                                         | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `iso`    | Current date/time as an ISO-8601 string (eg. `2017-04-22T20:47:05.335-04:00`). Uses `date.toISOString()` under the hood. | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### Functions

| Function   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `toFormat` | Format a given date/time into a custom string format. Refer to [table of tokens](https://moment.github.io/luxon/#/formatting?id=table-of-tokens) for available date/time tokens. <br><br> **Examples:**<br> <br> - `toFormat(now, 'yyyy LLL dd')` -> `2023 Feb 13`<br> - `toFormat(now, "HH 'hours and' mm 'minutes'")` -> `20 hours and 55 minutes`<br> <br> **Parameters:**<br> <br> - `now`: _`number`_ Date/time as milliseconds since epoch.<br> - `format`: _`string`_ Custom string format. | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### GlazeWM

### Provider config

GlazeWM provider doesn't take any config options.

### Variables

| Variable              | Description | Return type   | Supported OS                                                                                                                      |
| --------------------- | ----------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `workspacesOnMonitor` | TODO        | `Workspace[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |

## Host

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `60000`       |

### Variables

| Variable            | Description                                                                                                                                                                                                                                                  | Return type      | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `hostname`          | Name used to identify the device in various network-related activities.                                                                                                                                                                                      | `string \| null` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `osName`            | Name of the operating system. This is `Darwin` on MacOS, `Windows` on Windows, or the Linux distro name retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. `Debian GNU/Linux` on Debian).                                                    | `string \| null` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `osVersion`         | Operating system version. This is the version number on MacOS (eg. `13.2.1`), the major version + build number on Windows (eg. `11 22000`), or the Linux distro version retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. `9` on Debian 9). | `string \| null` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `friendlyOsVersion` | Friendly name of operating system version (eg. `MacOS 13.2.1`, `Windows 10 Pro`, `Linux Debian GNU/Linux 9`).                                                                                                                                                | `string \| null` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `bootTime`          | Time when the system booted since UNIX epoch in milliseconds (eg. `1699452379304`).                                                                                                                                                                          | `string`         | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `uptime`            | Time in milliseconds since boot.                                                                                                                                                                                                                             | `string`         | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### IP

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `3600000`     |

### Variables

| Variable          | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ----------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `address`         | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxCity`      | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxCountry`   | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxLatitude`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxLongitude` | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### Memory

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

### Variables

| Variable      | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `usage`       | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `freeMemory`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `usedMemory`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `totalMemory` | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `freeSwap`    | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `usedSwap`    | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `totalSwap`   | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### Monitors

### Provider config

Monitors provider doesn't take any config options.

### Variables

| Variable    | Description | Return type                | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ----------- | ----------- | -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `primary`   | TODO        | `MonitorInfo \| undefined` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `secondary` | TODO        | `MonitorInfo[]`            | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `all`       | TODO        | `MonitorInfo[]`            | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### Network

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

### Variables

| Variable     | Description | Return type          | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------ | ----------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interfaces` | TODO        | `NetworkInterface[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### Self

### Provider config

Self provider doesn't take any config options.

### Variables

| Variable       | Description                                          | Return type                                     | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| -------------- | ---------------------------------------------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `args`         | Args used to open the window.                        | `Record<string, string>`                        | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `env`          | Environment variables when window was opened.        | `Record<string, string>`                        | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `providers`    | Map of this element's providers and their variables. | `Record<string, unknown>`                       | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `id`           | ID of this element.                                  | `string`                                        | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `type`         | Type of this element.                                | `ElementType`                                   | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `rawConfig`    | Unparsed config for this element.                    | `unknown`                                       | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `parsedConfig` | Parsed config for this element.                      | `WindowConfig \| GroupConfig \| TemplateConfig` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `globalConfig` | Global user config.                                  | `GlobalConfig`                                  | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### Weather

### Provider config

| Option                | Description                                                                                            | Option type           | Default value |
| --------------------- | ------------------------------------------------------------------------------------------------------ | --------------------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds.                                                     | `number`              | `3600000`     |
| `latitude`            | Latitude to retrieve weather for. If not provided, latitude is instead estimated based on public IP.   | `number \| undefined` | `undefined`   |
| `longitude`           | Longitude to retrieve weather for. If not provided, longitude is instead estimated based on public IP. | `number \| undefined` | `undefined`   |

### Variables

| Variable          | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ----------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `address`         | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxCity`      | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxCountry`   | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxLatitude`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxLongitude` | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
