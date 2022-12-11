# データ型


## 目次

1. [型](#型)
1. [プリミティブ型](#プリミティブ型)
	1. [文字型](#文字型)
	1. [整数型](#整数型)
	1. [固定小数点数型](#固定小数点数型)
	1. [浮動小数点数型](#浮動小数点数型)
	1. [ブーリアン型](#ブーリアン型)
	1. [ポインタ型](#ポインタ型)
	1. [参照型](#参照型)
1. [Null](#null)
1. [コンテナ型](#コンテナ型)
	1. [配列](#配列)
	1. [リスト](#リスト)
	1. [スタック](#スタック)
	1. [キュー](#キュー)
	1. [ハッシュテーブル](#ハッシュテーブル)
	1. [連想配列](#連想配列)
	1. [セット](#セット)
	1. [木](#木)
	1. [グラフ](#グラフ)
1. [オブジェクト型](#オブジェクト型)
1. [データの複製](#データの複製)
	1. [ディープコピー](#ディープコピー)
	1. [シャローコピー](#シャローコピー)


## 型

[コンピュータ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミングの概要)が扱うデータを性質や属性によって分類したものを**データ型**という。[プログラム](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミングの概要)中で使用するデータについても、それがどの型のデータであるかがはっきりしていないといけない。[静的型付け言語](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#静的片付け言語)においては[プログラマ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミングの概要)が[ソースコード](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミングの概要)中に型を明示することによって[コンパイル](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#高水準言語)時にデータの型が決まり、[動的型付け言語](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#動的型付け言語)においては実行時に型を自動的に判定する。

## プリミティブ型

多くの[プログラミング言語](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミングの概要)では**プリミティブ型**（**基本データ型**、**原始型**）が標準的に用意されている。プリミティブ型に属する[型](#型)は[プログラミング言語](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミングの概要)においてそれ以上分解することができないシンプルな[型](#型)であり、基本的に[プロセッサ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#処理装置)が最も高速に[演算](/note/programming/chapters/04_operation.ja.md#プログラミングにおける演算)できるデータとなる。

### 文字型

**文字型**（ `char` ）は自然言語の1文字を格納するための[型](#型)。1[バイト](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)で表現できる**ASCII文字**だけでなく、文字種によって**マルチバイト文字**に対応しているものもある。文字セットとしては**UTF-8**や**Unicode**などのサポートが一般的。

文字型には1つの文字しか格納できないため、2つ以上の文字からなる文字列を扱いたい場合は[プログラミング言語](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミング言語の概要)ごとに定義される**文字列型**を用いる必要がある。文字列型の[オブジェクト](#オブジェクト型)を用意している言語もあれば、文字型の[配列](#配列)を文字列として扱う言語もある。文字列の終端を判別するための特別な文字として、**NULL文字**が用いられる（[Null](#null)とは区別する）。[配列](#配列)により文字列型を表現する[プログラミング言語](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミング言語の概要)においては、文字列の長さにNULL文字の分の1を加えたサイズの[メモリ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#記憶装置)領域が必要となる。

[ソースコード](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミング言語の概要)中でシングルクォート（ `'` ）で囲まれた部分は文字型として扱われ、ダブルクォート（ `"` ）で囲まれた部分は文字列型として扱われる（これらのクォーテーションの意味を使い分けない[プログラミング言語](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミング言語の概要)もある）。このようにクォーテーションで囲まれた、[ソースコード](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミング言語の概要)中に直接記述された文字・文字列のことをそれぞれ**文字リテラル**、**文字列リテラル**という。

```c言語
// C言語

int main()
{
    // 文字リテラル
    char character = 'a';
    printf("character: %c\n", character);

    // 文字列リテラル
    char str[] = "All roads lead to Rome.";
    printf("str: %s\n", str);

    return 0;
}
```

### 整数型

**整数型**（ `int`, `short`, `long` ）は整数値を格納するための[型](#型)。1[バイト](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)が表現できる範囲は0~255(2の8乗)であるため、より大きい数値を扱えるように2[バイト](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)や4[バイト](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)、8[バイト](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)の整数型などを用意している[プログラミング言語](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミング言語の概要)もある。**符号付き整数**と**符号なし整数**をサポートしているものもあり、値が負にならないことが分かっている場合には符号なし整数を用いた方が使用できる数の範囲が増える。

### 固定小数点数型

**固定小数点数型**（ `fixed` ）は小数を扱うための[型](#型)で、どの[ビット](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)を小数点とするかをあらかじめ決めておく。データの解析は容易であるが、表現できる数値の範囲が限定されてしまうため、[浮動小数点数](#浮動小数点型)を用いる場合が多い。

### 浮動小数点数型

**浮動小数点数型**（ `float`, `double` ）は小数を扱うための[型](#型)で、[IEEE 754](https://ja.wikipedia.org/wiki/IEEE_754)によって規定された浮動小数点方式と呼ばれる方式を用いる。

[10進数](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#2進数とプログラミング)において $12.345$ という数を浮動小数点数で表現すると、 $+0.012345 \times 10^3$ となる。この時 $+$ にあたる部分を符号部、 $^3$ にあたる部分を指数部、 $0.012345$ にあたる部分を仮数部という。また浮動小数点方式では、数値の有効範囲数を広げるために**正規化**（指数部を調整することで仮数部の最上位桁を0以外にする作業）を行う。前の例であれば、 $+0.12345 \times 10^2$ が正規化された浮動小数点数の表現となる。コンピュータが扱う数値は[2進数](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#2進数とプログラミング)であるため、[基数](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#2進数とプログラミング)は2となる。

- **符号部**には、その数が正(0)であるか負(1)であるかを格納する
- **指数部**には、基数に対する指数部分を[2進数](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#2進数とプログラミング)表記したものを格納する（指数部が負の数の場合は2の補数表現を使用）
- **仮数部**には、正規化した小数点以下の値を格納する

浮動小数点数には単精度と倍精度があり、**単精度**では32[ビット](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)（符号部1[ビット](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)、指数部8[ビット](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)、仮数部23[ビット](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)）、**倍精度**では64[ビット](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)（符号部1[ビット](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)、指数部11[ビット](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)、仮数部52[ビット](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#データの単位)）で表現する。

[固定小数点数](#固定小数点型)では限られた範囲の中であれば[誤差](/note/programming/chapters/04_operation.ja.md#誤差)なく計算することができるが、浮動小数点数では近似計算を前提としているため[誤差](/note/programming/chapters/04_operation.ja.md#誤差)が発生する。[コンピュータ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミングの概要)では無限小数や無理数の正確な値を扱うことができないため、有限の桁数で近似されている。[10進数](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#2進数とプログラミング)で表したときに有限小数であっても、[2進数](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#2進数とプログラミング)で表すと無限小数となる場合もあるので注意が必要（ $0.01$ など）。

### ブーリアン型

**ブーリアン型**（ `bool`, `boolean` ）は真偽値を扱うための[型](#型)で、 `true` か `false` のどちらかの値が格納される。この[型](#型)には対しては、[論理積](/note/programming/chapters/04_operation.ja.md#論理演算子)（ `AND` ）、[論理和](/note/programming/chapters/04_operation.ja.md#論理演算子)（ `OR` ）、排他的論理和（ `XOR` ）、[同値](/note/programming/chapters/04_operation.ja.md#比較演算子)、[非同値](/note/programming/chapters/04_operation.ja.md#比較演算子)、[否定](/note/programming/chapters/04_operation.ja.md#論理演算子)といった[論理演算](/note/programming/chapters/04_operation.ja.md#論理演算子)が可能。[条件分岐](/note/programming/chapters/05_control_flow.ja.md#条件分岐)において、[条件式](/note/programming/chapters/05_control_flow.ja.md#条件分岐)の評価結果として用いられる。

### ポインタ型

**ポインタ型**は[変数](/note/programming/chapters/02_variable.ja.md#プログラミングにおける変数)や[関数](/note/programming/chapters/06_function.ja.md#プログラミングにおける関数)が置かれた[メモリ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#記憶装置)上の[アドレス](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#メモリとアドレス)を格納するための特殊な[型](#型)。[プログラム](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミングの概要)中で利用される[変数](/note/programming/chapters/02_variable.ja.md#プログラミングにおける変数)や[関数](/note/programming/chapters/06_function.ja.md#プログラミングにおける関数)などのリソースは[メモリ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#記憶装置)上で固有の[アドレス](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#メモリとアドレス)を持っており、[アドレス](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#メモリとアドレス)を知っていればそのデータにアクセスすることができる。ポインタが指す先のデータを直接読み書きすることができたり、[メモリアドレス](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#メモリとアドレス)を足し引きして別の[アドレス](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#メモリとアドレス)にアクセスできるというメリットがあるが、一方で予期しない[アドレス](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#メモリとアドレス)のデータを変更してしまう場合があるなど深刻なバグにつながるという欠点もある。

![ポインタ](/note/programming/images/pointer.ja.jpg)

### 参照型

**参照型**は[ポインタ型](#ポインタ型)と同様、[変数](/note/programming/chapters/02_variable.ja.md#プログラミングにおける変数)や[関数](/note/programming/chapters/06_function.ja.md#プログラミングにおける関数)が置かれた[メモリ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#記憶装置)上の[アドレス](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#メモリとアドレス)を指す。[ポインタ型](#ポインタ型)と異なり、参照先に書き込むことはできない読み込み専用のデータ。また、参照先にデータが存在しない（[未初期化状態](/note/programming/chapters/02_variable.ja.md#変数の操作)）であることが禁止されている。これらの特性から、[ポインタ](#ポインタ型)に比べて安全に利用できるという利点がある。


## Null

**Null**は何もないことを表し、言語によって実装や扱いが異なる（[型](#型)として定義されている言語もあれば、特別な値として扱う言語もある）。空文字や0とは区別され、[変数](/note/programming/chapters/02_variable.ja.md#プログラミングにおける変数)に値が代入されていない状態などを表す。


## コンテナ型

**コンテナ型**（**コレクション型**）は同じ[型](#型)のデータを複数まとめるための抽象データ型。それぞれに特性や高速に動作する操作が異なるため、用途に応じて適切に使い分けることで[プログラム](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミングの概要)の質を向上させることができる。

また、コンテナ型が持つ各要素に対する[繰り返し処理](/note/programming/chapters/04_operation.ja.md#反復)の抽象化構造を**イテレータ**という。イテレータは[繰り返し処理](/note/programming/chapters/04_operation.ja.md#反復)の[for文](/note/programming/chapters/04_operation.ja.md#for文)などにおいて利用される。

各[プログラミング言語](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラミングの概要)ごとに用意されている[型](#型)や内部的なデータ構造が異なる場合があるので注意。

### 配列

**配列**は同じ[型](#型)のデータの集合を格納するための[データ型](#型)で、要素が[メモリ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#記憶装置)上で隣り合うように並べらえれる。要素には**インデックス**（添え字）を使ってアクセスすることができる。インデックスは配列の最初の要素を0とした整数値となっている。

```c
// C言語

char str[] = "Hello, world";
printf("%c\n", str[4]);    // 'o'が出力される

str[5] = '!';
printf("%s\n", str);       // "Hello! world"が出力される
```

配列には、[コンパイル](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#高水準言語)時に要素数が静的に決まる**固定長配列**と、実行時に要素数が動的に決まり要素の追加や削除ができる**可変長配列**がある。通常、固定長配列は[メモリ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#記憶装置)上の[スタック領域](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#メモリ上の領域)に格納され、可変長配列は[メモリ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#記憶装置)上の[ヒープ領域](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#メモリ上の領域)に格納される。また、[ヒープ領域](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#メモリ上の領域)に確保したサイズが足りなくなった場合には**メモリの再確保**（**アロケート**）が行われる。

可変長配列の末尾に対するデータの追加・削除にかかる[計算量](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラムの計算量)は $O(1)$ で高速であるが、任意のインデックスに対するデータの挿入・削除にかかる[計算量](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラムの計算量)は $O(N)$ （挿入するインデックスから後ろのデータをすべて移動する必要があるため）と[リスト](#リスト)に比べて遅い。一方で、任意のインデックスの要素へのアクセスにかかる[計算量](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラムの計算量)は常に $O(1)$ と高速である。探索にかかる[計算量](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラムの計算量)は基本的には $O(N)$ であるが、配列がソート済みであれば二分探索を用いて $O(\log{N})$ に抑えることもできる。

中間位置へのデータ挿入が頻繁に行われたり、要素数が非常に大きくなるようなデータに対しては[リスト](#リスト)を用いた方が良い場合もある。

### リスト

**リスト**（**連結リスト**、**リンクリスト**）は各ノードが任意のデータと他のノードへの[ポインタ](#ポインタ型)を持つようなデータ構造。

**単方向リスト**は各ノードが次のノードへの[ポインタ](#ポインタ型)を持つ。

**双方リスト**は各ノードが次のノードと前のノードへの[ポインタ](#ポインタ型)を持つ。

**循環リスト**は末尾のノードが先頭のノードへの[ポインタ](#ポインタ型)を持つ。

リストの任意の場所へのデータの挿入・削除にかかる[計算量](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラムの計算量)は $O(1)$ で高速である。一方で、要素へのアクセスにかかる[計算量](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラムの計算量)は $O(N)$ であり[配列](#配列)に比べて遅い。リストの要素へのアクセスするには、先頭の要素から順番に[ポインタ](#ポインタ型)をたどっていくしかない（[メモリ](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#記憶装置)上での配置が隣り合っているとは限らないため）。

### スタック

**スタック**はデータを**FILO**（First In Last Out、先入れ後出し）の構造で保持する。スタックに対してデータを追加することを**プッシュ**、スタックからデータを取り出すことを**ポップ**といい、これらを基本操作とするデータ型である。

### キュー

**キュー**はデータを**FIFO**（First In First Out、先入れ先出し）の構造で保持する。キューに対してデータを追加することを**エンキュー**、キューからデータを取り出すことを**デキュー**といい、これらを基本操作とするデータ型である。

キューに追加する要素に優先度を付け、優先度にもとづいてデータをソートし、取り出し順を変更するようなキューを**優先度付きキュー**という。

先頭、末尾の両方からデータの追加・取り出しを行うことができるようなキューを**両端キュー**（デック）という。

### ハッシュテーブル

**ハッシュテーブル**は、キーと値の組からなる**エントリ**を複数格納するデータ型。キーを元に生成された**ハッシュ値**をインデックスとして値を管理することで、検索や要素の追加を $O(1)$ の[計算量](/note/programming/chapters/01_basic_knowledge_of_programming.ja.md#プログラムの計算量)で実現することができる。ハッシュ値を算出するための[関数](/note/programming/chapters/06_function.ja.md#プログラミングにおける関数)を**ハッシュ関数**という。

複数の異なるキーから生成されるハッシュ値が同じものになってしまうことを**衝突**という。**連鎖法**では、同じハッシュ値で解決されるデータを[リンクリスト](#リスト)で管理する。**開番地法**では、衝突が発生したときに別の[関数](/note/programming/chapters/06_function.ja.md#プログラミングにおける関数)を用いて次の候補値を算出する。

### 連想配列

**連想配列**（**連想リスト**、**連想コンテナ**、**辞書**）はインデックスとして文字列などの整数値以外のデータ型を用いることができるような配列。連想配列の実装にはハッシュテーブルや**平衡二分探索木**などが用いられる。

### セット

**セット**（**集合**）は順序のないデータの集まりであり、重複するデータが存在しないことを保証する。

### 木

**木**はノードとノード間を結ぶエッジからなる木構造のデータ。1つのノードが複数の子ノードを持ち、親ノードを持たないトップクラスのノードを**ルートノード**（根ノード）、子ノードを持つノードを**ブランチノード**（枝ノード）、子ノードを持たないノードを**リーフノード**（葉ノード）という。

各ノードが持つ子ノードの数が2つに制限された木を**二分木**、葉の深さがなるべく揃うように構築された木を**平衡木**という。

### グラフ

**グラフ**は木と同様にノードとエッジからなるデータ構造であるが、木とは異なりノード同士の親子関係を持たない。


## オブジェクト型

**オブジェクト型**はオブジェクト指向のプログラミング言語において、クラスから生成されたオブジェクトを指す型。


## データの複製

プログラム中で用いるデータを複製する方法には、ディープコピーとシャローコピーがある。

### ディープコピー

**ディープコピー**ではメモリ上のデータの実体を別のメモリアドレスにそのまま複製する。プリミティブ型のデータの複製はディープコピーで実装されているプログラミング言語が多い。一方でコンテナ型のデータやオブジェクトは多くのメモリ領域を使用しており、ディープコピーを行うと処理にかかるコストが大きいため、明示しない限りはシャロ―コピーを行う場合が多い。このような都合からディープコピーは**高価なコピー**であるとされ、可能な限り避けるべきである。可変長配列のデータ追加の際でメモリ領域が不足した場合に発生するアロケートでもディープコピーが発生するため、あらかじめ十分な大きさのメモリ領域を確保しておいた方がよい。

### シャローコピー

**シャローコピー**ではメモリ上のデータの位置を指すポインタを取得する。コンテナ型のデータやオブジェクトはサイズが大きくなる場合があるため、シャローコピーを用いることで複製のコストを抑えている。ただし、ポインタが指すデータは共通となるため本当の意味での複製ではなく、データが書き換えられると全てのコピーが指す値が変更されてしまう点に注意が必要である。
