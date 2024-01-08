# tagged-tokens

see: [ラムダ式に含まれる全ての部分式に一意かつ列挙可能なタグを付与する方法 - 無駄と文化]()

上記の記事で定義したラムダ式の列形式へのタグ付けルールをプログラムに落とし込んだ参考実装です。

- [src/tokenize.ts](https://github.com/todays-mitsui/tagged-tokens/blob/main/src/tokenize.ts)
- [src/range_subexpr.ts](https://github.com/todays-mitsui/tagged-tokens/blob/main/src/range_subexpr.ts)
- [main.ts](https://github.com/todays-mitsui/tagged-tokens/blob/main/main.ts)

あたりを読むと主なロジックと使い方がわかるはずです。

## 実行

実行のために [Deno](https://deno.com/) をセットアップしておく必要があります。

```sh
# 実行
$ deno run main.ts

# テスト
$ deno test
```

## 実行イメージ

![deno run main.ts の実行イメージ](https://github.com/todays-mitsui/tagged-tokens/assets/3040456/977f4329-0520-4e19-a485-7fba7ce32f31)
