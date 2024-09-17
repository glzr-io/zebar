const bitUnits = ['b', 'Kb', 'Mb', 'Gb', 'Tb', 'Pb', 'Eb', 'Zb', 'Yb'];

const byteCommonUnits = [
  'B',
  'KB',
  'MB',
  'GB',
  'TB',
  'PB',
  'EB',
  'ZB',
  'YB',
];

const byteIECUnits = [
  'B',
  'KiB',
  'MiB',
  'GiB',
  'TiB',
  'PiB',
  'EiB',
  'ZiB',
  'YiB',
];

export enum DataUnit {
  BITS = 'bits',
  SI_BYTES = 'si_bytes',
  IEC_BYTES = 'iec_bytes',
}

export function convertBytes(
  bytes: number,
  decimals: number = 0,
  unitType: DataUnit = DataUnit.BITS,
) {
  let unitIndex = 1; // Kb/KB/KiB

  if (unitType === DataUnit.BITS) {
    bytes *= 8;
    return convert(1000, bitUnits, bytes, decimals, unitIndex);
  }

  if (unitType === DataUnit.SI_BYTES) {
    return convert(1000, byteCommonUnits, bytes, decimals, unitIndex);
  }

  if (unitType === DataUnit.IEC_BYTES) {
    return convert(1024, byteIECUnits, bytes, decimals, unitIndex);
  }

  return 'NoUnit';
}

function convert(
  k: number,
  units: string[],
  bytes: number,
  decimals: number,
  unitIndex: number,
) {
  const dm = decimals < 0 ? 0 : decimals;

  if (!+bytes) {
    return `${(0.0).toFixed(dm)} ${units[unitIndex]}`;
  }

  let i = Math.floor(Math.log(bytes) / Math.log(k));

  if (i < unitIndex) {
    i = unitIndex;
  }

  return `${(bytes / Math.pow(k, i)).toFixed(dm)} ${units[i]?.trimStart()}`;
}
