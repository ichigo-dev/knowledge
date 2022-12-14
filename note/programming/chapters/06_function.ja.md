# 関数


## 目次

1. [プログラミングにおける関数](#プログラミングにおける関数)
1. [引数](#引数)
1. [戻り値](#戻り値)
1. [関数定義](#関数定義)
1. [関数呼び出し](#関数呼び出し)
1. [標準関数](#標準関数)
1. [無名関数](#無名関数)
1. [コールバック関数](#コールバック関数)


## プログラミングにおける関数

**関数**（**サブルーチン**）は処理[ブロック](./02_variable.ja.md#ブロックとスコープ)に[識別子](./01_basic_knowledge_of_programming.ja.md#識別子)をつけて再利用可能にしたもので、[プログラム](./01_basic_knowledge_of_programming.ja.md#プログラミングの概要)中で[関数呼び出し](#関数呼び出し)を行うことで実行できる。


## 引数

[関数](#プログラミングにおける関数)には、**引数**として実行時に動的に情報を渡すことができる。[関数定義](#関数定義)では引数として受け取る情報に[識別子](./01_basic_knowledge_of_programming.ja.md#識別子)をつけた**仮引数**を用いて処理を記述する。[関数呼び出し](#関数呼び出し)の際に実際に処理したい情報を**実引数**として渡すことで、仮引数部分がデータに置き換えられて処理が実行される。[関数](#プログラミングにおける関数)は引数を複数個受け取ることも可能。仮引数にデフォルト値を指定しておくことで、[関数呼び出し](#関数呼び出し)側で引数を渡さなかった場合にデフォルト値で置き換えて処理を実行することができる。このような引数を**オプション引数**（**任意引数**）という。また、[関数定義](#関数定義)の際に引数の数を固定せず、呼び出し側で任意の数の引数を渡して処理を実行するような**可変長引数**をサポートしている言語もある（文字列のフォーマットなどによく利用される）。


## 戻り値

[関数](#プログラミングにおける関数)は処理を行った結果として**戻り値**を返却する。通常戻り値は1つしか返すことができないため、複数の値を返却したいような場合には、値を[オブジェクト](./03_data_type.ja.md#オブジェクト型)として返却したり、[引数](#引数)として受け取った[ポインタ](./03_data_type.ja.md#ポインタ型)に直接値を書き込んだりする。

[関数](#プログラミングにおける関数)内部で用意された[ローカル変数（局所変数）](./02_variable.ja.md#グローバル変数とローカル変数)は、[関数](#プログラミングにおける関数)の処理[ブロック](./02_variable.ja.md#ブロックとスコープ)内部でのみ有効な[変数](./02_variable.ja.md#プログラミングにおける変数)であるため、戻り値として使用する場合には注意が必要となる[プログラミング言語](./01_basic_knowledge_of_programming.ja.md#プログラミングの概要)もある。戻り値として返却したい値が[プリミティブ型](./02_variable.ja.md#プリミティブ型)の場合にはデータが[ディープコピー](./03_data_type.ja.md#ディープコピー)されるため問題ないが、[シャロ―コピー](./03_data_type.ja.md#シャローコピー)が発生するような値を返却したい場合には、[関数](#プログラミングにおける関数)の処理が終わった時点で[ローカル変数](./02_variable.ja.md#グローバル変数とローカル変数)が破棄されてしまうため**ダングリングポインタ**（無効なポインタ）が発生する可能性がある。そのため、明示的に確保した[メモリ](./01_basic_knowledge_of_programming.ja.md#記憶装置)に確保したデータの[ポインタ](./03_data_type.ja.md#ポインタ型)を返却したり、[引数](#引数)として受け取った[ポインタ](./03_data_type.ja.md#ポインタ型)に直接データを格納したりといった工夫が必要となる。

[関数](#プログラミングにおける関数)は `return` 文を用いて戻り値を呼び出し元に返却することができる。


## 関数定義

**関数定義**では[関数](#プログラミングにおける関数)の[識別子](./01_basic_knowledge_of_programming.ja.md#識別子)、処理内容、[仮引数](#引数)リスト、[戻り値](#戻り値)を記述する。

```c
// C言語

// 関数定義
int add( int a_, int b_ )
{
    int result = a_ + b_;
    return result;
}
```

```php
<?php

// PHP

// 関数定義
function add( $a_, $b_ )
{
    $result = $a_ + $b_;
    return $result;
}

?>
```

```javascript
// JavaScript

function add( a_, b_ )
{
    let result = a_ + b_;
    return result;
}
```


## 関数呼び出し

**関数呼び出し**では[関数](#プログラミングにおける関数)の[識別子](./01_basic_knowledge_of_programming.ja.md#識別子)と[仮引数](#引数)リストに対応した[実引数](#引数)を記述する。

```c
// C言語

// 関数プロトタイプ宣言
int add( int, int );

// メイン関数
int main()
{
    int a = 5;
    int b = 10;

    // 関数呼び出し
    int result = add(a, b);
    printf("Result: %d\n", result);

    return 0;
}

// 関数定義
int add( int a_, int b_ )
{
    int result = a_ + b_;
    return result;
}
```

```php
<?php

// PHP

// 関数定義
function add( $a_, $b_ )
{
    $result = $a_ + $b_;
    return $result;
}

$a = 5;
$b = 10;

// 関数呼び出し
$result = add($a, $b);
echo("Result: " . $result);

?>
```

```javascript
// JavaScript

// 関数定義
function add( a_, b_ )
{
    let result = a_ + b_;
    return result;
}

let a = 5;
let b = 10;

// 関数呼び出し
let result = add(a, b);
console.log("Result: " + result);
```


## 標準関数

**標準関数**は各[プログラミング言語](./01_basic_knowledge_of_programming.ja.md#プログラミングの概要)の仕様としてあらかじめ用意されている[関数](#プログラミングにおける関数)。使い方は通常の[関数](#プログラミングにおける関数)と同様で、仕様として定義されているため[プログラマ](./01_basic_knowledge_of_programming.ja.md#プログラミングの概要)が定義・実装する必要はない。用意されている[関数](#プログラミングにおける関数)の詳細については、各[プログラミング言語](./01_basic_knowledge_of_programming.ja.md#プログラミングの概要)のドキュメントなどを参照する必要がある。


## 無名関数

**無名関数**（**クロージャ**）は[識別子](./01_basic_knowledge_of_programming.ja.md#識別子)を持たない[関数](#プログラミングにおける関数)。**ラムダ式**などによりつくることができ、[変数](./02_variable.ja.md#プログラミングにおける変数)に代入したり[コールバック関数](#コールバック変数)として利用される。


## コールバック関数

**コールバック関数**は実装のテクニックのひとつで、[関数](#プログラミングにおける関数)に対して[引数](#引数)で渡される[関数](#プログラミングにおける関数)のことである。[関数](#プログラミングにおける関数)を呼び出した側が[引数](#引数)として[関数](#プログラミングにおける関数)を渡し、呼び出された側は渡された[関数](#プログラミングにおける関数)を呼び出し返すという仕組み。通常、[関数](#プログラミングにおける関数)を呼び出す側は[関数](#プログラミングにおける関数)が行う処理に対して介入できないが、コールバック関数を利用した場合は[関数](#プログラミングにおける関数)の外部から[関数](#プログラミングにおける関数)の動作を一部制御できる。
