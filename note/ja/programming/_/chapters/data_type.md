# 『データ型』

（最終更新： 2023-02-27）


## 目次

1. [型](#型)
1. [プリミティブ型](#プリミティブ型)
	1. [文字型](#文字型)
	1. [整数型](#整数型)
	1. [固定長小数点数型](#固定長小数点数型)
	1. [浮動小数点数型](#浮動小数点数型)
	1. [ブーリアン型](#ブーリアン型)
	1. [ポインタ型](#ポインタ型)
	1. [参照型](#参照型)
1. [複合型](#複合型)
1. [コンテナ型](#コンテナ型)
	1. [イテレータ](#イテレータ)
	1. [配列](#配列)
	1. [リスト](#リスト)
	1. [スタック](#スタック)
	1. [キュー](#キュー)
	1. [ハッシュテーブル](#ハッシュテーブル)
	1. [連想配列](#連想配列)
	1. [セット](#セット)
	1. [木](#木)
	1. [グラフ](#グラフ)
	1. [文字列型](#文字列型)
1. [オブジェクト型](#オブジェクト型)
1. [データの複製](#データの複製)
	1. [ディープコピー](#ディープコピ―)
	1. [シャローコピー](#シャローコピー)


## 型

**型**は、[プログラム](./basic_knowledge_of_programming.md#プログラム)で扱うデータを性質や属性によって分類してもの。[静的型付け言語](./basic_knowledge_of_programming.md#静的型付け言語)においては、[プログラマ](./basic_knowledge_of_programming.md#プログラマ)が[ソースコード](./basic_knowledge_of_programming.md#ソースコード)中に型を明示することによって[コンパイル](./basic_knowledge_of_programming.md#コンパイル)時にデータの型が決まる。[静的型付け言語](./basic_knowledge_of_programming.md#静的型付け言語)においては、実行時に自動的に型が判断される。


## プリミティブ型

**プリミティブ型**（**基本データ型**、**原始型**）は、[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)で標準的に用意されている最も基本的な[型](#型)。プリミティブ型に属する[型](#型)は、[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)においてそれ以上分解することができないシンプルな[型](#型)として定義されており、[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ-1)が高速に[演算](./operation.md#演算-1)できるという特徴がある。

### 文字型

**文字型**( `char` )は、[自然言語](./basic_knowledge_of_programming.md#プログラミング言語)の1文字を格納するための[型](#型)。1[バイト](../../../basics/_/chapters/computer_and_number.md#バイト)で表現できる[ASCII文字](../../../basics/information_theory/_/chapters/character_representation.md#asciiコード)や、[マルチバイト文字](../../../basics/information_theory/_/chapters/character_representation.md#マルチバイト文字)で様々な文字種に対応している[JISコード](../../../basics/information_theory/_/chapters/character_representation.md#jisコード)や[Unicode](../../../basics/information_theory/_/chapters/character_representation.md#unicode)などのサポートが一般的。

[ソースコード](./basic_knowledge_of_programming.md#ソースコード)中で `'` で囲まれた部分は文字型のデータとして扱われる（ `'` と `"` を使い分けない[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)もある）。このようにクォーテーションで囲まれた、[ソースコード](./basic_knowledge_of_programming.md#ソースコード)中に直接記述された文字のことを**文字リテラル**という。

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

**整数型**( `int, short, long' )は、整数値を格納するための[型](#型)。1[バイト](../../../basics/_/chapters/computer_and_number.md#バイト)が表現できる範囲は $0 \sim 255 (2^8)$ であるため、より大きい数値を扱えるように2[バイト](../../../basics/_/chapters/computer_and_number.md#バイト)や4[バイト](../../../basics/_/chapters/computer_and_number.md#バイト)、8[バイト](../../../basics/_/chapters/computer_and_number.md#バイト)の整数型などを用意している[プログラミング言語](./basic_knowledge_of_programming.md#プログラミングン言語)もある。**符号付き整数**と**符号なし整数**をサポートしている[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)もあり、値が負にならないことがわかっている場合には符号なし整数を用いた方が使用できる数の範囲が増える。

### 固定長小数点数型

**固定長小数点数型**( `fixed` )は、小数を扱うための[型](#型)で、どの[ビット](../../../basics/_/chapters/computer_and_number.md#ビット)を小数点とするかをあらかじめ決めておく。固定小数点数を扱うため、データの解析が容易である一方で、表せる数の範囲は[浮動小数点数](#浮動小数点数型)に比べて狭い。

### 浮動小数点数型

**浮動小数点数型**( `float, double` )は、小数を扱うための[型](#型)で、浮動小数点方式の小数を扱う。[単精度](../../../basics/discrete_mathematics/_/chapters/numeric_representation.md#浮動小数点数)と[倍精度](../../../basics/discrete_mathematics/_/chapters/numeric_representation.md#浮動小数点数)で別の[型](#型)として用意している[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)もある。

### ブーリアン型

**ブーリアン型**( `bool, boolean` )は、[真偽値](../../../basics/discrete_mathematics/_/chapters/set_and_proposition.md#真偽値)を扱うための[型](#型)で、 `true` か `false` のどちらかの値が格納される。この[型](#型)に対しては、[論理積](../../../basics/discrete_mathematics/_/chapters/logical_operation.md#and演算)( `AND` )、[論理和](../../../basics/discrete_mathematics/_/chapters/logical_operation.md#or演算)( `OR` )、[排他的論理和](../../../basics/discrete_mathematics/_/chapters/logical_operation.md#xor演算)( `XOR` )、同値、非同値、[否定](../../../basics/discrete_mathematics/_/chapters/logical_operation.md#not演算)といった[論理演算](../../../basics/discrete_mathematics/_/chapters/logical_operation.md#論理演算-1)が可能。[条件分岐](./control_flow.md#条件分岐)において評価結果としても用いられる。

### ポインタ型

**ポインタ型**は、[変数](./variable.md#変数-1)や[関数](./function.md#関数-1)の実体が格納された、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)上の[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)を保持しておくための[型](#型)。[配列](#配列)や[オブジェクト](#オブジェクト型)といった[複合型](#複合型)は[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)上の[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)に格納されており、実体にアクセスするためには[スタック領域](../../../computer/hardware/_/chapters/memory.md#スタック領域)のポインタを経由する必要がある。また、[スタック領域](../../../computer/hardware/_/chapters/memory.md#スタック領域)に格納されたデータに対してもポインタをつくることができる。

[プログラム](./basic_knowledge_of_programming.md#プログラム)中で利用される[変数](./variable.md#変数-1)や[関数](./function.md#関数-1)などのリソースは[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)上で固有の[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)に割り当てられており、[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)を知っていればそのデータにアクセスすることができる。また、[メモリアドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)の足し引きによって隣のデータにアクセスすることもできる。一方で、予期しない[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)のデータを変更してしまうと深刻な[バグ](./basic_knowledge_of_programming.md#バグ)に繋がるため、注意が必要である。

ポインタのポインタや、さらにそのポインタを作るといったこともできる。

### 参照型

**参照型**は[ポインタ型](#ポインタ型)と同様、[変数](./variable.md#変数-1)や[関数](./function.md#関数-1)が置かれた[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)上の[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)を指す[型](#型)。[ポインタ型](#ポインタ型)とは異なり、[参照](./variable.md#参照)先に書き込むことはできない、読み込み専用の[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)。また、[参照](./variable.md#参照)先にデータが存在しないことを禁止しており、[ポインタ](#ポインタ型)に比べて安全に利用できるという利点がある。


## Null

**Null**は、何もないことを表すデータで、[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)によって実装や扱いが異なる。[型](#型)として定義されている言語もあれば、特別な値として扱う言語もある。空文字や `0` とは区別され、[変数](./variable.md#変数-1)に値が[代入](./variable.md#代入)されていない状態などを表す。


## 複合型

**複合型**は、[プリミティブ型](#プリミティブ型)を組み合わせた構造を持つ[型](#型)。あらかじめ定義された汎用的な[型](#型)もあるが、多くの[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)では開発者が独自に定義することもできる。

各[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング)ごとに用意されている複合型や、内部的なデータ構造が異なる場合があるので注意。


## コンテナ型

**コンテナ型**（**コレクション型**）は、同じ[型](#型)のデータを複数まとめるための抽象データ[型](#型)。それぞれに特性や得意とする操作が異なるため、用途に応じて適切に使い分けることで[プログラム](./basic_knowledge_of_programming.md#プログラム)の質が向上する。

### イテレータ

**イテレータ**は、[コンテナ型](#コンテナ型)が持つ各要素に対する繰り返し処理の抽象化構造。[繰り返し処理](./control_flow.md#反復)の[for文](./control_flow.md#ofr文)などにおいて利用される。イテレータは[コンテナ](#コンテナ型)内の要素の[ポインタ](#ポインタ型)を指し示しており、[ループ](./control_flow.md#反復)が進むにつれて次の要素の[ポインタ](#ポインタ型)を返す。

### 配列

**配列**は、同じ[型](#型)のデータの集合を格納するためのデータ[型](#型)で、要素が[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)上で隣り合うように並べられる。要素には**インデックス**（添え字）を使ってアクセスすることができる。インデックスは配列の最初の要素を `0` とした整数値となっている。

```c
// C言語

char str[] = "Hello, world";

str[5] = "!";
printf("%s\n", str);     // "Hello! world"が出力される
```

配列には、[コンパイル](./basic_knowledge_of_programming.md#コンパイル)時に要素数が決まる**固定長配列**と、実行時に要素数を動的に追加したり削除したりできる**可変長配列**がある。通常、固定長配列は[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)上の[スタック領域](../../../computer/hardware/_/chapters/memory.md#スタック領域)に、可変長配列は[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)に格納される。可変長配列において、[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)に確保したサイズが不足した場合、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)領域の再確保（[アロケート](./basic_knowledge_of_programming.md#アロケート)）が発生するため、あらかじめ十分な領域を用意しておくとよい。

可変長配列の末尾に対するデータの追加・削除にかかる[計算量](../../../basics/information_theory/_/chapters/computational_complexity.md#計算量-1)は $O(1)$ と非常に高速である。一方、任意のインデックスに対するデータの追加・削除の際には、そのインデックスより後ろのデータ全てを移動する必要があるため、 $O(N)$ と低速となる。任意のインデックスの要素にアクセスするのにかかる[計算量](../../../basics/information_theory/_/chapters/computational_complexity.md#計算量-1)は常に $O(1)$ で高速である。データの探索にかかる[計算量](../../../basics/information_theory/_/chapters/computational_complexity.md#計算量-1)は基本的に $O(N)$ であるが、配列がソート済みである場合は[二分探索](./algorithm.md#二分探索)を用いて $O(\log{N})$ に抑えることもできる。

基本的には使い勝手の良い[コンテナ型](#コンテナ型)の構造となっているが、中間位置へのデータの挿入が頻繁に行われたり、要素数が非常に大きくなるようなデータに対しては[リスト](#リスト)を用いた方が良い場合もある。

### リスト

**リスト**（**連結リスト**、**リンクリスト**）は、各ノードが任意のデータと他のノードへの[ポインタ](#ポインタ型)を持つようなデータ構造。ノードが別のノードの[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)情報を持つことによって、データの集まりを表現している。

- **単方向リスト** : 各ノードが次のノードへのポインタを持つリスト
- **双方向リスト** : 各ノードが次のノードと前のノードへのポインタを持つリスト
- **循環リスト** : 末尾のノードが先頭のノードへのポインタを持つリスト（単方向と双方向がある）

リストは、前後のノードと[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)上で隣り合っている必要がないため、データ数が増えても[アロケート](./basic_knowledge_of_programming.md#アロケート)が発生しないという利点がある。

リストの任意の場所へのデータの挿入・削除にかかる[計算量](../../../basics/information_theory/_/chapters/computational_complexity.md#計算量-1)は $O(1)$ と高速である。一方で、要素へのアクセスにかかる[計算量](../../../basics/information_theory/_/chapters/computational_complexity.md#計算量-1)は $O(N)$ であり、[配列](#配列)に比べると遅い。これは、リストの要素へアクセスするには、先頭の要素から順番に[ポインタ](#ポインタ型)をたどっていく必要があるためである（[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)上での配置が隣り合っているとは限らないため）。

### スタック

**スタック**は、データを**FILO**（First In Last Out: 先入れ後出し）の構造で保持する[コンテナ](#コンテナ型)。スタックに対してデータを追加する操作を**プッシュ**、スタックからデータを取り出す操作を**ポップ**といい、これを基本操作とするデータ[型](#型)となる。最後にプッシュしたデータから取り出したいような場合に用いる（画面上のパネルの重なり、積み重ねた本、エレベータの乗り降り）。

### キュー

**キュー**は、データを**FIFO**（First In First Out: 先入れ先出し）の構造で保持する[コンテナ](#コンテナ型)。キューに対してデータを追加する操作を**エンキュー**、キューからデータを取り出す操作を**デキュー**といい、これを基本操作とするデータ[型](#型)となる。最初にエンキューしたデータから取り出したいような場合に用いる（[待ち行列](../../../basics/applied_mathematics/_/chapters/waiting_queue_theory.md#待ち行列)）。

キューに追加する要素に優先度をつけ、優先度に基づいてデータをソートし、取り出し順を制御するようなキューのことを**優先度つきキュー**という。

先頭、末尾の両方からデータの追加・取り出しを行うことができるようにしたキューのことを**両端キュー**（デック）という。

### ハッシュテーブル

**ハッシュテーブル**は、キーと値の組からなる**エントリ**を複数格納するデータ[型](#型)。キーを基に生成された**ハッシュ値**を[インデックス](#配列)として値を管理することで、検索や要素の追加を $O(1)$ の[計算量](../../../basics/information_theory/_/chapters/computational_complexity.md#計算量-1)で実現することができる。ハッシュ値を産出するための関数を**ハッシュ関数**という。

ハッシュ値を[インデックス](#配列)とした[配列](#配列)であり、[インデックス](#配列)が飛び飛びとなるため、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)効率は悪い。

複数の異なるキーから生成されるハッシュ値が同じものになってしまう場合があり、これを**衝突**という。**連鎖法**では、同じハッシュ値で解決されるデータを[リンクリスト](#リスト)で管理する。**開番地法**では、衝突が発生したときに別のハッシュ関数を用いて次の候補地となる[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)を算出する。

### 連想配列

**連想配列**（**連想リスト**、**連想コンテナ**、**辞書**）は、[インデックス](#配列)として文字列などの整数値以外のデータ[型](#型)を用いることができるような[コンテナ](#コンテナ型)。連想配列の実装には、[ハッシュテーブル](#ハッシュテーブル)や[平衡二分探索木](#木)などが用いられる。

### セット

**セット**（**集合**）は、順序のないデータの集まりを表現するデータ[型](#型)で、重複するデータが存在しないことを保証する。

### 木

**木**は、**ノード**とノード間を結ぶ**エッジ**からなる木構造のデータ。1つのノードが複数の子ノードを持ち、親ノードを持たないノードを**ルートノード**（**根ノード**）、子ノードを持つノードを**ブランチノード**（**枝ノード**）、子ノードを持たないノードを**リーフノード**（**葉ノード**）という。

各ノードが持つ子ノードの数が2つに制限された木を**二分木**、リーフの深さがなるべくそろうように構築された木を**平衡木**という。

### グラフ

**グラフ**は、[木](#木)と同様に[ノード](#木)と[エッジ](#木)からなるデータ構造であるが、[木](#木)とは異なり[ノード](#木)同士が親子関係を持たない。

### 文字列型

**文字列型**( `string` )は、複数の文字からなる文字列を表現するための[型](#型)。[文字型](#文字型)には1つの文字しか格納できないため、2つ以上の文字からなる文字列を取り扱いたい場合は[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)ごとに用意されている文字列型を用いる。文字列型の[オブジェクト](#オブジェクト型)を用意している場合もあれば、[文字型](#文字型)の[配列](#配列)を文字列として扱う言語もある。

[ソースコード](./basic_knowledge_of_programming.md#ソースコード)中で `"` で囲まれた部分は文字列型のデータとして扱われる（ `'` と `"` を使い分けない[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)もある）。このようにクォーテーションで囲まれた、[ソースコード](./basic_knowledge_of_programming.md#ソースコード)中に直接記述された文字のことを**文字列リテラル**という。

文字列の終端を判別するための特別な文字としては、**NULL文字**が用いられる（[Null](#null)とは異なる）。[配列](#配列)により文字列を表現する[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)では、文字列の長さにNULL文字の分の1を加えたサイズの[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)領域が必要となる。

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


## オブジェクト型

**オブジェクト型**は、[オブジェクト指向](./object_oriented.md#オブジェクト指向-1)の[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)において、[クラス](./object_oriented.md#クラス)から生成された[オブジェクト](./object_oriented.md#オブジェクト)を指すデータ[型](#型)。


## データの複製

[プログラム](./basic_knowledge_of_programming.md#プログラム)中で用いるデータを複製する方法には、[ディープコピー](#ディープコピー)と[シャローコピー](#シャローコピー)がある。

### ディープコピー

**ディープコピー**は、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)上のデータの実体を別の[メモリアドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)にそのまま複製する方法。多くの[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)では、[プリミティブ型](#プリミティブ型)のデータの複製はディープコピーによって行われる。

[コンテナ型](#コンテナ型)や[オブジェクト型](#オブジェクト型)のデータは多くの[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)領域を必要とするため、ディープコピーのコストが大きくなる。そのため、明示しない限りは[シャローコピー](#シャローコピー)となる[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)が多い。このような特徴から、ディープコピーは**高価なコピー**であるとされ、無駄なディープコピーは避けるべきである。

[可変長配列](#配列)のデータを追加するときに、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)不足が発生した場合、より広い[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)領域が[アロケート](./basic_knowledge_of_programming.md#アロケート)され、確保した領域に元のデータがディープコピーされる。

### シャローコピー

**シャローコピー**は、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)上のデータ[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)を指す[ポインタ](#ポインタ型)を取得する方式。コピー元と同じデータを指し示す[ポインタ](#ポインタ型)を取得する見かけ上のコピーであるため、データの実体が書き換えられると、コピー元もコピー先もデータが置き換わったように見える。そのため、コピー元かコピー先のどちらかのデータだけを変更したいような場合には[ディープコピー](#ディープコピー)を用いる必要がある。

[コンテナ型](#コンテナ型)や[オブジェクト型](#オブジェクト型)のデータは[ディープコピー](#ディープコピー)にかかるコストが大きいため、シャローコピーを用いることで複製のコストが抑えられる。
