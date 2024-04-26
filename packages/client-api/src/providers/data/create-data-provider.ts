import { type Owner } from 'solid-js';
import { createStore } from 'solid-js/store';

import type { DataProviderConfig } from '~/user-config';

export interface DataVariables { }

export async function createDataProvider(
  config: DataProviderConfig,
  owner: Owner,
) {
  const [dataVariables, setDataVariables] =
    createStore<DataVariables>(getDataVariables());

  function getDataVariables() {
    return {};
  }

  const bitUnits = ['b', 'Kb', 'Mb', 'Gb', 'Tb', 'Pb', 'Eb', 'Zb', 'Yb'];
  const byteCommonUnits = ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
  const byteIECUnits = [' B', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'];

  function convertBytes(bytes: number, decimals: number, minUnit: string) {
    let unitIndex = bitUnits.findIndex(u => u === minUnit);

    if (unitIndex !== -1) {
      bytes *= 8;
      return convert(1000, bitUnits, bytes, decimals, unitIndex);
    }

    unitIndex = byteCommonUnits.findIndex(u => u === minUnit);

    if (unitIndex !== -1) {
      return convert(1000, byteCommonUnits, bytes, decimals, unitIndex);
    }

    unitIndex = byteIECUnits.findIndex(u => u === minUnit);

    if (unitIndex !== -1) {
      return convert(1024, byteIECUnits, bytes, decimals, unitIndex);
    }

    return 'NoUnit';
  }

  function convert(k: number, units: string[], bytes: number, decimals: number, unitIndex: number) {
    const dm = decimals < 0 ? 0 : decimals;

    if (!+bytes) {
      return `${0.0.toFixed(dm)} ${units[unitIndex]}`;
    }

    let i = Math.floor(Math.log(bytes) / Math.log(k));

    if (i < unitIndex) {
      i = unitIndex;
    }

    return `${(bytes / Math.pow(k, i)).toFixed(dm)} ${units[i]?.trimStart()}`;
  }

  return {
    convertBytes,
  };
}
