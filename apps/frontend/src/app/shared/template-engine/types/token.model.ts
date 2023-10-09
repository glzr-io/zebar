import { TokenType } from './token-type.model';

export interface Token {
  type: TokenType;
  content: string;
  startIndex: number;
  endIndex: number;
}
