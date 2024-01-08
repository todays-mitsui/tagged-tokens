export type Expr = { left: Expr; right: Expr } | string;

export type Token = string;

export type Tag = number[];

export interface Range {
  start: number;
  end: number;
}
