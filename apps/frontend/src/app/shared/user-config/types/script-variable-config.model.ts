import { IsIn } from 'class-validator';
import { Expose, Type } from 'class-transformer';

export class ScriptVariableConfig {
  @IsIn(['script'])
  source: 'script';

  @Expose({ name: 'script_path' })
  scriptPath: string;

  @Expose({ name: 'script_args' })
  scriptArgs: string;

  @Expose({ name: 'refresh_interval_ms' })
  @Type(() => Number)
  refreshIntervalMs: number;
}
