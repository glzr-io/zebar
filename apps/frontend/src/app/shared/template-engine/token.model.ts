export interface Token {
  type: 'text' | 'tag start' | 'tag end' | 'output';
  content: string;
  startIndex: number;
  endIndex: number; // TODO: Maybe not?
}

export interface TagStartToken extends Token {
  tagType: 'if' | 'else' | 'else if' | 'for' | 'switch' | 'case';
  expression?: string;
}

export interface OutputToken extends Token {
  expression: string;
}
