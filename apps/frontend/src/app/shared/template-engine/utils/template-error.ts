export class TemplateError extends Error {
  public templateIndex: number;

  constructor(message: string, templateIndex: number) {
    super(message);
    this.templateIndex = templateIndex;
  }
}
