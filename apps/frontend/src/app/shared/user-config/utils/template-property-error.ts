export class TemplatePropertyError extends Error {
  public path: string;
  public template: string;
  public templateIndex: number;

  constructor(
    message: string,
    path: string,
    template: string,
    templateIndex: number,
  ) {
    super(message);
    this.path = path;
    this.template = template;
    this.templateIndex = templateIndex;
  }
}
