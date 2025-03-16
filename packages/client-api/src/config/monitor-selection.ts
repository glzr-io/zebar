export type MonitorSelection =
  | {
      type: 'all' | 'primary' | 'secondary';
    }
  | {
      type: 'index';
      match: number;
    }
  | {
      type: 'name';
      match: string;
    };
