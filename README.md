# Zebar

**Zebar lets you create customizable and cross-platform taskbars, desktop widgets, and popups.**

![sample-config-bar](https://github.com/glzr-io/zebar/assets/34844898/859cd563-5d7b-4236-b9bd-88acdf82d7e4)

[🔧 Development & how to contribute](https://github.com/glzr-io/zebar/blob/main/CONTRIBUTING.md)

#### Readme contents

1. [Installation](#%EF%B8%8F-installation)
2. [Migration from GlazeWM bar](#%EF%B8%8F-migration-from-glazewm-bar)
3. [Intro to Zebar](#-intro-to-zebar)
   - [Styled with HTML + CSS](#concept-1-styled-with-html--css)
   - [Reactive "providers"](#concept-2-reactive-providers)
   - [Templating language](#concept-3-templating-language)
4. [Providers](#-providers)

## ⚙️ Installation

**Downloads for Windows, MacOS, and Linux are available in the [latest release](https://github.com/glzr-io/zebar/releases)**.

A default start script can also be downloaded from the release. Run the script after install to launch the default bar, which will create a config file located at `%userprofile%/.glzr/zebar/config.yaml`.

## ➡️ Migration from GlazeWM bar

Modify the following GlazeWM config options (at `%userprofile%/.glaze-wm/config.yaml`):

```yaml
gaps:
  # Add more spacing at the top.
  outer_gap: '45px 20px 20px 20px'

bar:
  # Disable the built-in GlazeWM bar.
  enabled: false

window_rules:
  # Ignore the bar window.
  - command: 'ignore'
    match_process_name: '/Zebar/'
```

## ➡️ Usage with Komorebi

Modify the `float_rules` in the Komorebi config options (at `%userprofile%/komorebi.json`).

```json
{
  "float_rules": [
    {
      "kind": "Exe",
      "id": "Zebar.exe",
      "matching_strategy": "Equals"
    }
  ]
}
```

And in the Zebar config (if using the default generated one), replace the GlazeWM element with the following:

```yaml
    template/workspaces:
      styles: |
        display: flex;
        align-items: center;

        .workspace {
          background: rgba(255, 255, 255, 0.05);
          margin-right: 4px;
          width: 30px;
          height: 30px;
          color: #ffffffe6;
          border: none;
          border-radius: 2px;

          &.active {
            background: rgba(255, 255, 255, 0.1);
          }
        }
      providers: ['komorebi']
      template: |
        @for (workspace of komorebi.currentWorkspaces) {
          <button class="workspace {{ workspace === komorebi.focusedWorkspace && 'active' }}">
            {{ workspace.name }}
          </button>
        }
```

## 🌟 Intro to Zebar

There's 3 big differences that set Zebar apart from other similar projects:

- Styled with HTML + CSS
- Reactive "providers" for modifying the bar on the fly
- Templating language

### Concept 1: Styled with HTML + CSS

The **_entire_** html + css of the bar can be customized via the user config. CSS/SCSS can be added to an element via the `styles` property, and child divs can be created via `template/<id>` and `group/<id>` properties.

A basic config might look like this:

```yaml
# Define a new window with an ID of 'example', which can then be launched
# by running 'zebar open example'.
window/example:
  width: '200'
  height: '200'
  position_x: '0'
  position_y: '0'
  styles: |
    background: lightgreen;
    height: 100%;
    width: 100%;
  # Add a child div for showing CPU usage. It uses the CPU provider to
  # get a variable for the current usage (more on that below).
  template/cpu:
    providers: ['cpu']
    template: |
      <p>{{ cpu.usage }}</p>
```

Running `zebar open example` in a terminal will create an instance of this window config. It'll launch a 200x200 window in the corner of the screen where the CPU changes over time.

<p align="middle">
  <img align="top" src="https://github.com/glzr-io/zebar/assets/34844898/e5cd7b7e-97ab-4a6b-b952-8ed8302b710f">
  <img align="top" src="https://github.com/glzr-io/zebar/assets/34844898/3ed88f73-808d-4734-8b99-411cfb2e2b38">
</p>
<p align="middle">
  <i>The resulting window and underlying HTML from the above config.</i>
</p>

`group/<id>` properties are used to add a child div, whereas `template/<id>` properties are used to add a child div that can have a custom HTML template. `group/<id>` properties can be nested infinitely, whereas `template/<id>` properties cannot be nested. The order of these config properties matters as can be seen from the resulting HTML (pic 3).

```yaml
window/example:
  width: '200'
  height: '200'
  position_x: '0'
  position_y: '0'
  group/nested1:
    group/nested2:
      group/nested3:
        template/my_template1:
          template: |
            <span>The mitochondria is the powerhouse of the cell</span>
            <img src="https://google.com/mitochondria.jpg">
  template/my_template2:
    template: |
      <span>Another template</span>
```

<p align="middle">
  <img align="top" src="https://github.com/glzr-io/zebar/assets/34844898/3ea640bf-f1db-4adf-bdfe-93634960d215" width="400px">
</p>
<p align="middle">
  <i>The resulting HTML from the above config.</i>
</p>

### Concept 2: Reactive "providers"

Rather than having predefined components (eg. a cpu component, battery component, etc), Zebar instead introduces **providers**. Providers are a collection of functions and/or variables that can change over time. When a variable changes, it'll cause a reactive change **_wherever_** it is used in the config.

```yaml
window/example:
  providers: ['cpu', 'memory']
  width: '200'
  height: '200'
  # Set position of the window to be based off current memory/cpu usage. We
  # need to round the values since `position_x` and `position_y` only accept
  # whole numbers.
  position_x: '{{ Math.round(cpu.usage) }}'
  position_y: '{{ Math.round(memory.usage) }}'
  template/cpu_and_memory:
    template: |
      CPU usage: {{ cpu.usage }}
      Memory usage: {{ memory.usage}}
```

The above will create a window that jumps around the screen whenever cpu and memory usage changes. _All config properties are reactive to changes in providers._

Providers "trickle down", meaning that a provider declared on a parent element (eg. `window/example`) will be available to any elements below (eg. `template/cpu-and-memory`). So we don't have to repeat declaring a CPU provider if we need it in multiple places; instead move the provider to a parent element.

Providers can also optionally take some config options. For example, the CPU provider refreshes every 5 seconds by default but that can be changed by defining the providers as such:

```yaml
window/example:
  providers:
    - type: 'cpu'
      refresh_interval: 3000
    - type: 'weather'
      latitude: 51.509865
      longitude: -0.118092
```

A full list of providers and their configs is available [here](#-providers).

### Lastly, concept 3: Templating language

Zebar's templating language has support for interpolation tags, if-else statements, for-loops, and switch statements. Just like providers, the templating syntax can be used on any config property (this includes switch, if-else statements etc).

#### Interpolation tags

```yaml
window/example:
  providers: ['weather']
  # Window is only resizable when there is nice weather.
  resizable: "{{ weather.status === 'sunny_day' }}"
```

Any arbitrary JavaScript is accepted within interpolation tags (eg. `{{ Math.random() }}`). However, it'll only re-evaluated when a provider changes.

```yaml
window/example:
  template/weather:
    providers: ['weather']
    # Template will only change when weather temperature variable is updated.
    template: |
      <span>Random number: {{ Math.random() }}</span>
      <span>Temperature: {{ weather.celsiusTemp }}</span>
```

#### If-else statements

```yaml
window/example:
  template/weather:
    providers: ['weather']
    styles: |
      .hot {
        color: red;
      }
    template: |
      @if (weather.celsiusTemp > 30) {
        <p class="hot">It's hot yo</p>
      } @else if (weather.celsiusTemp > 20) {
        <p>It's not that bad</p>
      } @else {
        <p>It's chilly here</p>
      }
```

#### For-loops

```yaml
window/example:
  template/fruit:
    template: |
      @for (fruit of ['apple', 'orange', 'pineapple']) {
        <span>{{ fruit }}</span>
      }
```

#### Switch statements

```yaml
window/example:
  template/weather:
    providers: ['weather']
    template: |
      @switch (weather.status) {
        @case ('clear_day') {<i class="nf nf-weather-day_sunny"></i>}
        @case ('cloudy_day') {<i class="nf nf-weather-day_cloudy"></i>}
        @case ('snow_day') {<i class="nf nf-weather-day_snow"></i>}
        @default {<i class="nf nf-weather-day_sunny"></i>}
      }
```

## 🧩 Providers

- [battery](#Battery)
- [cpu](#CPU)
- [data](#Data)
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

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

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

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

### Variables

| Variable            | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `frequency`         | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `usage`             | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `logicalCoreCount`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `physicalCoreCount` | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `vendor`            | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

## Data

## Functions

| Function       | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `convertBytes` | Convert a given number (bytes) into a custom string format. <br><br>**Parameters**<br><br> - `bytes`: _`number`_ Bytes to convert.<br> - `decimals`: _`number`_ The number of decimals to convert the bytes up to. <br> - `minUnit`: _`string`_ The min unit to convert the bytes to. There are 3 options: bits ('b', 'Kb', 'Mb', 'Gb', 'Tb', 'Pb', 'Eb', 'Zb', 'Yb'), bytes using SI standard ('B', 'KB', 'MB', ... , 'YB') and bytes using IEC standard (' B', 'KiB', 'MiB', ... , 'YiB').<br><br>**Examples:**<br><br> - `convertBytes(1024, 2, 'b')` -> `8.19 Kb`<br> - `convertBytes(1024, 2, 'B')` -> `1.02 KB`<br> - `convertBytes(1024, 2, ' B')` -> `1.00 KiB`<br> - `convertBytes(500, 2, 'KiB')` -> `0.49 KiB`<br> - `convertBytes(2000, 2, 'KiB')` -> `1.95 KiB` | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

## Date

### Provider config

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval` | How often this provider refreshes in milliseconds. | `number`    | `1000`        |

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

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval` | How often this provider refreshes in milliseconds. | `number`    | `60000`       |

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

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval` | How often this provider refreshes in milliseconds. | `number`    | `3600000`     |

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

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

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

| Option             | Description                                        | Option type | Default value |
| ------------------ | -------------------------------------------------- | ----------- | ------------- |
| `refresh_interval` | How often this provider refreshes in milliseconds. | `number`    | `5000`        |

### Variables

| Variable           | Description | Return type          | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ------------------ | ----------- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `defaultInterface` | TODO        | `NetworkInterface`   | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `defaultGateway`   | TODO        | `Gateway`            | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `interfaces`       | TODO        | `NetworkInterface[]` | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `traffic`          | TODO        | `NetworkTraffic`     | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |

### NetworkTraffic

| Variable      | Description                   | Return type |
| ------------- | ----------------------------- | ----------- |
| `received`    | Received bytes per second.    | `number`    |
| `transmitted` | Transmitted bytes per second. | `number`    |

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

| Option             | Description                                                                                            | Option type           | Default value |
| ------------------ | ------------------------------------------------------------------------------------------------------ | --------------------- | ------------- |
| `refresh_interval` | How often this provider refreshes in milliseconds.                                                     | `number`              | `3600000`     |
| `latitude`         | Latitude to retrieve weather for. If not provided, latitude is instead estimated based on public IP.   | `number \| undefined` | `undefined`   |
| `longitude`        | Longitude to retrieve weather for. If not provided, longitude is instead estimated based on public IP. | `number \| undefined` | `undefined`   |

### Variables

| Variable          | Description | Return type | Supported OS                                                                                                                                                                                                                                                                                                                                                                                |
| ----------------- | ----------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `address`         | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxCity`      | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxCountry`   | TODO        | `string`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxLatitude`  | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
| `approxLongitude` | TODO        | `number`    | <img src="https://github.com/glzr-io/zebar/assets/34844898/568e90c8-cd32-49a5-a17f-ab233d41f1aa" alt="microsoft icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/005a0760-da9d-460e-b533-9b2aba7f5c03" alt="apple icon" width="24"><img src="https://github.com/glzr-io/zebar/assets/34844898/1c5d91b1-879f-42a6-945e-912a11daebb4" alt="linux icon" width="24"> |
