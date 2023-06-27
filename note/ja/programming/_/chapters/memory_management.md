# 『メモリ管理』ノート

（最終更新： 2023-04-17）


## 目次

1. [プログラムのメモリ管理](#プログラムのメモリ管理)
	1. [ヒープ領域の確保と解放](#ヒープ領域の確保と解放)
	1. [メモリリーク](#メモリリーク)
	1. [ダブルフリー](#ダブルフリー)
	1. [ダングリングポインタ](#ダングリングポインタ)
	1. [ガベージコレクション](#ガベージコレクション)
	1. [アロケート](#アロケート)


## プログラムのメモリ管理

[プログラム](./programming.md#プログラム)中で用いられる[変数](./variable.md#変数)や[関数](./function.md#関数)は、[プログラム](./programming.md#プログラム)に割り当てられた[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)領域内に格納される。[グローバル変数](./variable.md#グローバル変数)は[静的領域](../../../computer/hardware/_/chapters/memory.md#静的領域)に、[関数](./function.md#関数)内で使用される[ローカル変数](./variable.md#ローカル変数)は[スタック領域](../../../computer/hardware/_/chapters/memory.md#スタック領域)に、[配列](./data_type.md#配列)や[オブジェクト](./object_oriented.md#オブジェクト)といった可変長のデータの実体は[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)に格納される。

[プログラミング言語](./programming.md#プログラミング言語)の[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)管理においては、次の2つの特徴を持つことが望ましい。

- [メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)が[プラグラマ](./programming.md#プログラマ)が選んだタイミングで適切に解放されること。これにより[プログラム](./programming.md#プログラム)が無駄な[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)を消費しないようにする。
- 解放済みの[オブジェクト](./object_oriented.md#オブジェクト)への[ポインタ](./data_type.md#ポインタ型)（[ダングリングポインタ](#ダングリングポインタ)）を使ってしまうことがないこと。これは[未定義動作](./programming.md#未定義動作)となり、クラッシュやセキュリティホールにつながる。

これらの特徴は相反しており、これら2つを両立することは難しい。[プログラマ](./programming.md#プログラマ)が任意のタイミングで値の[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)を解放できる言語については、[ポインタ](./data_type.md#ポインタ型)が存在するうちに値を解放してしまうと、その[ポインタ](./data_type.md#ポインタ型)の参照する先がなくなってしまう（[ダングリングポインタ](#ダングリングポインタ)）。多くの[プログラミング言語](./programming.md#プログラミング言語)はこれらのうちどちらかを諦めており、トレードオフを選択している。

- 安全第一の[プログラミング言語](./programming.md#プログラミング言語)では、[ガベージコレクション](#ガベージコレクション)を用いて[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)管理を行う。[オブジェクト](./object_oriented.md#オブジェクト)へ到達できる[ポインタ](./data_type.md#ポインタ型)がすべてなくなったところで自動的にその[オブジェクト](./object_oriented.md#オブジェクト)を解放する。[Python](./programming_language.md#python)、[JavaScript](./programming_language.md#javascript)、[Java](./programming_language.md#java)、[C#](./programming_language.md#c-1)、Haskellなどのほとんどの近代的な言語はこれに属する。ただし、[ガベージコレクション](#ガベージコレクション)を用いると望んだタイミングで[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)が解放されない可能性がある。
- 制御優先の[プログラミング言語](./programming.md#プログラミング言語)では、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)解放の責任を[プログラマ](./programming.md#プログラマ)に委ねる。[ダングリングポインタ](#ダングリングポインタ)を避ける責任も[プログラマ](./programming.md#プログラマ)に課せられ、[プログラマ](./programming.md#プログラマ)がミスを侵さなければ[ガベージコレクション](#ガベージコレクション)を用いるよりも良い方法である（ただし人はミスをするため、危険な言語であるとも言える）。これに属する代表的な言語は[C](./programming_language.md#c言語)と[C++](./programming_language.md#c)だけである。

[ポインタ](./data_type.md#ポインタ型)の扱いに成約を加えたことでこれら2つを両立した言語として、[Rust](./programming_language.md#rust)がある。

### ヒープ領域の確保と解放

[スタック領域](../../../computer/hardware/_/chapters/memory.md#スタック領域)に格納された[変数](./variable.md#変数)は、[関数](./function.md#関数)の実行が完了すると自動的に解放されるが、[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)に格納したデータは別の手段により解放する必要がある。[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)のデータを解放する手段は[プログラミング言語](./programming.md#プログラミング言語)によって異なる。[C言語](./programming_language.md#c言語)や[C++](./programming_language.md#c)では、[プログラマ](./programming.md#プログラマ)が明示的に[ソースコード](./programming.md#ソースコード)中で[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)の確保・解放を宣言する必要があり、その他のほとんどの[プログラミング言語](./programming.md#プログラミング言語)では[ガベージコレクション](#ガベージコレクション)を用いる。また、[Rust](./programming_language.md#rust)では[ガベージコレクション](#ガベージコレクション)を用いずに[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)の解放に関する一定のルールを設けることで、[プログラマ](./programming.md#プログラマ)が[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)管理に関する責任を負わなくてよいようにしている。

### メモリリーク

**メモリリーク**は、[プログラム](./programming.md#プログラム)が使い終わった[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)の解放を忘れることで、使用できなくなる[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)領域ができてしまうこと。[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)管理に関する[バグ](./programming.md#バグ)の一種。

### ダブルフリー

**ダブルフリー**（[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)の二重解放、多重フリー）は、既に解放された[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)の[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)を、再度解放しようとすること。[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)管理に関する[バグ](./programming.md#バグ)の一種。

### ダングリングポインタ

**ダングリングポインタ**は、[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)に格納されたデータの[アドレス](../../../computer/hardware/_/chapters/memory.md#アドレス)を格納した[スタック領域](../../../computer/hardware/_/chapters/memory.md#アドレス)の[ポインタ](./data_type.md#ポインタ型)において、既に解放された[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)を指し示す[ポインタ](./data_type.md#ポインタ型)。[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)管理に関する[バグ](./programming.md#バグ)の一種。

### ガベージコレクション

**ガベージコレクション**（**GC**）は、[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)のデータを定期的に確認し、不要になったデータを解放する[プログラム](./programming.md#プログラム)。ガベージコレクションを用いる場合、[ソースコード](./programming.md#ソースコード)中で[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)の解放を明示する必要はない。

[メモリリーク](#メモリリーク)や[ダブルフリー](#ダブルフリー)、[ダングリングポインタ](#ダングリングポインタ)といった[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)に関する[バグ](./programming.md#バグ)を回避するこおとができる。ただし、[プログラマ](./programming.md#プログラマ)自身が[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)管理をする[プログラミング言語](./programming.md#プログラミング言語)に比べてパフォーマンスは低く、[プログラム](./programming.md#プログラム)の実行に関係のない[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)（ガベージコレクション）を動作させる必要があるというデメリットがある。また、ガベージコレクションが動くタイミングで[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)の動作が遅くなる可能性がある。

### アロケート

**アロケート**（**アロケーション**）は、データを配置するために必要な[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)領域を確保すること。可変長データを拡張しようとしたときに、あらかじめ確保した[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)領域が不足した場合、改めて十分な空き領域をアロケートする必要がある。基本的にアロケートは低速な操作であるため、アロケートが発生する回数が少なくなるようにすることで[プログラム](./programming.md#プログラム)を高速化することができる場合がある。
