# 『変数』ノート

（最終更新： 2023-04-03）


## 目次

1. [変数](#変数)
	1. [グローバル変数](#グローバル変数)
	1. [ローカル変数](#ローカル変数)
1. [変数の操作](#変数の操作)
	1. [宣言](#宣言)
	1. [代入](#代入)
	1. [初期化](#初期化)
	1. [参照](#参照)
1. [定数](#定数)


## 変数

**変数**は、[プログラム](./programming.md#プログラム)中で用いるデータに固有の名前をつけたもの。物理的にはデータは[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)に格納されており、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)上のデータには[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)を使ってアクセスすることができる。しかし、[メモリアドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)はただの数値の羅列であり、人間にはわかりずらいため、[ソースコード](./programming.md#ソースコード)中では格納しているデータの性質や役割を表す[識別子](./programming.md#識別子)をつけた変数を用いる。

### グローバル変数

**グローバル変数**は、[グローバルスコープ](./control_flow.md#グローバル)に宣言された[変数](#変数)。

### ローカル変数

**ローカル変数**（**自動変数**、**局所変数**）は、[ローカルスコープ](./control_flow.md#ローカル)に宣言された[変数](#変数)。

同じ[識別子](./programming.md#識別子)の[変数](#変数)を複数回宣言できない[プログラミング言語](./programming.md#プログラミング言語)においても、[スコープ](./control_flow.md#スコープ)が異なれば同じ[識別子](./programming.md#識別子)を用いることができる。


## 変数の操作

```c
// C言語

int main()
{
    int x;               // 宣言
    int y = 5;           // 初期化
    x = 3;               // 代入
    printf("%d\n", x);   // 参照

    return 0;
}
```

```php
<?php

// PHP

$x = 5;     // 宣言、初期化、代入
echo($x);   // 参照

?>
```

```javascript
// JavaScript

let x;            // 宣言
let y = 5;        // 初期化
x = 3;            // 代入
console.log(x);   // 参照
```

### 宣言

**宣言**は、[プログラム](./programming.md#プログラム)中で用いる[変数](#変数)の名前を明示する操作。[静的型付け言語](./programming.md#静的型付け言語)の多くは、宣言時に[変数](#変数)の[型](./data_type.md#型)も指定する必要がある。

[プログラミング言語](./programming.md#プログラミング言語)によっては、同じ[変数](#変数)名を複数回宣言しようとすると[エラー](./programming.md#エラー)となる。一方で、新しい[変数](#変数)で前の[変数](#変数)を上書きする（**シャドーイング**）仕様の[プログラミング言語](./programming.md#プログラミング言語)もある。

### 代入

**代入**は、[宣言](#宣言)した[変数](#変数)に対してデータを関連付ける操作。既にデータが紐づけられている[変数](#変数)に対して別のデータを紐づけることもでき、これを**再代入**という。代入のための[演算子](./operation.md#代入演算子)としては `=` が用いられる場合が多い（数学的な意味では `=` は「等価」であるが、多くの[プログラミング言語](./programming.md#プログラミング言語)では等価は `==` で表される）。

[変数](#変数)名に対してデータを関連付けるように、何かに対して別の何かを紐づけることを、[プログラミング](./programming.md#プログラミング)では**束縛**（**バインド**）という。

### 初期化

**初期化**は、[変数](#変数)の[宣言](#宣言)と同時に[代入](#代入)を行う操作。[定数](#定数)のような、後から変更することを許可しない[変数](#変数)を[宣言](#宣言)したい場合は、初期化が必須となる。

### 参照

**参照**は、[変数](#変数)名に紐づくデータを取り出す操作。データが[代入](#代入)されていない[変数](#変数)（**未初期化状態**の[変数](#変数)）を参照することは不正な操作となり、[プログラミング言語](./programming.md#プログラミング言語)によっては[未定義動作](./programming.md#未定義動作)となる。これを防ぐため、[宣言](#宣言)時や実行時に自動的に初期値（[整数型](./data_type.md#整数型)であれば `0` 、[文字列型](./data_type.md#文字列型)であれば空文字など）を割り当てる[プログラミング言語](./programming.md#プログラミング言語)もある。


## 定数

**定数**は、[初期化](#初期化)した後に別のデータを[再代入](#代入)できない[変数](#変数)。[変数](#変数)は後からデータを紐づけることができるが、定数はできないので、必ず[初期化](#初期化)する必要がある。定数の[初期化](#初期化)には `const` キーワードが用いられる場合が多い。[プログラム](./programming.md#プログラム)中で変更されてはいけないデータや、数学的・物理的な定数に用いられるのが一般的。

```c
// C言語

// 変数の初期化時にconstキーワードをつけることで定数にすることができる
const int max_num = 1000;
const float pi = 3.14159;
```

```javascript
// JavaScript

// letの代わりにconstを用いることで定数を初期化する
const max_num = 1000;
const pi = 3.14159;
```

定数は紐づくデータは変わらないが、[ポインタ](./data_type.md#ポインタ型)経由などでデータ自体を書き換えることができてしまうため、必ずしも[参照](#参照)した中身が一致するとは限らない点に注意が必要である。

また `const` 定数と似たような使われ方をする機能として、 `define` を利用できる[プログラミング言語](./programming.md#プログラミング言語)もある。これは一般的には、[ソースコード](./programming.md#ソースコード)中の[文字列](./data_type.md#文字列型)を[コンパイル](./programming.md#コンパイル)時に別の値に置き替える機能のことを指す。

```c
// C言語

// defineマクロを用いて値を定義する
#define MAX_NUM 1000
#define PI 3.14159
```

```php
<?php

// PHP

// define関数を用いて値を定義する
define("MAX_NUM", 1000);
define("PI", 3.14159);

?>
```
