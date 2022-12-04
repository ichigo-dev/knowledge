# プログラミングの基礎知識


## 目次

1. [プログラミングの概要](#プログラミングの概要)
1. [2進数とプログラミング](#2進数とプログラミング)
	1. [データの単位](#データの単位)
1. [プログラムが動く仕組み](#プログラムが動く仕組み)
	1. [処理装置](#処理装置)
	1. [記憶装置](#記憶装置)
	1. [入出力装置](#入出力装置)
	1. [メモリとアドレス](#メモリとアドレス)
	1. [メモリ上の領域](#メモリ上の領域)
	1. [プログラムの処理の実行](#プログラムの処理の実行)
	1. [プログラムの計算量](#プログラムの計算量)
1. [アセンブリ言語](#アセンブリ言語)
1. [高水準言語](#高水準言語)
	1. [コンパイラ言語](#コンパイラ言語)
	1. [インタプリタ言語](#インタプリタ言語)
	1. [JITコンパイラ](#jitコンパイラ)
	1. [スクリプト言語](#スクリプト言語)
1. [型システムによる分類](#型システムによる分類)
	1. [動的型付け言語](#動的型付け言語)
	1. [静的型付け言語](#静的型付け言語)
1. [プログラミングを始める](#プログラミングを始める)
	1. [プログラミングの準備](#プログラミングの準備)
	1. [プログラムの基礎文法](#プログラムの基礎文法)
	1. [コメント](#コメント)
	1. [識別子](#識別子)
	1. [予約語](#予約語)
	1. [命名規則](#命名規則)
1. [プログラミング言語の種類](#プログラミング言語の種類)
	1. [C言語](#c言語)
	1. [Java](#java)
	1. [Python](#python)
	1. [PHP](#php)
	1. [JavaScript](#javascript)


## プログラミングの概要

**コンピュータ**とは電気信号により計算を行う機械のことで、コンピュータの処理の手順や内容を示したものを**プログラム**と呼ぶ。**プログラム**を書く人を**プログラマ**といい、プログラムを実装することを**プログラミング**という。プログラムを記述する際には、様々な**プログラミング言語**が用いられる。目的となるタスクを解決するための処理手順のことを**アルゴリズム**といい、良いアルゴリズムで書かれたプログラムは良いプログラム（バグがなく、処理が高速）であるといえる。

プログラマには、世の中の様々なタスクや問題をプログラミングによって解決するという役割がある。毎日の単純作業のくり返しを減らしたり（表計算ソフトなど）、ミスが許されない場面で正確に動作することが保証されたシステムを利用したり（銀行のシステムや信号機など）、さらに人間には難しいタスクの解決を行ったり（AIによる異常検知など）と、プログラミングスキルの応用範囲は多岐にわたる。


## 2進数とプログラミング

コンピュータは様々な電子部品から構成されており、これらの電子部品が電気信号を高速で伝達することで処理を行っている。コンピュータの中で扱われる電気信号の種類は、**閾値**を超えているか超えていないかの2種類しかない。そのため閾値を超えていれば1、超えていなければ0という2つの数字でデータを表現することができる。このように0と1のみを用いて数字を表現する方法を**2進数**という。一方で、一般的に普及している0~9までの10つの数字を用いて数を表現する方法を**10進数**という。その他にも、**8進数**や**16進数**などは2進数との相互変換が容易であるため、コンピュータの世界で頻繁に利用されている。

先述の通り、コンピュータが理解して処理できる信号（**機械語**、マシン語）は2進数で表すことができるデータに限られるため、プログラムも最終的には2進数で記述しなければならない。例えば、次の2進数の羅列は標準出力に `Hello, world` を表示して終了するプログラムである。

```
00101110 01110000 10100001 11100001 01011101 00110011
00011111 10010000 00111111 11　
```

上の例からわかる通り、機械語で人間がプログラムを記述することは非常に複雑で可読性に乏しい。そのため、人間が理解しやすいようにプログラミング言語を用いてプログラムを作成し、これを機械語に変換してコンピュータが処理できるようにする。

次のC言語ソースコードは上の機械語コードと同じ処理を記述したプログラムで、こちらの方が人間が扱うのに適しているのは自明である。

```c
#include <stdio.h>

int main()
{
    printf("Hello, world\n");
    return 0;
}
```

### データの単位

コンピュータが扱うデータの最小単位は2進数の1桁（0か1か）となっており、これを**ビット**という単位で表す。また、8ビットのデータをまとめたものに対しては**バイト**という単位を用いる。


## プログラムが動く仕組み

### 処理装置

コンピュータの構成要素である**処理装置**（**プロセッサ**）は機械語の命令を実行するためのハードウェアである。**CPU**（**中央演算装置**）などがこれにあたり、インテルやAMDといったメーカーのものが一般的に流通している。プロセッサは実際に命令を処理するための**演算装置**や、演算装置をコントロールするための**制御装置**、命令やデータを格納するためのレジスタなどから構成される。プロセッサの性能はそのコンピュータの処理性能に大きく影響するため、プロセッサがプログラムの実行要件を満たしていない場合はプログラムを利用できない場合もある。

### 記憶装置

コンピュータの構成要素である**記憶装置**はプログラムやデータを保存しておくためのもので、役割によってレジスタやメモリ、ストレージなどに分類される。

**レジスタ**はプロセッサ内における記憶装置であり、論理回路が処理や演算に直接用いることからコンピュータの中でもっとも高速に動作する記憶装置である。しかし、保持できるデータの容量は少ない。プロセッサは他にも、直近に使用したデータや使用頻度の高いデータを保持しておくための**キャッシュメモリ**を持つものもある。

**メインメモリ**（**主記憶装置**）はCPUがアクセスできる記憶領域で、実行中のプログラムやそのプログラムが用いるデータなどを保持する。また、コンピュータの電源を落とすと保持していた内容が失われる**揮発性メモリ**のため、永続的に残したいデータについてはストレージに書き込む必要がある。プログラムの実行にかかわる部分なので高速に動作する**RAM**(Random Access Memory)が用いられることが多い。

**ストレージ**（**補助記憶装置**）はコンピュータの電源が切れても内容を保持し続ける**不揮発性メモリ**で、CPUから直接アクセスすることはできない。磁気ディスクを用いた**HDD**(Hard Disk Drive)やフラッシュメモリを用いた**SSD**(Solid State Drive)などの種類がある。また、ソフトウェアが焼かれたCDやゲームソフトは読み込み専用であるため**ROM**(Read Only Memory)と呼ばれる。

### 入出力装置

コンピュータの構成要素である**入出力装置**はプログラムに対してデータを入力したり、プログラムの結果やデータを外部に出力したりするためのものである。マウスやキーボードなどは入力装置にあたり、ディスプレイやプリンタなどは出力装置にあたる。

コンピュータに対して接続した周辺機器をコントロールするには**デバイスドライバ**と呼ばれるソフトウェアが必要となる。

### メモリとアドレス

プログラムがアクセスできる記憶装置はメモリであり、メモリ上のデータをレジスタに読み込んだり、レジスタの計算結果をメモリに書き出したりすることで処理を進める。メモリ上のどこにデータが存在するかを指し示すために、メモリ上には1バイトごとに**アドレス**が割り振られている。

![メモリとアドレス](/note/programming/images/memory_and_address.ja.jpg)

### メモリ上の領域

メモリは大きく4つの領域にわかれており、それぞれ役割が異なる。

**テキスト領域**は機械語にコンパイルされたプログラムが格納され、この領域の機械語命令が1行ずつプロセッサに読み込まれていくことでプログラムが実行される。

**静的領域**はグローバル変数などの静的変数が格納される。プログラムの開始から終了まで確保される固定的な領域。

**スタック領域**はローカル変数が格納される領域で、関数が実行されるときに一時的に利用される。プログラムが利用できるメモリ領域の最下部から順番に変数が積まれていき、使い終わった変数は順に取り除かれていく。プリミティブなデータ型などはスタック領域に直接格納され、オブジェクト型のデータなどはポインタがスタック領域に格納されて実体はヒープ領域に格納される。

**ヒープ領域**はプログラムが動的に確保と解放を繰り返すことができるメモリ領域で、コンパイル時にサイズが決まっていないような可変長のコンテナやオブジェクト型データの実体が格納される。

スタック領域に格納されたデータは関数の実行が終わると自動的に解放されるが、ヒープ領域に格納したデータは別の手段により解放する必要がある。ヒープ領域のメモリを解放する手段は各プログラミング言語によって異なるが、C言語やC++ではプログラマが明示的にソースコード中で確保・解放を宣言する必要があり、そのほかのほとんどのプログラミング言語では**ガベージコレクション**（GC）と呼ばれるメモリ管理用のプログラムを利用する。ガベージコレクションを用いないC言語などは、ガベージコレクションを用いる言語に比べて不要なメモリ領域を使用せず高速に動作させることができるが、プログラマがメモリの解放を忘れると**メモリリーク**というバグに繋がるなど、高いプログラマのスキルが求められる。また、Rustではガベージコレクションを用いずにメモリの解放に関する一定のルールを設けることで、プログラマをメモリ管理の責務から解放している。

### プログラムの処理の実行

プロセッサが実行する1つの命令パターンを2進数で表したものを**オペコード**といい、その操作対象を**オペランド**という。

実行されるプログラムはストレージからメモリに読み込まれ、CPUが機械語を解釈しながら処理を実行していく。次に処理する命令が書かれているアドレスを保持ししておくレジスタを**プログラムカウンタ**といい、プログラムカウンタが示すアドレスの命令を**命令レジスタ**に読み込むことで処理が実行される。プログラムカウンタから機械語コードを読み取ることを**フェッチ**、その意味を解析することを**デコード**、解析した命令を実行することを**エグゼキュート**、結果をメモリに書き込むことを**ストア**という。フェッチの後にプログラムカウンタを1つ進めることで、命令が順次実行されていくという流れとなる。

### プログラムの計算量

プログラムがどれほど高速に動作するかを表す指標として、**計算量**（**オーダー**）がある。これは、処理対象となるデータの数が増えたときに、処理にかかる時間や必要とするメモリ容量がどれほど増えるのかを指しており、**オーダー記法**（**ランダウの記号**）を用いて表される。**時間計算量**は処理時間の目安を見積もるための指標で、**空間計算量**はどれほどのメモリを消費するかを見積もるための指標。

代表的な計算量には以下のような種類があり、上に記載したものほど処理が高速である。

| オーダー記法  | 用語                     | 概要                                                           | 例                                                                              |
|---------------|--------------------------|----------------------------------------------------------------|---------------------------------------------------------------------------------|
| $O(1)$        | 定数時間                 | 処理時間がデータ量に依存しない                                 | 配列要素へのアクセス、ハッシュテーブルによるデータ検索、連結リストへの追加/削除 |
| $O(\log{N})$  | 対数時間                 | データ量が増えても計算量がほとんど増えない                     | ソート済み配列の二分探索                                                        |
| $O(N)$        | 線形時間                 | forループ1回分で、データ量の分だけ時間がかかる                 | ソートしていない配列の探索                                                      |
| $O(N\log{N})$ | 準線形時間、線形対数時間 | 線形時間に比べるとやや遅い                                     | クイックソート、マージソート、ヒープソート                                      |
| $O(N^2)$      | 二乗時間                 | 要素から全ての組み合わせについて調べるようなアルゴリズム       | 挿入ソート、バブルソート                                                        |
| $O(N^3)$      | 多項式時間               | かなり重いので場合によっては改善が必要                         | 行列計算                                                                        |
| $O(k^N)$      | 指数時間                 | 要素を取り出す時のすべての組み合わせを調べるようなアルゴリズム | ルービックキューブの総当たりによる解法                                          |
| $O(N!)$       | 階乗時間                 | $N$ の階乗に比例した時間                                        | 巡回セールスマン問題の総当たりによる解法                                        |


## アセンブリ言語

オペコードに1対1で対応した英単語や記号で表記される命令ことを**ニーモニック**といい、これを組み合わせて記述するプログラミング言語を**アセンブリ言語**という。アセンブリ言語を機械語に変換する操作を**アセンブル**といい、この操作を行うソフトウェアを**アセンブラ**という。プロセッサが異なれば使用できるアセンブリ言語も異なるので注意が必要である。

アセンブリ言語の一種である**Netwide Assembler**を用いて `Hello, world` を出力するプログラムを以下に示す。

```asm
section .data
    msg db      "Hello, world"      ; データセクションで文字列を宣言

section .text
    global _start                   ; プログラムのエントリポイントとして_startを呼び出し

_start:
    mov     rax, 1                  ; sys_writeシステムコールをraxレジスタにセット
    mov     rdi, 1                  ; 第一引数（ファイルディスクリプタ、1は標準出力）をrdiレジスタにセット
    mov     rsi, msg                ; 第二引数（出力したい文字列）をrsiレジスタにセット
    mov     rdx, 12                 ; 第三引数（データのサイズ）をrdxレジスタにセット
    syscall                         ; システムコールを発行
    mov     rax, 60                 ; exitシステムコールをraxレジスタにセット
    mov     rdi, 0                  ; 第一引数（終了コード、0は正常終了）をrdiレジスタにセット
    syscall                         ; システムコールを発行
```

プログラミング言語の中でも機械語に近い言語を**低水準言語**（低級言語）と呼び、アセンブリ言語もこれに分類される。機械語に近いため高速に動作する一方で、プログラムが長くなりやすく柔軟性に乏しいことから、性能を最大限引き出す目的などがない限りは後述の高水準言語が用いられることがほとんどとなる。


## 高水準言語

世の中の多くのプログラミング言語は、機械語を意識することなく記述できるようになっている。このように、低水準言語に比べて抽象度が高く人間にとって分かりやすい言語のことを**高水準言語**（高級言語）と呼ぶ。高水準言語には、プロセッサに依存したプログラムを書かなくてよい（機械語への変換の段階で様々なプロセッサに対応させることができる）というメリットもある。代表的な高水準言語には、C系言語（C, C#, C++）、Java、Python、PHP、JavaScriptなどがある。高水準言語には広い用途で利用される**汎用プログラミング言語**と、限られた分野で活躍する**専用プログラミング言語**がある。それぞれの言語には得手不得手があり、C言語は処理速度が高速であることからCPUやサーバなどの低レイヤの実装に向いており、JavaScriptは一般的なWebブラウザで動作することからWebコンテンツ向けとなる。

高水準言語で書かれたプログラムをコンピュータに処理させるためには、アセンブリ言語と同じく機械語への変換が必要となる。高水準言語を機械語へ変換する操作を**コンパイル**といい、この操作を行うソフトウェアを**コンパイラ**という。高水準言語は、ソースコードをコンパイルして実行する際の方式によってコンパイラ言語とインタプリタ言語に分類することができる。

### コンパイラ言語

**コンパイラ言語**では、プログラムの処理を実行する前にあらかじめ機械語への変換を行っておくという方式を用いる。実際にプログラムを利用する際には、機械語に変換された**実行ファイル**を実行する。事前に変換処理を済ませているため、実行前に不具合を発見できる、インタプリタ言語に比べて実行速度が高速であるといった利点がある。一方で、実行時に起こりうる様々なパターンに対応できるようにプログラミングする必要があり、スクリプト言語と比較しても難易度が高い言語が多い。コンパイルした実行ファイルはプロセッサに依存するため、マルチプラットフォームに対応できないという欠点もある。

代表的なコンパイラ言語としては、C言語やGo, Rustなどがある。

### インタプリタ言語

**インタプリタ言語**では、プログラムの実行時にソースコードを1行ずつコンパイルしながら処理を進めるという方式を用いる。実行時に毎回コンパイル作業が必要となるため、実行速度はコンパイラ言語に比べて遅い傾向にある。しかし、コンパイルを行わずにすぐに実行結果を確認できるためデバッグが容易で、実装にかかるコストが小さくなるという利点がある。

代表的なインタプリタ言語としては、PythonやPHP、JavaScriptなどがある。

### JITコンパイラ

**JITコンパイラ**では、ソースコードを環境に依存しない**中間コード**（**バイトコード**）に変換しておき、実行時にバイトコードをコンパイルして実行する方式を撮る。実行時にコンパイルするという点ではインタプリタ方式と似ているが、インタプリタ方式に比べて高速な動作を実現できる。コンパイラ言語では直接機械語のコードを生成するため環境に対する依存が発生するが、この方式を採るプログラミング言語ではプログラムの実行機である**ランタイム**が利用できる環境であればどこでも実行できるというメリットがある。

JITコンパイラを用いる代表的な言語としてはJavaがあげられる。

### スクリプト言語

プログラミング言語のうち、比較的容易に記述・実行ができるものを**スクリプト言語**という。定義は曖昧であるが、インタプリタ言語の多くはスクリプト言語に分類され、PythonやJavaScript、PHP、Rubyなどがこれに含まれる。


## 型システムによる分類

プログラミングにおいては使用するデータの型（数値、文字列など）を意識する必要がある。データの型を実行時に決定するか、実行前に決定するかによって、プログラミング言語を動的型付け言語と静的型付け言語に分類することができる。

### 動的型付け言語

**動的型付け言語**では、実行時にデータの型を決定する方式を用いる。プログラム中にデータ型を記述する必要がないため、ソースコードがシンプルになり読みやすくなる。ただし、データが読み込まれるたびにそのデータの型を推測する必要があるため、実行速度は静的型付け言語に比べて遅い傾向にある。

また、実行時に型が決定するという抽象化の性質から、ある関数の引数として同じインタフェースを持つ異なるオブジェクトを取ることができるという特徴があり、これを利用したプログラミングスタイルを**ダックタイピング**という。

### 静的型付け言語

**静的型付け言語**では、実行前にデータの型を決定する方式を用いる。コンパイラ言語の多くは静的片付け言語となっている。プログラム中で使用するデータに対して、そのデータをどの型として扱うかを明示（**型注釈**）する必要がある。事前にデータの型を確定させておくことで、想定外のデータに対して処理が行われるような不具合を防ぐことができる。

Rustのような新しい言語では、ソースコード中に型を明示せずとも文脈から自動的に型を決定する**型推論**の機能が搭載されているものもあり、記述量が抑えられるという動的型付け言語の利点を享受することができる。


## プログラミングを始める

### プログラミングの準備

プログラミングを始めるために必要なものは、各種プログラムを実行するための**実行環境**と、プログラムを記述するための**テキストエディタ**のみで、基本的には特別なものは必要としない。実行環境とは、プログラマが記述したプログラムをコンピュータが処理可能な機械語に変換する工程を行うための環境のこと。テキストエディタにはプログラミングを便利に行うための補助機能が搭載されたものなど様々な種類があるが、PCに最初からインストールされている（Windowsのメモ帳など）ようなものでも問題ない。代表的なテキストエディタには次のようなものがある。

- [サクラエディタ](https://sakura-editor.github.io/)
- [秀丸エディタ（有料）](https://hide.maruo.co.jp/software/hidemaru.html)
- [Visual Studio Code](https://code.visualstudio.com/)
- [Neovim](https://neovim.io/)

### プログラムの基礎文法

プログラムを作成する際には、各プログラミング言語が規定する**文法**（シンタックス）を守らなければならない。これはコンパイラがソースコードを解析する際に、誤った文法で記されたプログラムがあると機械語に変換できなくなるためである。プログラミング言語の文法は人間が使う**自然言語**に比べて非常に厳密であるため、常に正しい文法を心掛ける必要がある。

文法には言語による差異はあるが、例として次のようなものがあげられる。

- プログラムは半角アルファベットと数字、各種記号を用いて記述する
- `{` と `}` は対応しており、 `{}` で囲まれた部分は1つの処理のまとまりを表す
- ひとつの文は `;` で終わらなければならない
- 基本的には、プログラムは上から下へと順次実行される

### コメント

ソースコード中には処理の内容を分かりやすくするために**コメント**（注釈）を記すことができる。コメントはコンパイル時に無視されるため、処理の内容やデータに関する情報などを自然言語を用いて示すことが可能である。

一般的なプログラミング言語におけるコメントの書き方は次のようになっている。

```
; アセンブリ言語などで用いられる1行のコメント

// 高水準言語でよく用いられる1行のコメント

/*
    高水準言語でよく用いられる
    複数行に対応したコメント
*/
```

### 識別子

ソースコード中では、データや処理のまとまりなどにプログラマが任意の名前（**識別子**）をつけるができる。識別子には、そのデータの性質や状態、役割を表す名前を用いることが一般的。

識別子の命名に関しては、各プログラミング言語ごとにルールが存在する。次に示すのは、多くのプログラミング言語で共通しているルールである。

- 変数名にはアルファベット、数字、アンダースコアが使用できる
- アルファベットの大文字と小文字は区別される
- 1文字目はアルファベットかアンダースコアのみ使用できる（数字から始まる変数名は使用できない）
- 予約語は使用できない
- 複数の単語からなる識別子において、単語間にスペースを用いることはできない

プログラミングにおいて良い識別子を命名することは、ソースコードの保守性を高める意味で非常に重要である。開発チームごとのルールや文化は存在するが、よく使われる単語を用いた、短すぎず長すぎない、誰が見ても分かりやすい識別子を心がけるべきである。

### 予約語

**予約語**とは各プログラミング言語で規程されている、識別子として利用できない文字列のことである。データの型や制御構文に用いられる単語などがこれにあたり、プログラムの構文解析の妨げになる可能性があるため使用できない。また、将来的にプログラミング言語に取り入れられる予定の機能に関するキーワードも予約語となっている場合がある。

予約語でなくても、他のプログラミング言語で予約語になっているものや将来的に使用できなくなる可能性のある文字列は識別子としてふさわしくないため避けるべきである。

以下に示すのはC言語における予約語の一部。

```c
auto, break, case, char, const, continue, default, do, double, else, enum,
extern, float, for, goto, if, int, long, register, return, signed, sizeof,
...
```

### 命名規則

**命名規則**とは識別子の命名に関する規則である。プログラミング言語の仕様として定められているものではなく、開発チーム内などで識別子に一貫性を持たせるために設けることが多い。また、各プログラミング言語によっても文化が異なるため、どの命名規則を用いるのが正しいといった正解はない。1つのプログラム中で複数の命名規則を組み合わせることも多い（変数はスネークケース、関数はキャメルケースを用いるといった具合）。

特に複数の単語からなる識別子を使用する場合には、それらの単語の区切りをどのように判別するか（識別子にスペースは使用できないため）によって分類することができる。

**パスカルケース**ではすべての単語の1文字目を大文字で始め、単語をそのままつなげる。

```
User
UserName
UpdateUserName
```

**キャメルケース**では2つ目以降の単語の1文字目を大文字で始め、単語をそのままつなげる。

```
user
userName
updateUserName
```

**スネークケース**ではすべての単語を小文字にし、単語同士をアンダースコアでつなげる。

```
user
user_name
update_user_name
```

**アッパーケース**ではすべての単語を大文字にし、単語同士をアンダースコアでつなげる。

```
USER
USER_NAME
UPDATE_USER_NAME
```

**ケバブケース**ではすべての単語を小文字にし、単語同士をハイフンでつなげる。

```
user
user-name
update-user-name
```

**ハンガリアン記法**では識別子の先頭や末尾にデータの性質を表すプレフィックス/サフィックスをつける。


## プログラミング言語の種類

### C言語

**C言語**は1972年に開発された汎用プログラミング言語で、高水準言語であるがハードウェアよりの記述が可能な低水準言語のような特徴も持つ。コンパイラ言語・静的型付け言語に分類される。OSやデバイスドライバの開発など、あらゆる分野で利用されている。習得難易度はスクリプト言語に比べて高く、限られたハードウェア資産で効率的に実行できるプログラムが必要となるケースに適している。

```c
#include <stdio.h>

int main()
{
    int input_num;
    printf("> ");
    scanf("%d", &input_num);

    if( input_num % 2 == 0 )
    {
        printf("It is an even number : %d\n", input_num);
    }
    else
    {
        printf("It is an odd number : %d\n", input_num);
    }

    return 0;
}
```

C言語の機能や特徴を継承しつつ表現力を高めたC++では、オブジェクト指向言語で用いられるクラスの機能を取り入れている。他にも、テンプレートや演算子のオーバーロードなど、豊富な機能を持つ。

C#もC言語に影響を受けて開発されたプログラミング言語であり、CやC++と比較して様々な制限や改良が加えられている。Windowsの.NET Framework上で開発・実行されるプログラミング言語であるが、中間言語にコンパイルすることでマルチプラットフォームでの実行にも対応することができる。

### Java

**Java**はCに影響を受けた汎用プログラミング言語。静的型付け言語に分類され、メモリ管理にはガベージコレクションを用いる。様々なプラットフォームで実行できるようにJITコンパイラ方式を用いており、Javaプログラムを実行するためのソフトウェアを**JVM**（Java仮想マシン）という。また、C++と同じくオブジェクト指向を取り入れた言語でもある。非常に人気が高く、世界中で最も使用されているプログラミング言語のひとつである。

```java
import java.util.Scanner;

public class Oddeven
{
    public static void main( String[] args )
    {
        Scanner scanner = new Scanner(System.in);
        System.out.print("> ");
        int input_num = scanner.nextInt();

        if( input_num % 2 == 0 )
        {
            System.out.println("It is an even number : " + input_num);
        }
        else
        {
            System.out.println("It is an odd number : " + input_num);
        }

        scanner.close();
    }
}
```

### Python

**Python**はインタプリタ方式の汎用プログラミング言語。動的型付け言語に分類され、メモリの管理にはガベージコレクションを用いる。可読性が高く、記述が容易であるためプログラミングの入門にも適しており、スピード感のある開発に用いられることが多い。機械学習分野のライブラリが充実いるほか、Web開発にも用いられる。

```python
input_num = int(input("> "))

if ( input_num % 2 ) == 0:
    print("It is an even number : {0}".format(input_num))
else:
    print("It is an even number : {0}".format(input_num))
```

### PHP

**PHP**は動的なWebサイトを作成するためのツールから派生したスクリプト言語。Web開発に特化しており、HTMLに埋め込むような記法を用いることができるという特徴がある。そのほかにも、学習コストが低い点やデータベースアクセスを容易に行えるという強みもある。

```php
<?php
    $input_num = $_POST["input_num"];

    if( $input_num % 2 === 0 )
    {
        echo("It is an even number : " . $input_num);
    }
    else
    {
        echo("It is an odd number : " . $input_num);
    }
?>
```

### JavaScript

**JavaScript**は一般的なWebブラウザ上で実行されるプログラミング言語。インタプリタ言語・動的型付け言語に分類される。Webページ上のコンテンツに動きを与える目的でよく用いられており、近年ではサーバサイドにおける実行環境の登場や**SPA**(Single Page Application)の普及により使用機会が増えている。習得難易度は比較的低く、ブラウザさえあれば実行環境が整うという手軽さもメリットのひとつである。

```javascript
let input_num = document.getElementById('#input_num').value;

if( input_num % 2 === 0 )
{
    console.log("It is an even number : " + input_num);
}
else
{
    console.log("It is an even number : " + input_num);
}
```