import { TokenType } from './token-type.model';

export interface Token {
  type: TokenType;
  substring: string;
  startIndex: number;
  endIndex: number;
}
