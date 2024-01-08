import { app } from "./app.ts";
import { range_subexpr } from "./range_subexpr.ts";
import { tokenize } from "./tokenize.ts";
import { Tag, Token } from "./types.ts";

// ```ABC`DEF`GH
const expr = app(
  app(app(app("A", "B"), "C"), app(app("D", "E"), "F")),
  app("G", "H"),
);

console.info(expr);
// => {
//   left: {
//     left: { left: { left: "A", right: "B" }, right: "C" },
//     right: { left: { left: "D", right: "E" }, right: "F" }
//   },
//   right: { left: "G", right: "H" }
// }

// ========================================================================== //

// トークン列に分解, 各トークンを一意に識別するタグ列を得る
const [tokens, tags]: [Array<Token>, Array<Tag>] = tokenize(expr);

// トークン列とタグ列の長さは等しい
console.assert(tokens.length === tags.length);

for (let i = 0; i < tokens.length; i++) {
  console.info(`${tokens[i]}: ${tags[i]}`);
  // =>
  //   `: 4
  //   `: 3
  //   `: 2
  //   `: 1
  //   A: 0
  //   B: 1,0
  //   C: 2,0
  //   `: 3,2
  //   `: 3,1
  //   D: 3,0
  //   E: 3,1,0
  //   F: 3,2,0
  //   `: 4,1
  //   G: 4,0
  //   H: 4,1,0
}

// ========================================================================== //

// タグから部分式の範囲を特定し, 部分式のトークン列を得る
const print_subexpr = (tag: Tag) => {
  const { start, end } = range_subexpr(tags, tag)!;
  console.info({ start, end });

  const subexpr = tokens.slice(start, end).join("");
  console.info(subexpr);
};

print_subexpr([2]);
// => { start: 2, end: 7 }
// => `ABC

print_subexpr([3]);
// => { start: 1, end: 12 }
// => ```ABC``DEF

print_subexpr([3, 1]);
// => { start: 8, end: 11 }
// => `DE

print_subexpr([3, 2]);
// => { start: 7, end: 12 }
// => ``DEF

// ========================================================================== //

// トークン - タグ - 部分式 の対応関係を表形式で表示する
console.table(tokens.map((token, i) => {
  const tag = tags[i];
  const { start, end } = range_subexpr(tags, tag)!;
  const subexpr = tokens.slice(start, end).join("");
  return { token, tag, start, end, subexpr};
}));
// =>
//   ┌───────┬───────┬─────────────┬───────┬─────┬───────────────────┐
//   │ (idx) │ token │ tag         │ start │ end │ subexpr           │
//   ├───────┼───────┼─────────────┼───────┼─────┼───────────────────┤
//   │     0 │ "`"   │ [ 4 ]       │     0 │  15 │ "````ABC``DEF`GH" │
//   │     1 │ "`"   │ [ 3 ]       │     1 │  12 │ "```ABC``DEF"     │
//   │     2 │ "`"   │ [ 2 ]       │     2 │   7 │ "``ABC"           │
//   │     3 │ "`"   │ [ 1 ]       │     3 │   6 │ "`AB"             │
//   │     4 │ "A"   │ [ 0 ]       │     4 │   5 │ "A"               │
//   │     5 │ "B"   │ [ 1, 0 ]    │     5 │   6 │ "B"               │
//   │     6 │ "C"   │ [ 2, 0 ]    │     6 │   7 │ "C"               │
//   │     7 │ "`"   │ [ 3, 2 ]    │     7 │  12 │ "``DEF"           │
//   │     8 │ "`"   │ [ 3, 1 ]    │     8 │  11 │ "`DE"             │
//   │     9 │ "D"   │ [ 3, 0 ]    │     9 │  10 │ "D"               │
//   │    10 │ "E"   │ [ 3, 1, 0 ] │    10 │  11 │ "E"               │
//   │    11 │ "F"   │ [ 3, 2, 0 ] │    11 │  12 │ "F"               │
//   │    12 │ "`"   │ [ 4, 1 ]    │    12 │  15 │ "`GH"             │
//   │    13 │ "G"   │ [ 4, 0 ]    │    13 │  14 │ "G"               │
//   │    14 │ "H"   │ [ 4, 1, 0 ] │    14 │  15 │ "H"               │
//   └───────┴───────┴─────────────┴───────┴─────┴───────────────────┘
