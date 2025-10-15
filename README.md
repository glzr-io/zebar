<div align="center">

  <br>
  <img src="https://github.com/user-attachments/assets/d1e10485-2cbb-4434-9d98-5c74702eebcc" width="350" alt="Zebar logo" />
  <br>

# Zebar 🦓

**Zebar lets you create customizable and cross-platform desktop widgets.**

[![Discord invite][discord-badge]][discord-link]
[![Good first issues][issues-badge]][issues-link]

[Installation](#%EF%B8%8F-installation) •
[Intro](#-intro-to-zebar) •
[FAQ](#-faq) •
[Contributing ↗](https://github.com/glzr-io/zebar/blob/main/CONTRIBUTING.md)

![zebar-demo](https://github.com/user-attachments/assets/7a3aa5d2-2c7a-4171-9ede-d88839b91666)

</div>

## ⚙️ Installation

**Downloads for Windows, MacOS, and Linux are available in the [latest release](https://github.com/glzr-io/zebar/releases)**.

For building locally, follow the instructions [here](https://github.com/glzr-io/zebar/blob/main/CONTRIBUTING.md).

## 🚀 Quick start

### Accessing Zebar GUI

Click the Zebar icon in your system tray to open the GUI interface.

### Installing widgets

Navigate to the marketplace within the GUI to browse and install widget packs.

### Auto-starting widgets

Widgets can be set to run on startup by right clicking the Zebar tray icon then going to `Widget packs` -> `<pack name>` -> `<widget name>` -> `Run on startup` and clicking the widget name to check it.

## 🌟 Intro to Zebar

Widgets are powered by native webviews (_similar_ to Electron, but more lightweight).

## Creating your own widgets

Zebar looks within the `~/.glzr/zebar` directory for directories containing a `zpack.json` file. Each `zpack.json` file defines a widget pack. Each widget pack can contain multiple widgets, each defined  individually within the `zpack.json` file.

For example, a widget pack config could be located at `~/.glzr/zebar/example-widget/zpack.json`, which would create a widget pack with an id of `example-widget`.
Zebar only looks for `zpack.json` files one level deep within the `~/.glzr/zebar` directory, so `~/glzr/zebar/example-widget/widget-1/zpack.json` is invalid.

In the `My widgets` tab of the Zebar GUI, you can click `Create new pack` to generate a widget pack scaffold with the given name and description.

Within each widget pack, in the `Widgets` section you can click `Add widget` to create a new widget within that pack. When creating a new widget you can pick between some templates to get you started.

> Apart from the React buildless template, other available templates require you to have Node.js and a package manager (npm/pnpm) installed. For these templates, run the build command (e.g. pnpm build) from the widget directory after any source code changes.

### Modifying marketplace widgets

To edit a marketplace widget without fear of your changes being overridden, you should copy the folder from the marketplace directory (located at `%AppData%/zebar/downloads/`) to your own Zebar directory (located at `~/.glzr/zebar/`).

> **DO NOT** re-publish other peoples widgets without permission.

### Advanced Customization

#### What is the zpack.json file?

Widgets are stored in packs, with each pack being defined by a `zpack.json` file. Each pack can contain multiple widgets, each individually defined within `zpack.json`.

Widgets consist of an HTML file, and any surrounding assets (e.g. CSS, JS, images, etc.).

The schema for `zpack.json` can be found [here](https://github.com/glzr-io/zebar/blob/main/resources/zpack-schema.json) although all options can also be configured via the GUI.

#### Querying system information

Zebar exposes various system information (referred to as "providers") which can be used and displayed by your frontend. This includes stats like CPU usage, battery info, various window manager integrations, and lots more. See [Providers](#-providers) for a list of available providers.

## ❓ FAQ

**Q: Help! On Windows, Zebar is failing to start?**

In some cases, updating to the latest Microsoft Webview2 version is needed ([standalone download](https://developer.microsoft.com/en-us/microsoft-edge/webview2/?form=MA13LH#download)). Run the "Evergreen Standalone Installer" as administrator.

## 🧩 Providers

Through the `zebar` NPM package, Zebar exposes various system information via reactive "providers". Providers are a collection of functions and variables that can change over time.

- [audio](#audio)
- [battery](#battery)
- [cpu](#cpu)
- [date](#date)
- [disk](#disk)
- [glazewm](#glazewm)
- [host](#host)
- [ip](#ip)
- [keyboard](#keyboard)
- [komorebi](#komorebi)
- [media](#media)
- [memory](#memory)
- [network](#network)
- [systray](#systray)
- [weather](#weather)

### Audio

#### Config

No config options.

#### Outputs

| Variable            | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `defaultPlaybackDevice`         | Default audio playback device.        | `AudioDevice \| null`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `defaultRecordingDevice`         | Default audio recording device.        | `AudioDevice \| null`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `playbackDevices`         | All audio playback devices.        | `AudioDevice[]`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `recordingDevices`         | All audio recording devices.        | `AudioDevice[]`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `allDevices`         | All audio devices (both playback and recording).        | `AudioDevice[]`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |

| Function   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `setVolume` | Sets the volume of an audio device. Changes the volume of the default playback device, unless `SetVolumeOptions.deviceId` is specified. <br><br> **Parameters:**<br> - `volume`: _`number`_ Volume as a % of maximum volume. Returned value is between `0` and `100`. <br> - `options`: _`SetVolumeOptions \| undefined`_ Additional options.<br>  | `Promise<void>`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |

#### Related types

#### `AudioDevice`

| Variable           | Description                   | Return type             |
| ------------------ | ----------------------------- | ----------------------- |
| `deviceId`         | Device ID. | `string` |
| `name`      | Friendly display name of device. | `string` |
| `volume`    | Volume as a % of maximum volume. Returned value is between `0` and `100`. | `number` |
| `type` | Type of the device. | `'playback' \| 'recording'` |
| `isDefaultPlayback` | `true` if the device is selected as the default playback device.| `boolean` |
| `isDefaultRecording` | `true` if the device is selected as the default recording device.| `boolean` |

#### `SetVolumeOptions`

| Variable           | Description                   | Return type             |
| ------------------ | ----------------------------- | ----------------------- |
| `deviceId`         | Device ID to set the volume of. | `string \| undefined` |

### Battery

#### Config

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refreshInterval` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

#### Outputs

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

#### Config

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refreshInterval` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

#### Outputs

| Variable            | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `frequency`         | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `usage`             | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `logicalCoreCount`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `physicalCoreCount` | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `vendor`            | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

## Date

#### Config

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `formatting`         | Formatting of the current date into a custom string format. Affects the output of [`formatted`](#outputs-2). <br><br>Refer to [table of tokens](https://moment.github.io/luxon/#/formatting?id=table-of-tokens) for available date/time tokens. <br><br> **Examples:**<br> - `'yyyy LLL dd'` -> `2023 Feb 13`<br> - `"HH 'hours and' mm 'minutes'"` -> `20 hours and 55 minutes` | `string`    | `EEE d MMM t`       |
| `timezone`         | Either a UTC offset (eg. `UTC+8`) or an IANA timezone (eg. `America/New_York`). Affects the output of [`formatted`](#outputs-2).<br><br> A full list of available IANA timezones can be found [here](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones#List).| `string`    | `local`       |
| `locale`           | An ISO-639-1 locale, which is either a 2-letter language code (eg. `en`) or a 4-letter language + country code (eg. `en-gb`). Affects the output of [`formatted`](#outputs-2).<br><br> A full list of ISO-639-1 locales can be found [here](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes#Table).  | `string`    |       |
| `refreshInterval` | How often this provider refreshes in milliseconds. | `number`    | `1000`        |

#### Outputs

| Variable | Description                                                                                                              | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| -------- | ------------------------------------------------------------------------------------------------------------------------ | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `formatted` | Current date/time as a formatted string. | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `new`    | Current date/time as a JavaScript `Date` object. Uses `new Date()` under the hood.                                       | `Date`      | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `now`    | Current date/time as milliseconds since epoch. Uses `Date.now()` under the hood.                                         | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `iso`    | Current date/time as an ISO-8601 string (eg. `2017-04-22T20:47:05.335-04:00`). Uses `date.toISOString()` under the hood. | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### Disk

#### Config

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refreshInterval` | How often this provider refreshes in milliseconds. | `number`    | `60000`        |

#### Outputs

| Variable            | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `disks`         | Available disks on the system.        | `Disk[]`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

#### Related types

#### `Disk`

| Variable           | Description                   | Return type             |
| ------------------ | ----------------------------- | ----------------------- |
| `name`         | Disk name.    | `string \| null` |
| `fileSystem`      | File system used on the disk (e.g. `EXT4`, `NTFS`). | `string` |
| `mountPoint`    | Mount point of the disk (e.g. `/`, `C:\\`).         | `string` |
| `totalSpace` | Total disk size.      | `DataSizeMeasure` |
| `availableSpace` | Available disk size.      | `DataSizeMeasure` |
| `isRemovable` | Whether the disk is removable.      | `boolean` |
| `driveType` | Type of disk (e.g. `HDD`, `SSD`).      | `string` |

#### `DataSizeMeasure`

| Variable   | Description                                                                 | Return type |
| ---------- | --------------------------------------------------------------------------- | ----------- |
| `bytes`    | Raw byte value.                                                             | `number`    |
| `siValue`  | Bytes converted in according to the SI standard. 1000 bytes in a kilobyte.  | `number`    |
| `siUnit`   | Unit of the converted bytes in according to the SI standard. KB, MB, ...    | `string`    |
| `iecValue` | Bytes converted in according to the IEC standard. 1024 bytes in a kibibyte. | `number`    |
| `iecUnit`  | Unit of the converted bytes in according to the IEC standard. KiB, MiB, ... | `string`    |

### GlazeWM

#### Config

No config options.

#### Outputs

| Variable              | Description | Return type   | Supported OS                                                                                                                      |
| --------------------- | ----------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `displayedWorkspace` | Workspace displayed on the current monitor.        | `Workspace` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `focusedWorkspace` | Workspace that currently has focus (on any monitor).        | `Workspace` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `currentWorkspaces` | Workspaces on the current monitor.        | `Workspace[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `allWorkspaces` | Workspaces across all monitors.        | `Workspace[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `allMonitors` | All monitors.        | `Monitor[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `allWindows` | All windows.        | `Window[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `focusedMonitor` | Monitor that currently has focus.        | `Monitor` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `currentMonitor` | Monitor that is nearest to this Zebar widget.        | `Monitor` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `focusedContainer` | Container that currently has focus (on any monitor).        | `Container` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `tilingDirection` | Tiling direction of the focused container.        | `TilingDirection` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `bindingModes` | Active binding modes.        | `BindingModeConfig[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `isPaused` | Whether the WM is paused. | `boolean` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |

| Function   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `runCommand` | Invokes a WM command. <br><br> **Examples:**<br> - `runCommand("focus --workspace 1")`<br> - `runCommand("set-floating", containerId)`<br> <br> **Parameters:**<br> - `command`: _`string`_ WM command to run (e.g. `"focus --workspace 1"`).<br> - `subjectContainerId`: _`string \| undefined`_ (Optional) ID of container to use as subject. If not provided, this defaults to the currently focused container. | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |

## Host

#### Config

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refreshInterval` | How often this provider refreshes in milliseconds. | `number`    | `60000`       |

#### Outputs

| Variable            | Description                                                                                                                                                                                                                                                  | Return type      | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `hostname`          | Name used to identify the device in various network-related activities.                                                                                                                                                                                      | `string \| null` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `osName`            | Name of the operating system. This is `Darwin` on MacOS, `Windows` on Windows, or the Linux distro name retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. `Debian GNU/Linux` on Debian).                                                    | `string \| null` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `osVersion`         | Operating system version. This is the version number on MacOS (eg. `13.2.1`), the major version + build number on Windows (eg. `11 22000`), or the Linux distro version retrieved from either `/etc/os-release` or `/etc/lsb-release` (eg. `9` on Debian 9). | `string \| null` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `friendlyOsVersion` | Friendly name of operating system version (eg. `MacOS 13.2.1`, `Windows 10 Pro`, `Linux Debian GNU/Linux 9`).                                                                                                                                                | `string \| null` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `bootTime`          | Time when the system booted since UNIX epoch in milliseconds (eg. `1699452379304`).                                                                                                                                                                          | `string`         | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `uptime`            | Time in milliseconds since boot.                                                                                                                                                                                                                             | `string`         | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### IP

#### Config

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refreshInterval` | How often this provider refreshes in milliseconds. | `number`    | `3600000`     |

#### Outputs

| Variable          | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ----------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `address`         | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxCity`      | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxCountry`   | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxLatitude`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxLongitude` | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### Keyboard

#### Config

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refreshInterval` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

#### Outputs

| Variable      | Description                           | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------- | ------------------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `layout`    | Current keyboard layout, for example 'en-US'. | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24">

### Komorebi

#### Config

No config options.

#### Outputs

| Variable              | Description | Return type   | Supported OS                                                                                                                      |
| --------------------- | ----------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `displayedWorkspace` | Workspace displayed on the current monitor.        | `KomorebiWorkspace` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `focusedWorkspace` | Workspace that currently has focus (on any monitor).        | `KomorebiWorkspace` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `currentWorkspaces` | Workspaces on the current monitor.        | `KomorebiWorkspace[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `allWorkspaces` | Workspaces across all monitors.        | `KomorebiWorkspace[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `allMonitors` | All monitors.        | `KomorebiMonitor[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `focusedMonitor` | Monitor that currently has focus.        | `KomorebiMonitor` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `currentMonitor` | Monitor that is nearest to this Zebar widget.        | `KomorebiMonitor` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |

### Media

#### Config

No config options.

#### Outputs

| Variable            | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `currentSession`         | Currently playing media session.        | `MediaSession \| null`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `allSessions`         | All active media sessions.        | `MediaSession[]`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |

| Function   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `play` | Plays the media session. Affects the current media session, unless `MediaControlOptions.sessionId` is specified. <br><br> **Parameters:**<br>`options`: _`MediaControlOptions \| undefined`_ Additional options.<br>  | `Promise<void>`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `pause` | Pauses the media session. Affects the current media session, unless `MediaControlOptions.sessionId` is specified. <br><br> **Parameters:**<br>`options`: _`MediaControlOptions \| undefined`_ Additional options.<br>  | `Promise<void>`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `togglePlayPause` | Toggles between playing/pausing the media session. Affects the current media session, unless `MediaControlOptions.sessionId` is specified. <br><br> **Parameters:**<br>`options`: _`MediaControlOptions \| undefined`_ Additional options.<br>  | `Promise<void>`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `next` | Skips to the next track. Affects the current media session, unless `MediaControlOptions.sessionId` is specified. <br><br> **Parameters:**<br>`options`: _`MediaControlOptions \| undefined`_ Additional options.<br>  | `Promise<void>`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |
| `previous` | Skips to the previous track. Affects the current media session, unless `MediaControlOptions.sessionId` is specified. <br><br> **Parameters:**<br>`options`: _`MediaControlOptions \| undefined`_ Additional options.<br>  | `Promise<void>`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |

#### Related types

#### `MediaSession`

| Variable           | Description                   | Return type             |
| ------------------ | ----------------------------- | ----------------------- |
| `sessionId`         | Unique identifier for the media session.    | `string` |
| `title`         | Title of the media session.    | `string` |
| `artist`      | Artist of the media session. | `string \| null` |
| `albumTitle`    | Album title of the media session.         | `string \| null` |
| `albumArtist` | Artist of the album (may differ from track artist). | `string \| null` |
| `trackNumber` | Position of the track within its album.      | `number` |
| `startTime` | Timestamp (in seconds) where playback begins.      | `number` |
| `endTime` | Timestamp (in seconds) where playback ends.      | `number` |
| `position` | Timestamp (in seconds) of current playback position.      | `number` |
| `isPlaying` | Whether the media session is currently playing.      | `boolean` |
| `isCurrentSession` | Whether this is the currently active session (i.e. `currentSession`).      | `boolean` |

#### `MediaControlOptions`

| Variable           | Description                   | Return type             |
| ------------------ | ----------------------------- | ----------------------- |
| `sessionId`         | ID of the session to control. If `undefined`, the current session is used. | `string \| undefined` |

### Memory

#### Config

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refreshInterval` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

#### Outputs

| Variable      | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `usage`       | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `freeMemory`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `usedMemory`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `totalMemory` | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `freeSwap`    | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `usedSwap`    | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `totalSwap`   | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### Network

#### Config

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refreshInterval` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

#### Outputs

| Variable           | Description                            | Return type          | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------ | -------------------------------------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `defaultInterface` | TODO                                   | `NetworkInterface`   | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `defaultGateway`   | TODO                                   | `Gateway`            | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `interfaces`       | TODO                                   | `NetworkInterface[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `traffic`          | Returns the network traffic per second. | `NetworkTraffic`     | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

#### Related types

#### `NetworkTraffic`

| Variable           | Description                   | Return type             |
| ------------------ | ----------------------------- | ----------------------- |
| `received`         | Received bytes per second.    | `DataSizeMeasure` |
| `transmitted`      | Transmitted bytes per second. | `DataSizeMeasure` |
| `totalReceived`    | Total received bytes.         | `DataSizeMeasure` |
| `totalTransmitted` | Total transmitted bytes.      | `DataSizeMeasure` |

#### `DataSizeMeasure`

| Variable   | Description                                                                 | Return type |
| ---------- | --------------------------------------------------------------------------- | ----------- |
| `bytes`    | Raw byte value.                                                             | `number`    |
| `siValue`  | Bytes converted in according to the SI standard. 1000 bytes in a kilobyte.  | `number`    |
| `siUnit`   | Unit of the converted bytes in according to the SI standard. KB, MB, ...    | `string`    |
| `iecValue` | Bytes converted in according to the IEC standard. 1024 bytes in a kibibyte. | `number`    |
| `iecUnit`  | Unit of the converted bytes in according to the IEC standard. KiB, MiB, ... | `string`    |

### Systray

#### Config

No config options.

#### Outputs

| Variable          | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ----------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `icons`         | All Windows taskbar system tray icons.  | `SystrayIcon[]`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"> |

#### Related Types

#### SystrayIcon

| Variable          | Description | Return type |
| ----------------- | ----------- | ----------- |
| `id`         | ID of the icon. |  `string`    |
| `iconBytes`         | Icon image as a PNG byte array. |  `number[]`    |
| `iconBlob`         | Icon image as a PNG blob. |  `Blob`    |
| `iconHash`         | Hash of the icon image. Used to determine if the icon image has changed.  |  `string`    |
| `iconUrl`         | Icon image URL ready to be be referenced in `<img>` elements.  |  `string`    |
| `tooltip`         | Tooltip text when hovering over the icon.  |  `string`    |

### Weather

#### Config

| Option             | Description                                                                                            | Option type           | Default value |
| ------------------ | ------------------------------------------------------------------------------------------------------ | --------------------- | ------------- |
| `latitude`         | Latitude to retrieve weather for. If not provided, latitude is instead estimated based on public IP.   | `number \| undefined` | `undefined`   |
| `longitude`        | Longitude to retrieve weather for. If not provided, longitude is instead estimated based on public IP. | `number \| undefined` | `undefined`   |
| `refreshInterval` | How often this provider refreshes in milliseconds.                                                     | `number`              | `3600000`     |

#### Outputs

| Variable          | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ----------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `isDaytime`         | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `status`      | TODO        | `WeatherStatus`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `celsiusTemp`   | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `fahrenheitTemp`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `windSpeed` | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

[discord-badge]: https://img.shields.io/discord/1041662798196908052.svg?logo=discord&colorB=7289DA
[discord-link]: https://discord.gg/ud6z3qjRvM
[issues-badge]: https://img.shields.io/badge/good_first_issues-7057ff
[issues-link]: https://github.com/glzr-io/zebar/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22
