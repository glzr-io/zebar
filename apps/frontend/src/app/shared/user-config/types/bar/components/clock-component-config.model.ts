import { IsIn } from 'class-validator';

import { ComponentConfigBase } from '../component-config-base.model';

export class ClockComponentConfig extends ComponentConfigBase {
  @IsIn(['clock'])
  type: 'clock';
}
