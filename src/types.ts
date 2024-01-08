/// ラムダ式の木形式
export type Expr = { left: Expr; right: Expr } | string;

/// トークン, ラムダ式を列形式で表現したときの最小単位
export type Token = string;

/// タグ, ラムダ式の中でのトークンの位置を一意に表す
export type Tag = number[];

/// 範囲, 部分式の開始位置と終了位置を表す
/// end は範囲に含まれない (半開区間)
export interface Range {
  start: number;
  end: number;
}
