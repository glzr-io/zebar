# Zebar

**Zebar lets you create customizable and cross-platform taskbars, desktop widgets, and popups.**

[🔧 Development & how to contribute](https://github.com/glzr-io/zebar/blob/main/CONTRIBUTING.md)

[[Screenshot of sample config]]

## Installation

**Downloads for Windows, MacOS, and Linux are available in the [latest release](https://github.com/glzr-io/zebar/releases)**. After installing, you can run the default start script located at `%userprofile%/.glzr/zebar/start.cmd` (Windows) or `$HOME/.glzr/zebar/start.sh` (MacOS/Linux).

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

| Variable           | Description                                                                     | Return type                                                        | Supported OS |
| ------------------ | ------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ------------ |
| `chargePercent`    | Percentage of (aka. 'state of charge'). Returned value is between `0` to `100`. | `number`                                                           | [logos]      |
| `healthPercent`    | Returned value is between `0` to `100`.                                         | `number`                                                           | [logos]      |
| `cycleCount`       | Number of charge/discharge cycles.                                              | `number`                                                           | [logos]      |
| `state`            | aaa                                                                             | `'discharging' \| 'charging'   \| 'full'  \| 'empty' \| 'unknown'` | [logos]      |
| `isCharging`       | Whether the battery is in a `charging` state.                                   | `boolean`                                                          | [logos]      |
| `timeTillEmpty`    | Approximate time in milliseconds till battery is empty.                         | `number \| null`                                                   | [logos]      |
| `timeTillFull`     | Approximate time in milliseconds till battery is fully charged.                 | `number \| null`                                                   | [logos]      |
| `powerConsumption` | Battery power consumption in watts.                                             | `number`                                                           | [logos]      |
| `voltage`          | Battery voltage.                                                                | `number \| null`                                                   | [logos]      |

### CPU

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

### Variables

| Variable            | Description | Return type | Supported OS |
| ------------------- | ----------- | ----------- | ------------ |
| `frequency`         | TODO        | `number`    | [logos]      |
| `usage`             | TODO        | `number`    | [logos]      |
| `logicalCoreCount`  | TODO        | `number`    | [logos]      |
| `physicalCoreCount` | TODO        | `number`    | [logos]      |
| `vendor`            | TODO        | `string`    | [logos]      |

## Date

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `1000`        |

### Variables

| Variable | Description                                                                                                              | Return type | Supported OS |
| -------- | ------------------------------------------------------------------------------------------------------------------------ | ----------- | ------------ |
| `new`    | Current date/time as a JavaScript `Date` object. Uses `new Date()` under the hood.                                       | `Date`      | [logo]       |
| `now`    | Current date/time as milliseconds since epoch. Uses `Date.now()` under the hood.                                         | `number`    | [logo]       |
| `iso`    | Current date/time as an ISO-8601 string (eg. `2017-04-22T20:47:05.335-04:00`). Uses `date.toISOString()` under the hood. | `string`    | [logo]       |

### Functions

| Function   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Return type | Supported OS |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | ------------ |
| `toFormat` | Format a given date/time into a custom string format. Refer to [table of tokens](https://moment.github.io/luxon/#/formatting?id=table-of-tokens) for available date/time tokens. <br><br> **Examples:**<br> <br> - `toFormat(now, 'yyyy LLL dd')` -> `2023 Feb 13`<br> - `toFormat(now, "HH 'hours and' mm 'minutes'")` -> `20 hours and 55 minutes`<br> <br> **Parameters:**<br> <br> - `now`: _`number`_ Date/time as milliseconds since epoch.<br> - `format`: _`string`_ Custom string format. | `string`    | [logos]      |

### GlazeWM

### Provider config

GlazeWM provider doesn't take any config options.

### Variables

| Variable              | Description | Return type   | Supported OS |
| --------------------- | ----------- | ------------- | ------------ |
| `workspacesOnMonitor` | TODO        | `Workspace[]` | [logos]      |

## Host

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `60000`       |

### Variables

| Variable            | Description                                                                                                                                                                                                                                                  | Return type      | Supported OS |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------- | ------------ |
| `hostname`          | Name used to identify the device in various network-related activities.                                                                                                                                                                                      | `string \| null` | [logos]      |
| `osName`            | Name of the operating system. This is `Darwin` on MacOS, `Windows` on Windows, or the Linux distro name retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. `Debian GNU/Linux` on Debian).                                                    | `string \| null` | [logos]      |
| `osVersion`         | Operating system version. This is the version number on MacOS (eg. `13.2.1`), the major version + build number on Windows (eg. `11 22000`), or the Linux distro version retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. `9` on Debian 9). | `string \| null` | [logos]      |
| `friendlyOsVersion` | Friendly name of operating system version (eg. `MacOS 13.2.1`, `Windows 10 Pro`, `Linux Debian GNU/Linux 9`).                                                                                                                                                | `string \| null` | [logos]      |
| `bootTime`          | Time when the system booted since UNIX epoch in milliseconds (eg. `1699452379304`).                                                                                                                                                                          | `string`         | [logos]      |
| `uptime`            | Time in milliseconds since boot.                                                                                                                                                                                                                             | `string`         | [logos]      |

### IP

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `3600000`     |

### Variables

| Variable          | Description | Return type | Supported OS |
| ----------------- | ----------- | ----------- | ------------ |
| `address`         | TODO        | `string`    | [logos]      |
| `approxCity`      | TODO        | `string`    | [logos]      |
| `approxCountry`   | TODO        | `string`    | [logos]      |
| `approxLatitude`  | TODO        | `number`    | [logos]      |
| `approxLongitude` | TODO        | `number`    | [logos]      |

### Memory

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

### Variables

| Variable      | Description | Return type | Supported OS |
| ------------- | ----------- | ----------- | ------------ |
| `usage`       | TODO        | `number`    | [logos]      |
| `freeMemory`  | TODO        | `number`    | [logos]      |
| `usedMemory`  | TODO        | `number`    | [logos]      |
| `totalMemory` | TODO        | `number`    | [logos]      |
| `freeSwap`    | TODO        | `number`    | [logos]      |
| `usedSwap`    | TODO        | `number`    | [logos]      |
| `totalSwap`   | TODO        | `number`    | [logos]      |

### Monitors

### Provider config

Monitors provider doesn't take any config options.

### Variables

| Variable    | Description | Return type                | Supported OS |
| ----------- | ----------- | -------------------------- | ------------ |
| `primary`   | TODO        | `MonitorInfo \| undefined` | [logos]      |
| `secondary` | TODO        | `MonitorInfo[]`            | [logos]      |
| `all`       | TODO        | `MonitorInfo[]`            | [logos]      |

### Network

### Provider config

| Option                | Description                                        | Option type | Default value |
| --------------------- | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

### Variables

| Variable     | Description | Return type          | Supported OS |
| ------------ | ----------- | -------------------- | ------------ |
| `interfaces` | TODO        | `NetworkInterface[]` | [logos]      |

### Self

### Provider config

Self provider doesn't take any config options.

### Variables

| Variable       | Description                                          | Return type                                     | Supported OS |
| -------------- | ---------------------------------------------------- | ----------------------------------------------- | ------------ |
| `args`         | Args used to open the window.                        | `Record<string, string>`                        | [logos]      |
| `env`          | Environment variables when window was opened.        | `Record<string, string>`                        | [logos]      |
| `providers`    | Map of this element's providers and their variables. | `Record<string, unknown>`                       | [logos]      |
| `id`           | ID of this element.                                  | `string`                                        | [logos]      |
| `type`         | Type of this element.                                | `ElementType`                                   | [logos]      |
| `rawConfig`    | Unparsed config for this element.                    | `unknown`                                       | [logos]      |
| `parsedConfig` | Parsed config for this element.                      | `WindowConfig \| GroupConfig \| TemplateConfig` | [logos]      |
| `globalConfig` | Global user config.                                  | `GlobalConfig`                                  | [logos]      |

### Weather

### Provider config

| Option                | Description                                                                                            | Option type           | Default value |
| --------------------- | ------------------------------------------------------------------------------------------------------ | --------------------- | ------------- |
| `refresh_interval_ms` | How often this provider refreshes in milliseconds.                                                     | `number`              | `3600000`     |
| `latitude`            | Latitude to retrieve weather for. If not provided, latitude is instead estimated based on public IP.   | `number \| undefined` | `undefined`   |
| `longitude`           | Longitude to retrieve weather for. If not provided, longitude is instead estimated based on public IP. | `number \| undefined` | `undefined`   |

### Variables

| Variable          | Description | Return type | Supported OS |
| ----------------- | ----------- | ----------- | ------------ |
| `address`         | TODO        | `string`    | [logos]      |
| `approxCity`      | TODO        | `string`    | [logos]      |
| `approxCountry`   | TODO        | `string`    | [logos]      |
| `approxLatitude`  | TODO        | `number`    | [logos]      |
| `approxLongitude` | TODO        | `number`    | [logos]      |
