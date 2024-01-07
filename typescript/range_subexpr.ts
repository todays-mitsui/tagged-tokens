import { Expr, Range, Tag, Token } from "./types.ts";

export function range_subexpr(tags: Array<Tag>, prefix: Tag): Range | null {
  const prefixes = gen_prefixes(prefix);

  const start = tags.findIndex((tag) =>
    prefixes.some((prefix) => is_prefix(prefix, tag))
  );

  if (start < 0) return null;

  const end = tags.findIndex(
    (tag, i) => i > start && !prefixes.some((prefix) => is_prefix(prefix, tag)),
  );

  return { start, end: end < 0 ? tags.length : end };
}

function gen_prefixes(prefix: Tag): Array<Tag> {
  const last_index = prefix.pop();

  if (last_index == null) throw new Error("prefix is empty");

  const prefixes: Array<Tag> = [];
  for (let i = last_index; i >= 0; i--) {
    prefixes.push([...prefix, i]);
  }

  return prefixes;
}

function is_prefix(prefix: Tag, tag: Tag): boolean {
  if (prefix.length > tag.length) return false;

  for (let i = 0; i < prefix.length; i++) {
    if (prefix[i] !== tag[i]) return false;
  }

  return true;
}
