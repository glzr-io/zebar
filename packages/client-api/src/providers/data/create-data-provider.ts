import { type Owner, onCleanup, runWithOwner } from 'solid-js';
import { createStore } from 'solid-js/store';

import type { DataProviderConfig } from '~/user-config';

export interface DataVariables { }

export async function createDataProvider(
  config: DataProviderConfig,
  owner: Owner,
) {
  const [dataVariables, setDataVariables] =
    createStore<DataVariables>(getDataVariables());

  const interval = setInterval(
    () => setDataVariables(getDataVariables()),
    config.refresh_interval,
  );

  runWithOwner(owner, () => {
    onCleanup(() => clearInterval(interval));
  });

  function getDataVariables() {
    return {};
  }

  function toPrettyBytes(bytes: number, decimals = 2) {
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

    return formatBytes(k, sizes, bytes, decimals);
  }

  function toPrettyBits(bytes: number, decimals = 2) {
    const k = 1000;
    const sizes = ['b', 'Kb', 'Mb', 'Gb', 'Tb', 'Pb', 'Eb', 'Zb', 'Yb'];

    bytes *= 8;

    return formatBytes(k, sizes, bytes, decimals);
  }

  function formatBytes(k: number, sizes: string[], bytes: number, decimals = 2) {
    const dm = decimals < 0 ? 0 : decimals;

    if (!+bytes)
      return `${0.0.toFixed(dm)} ${sizes[0]}`;

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
  }

  return {
    toPrettyBytes,
    toPrettyBits,
  };
}

/*
fn to_pretty_bytes(input_in_bytes: u64, timespan_in_ms: u64) -> String {   
    let seconds = timespan_in_ms as f32 / f32::powf(10.0, 3.0);
    let input = input_in_bytes as f32 / seconds;

    if input < 1024.0 {
        return format!("0.0 KB");
    }

    let magnitude = input.log(1024 as f32) as u32;
    let base: Option<DataUnit> = num::FromPrimitive::from_u32(magnitude);
    let result = input as f32 / ((1 as u64) << (magnitude * 10)) as f32;
    
    match base {
        Some(DataUnit::B) => format!("{result:.1} B"),
        Some(unit) => format!("{result:.1} {unit}B"),
        None => format!("Unknown data unit"),
    }
}

fn to_pretty_bits(input_in_bytes: u64, timespan_in_ms: u64) -> String {
    if input_in_bytes < 1000 {
        return format!("0.0 Kb");
    }

    let input = input_in_bytes * 8;

    let seconds = timespan_in_ms as f32 / f32::powf(10.0, 3.0);
    let magnitude = input.ilog(1000);
    let base: Option<DataUnit> = num::FromPrimitive::from_u32(magnitude);
    let result = (input as f32 / seconds) / f32::powf(1000.0, magnitude as f32);

    match base {
        Some(DataUnit::B) => format!("{result:.2} b"),
        Some(unit) => format!("{result:.2} {unit}b"),
        None => format!("Unknown data unit"),
    }
}
*/
