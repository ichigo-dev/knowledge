# 『プログラミング』ノート

（最終更新： 2023-04-17）


## 目次

1. [プログラミング](#プログラミング)
	1. [プログラム](#プログラム)
	1. [プログラマ](#プログラマ)
	1. [プログラミング言語](#プログラミング言語)
	1. [ソースコード](#ソースコード)
1. [機械語](#機械語)
	1. [オペコード](#オペコード)
1. [低水準言語](#低水準言語)
	1. [アセンブリ言語](#アセンブリ言語)
	1. [アセンブル](#アセンブル)
	1. [アセンブラ](#アセンブラ)
1. [高水準言語](#高水準言語)
	1. [コンパイル](#コンパイル)
	1. [コンパイラ](#コンパイラ)
	1. [コンパイラ言語](#コンパイラ言語)
	1. [インタプリタ言語](#インタプリタ言語)
	1. [JITコンパイラ](#jitコンパイラ)
	1. [スクリプト言語](#スクリプト言語)
	1. [静的型付け言語](#静的型付け言語)
	1. [動的型付け言語](#動的型付け言語)
	1. [汎用プログラミング言語](#汎用プログラミング言語)
	1. [専用プログラミング言語](#専用プログラミング言語)
	1. [トランスコンパイル](#トランスコンパイル)
1. [プログラミングパラダイム](#プログラミングパラダイム)
	1. [命令型プログラミング](#命令型プログラミング)
	1. [宣言型プログラミング](#宣言型プログラミング)
	1. [構造化プログラミング](#構造化プログラミング)
	1. [オブジェクト指向プログラミング](#オブジェクト指向プログラミング)
	1. [関数型プログラミング](#関数型プログラミング)
	1. [論理プログラミング](#論理プログラミング)
	1. [制約プログラミング](#制約プログラミング)
	1. [マルチパラダイム言語](#マルチパラダイム言語)
1. [プログラムの基礎と文法](#プログラムの基礎と文法)
	1. [文法](#文法)
	1. [コメント](#コメント)
	1. [エラー](#エラー)
	1. [例外](#例外)
	1. [バグ](#バグ)
	1. [デバッグ](#デバッグ)
	1. [未定義動作](#未定義動作)
	1. [識別子](#識別子)
	1. [予約語](#予約語)
1. [プログラミングを始める](#プログラミングを始める)


## プログラミング

**プログラミング**は、[プログラム](#プログラム)を記述して[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)へ命令を行い、目的とするタスクを解決すること。

[プログラマ](#プログラマ)には、世の中の様々なタスクや問題をプログラミングによって解決するという役割がある。毎日の単純作業のくり返しを減らしたり（表計算[ソフト](../../../computer/software/_/chapters/software.md#ソフトウェア)など）、ミスが許されない場面で正確に動作することが保証されたシステムを利用したり（銀行のシステムや信号機など）、さらに人間には難しいタスクの解決を行ったり（[AI](../../../artificial_intelligence/_/chapters/artificial_intelligence.md#人工知能)による異常検知など）と、プログラミングスキルの応用範囲は多岐にわたる。

### プログラム

**プログラム**は、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)が処理を行う手順や内容を示したもの。

### プログラマ

**プログラマ**は、[プログラム](#プログラム)を記述する職業。

### プログラミング言語

**プログラミング言語**は、[プログラム](#プログラム)を記述する際に用いられる言語。プログラミング言語に対して、人間が日常生活で用いる言語のことを**自然言語**という。

### ソースコード

**ソースコード**（**ソース**、**コード**）は、[プログラミング言語](#プログラミング言語)で書かれた[プログラム](#プログラム)。


## 機械語

**機械語**（**マシン語**、**オブジェクトコード**、**ネイティブコード**）は、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)が直接解釈できる[プログラム](#プログラム)で、[2進数](../../../basics/discrete_mathematics/_/chapters/radix.md#2進数)の羅列からなる。機械語は次の例のような[プログラム](#プログラム)となっており、人間が記述したり内容を理解するのには適していない。そのため、人間が理解しやすいように[プログラミング言語](#プログラミング言語)を用いて[ソースコード](#ソースコード)を作成し、専用の[プログラム](#プログラム)によって[ソースコード](#ソースコード)を機械語に変換するのが一般的。[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)ごとに対応する機械語も異なるので注意が必要。

```
0111 1111 0100 0101 0100 1100 0100 0101 0000 0010 0000 0001 0000 0001 0000 0000
0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000
0000 0010 0000 0000 0011 1110 0000 0000 0000 0001 0000 0000 0000 0000 0000 0000
0000 0000 0000 0000 0000 0100 0100 0000 0000 0000 0000 0000 0000 0000 0000 0000

...(以下略)
```

### オペコード

**オペコード**は、[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)に与える[機械語](#機械語)の命令の識別番号。


## 低水準言語

**低水準言語**（**低級言語**）は、[プログラミング言語](#プログラミング言語)の中でも[機械語](#機械語)に近い言語。低水準言語は[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)資源を効率よく使うことができ、高速に動作するという利点があるが、[プログラム](#プログラム)が長くなりやすく柔軟性に乏しいことから、組込みや[ファームウェア](../../../computer/_/chapters/computer.md#ファームウェア)といった分野など、限られた[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)資源で性能を最大限引き出すという目的がない限りは[高水準言語](#高水準言語)が用いられることがほとんどとなる。

### アセンブリ言語

**アセンブリ言語**は、ニーモニックという命令を組み合わせて記述する[プログラミング言語](#プログラミング言語)。**ニーモニック**は、[機械語](#機械語)の命令に1対1で対応する、英単語や記号で表記した命令。[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)ごとに対応するアセンブリ言語も異なるので注意が必要。

アセンブリ言語の一種である**Netwide Assembler**を用いて `Hello, world` を出力する[プログラム](#プログラム)を以下に示す。

```asm
section .data
    msg db      "Hello, world"      ; データセクションで文字列を宣言

section .text
    global _start                   ; プログラムのエントリポイントとして_startを呼び出し

_start:
    mov     rax, 1                  ; sys_writeシステムコール(1)をraxレジスタにセット
    mov     rdi, 1                  ; 第一引数（ファイルディスクリプタ、1は標準出力）をrdiレジスタにセット
    mov     rsi, msg                ; 第二引数（出力したい文字列）をrsiレジスタにセット
    mov     rdx, 12                 ; 第三引数（データのサイズ）をrdxレジスタにセット
    syscall                         ; システムコールを発行
    mov     rax, 60                 ; exitシステムコール(60)をraxレジスタにセット
    mov     rdi, 0                  ; 第一引数（終了コード、0は正常終了）をrdiレジスタにセット
    syscall                         ; システムコールを発行
```

### アセンブル

**アセンブル**は、[アセンブリ言語](#アセンブリ言語)で書かれた[プログラム](#プログラム)を[機械語](#機械語)に変換する操作。

### アセンブラ

**アセンブラ**は、[アセンブル](#アセンブル)を行うための[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)。


## 高水準言語

**高水準言語**（**高級言語**）は、[低水準言語](#低水準言語)に比べて抽象度が高く、人間にとってわかりやすい[プログラミング言語](#プログラミング言語)。[機械語](#機械語)を意識することなく記述することができ、[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)に依存しない[プログラム](#プログラム)を作ることができる（[機械語](#機械語)への変換の段階で様々な[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)に対応させることができるため）。

代表的な高水準言語としては、C系言語（[C](./programming_language.md#c言語), [C#](./programming_language.md#c-1), [C++](./programming_language.md#c)等）や[Java](./programming_language.md#java)、[Python](./programming_language.md#python)、[PHP](./programming_language.md#php)、[JavaScript](./programming_language.md#javascript)などがある。それぞれの言語には得手不得手があり、[C言語](./programming_language.md#c言語)であれば処理速度が高速であることから[CPU](../../../computer/hardware/_/chapters/processor.md#cpu)や[サーバ](../../../computer/_/chapters/computer.md#サーバ)などの低レイヤの実装に向いており、[JavaScript](./programming_language.md#javascript)であれば一般的な[Webブラウザ](../../../network/_/chapters/web.md#webブラウザ)で動作することから[Web](../../../network/_/chapters/web.md#web)フロントエンド向けとなっている。

高水準言語は種類が多く、[ソースコード](#ソースコード)の[コンパイル・](#コンパイル)実行方式や、[データ型](./data_type.md#型)の決定方式などによって分類することができる。

### コンパイル

**コンパイル**は、[高水準言語](#高水準言語)で書かれた[ソースコード](#ソースコード)を[機械語](#機械語)に変換する操作。

### コンパイラ

**コンパイラ**は、[コンパイル](#コンパイル)を行うための[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)。

### コンパイラ言語

**コンパイラ言語**は、[プログラム](#プログラム)を実行する前に、あらかじめ[ソースコード](#ソースコード)全体の[コンパイル](#コンパイル)を行う方式の言語。[プログラム](#プログラム)を利用する際には、[機械語](#機械語)に変換された[実行ファイル](../../../computer/software/_/chapters/file_system.md#実行ファイル)を実行する。

事前に[コンパイル](#コンパイル)を済ませているため、実行前に不具合を発見できたり、実行時の[コンパイル](#コンパイル)コストがないため実行速度が速いといった利点がある。一方で、実行時に起こりうる様々なパターンに対応できるように[プログラミング](#プログラミング)する必要があり、[インタプリタ言語](#インタプリタ言語)と比較しても難易度が高い言語が多い。また、[コンパイル](#コンパイル)済みの[実行ファイル](../../../computer/software/_/chapters/file_system.md#実行ファイル)は[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)に依存するため、[マルチプラットフォーム](../../../computer/software/_/chapters/software.md#マルチプラットフォーム)に対応できないという欠点もある（それぞれの[プラットフォーム](../../../computer/software/_/chapters/software.md#プラットフォーム)用に[コンパイル](#コンパイル)する必要がある）。

代表的なコンパイラ言語には、[C言語](./programming_language.md#c言語)や[Swift](./programming_language.md#swift)、[Go](./programming_language.md#go)、[Rust](./programming_language.md#rust)といったものがある。

### インタプリタ言語

**インタプリタ言語**は、[プログラム](#プログラム)の実行時に[ソースコード](#ソースコード)を1行ずつ[コンパイル](#コンパイル)しながら処理を進める方式の言語。実行時に毎回[コンパイル](#コンパイル)を行うコストが発生するため、実行速度は[コンパイラ言語](#コンパイラ言語)に劣る。しかし、[コンパイル](#コンパイル)を行わずに実行結果をすぐに確認できるため、[デバッグ](#デバッグ)が容易で、開発にかかるコストは小さくなる。

代表的なインタプリタ言語には、[Python](./programming_language.md#python)や[PHP](./programming_language.md#php)、[JavaScript](./programming_language.md#javascript)、Rubyなどがある。

### JITコンパイラ

**JITコンパイラ**(Just-In Time Compiler)は、[ソースコード](#ソースコード)を環境に依存しない**中間コード**（**バイトコード**）に変換しておき、実行時にバイトコードを[コンパイル](#コンパイル)して実行する方式をとる。実行時に[コンパイル](#コンパイル)する点では[インタプリタ言語](#インタプリタ言語)と似ているが、[字句解析](../../../basics/information_theory/_/chapters/compiler_theory.md#字句解析)などのコストを省けるため、[インタプリタ言語](#インタプリタ言語)より高速に動作する。また、[コンパイラ言語](#コンパイラ言語)では直接[機械語](#機械語)のコードを生成するため、[プログラム](#プログラム)が実行環境（[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)など）に依存してしまうが、この方式では[プログラム](#プログラム)の実行機である[ランタイム](../../../computer/software/_/chapters/software.md#ランタイム)（[フレームワーク](../../../computer/software/_/chapters/package.md#フレームワーク)）が利用できる環境であればひとつの中間コードで実行できるというメリットもある。

JITコンパイラを用いる代表的な言語には、[Java](./programming_language.md#java)や[C#](./programming_language.md#c-1)などがある。

### スクリプト言語

**スクリプト言語**は、[プログラミング言語](#プログラミング言語)のうち比較的容易に記述・実行ができる言語。定義は曖昧であるが、[プログラミング](#プログラミング)の初学者にとっても学びやすく、開発スピードが早いため小規模な開発に向いている。[インタプリタ言語](#インタプリタ言語)はスクリプト言語に分類されることが多く、[Python](./programming_language.md#python)や[PHP](./programming_language.md#php)、[JavaScript](./programming_language.md#javascript)、Rubyなどがスクリプト言語に含まれる。

### 静的型付け言語

**静的型付け言語**は、実行前にデータの[型](./data_type.md#型)を決定する方式の言語。[コンパイラ言語](#コンパイラ言語)の多くは静的型付け言語となっており、[プログラム](#プログラム)中で使用するデータに対して、そのデータをどの[型](./data_type.md#型)として扱うかを明示（**型注釈**）する必要がある。事前にデータの[型](./data_type.md#型)を確定させておくことで、想定外の処理が行われることによる不具合を減らすことができるなど、[プログラム](#プログラム)の信頼性が向上する。

また、[Rust](./programming_language.md#rust)や[Go](./programming_language.md#go)のような新しい静的型付け言語では、[ソースコード](#ソースコード)中に[型](./data_type.md#型)を明示せずとも、[コンパイラ](#コンパイラ)が文脈から自動的に[型](./data_type.md#型)を判断する**型推論**の機能が搭載されているものもあり、[ソースコード](#ソースコード)の記述量が増え冗長になるという静的型付け言語の欠点を軽減できる。

### 動的型付け言語

**動的型付け言語**は、実行時にデータの[型](./data_type.md#型)を決定する方式の言語。[ソースコード](#ソースコード)中で[型](./data_type.md#型)を明示する必要はなく、[コンパイラ](#コンパイラ)が文脈から全てのデータの[型](./data_type.md#型)を自動的に判断する。[ソースコード](#ソースコード)がシンプルになり読みやすくなる一方で、データが読み込まれるたびにそのデータの[型](./data_type.md#型)を推測する必要があるため、実行速度は[静的型付け言語](#静的型付け言語)に劣る。また、データが想定していない[型](./data_type.md#型)として扱われる可能性もあるため注意する必要がある。

実行時に[型](./data_type.md#型)を決定するという抽象化の性質から、[関数](./function.md#関数)がとる[引数](./function.md#引数)の[型](./data_type.md#型)も柔軟に判断することができるため、同じインタフェースを持つ異なる[オブジェクト](./object_oriented.md#オブジェクト)をひとつの[関数](./function.md#関数)で扱える。これを利用したプログラミングスタイルとして[ダックタイピング](./object_oriented.md#ダックタイピング)がある。

### 汎用プログラミング言語

**汎用プログラミング言語**は、広い用途で利用される[高水準言語](#高水準言語)。

### 専用プログラミング言語

**専用プログラミング言語**は、限られた分野で活躍する[高水準言語](#高水準言語)。

### トランスコンパイル

**トランスコンパイル**は、ある[プログラミング言語](#プログラミング言語)で書かれた[ソースコード](#ソースコード)を、別の[プログラミング言語](#プログラミング言語)の[コード](#ソースコード)に変換する処理。


## プログラミングパラダイム

**プログラミングパラダイム**は、[プログラミング](#プログラミング)の考え方や記述方法の枠組み。ひとつの[プログラミング言語](#プログラミング言語)は、複数のプログラミングパラダイムを組み合わせている場合（[マルチパラダイム言語](#マルチパラダイム言語)）が多い。

### 命令型プログラミング

**命令型プログラミング**（**手続き型プログラミング**）は、[識別子](#識別子)をつけた命令ブロック（[関数](./function.md#関数)）の定義と呼び出しを組み合わせることで[プログラム](#プログラム)全体を組み立てることを土台とした[プログラミングパラダイム](#プログラミングパラダイム)。[手続き](./function.md#関数)は、[プログラム](#プログラム)内のあらゆるポイントから呼び出すことができ、[手続き](./function.md#関数)内の命令コード行の終端に到達すると、その[手続き](./function.md#関数)を呼び出したポイントの次の命令に制御が移される（**復帰**）。[C言語](./programming_language.md#c言語)や[C++](./programming_language.md#c)、[PHP](./programming_language.md#php)、[Python](./programming_language.md#python)など、多くの[プログラミン言語](#プログラミング言語)は命令型プログラミングをサポートしている。

### 宣言型プログラミング

**宣言型プログラミング**は、達成したい目的を記述し、その命令の実行手順については[コンパイラ](#コンパイラ)に任せるような言語。[プログラム](#プログラム)の本質がわかりやすく、記述がシンプルになるものの、複雑な処理を行うことはできない。[データベース](../../../development/database/_/chapters/database.md#データベース)に対する[クエリ](../../../development/database/_/chapters/sql.md#クエリ)を記述する[SQL](../../../development/database/_/chapters/sql.md#sql)は宣言型に準拠した言語である。

または、数学論理学に根ざした流れをくむ、[関数型プログラミング](#関数型プログラミング)や[論理プログラミング](#論理プログラミング)、[制約プログラミング](#制約プログラミング)を総称して宣言型プログラミングと呼ぶこともある。

### 構造化プログラミング

**構造化プログラミング**は、[プログラム](#プログラム)の処理手順を明確にし、判読性を向上させることを目的とした[プログラミングパラダイム](#プログラミングパラダイム)。一般的には、順次、[分岐](./control_flow.md#条件分岐)、[反復](./control_flow.md#反復)といった3つの[制御構文](./control_flow.md#制御フロー)によって処理の流れを記述する。また、[プログラム](#プログラム)を任意に分割した部分[プログラム](#プログラム)（[サブルーチン](./function.md#関数)とコードブロック）の階層的な組み合わせによる構造化も指している。[C言語](./programming_language.md#c言語)や[C++](./programming_language.md#c)、[Java](./programming_language.md#java)、[PHP](./programming_language.md#php)といった多くの言語は構造化プログラミングをサポートしている。

### オブジェクト指向プログラミング

**オブジェクト指向プログラミング**は、あらゆるものを[オブジェクト](./object_oriented.md#オブジェクト)として表現する[プログラミングパラダイム](#プログラミングパラダイム)。[プログラム](#プログラム)を手順ではなく、モノの作成と操作として見る考え方。[C++](./programming_language.md#c)や[Java](./programming_language.md#java)、[PHP](./programming_language.md#php)、[Python](./programming_language.md#python)などは[オブジェクト指向](./object_oriented.md#オブジェクト指向)を取り入れた[プログラミング言語](#プログラミング言語)に分類される。[オブジェクト指向](./object_oriented.md#オブジェクト指向)をサポートする[プログラミング言語](#プログラミング言語)は大抵の場合、[命令型プログラミング](#命令型プログラミング)との組み合わせで用いられる。

### 関数型プログラミング

**関数型プログラミング**は、数学的な関数を主軸として[プログラミング](#プログラミング)を行う[プログラミングパラダイム](#プログラミングパラダイム)。関数型プログラミングでは、**参照透過性**（入力が決まると出力が一意に決まり、他のデータへの[副作用](./function.md#副作用)がない）を満たした[関数](./function.md#関数)を使って組み立てた式が主役となる。[オブジェクト指向プログラミング](#オブジェクト指向プログラミング)では[状態](./object_oriented.md#プロパティ)と[振る舞い](./object_oriented.md#メソッド)を[オブジェクト](./object_oriented.md#オブジェクト)によって管理していたが、関数型プログラミングでは状態と振る舞いを切り離している。ClojureやElang、Haskell、Scala、[Rust](./programming_language.md#rust)などは関数型プログラミングに分類される。

### 論理プログラミング

**論理プログラミング**は、あらかじめ事実やルールを定義しておき、最終的に問い合わせを行うことで解を得る[プログラミングパラダイム](#プログラミングパラダイム)。既知の理論を基にして、新たな仮説を説明できるかといったことが[プログラム](#プログラム)的に証明できる。代表的な論理プログラミング言語には、Prologがある。

### 制約プログラミング

**制約プログラミング**は、[変数](./variable.md#変数)間の関係を制約という形で記述し、**制約ソルバー**により与えた制約を満たす解を探索する[プログラミングパラダイム](#プログラミングパラダイム)。ツールキットのような形で[ライブラリ](../../../computer/software/_/chapters/package.md#ライブラリ)として各[プログラミング言語](#プログラミング言語)に提供されている場合が多く、代表的な[プログラミング言語](#プログラミング言語)と言えるものは少ない。

### マルチパラダイム言語

**マルチパラダイム言語**は、複数の[プログラミングパラダイム](#プログラミングパラダイム)に対応する[プログラミング言語](#プログラミング言語)の総称。[命令型プログラミング](#命令型プログラミング)や[構造化プログラミング](#構造化プログラミング)といった[パラダイム](#プログラミングパラダイム)は、他の[パラダイム](#プログラミングパラダイム)と組み合わせて用いられることが多い。


## プログラムの基礎と文法

### 文法

**文法**（**シンタックス**）は、各[プログラミング言語](#プログラミング言語)が規定している[プログラム](#プログラム)の書き方。文法が誤っていると、[コンパイラ](#コンパイラ)が[ソースコード](#ソースコード)を正しく解析できず、[エラー](#エラー)となる。[プログラミング言語](#プログラミング言語)の文法は、[自然言語](#プログラミング言語)の文法に比べて非常に厳密であるため、[プログラミング](#プログラミング)を行うときには常に正しい文法を心掛けなくてはならない。

文法は[プログラミング言語](#プログラミング言語)ごとに様々であるが、例として次のようなものがある。

- [プログラム](#プログラム)は半角アルファベットと数字、各種記号によって記述する
- `{` と `}` は対応しており、 `{}` で囲まれた部分は処理のひとつのまとまりを表す
- ひとつの文は `;` で終わらなければならない
- [プログラム](#プログラム)は上から下へと順次実行される

### コメント

**コメント**（注釈）は、[ソースコード](#ソースコード)中で[コンパイル](#コンパイル)時に無視される部分。[プログラム](#プログラム)の内容や補足を記述したり、一時的に無効化したい[プログラム](#プログラム)を**コメントアウト**したりといった使い方がある。

一般的な[プログラミング言語](#プログラミング言語)におけるコメントの書き方は次のようになっている。

```
; アセンブリ言語などで用いられる1行のコメント

// 高水準言語でよく用いられる1行のコメント

/*

    高水準言語でよく用いられる
    複数行に対応したコメント

*/
```

### エラー

**エラー**は、[コンパイル](#コンパイル)時・実行時に[プログラム](#プログラム)が正常に処理を続行できなくなるような致命的な問題の発生により中断・終了されることを指す。[シンタックス](#文法)の誤りや、[アルゴリズム](./algorithm.md#アルゴリズム)の[バグ](#バグ)等により発生する。

代表的なエラーには次のようなものがある。

- **ゼロ除算**（整数を0で割るような[演算](./operation.md#演算)を試みた）
- [配列](./data_type.md#配列)の要素数より大きい[インデックス](./data_type.md#配列)にアクセスを試みた
- 存在しない[ファイル](../../../computer/software/_/chapters/file_system.md#ファイル)を参照しようとした
- ユーザが想定外の入力を行った
- [データベース](../../../development/database/_/chapters/database.md#データベース)や[サーバ](../../../computer/_/chapters/computer.md#サーバ)への接続に失敗した

### 例外

**例外**は、[プログラミング言語](#プログラミング言語)が想定可能な[エラー](#エラー)のことで、例外発生時の処理を[ソースコード](#ソースコード)中に記述しておき、[プログラム](#プログラム)が止まらないようにすることができる。[プログラムが](#プログラム)想定できない[エラー](#エラー)が発生した場合は、[プログラム](#プログラム)が[OS](../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)によって強制的に終了される。

### バグ

**バグ**は、[プログラム](#プログラム)上の誤りや不具合。[文法](#文法)上の[エラー](#エラー)や、[エラー](#エラー)ではないものの[プログラム](#プログラム)が仕様通りに動作しないような欠陥のことをいう。

### デバッグ

**デバッグ**は、[プログラム](#プログラム)の[バグ](#バグ)を見つけて修正すること。

### 未定義動作

**未定義動作**は、[プログラミング言語](#プログラミング言語)の仕様において動作が予測できないと規定されている[プログラム](#プログラム)を実行した結果のことで、[C言語](./programming_language.md#c言語)や[C++](./programming_language.md#c)といった言語ではいくつかの未定義動作が定められている。未定義動作を設けることにより、パフォーマンスの高い[コンパイラ](#コンパイラ)を作成することが容易になる。一方で、未定義動作は[プログラム](#プログラム)の[バグ](#バグ)となる可能性が高く、実行環境によって結果が変わることもあるため発見が難しい。

### 識別子

**識別子**（**シンボル**）は、[ソースコード](#ソースコード)中でデータや処理のまとまりに対して[プログラマ](#プログラマ)がつける名前。識別子には、そのデータの性質や状態、役割を表す名前をつけるのが一般的。

識別子の命名に関しては、各[プログラミング言語](#プログラミング言語)ごとにルールが存在する。次に示すのは、多くの[プログラミング言語](#プログラミング言語)に共通しているルールの一部。

- [変数](./variable.md#変数)名にはアルファベット、数字、アンダースコアが使用できる
- アルファベットの大文字と小文字は区別される
- 1文字目はアルファベットかアンダースコアが使用できる（数字から始まる[変数](./variable.md#変数)名は使用できない）
- [予約語](#予約語)は使用できない（[予約語](#予約語)を含む[変数](./variable.md#変数)名は使用できる）
- 識別子中にスペースを含むことはできない（複数の単語からなる識別子において、単語間にスペースを用いるなど）

[プログラミング](#プログラミング)において良い識別子を命名することは、[ソースコード](#ソースコード)の可読性やメンテナンス性を向上させるために非常に重要である。言語ごとのコミュニティや開発チームごとのルール・文化に従い、一般的な単語を用いた、誰が見てもわかりやすい識別子を心掛けるべきである。

### 予約語

**予約語**は、各[プログラミング言語](#プログラミング言語)において規定されている、[識別子](#識別子)として利用できない文字列。データの[型](./data_type.md#型)や[制御構文](./control_flow.md#制御フロー)に用いられる単語などがこれにあたり、[プログラム](#プログラム)の[構文解析](../../../basics/information_theory/_/chapters/compiler_theory.md#構文解析)の妨げになるなどの理由から使用できなくなっている。また、将来的に[プログラミング言語](#プログラミング言語)に取り入れられる予定の機能に関するキーワードも予約語となっている場合もある。

予約語ではなくとも、他の[プログラミング言語](#プログラミング言語)で予約語になっているものや、将来的に使用できなくなる可能性のある文字列は[識別子](#識別子)としてふさわしくないので、避けた方がよい。

以下に示すのは、[C言語](./programming_language.md#c言語)の予約語の一部。

```c
auto, break, case, char, const, continue, default, do, double, else, enum,
extern, float, for, goto, if, int, long, register, return, signed, sizeof,
...
```


## プログラミングを始める

[プログラミング](#プログラミング)を始めるために必要なものは、各種[プログラミング言語](#プログラミング言語)の実行環境と、[プログラム](#プログラム)を記述するためのテキストエディタのみで、基本的には特別なものは必要としない。**実行環境**は、[プログラム](#プログラム)を[コンパイル](#コンパイル)して実行するための環境。**テキストエディタ**には、[プログラミング](#プログラミング)を便利に行うための補助機能を提供するものや、実行環境が搭載された**統合開発環境**など様々なものがあるが、PCにデフォルトでインストールされているようなもの（[Windows](../../../computer/software/_/chapters/operating_system.md#windows)のメモ帳など）でも問題ない。代表的なテキストエディタには次のようなものがある。

- [サクラエディタ](https://sakura-editor.github.io/)
- [秀丸エディタ（有料）](https://hide.maruo.co.jp/software/hidemaru.html)
- [Visual Studio Code](https://code.visualstudio.com/)
- [Neovim](https://neovim.io/)
