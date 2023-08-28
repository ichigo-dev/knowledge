# 『データ』ノート

（最終更新： 2023-08-28）


## 目次

1. [リテラル](#リテラル)
1. [変数](#変数)
1. [データ型](#データ型)
	1. [Undefined](#undefined)
	1. [Null](#null)
	1. [真偽値](#真偽値)
	1. [数値](#数値)
	1. [長整数](#長整数)
	1. [文字列](#文字列)


## リテラル

**リテラル**は、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)の中に直接記述できる記述できる値。[JavaScript](./javascript.md#javascript)において、[プリミティブ](../../../../programming/_/chapters/data_type.md#プリミティブ型)な値を表現するために使用できるリテラルは以下の通り。

| リテラル       | データ型         | 例                          |
| -------------- | ---------------- | --------------------------- |
| Nullリテラル   | Null型           | `null`                      |
| 真偽値リテラル | 真偽値型         | `true` 、 `false`           |
| 数値リテラル   | 数値型、長整数型 | `1` 、 `32.0` ...           |
| 文字列リテラル | 文字列型         | `'a'` 、 `"こんにちは"` ... |


## 変数

**変数**は、[オブジェクト](#オブジェクト)や[プリミティブ](../../../../programming/_/chapters/data_type.md#プリミティブ型)に名前をつけ、その名前で[参照](../../../../programming/_/chapters/variable.md#参照)を可能にする機能。[JavaScript](./javascript.md#javascript)において、変数の[宣言](../../../../programming/_/chapters/variable.md#宣言)には `let` 、 `const` または `var` キーワードを使用する。

```js
// 変数宣言（初期化）の例
let value1;
const value2 = "Hello";
var value3 = true;
```

これらのキーワードの違いは以下の通り。**ホイスティング**は、変数が[宣言](../../../../programming/_/chapters/variable.md#宣言)されていなかった際の挙動のこと。 `var` は古い[変数宣言](../../../../programming/_/chapters/variable.md#宣言)のキーワードで、再宣言が可能であることや、[エラー](../../../../programming/_/chapters/programming.md#エラー)が発生しないことから、 `let` や `const` を使用することが推奨されている。 `const` は[再代入](../../../../programming/_/chapters/variable.md#代入)ができないことから、[定数](../../../../programming/_/chapters/variable.md#定数)の[宣言](../../../../programming/_/chapters/variable.md#宣言)に使用される。

|                | `const`  | `let`    | `var`     |
| -------------- | :------: | :------: | :-------: |
| 再宣言         | x        | x        | o         |
| 再代入         | x        | o        | o         |
| スコープ       | ブロック | ブロック | 関数      |
| ホイスティング | エラー   | エラー   | undefined |


## データ型

### Undefined

**Undefined**(undefined)は、[JavaScript](./javascript.md#javascipt)において、未定義の値のための型。値としては `undefined` のみを取り（ `undefined` は[リテラル](#リテラル)ではなく[グローバル変数](../../../../programming/_/chapters/variable.md#グローバル変数)）、[宣言](../../../../programming/_/chapters/variable.md#宣言)だけを行って[初期化](../../../../programming/_/chapters/variable.md#初期化)されていない[変数](#変数)の値は `undefined` となる。未初期化変数を使用することはほとんどないため、基本的には不具合が発生している場合（またはそのチェックのための比較）にのみ目にする型。処理の結果が無効であることを表したい場合は、 `undefined` ではなく `null` を用いるのが適切。

### Null

**Null**(null)は、[JavaScript](./javascript.md#javascipt)において、無効な[オブジェクト](#オブジェクト)であることを表す型。値としては `null` [リテラル](#リテラル)のみを取る。 `undefined` を直接扱うことはあまりないが、 `null` は[変数](#変数)の内容が無効である場合に明示的に[代入](../../../../programming/_/chapters/variable.md#代入)することがある。ただし、 `null` は数多くの脆弱性や障害の原因にもなるため、扱いには注意が必要。

### 真偽値

**真偽値**(boolean)は、[JavaScript](./javascript.md#javascipt)において、真( `true` )と偽( `false` )という2つの値を表現するための型。 `true` と `false` はそれぞれ[リテラル](#リテラル)であり、[ソースコード](../../../../programming/_/chapters/programming.md#ソースコード)上に直接記述することができる。真偽値には[論理演算](../../../../programming/_/chapters/operation.md#論理演算)を適用することができ、条件の組み合わせの評価などが実現できる。

[JavaScript](./javascript.md#javascipt)では全ての型の値を真偽値のように扱うことができ、 `true` のような値を**truthy**、 `false` のような値を**falsy**と呼ぶ。 `falsy` な値は以下の通りで、これらに属さない値は `truthy` となる。

| 値          | 型        | 意味         |
| ----------- | --------- | ------------ |
| `false`     | boolean   | 偽値         |
| `0`         | number    | 数値の0      |
| `-0`        | number    | 数値の-0     |
| `NaN`       | number    | Not a Number |
| `0n`        | bigint    | 長整数の0    |
| `""`        | string    | 空文字列     |
| `null`      | null      | null         |
| `undefined` | undefined | undefined    |

### 数値

**数値**(number)は、[JavaScript](./javascript.md#javascipt)において、[浮動小数点数](../../../../basics/discrete_mathematics/_/chapters/numeric_representation.md#浮動小数点数)を表す型。数値には[算術演算](../../../../programming/_/chapters/operation.md#算術演算)を適用することができ、様々な計算が行える。数値に変換できない値は**NaN**(Not a Number)という[グローバル変数](../../../../programming/_/chapters/variable.md#グローバル変数)に置き換えられる。

### 長整数

**長整数**(bigint)は、[JavaScript](./javascript.md#javascipt)において、整数を表す型。[number](#数値)とは異なり、表現できる数が整数に限定される代わりに任意の大きさの値を[代入](../../../../programming/_/chapters/variable.md#代入)することができる。長整数の[リテラル](#リテラル)は数値の末尾に `n` をつけて表現される。長整数と[number](#数値)との[算術演算](../../../../programming/_/chapters/operation.md#算術演算)（例えば、 `3n + 20` など）は[エラー](../../../../programming/_/chapters/programming.md#エラー)となる。

### 文字列

**文字列**(string)は、[JavaScript](./javascript.md#javascipt)において、文字の連なりを表す型。文字列の[リテラル](#リテラル)はダブルクォーテーションもしくはシングルクォーテーションで文字の並びを囲むことで表現できる。[リテラル](#リテラル)の中で特別な意味を持つ文字（クォーテーションなど）を含めたい場合は、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)が正しく解析されることを妨げないよう、バックスラッシュをつけて[エスケープ](../../../../programming/_/chapters/data_type.md#文字列型)する必要がある。
