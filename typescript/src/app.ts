import { Expr } from "./types.ts";

export function app(left: Expr, right: Expr): Expr {
  return { left, right };
}
