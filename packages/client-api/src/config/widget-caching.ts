export type WidgetCaching = {
  defaultDuration: number;
  rules: WidgetCachingRule[];
};

export type WidgetCachingRule = {
  urlRegex: string;
  duration: number;
};
