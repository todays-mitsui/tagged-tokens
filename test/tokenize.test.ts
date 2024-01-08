import { assertEquals } from "https://deno.land/std@0.211.0/testing/asserts.ts";
import { app } from "../src/app.ts";
import { tokenize } from "../src/tokenize.ts";

Deno.test("tokenize", () => {
  const expr = app(
    app(app(app("A", "B"), "C"), app(app("D", "E"), "F")),
    app("G", "H"),
  );

  const [tokens, tags] = tokenize(expr);

  assertEquals(tokens, [
    "`",
    "`",
    "`",
    "`",
    "A",
    "B",
    "C",
    "`",
    "`",
    "D",
    "E",
    "F",
    "`",
    "G",
    "H",
  ]);

  assertEquals(tags, [
    [4],
    [3],
    [2],
    [1],
    [0],
    [1, 0],
    [2, 0],
    [3, 2],
    [3, 1],
    [3, 0],
    [3, 1, 0],
    [3, 2, 0],
    [4, 1],
    [4, 0],
    [4, 1, 0],
  ]);
});
