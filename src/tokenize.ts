import { Expr, Tag, Token } from "./types.ts";

/**
 * 式をトークン列に分割する
 *
 * @param expr 分割前の式
 * @returns [トークン列, タグ列]
 */
export function tokenize(expr: Expr): [Array<Token>, Array<Tag>] {
  return _tokenize(expr, []);
}

function _tokenize(expr: Expr, tag: Tag): [Array<Token>, Array<Tag>] {
  const [left_var_name, right_exprs] = breakdown(expr);

  const tokens: Array<Token> = [];
  const tags: Array<Tag> = [];

  for (let i = right_exprs.length; i >= 1; i--) {
    tokens.push("`");
    tags.push([...tag, i]);
  }

  tokens.push(left_var_name);
  tags.push([...tag, 0]);

  for (let i = 1; i <= right_exprs.length; i++) {
    const right_expr = right_exprs[i - 1];
    const right_tag = [...tag, i];

    const [right_tokens, right_tags] = _tokenize(right_expr, right_tag);

    tokens.push(...right_tokens);
    tags.push(...right_tags);
  }

  return [tokens, tags];
}

/**
 * 式を最左項と右部分式の配列に分割する
 *
 * @param expr 分割前の式
 * @returns [最左項の変数名, 右部分式の配列]
 */
function breakdown(expr: Expr): [string, Expr[]] {
  let left_expr = expr;
  const right_exprs: Expr[] = [];

  while (
    typeof left_expr === "object" && "left" in left_expr && "right" in left_expr
  ) {
    right_exprs.unshift(left_expr.right);
    left_expr = left_expr.left;
  }

  return [left_expr, right_exprs];
}
