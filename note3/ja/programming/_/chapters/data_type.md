# 『データ型』

（最終更新： 2023-01-24）


## 目次

1. [型](#型)
1. [プリミティブ型](#プリミティブ型)
	1. [文字型](#文字型)
	1. [整数型](#整数型)
	1. [固定長小数点数型](#固定長小数点数型)
	1. [浮動小数点数型](#浮動小数点数型)
	1. [ブーリアン型](#ブーリアン型)

	1. [文字列型](#文字列型)


## 型

**型**は、プログラムで扱うデータを性質や属性によって分類してもの。静的型付け言語においては、プログラマがソースコード中に型を明示することによってコンパイル時にデータの型が決まる。静的型付け言語においては、実行時に自動的に型が判断される。


## プリミティブ型

**プリミティブ型**（**基本データ型**、**原始型**）は、プログラミング言語で標準的に用意されている最も基本的な型。プリミティブ型に属する型は、プログラミング言語においてそれ以上分解することができないシンプルな型として定義されており、プロセッサが高速に演算できるという特徴がある。

### 文字型

**文字型**( `char` )は、自然言語の1文字を格納するための型。1バイトで表現できるASCII文字や、マルチバイト文字で様々な文字種に対応しているJISコードやUnicodeなどのサポートが一般的。

ソースコード中で `'` で囲まれた部分は文字型のデータとして扱われる（ `'` と `"` を使い分けないプログラミング言語もある）。このようにクォーテーションで囲まれた、ソースコード中に直接記述された文字のことを**文字リテラル**という。

```c
// C言語

int main()
{
    // 文字リテラル
    char charcter = 'a';
    printf("character: %c\n", character);

    return 0;
}
```

### 整数型

**整数型**( `int, short, long' )は、整数値を格納するための型。1バイトが表現できる範囲は0~255(2の8乗)であるため、より大きい数値を扱えるように2バイトや4バイト、8バイトの整数型などを用意しているプログラミング言語もある。**符号付き整数**と**符号なし整数**をサポートしているプログラミング言語もあり、値が負にならないことがわかっている場合には符号なし整数を用いた方が使用できる数の範囲が増える。

### 固定長小数点数型

**固定長小数点数型**( `fixed` )は、小数を扱うための型で、どのビットを小数点とするかをあらかじめ決めておく。固定小数点数を扱うため、データの解析が容易である一方で、表せる数の範囲は浮動小数点数に比べて狭い。

### 浮動小数点数型

**浮動小数点数型**( `float, double` )は、小数を扱うための型で、浮動小数点方式の小数を扱う。単精度と倍精度で別の型として用意しているプログラミング言語もある。

### ブーリアン型

**ブーリアン型**( `bool, boolean` )は、真偽痴を扱うための型で、 `true` か `false` のどちらかの値が格納される。この型に対しては、論理積( `AND` )、論理和( `OR` )、排他的論理和( `XOR` )、同値、非同値、否定といった論理演算が可能。条件分岐において評価結果としても用いられる。


## 複合型

### 文字列型

**文字列型**( `string` )は、複数の文字からなる文字列を表現するための型。文字型には1つの文字しか格納できないため、2つ以上の文字からなる文字列を取り扱いたい場合はプログラミング言語ごとに用意されている文字列型を用いる。文字列型のオブジェクトを用意している場合もあれば、文字型の配列を文字列として扱う言語もある。

ソースコード中で `"` で囲まれた部分は文字列型のデータとして扱われる（ `'` と `"` を使い分けないプログラミング言語もある）。このようにクォーテーションで囲まれた、ソースコード中に直接記述された文字のことを**文字列リテラル**という。

文字列の終端を判別するための特別な文字として、**NULL文字**が用いられる（Nullとは異なる）。配列により文字列を表現するプログラミング言語の一部では、文字列の長さにNULL文字の分の1を加えたサイズのメモリ領域が必要となる。

```c
// C言語

int main()
{
    // 文字列リテラル
    char str[] = "All roads lead to Rome.";
    printf("str: %s\n", str);

    return 0;
}
```