import { TokenType } from './token-type.model';

export interface Token {
  type: TokenType;
  index: number;
}
