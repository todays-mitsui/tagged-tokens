import { assertEquals } from "https://deno.land/std@0.211.0/testing/asserts.ts";
import { range_subexpr } from "../src/range_subexpr.ts";

Deno.test("range_subexpr", () => {
  const tags = [
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
  ];

  assertEquals(range_subexpr(tags, [2]), { start: 2, end: 7 });
  assertEquals(range_subexpr(tags, [3, 1]), { start: 8, end: 11 });
});
