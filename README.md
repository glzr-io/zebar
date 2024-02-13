# Zebar

Zebar is a way to create customizable and cross-platform taskbars, desktop widgets, and popups.

[Development & how to contribute](https://github.com/glzr-io/zebar/blob/main/CONTRIBUTING.md)

## Providers

### Battery

### Provider config

Battery provider doesn't take any configuration options.

### Variables

| Variable           | Description                                                                     | Return type                                                        | Supported OS |
| ------------------ | ------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ------------ |
| `chargePercent`    | Percentage of (aka. 'state of charge'). Returned value is between `0` to `100`. | `number`                                                           | [logos]      |
| `healthPercent`    | Returned value is between `0` to `100`.                                         | `number`                                                           | [logos]      |
| `cycleCount`       | Number of charge/discharge cycles.                                              | `number`                                                           | [logos]      |
| `state`            | aaa                                                                             | `'discharging' \| 'charging'   \| 'full'  \| 'empty' \| 'unknown'` | [logos]      |
| `isCharging`       | Whether the battery is in a `charging` state.                                   | `boolean`                                                          | [logos]      |
| `timeTillEmpty`    | Approximate time in seconds till battery is empty.                              | `number \| null`                                                   | [logos]      |
| `timeTillFull`     | Approximate time in seconds till battery is fully charged.                      | `number \| null`                                                   | [logos]      |
| `powerConsumption` | Battery power consumption in watts.                                             | `number`                                                           | [logos]      |
| `voltage`          | Battery voltage.                                                                | `number \| null`                                                   | [logos]      |

## Date

### Provider config

Date provider doesn't take any configuration options.

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

## Host

### Provider config

Host provider doesn't take any configuration options.

### Variables

| Variable            | Description                                                                                                                                                                                                                                                  | Return type      | Supported OS |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------- | ------------ |
| `hostname`          | Name used to identify the device in various network-related activities.                                                                                                                                                                                      | `string \| null` | [logos]      |
| `osName`            | Name of the operating system. This is `Darwin` on MacOS, `Windows` on Windows, or the Linux distro name retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. `Debian GNU/Linux` on Debian).                                                    | `string \| null` | [logos]      |
| `osVersion`         | Operating system version. This is the version number on MacOS (eg. `13.2.1`), the major version + build number on Windows (eg. `11 22000`), or the Linux distro version retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. `9` on Debian 9). | `string \| null` | [logos]      |
| `friendlyOsVersion` | Friendly name of operating system version (eg. `MacOS 13.2.1`, `Windows 10 Pro`, `Linux Debian GNU/Linux 9`).                                                                                                                                                | `string \| null` | [logos]      |
| `bootTime`          | Time when the system booted since UNIX epoch in milliseconds (eg. `1699452379304`).                                                                                                                                                                          | `string`         | [logos]      |
| `uptime`            | Time in milliseconds since boot.                                                                                                                                                                                                                             | `string`         | [logos]      |
