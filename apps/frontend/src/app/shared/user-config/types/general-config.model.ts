import { Expose, Type } from 'class-transformer';

export class GeneralConfig {
  @Expose({ name: 'position_x' })
  positionX: string;

  @Expose({ name: 'position_y' })
  positionY: string;

  width: string;

  height: string;

  @Type(() => Number)
  opacity: number;

  @Expose({ name: 'enable_devtools' })
  @Type(() => Boolean)
  enableDevtools: boolean;

  @Expose({ name: 'enable_default_styles' })
  @Type(() => Boolean)
  enableDefaultStyles: boolean;

  @Expose({ name: 'global_styles' })
  globalStyles: string;

  @Expose({ name: 'global_stylesheet_path' })
  globalStylesheetPath: string;
}
