import { Range, Tag } from "./types.ts";

// タグ prefix から部分式の範囲を特定する
//
// @returns 部分式の範囲, end は範囲に含まれない (半開区間)
export function range_subexpr(tags: Array<Tag>, prefix: Tag): Range | null {
  const prefixes = gen_prefixes(prefix);

  const start = tags.findIndex((tag) =>
    prefixes.some((prefix) => is_prefix(prefix, tag))
  );

  if (start < 0) return null;

  let end = tags.findIndex(
    (tag, i) => i > start && !prefixes.some((prefix) => is_prefix(prefix, tag)),
  );
  end = end < 0 ? tags.length : end;

  return { start, end };
}

/**
 * 1つのタグを元に, 末尾のインデックスをデクリメントしたタグの配列を生成する
 *
 * @example
 *   gen_prefixes([1, 4, 5])
 *   // => [
 *     [1, 4, 5],
 *     [1, 4, 4],
 *     [1, 4, 3],
 *     [1, 4, 2],
 *     [1, 4, 1],
 *     [1, 4, 0],
 *   ]
 */
function gen_prefixes(prefix: Tag): Array<Tag> {
  prefix = [...prefix];
  const last_index = prefix.pop();

  if (last_index == null) throw new Error("prefix is empty");

  const prefixes: Array<Tag> = [];
  for (let i = last_index; i >= 0; i--) {
    prefixes.push([...prefix, i]);
  }

  return prefixes;
}

/**
 * prefix が tag の先頭部分に一致するかどうかを判定する
 */
function is_prefix(prefix: Tag, tag: Tag): boolean {
  if (prefix.length > tag.length) return false;

  for (let i = 0; i < prefix.length; i++) {
    if (prefix[i] !== tag[i]) return false;
  }

  return true;
}
