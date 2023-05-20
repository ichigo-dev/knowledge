# 『Rust』ノート

（最終更新： 2023-05-20）


## 目次
1. [Rust](#rust)
	1. [メモリ安全](#メモリ安全)
	1. [並列プログラミング](#並列プログラミング)
	1. [テスト機構](#テスト機構)
	1. [Rustにできないこと](#rustにできないこと)
1. [Rust開発ツール](#rust開発ツール)
	1. [rustup](#rustup)
	1. [Cargo](#cargo)
	1. [rustc](#rustc)
	1. [rustdoc](#rustdoc)


## Rust

**Rust**は、システムプログラミングのための言語であり、[C](../../../_/chapters/programming_language.md#c言語)や[C++](../../../_/chapters/programming_language.md#c)といった言語を置き替える次世代の[プログラミング言語](../../../_/chapters/programming.md#プログラミング言語)となることを目指している。

### メモリ安全

システムプログラミングにおいて一般的に用いられている[C](../../../_/chapters/programming_language.md#c言語)や[C++](../../../_/chapters/programming_language.md#c)といった言語においては、[未定義動作](../../../_/chapters/programming.md#未定義動作)を生じる可能性がある。[プログラマ](../../../_/chapters/programming.md#プログラマ)は、[未定義動作](../../../_/chapters/programming.md#未定義動作)を避けるためのルールを意識しながら[プログラム](../../../_/chapters/programming.md#プログラム)を作成する必要がある。

[Rust](#rust)は、[ダングリングポインタ](../../../_/chapters/memory_management.md#ダングリングポインタ)やメモリの[多重フリー](../../../_/chapters/memory_management.md#ダブルフリー)、nullポインタの参照解決といった[未定義動作](../../../_/chapters/programming.md#未定義動作)を[コンパイル](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイル)時に検出することができる。そのため、[コンパイラ](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイラ)のチェックをパスした[プログラム](../../../_/chapters/programming.md#プログラム)は、[未定義動作](../../../_/chapters/programming.md#未定義動作)を引き起こさないことが保証されている。これを実現するために、[Rust](#rust)には多くの制約が設けられており、学習コストは他の[プログラミング言語](../../../_/chapters/programming.md#プログラミング言語)と比較しても高いといえる。

### 並列プログラミング

並列プログラミングは、[プログラム](../../../_/chapters/programming.md#プログラム)の実行[プロセス](../../../../computer/software/_/chapters/operating_system.md#プロセス)上で複数の[スレッド](../../../../computer/software/_/chapters/operating_system.md#スレッド)を用いて、高速動作を実現する方法。シングルスレッドコードでは起こりえないような[未定義動作](../../../_/chapters/programming.md#未定義動作)に対しても注意を払う必要があり、一般的に非常に難しいことが知られている。

[Rust](#rust)は、安全性のために設けられた制約の副産物として、並列プログラミングにおけるデータ競合を避けられるようになっている。読み込み専用であればデータを[スレッド](../../../../computer/software/_/chapters/operating_system.md#スレッド)間で共有することが許されているが、変更される可能性があるデータに対しては、排他ロックや条件変数、チャネル、アトミック変数といった同期機構を用いてのみアクセスすることができる。

### 実行速度

[C++](../../../_/chapters/programming_language.md#c)には**ゼロオーバヘッド原則**という、実行することに対して余計なコード（ガベージコレクションなど）でCPUを消費しない、という考え方がある。

[Rust](#rust)には**ゼロコスト抽象化**という、抽象化の処理に最小限のコストしか払わないという考え方がある。例えば、[Rust](#rust)の抽象型であるtraitは[コンパイル](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイル)時に静的ディスパッチ（具体的な型に変換）されるため、実行時の[オーバヘッド](../../../../system/_/chapters/system_performance_evaluation.md#オーバヘッド)がない。

[Rust](#rust)は[C++](../../../_/chapters/programming_language.md#c)と同様に、[プログラマ](../../../_/chapters/programming.md#プログラマ)が[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)の能力を最大限活用できるような[コード](../../../_/chapters/programming.md#ソースコード)を書くことをサポートする言語である。

### テスト機構

[Rust](#rust)には[プログラミング言語](../../../_/chapters/progamming.md#プログラミング言語)自体にテスト機構が組み込まれており、ユニットテストを実行するための環境が整えられている。

### Rustにできないこと

[Rust](#rust)には[コンパイラ](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイラ)が[メモリ安全](#メモリ安全)性を保証するための仕組み（[所有権](./ownership_and_move.md#所有権)など）が備わっており、高度な操作をしようとしたときに[C++](../../../_/chapters/programming_language.md#c)ほどの柔軟性がない、という欠点がある。例として、[C++](../../../_/chapters/programming_language.md#c)では[代入演算](../../../_/chapters/operation.md#代入演算)子をオーバロードして特別なコピーやmove[コンストラクタ](../../../_/chapters/object_oriented.md#コンストラクタ)を定義することができるが、[Rust](#rust)ではそのようなことはできない。このように[C++](../../../_/chapters/programming_language.md#c)では、ごく普通に見える[コード](../../../_/chapters/programming.md#ソースコード)の中で様々なトリックを用いることができる。


## Rust開発ツール

### rustup

**rustup**は、[Rust](#rust)のインストールやアップデートを行うためのツール。 `rustup` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)で、[Rust](#rust)の[コンパイル](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイル)に必要なツールチェーンやクロスコンパイル環境を構築することができる。

- [rustupのインストール](https://rustup.rs/)

`rustup install` は、Rustのツールチェーンをインストールする[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。配布用チャネルとして、安定リリース版の `stable` 、次期バージョンに向けたベータ版の `beta` 、開発版の `nightly` を指定できる。

```sh
# 安定版のRustツールチェーンをインストール
$ rustup install stable
```

`rustup update` は、rustup自身や[Rust](#rust)をアップデートする[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# rustup自身のアップデートがないかをチェックし、ある場合はアップデート
$ rustup self update

# Rustのアップデートがないかをチェックし、ある場合はアップデート
$ rustup update
```

`rustup component` は、開発時に便利なツールをコンポーネントとして管理する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# コンポーネントを追加
$ rustup component add <name>
```

### Cargo

**Cargo**は、[Rust](#rust)のビルドシステムと[パッケージマネージャ](../../../../computer/software/_/chapters/package.md#パッケージマネージャ)を兼ね備えたツール。Cargoを用いることで、[ソースコード](../../../_/chapters/programming.md#ソースコード)のビルドや、依存している[パッケージ](../../../../computer/software/_/chapters/package.md#パッケージ)のダウンロードなどが[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)ひとつで完結するようになる。

`cargo new` および `cargo init` は、[Rust](#rust)のプロジェクトを作成するための[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。 `cargo new` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)は引数にプロジェクト名を指定することで、新しいプロジェクトの[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)内に開発のための環境が作成される。一方、 `cargo init` は[カレントディレクトリ](../../../../computer/software/_/chapters/file_system.md#カレントディレクトリ)を[Rust](#rust)プロジェクトとするための[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。 `--bin` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)をつけることで[バイナリ](../../../../basics/_/chapters/computer_and_number.md#バイナリ)向けのプロジェクト、 `--lib` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)をつけることで[ライブラリ](../../../../computer/software/_/chapters/package.md#ライブラリ)向けのプロジェクトとして初期化される（デフォルトは `--bin` ）。

```sh
# 新しいプロジェクトディレクトリの中に開発の環境を作成
$ cargo new <name>

# カレントディレクトリを新しいプロジェクトとして初期化
$ cargo init

# 新しいライブラリ向けのプロジェクトを作成
$ cargo new <name> --lib
```

`cargo build` は、Cargoが管理するプロジェクトをビルドする[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。バイナリプロジェクトの場合、 `target/debug/<name>` に[実行可能ファイル](../../../../computer/software/_/chapters/file_system.md#実行ファイル)が作成される。 `--release` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、リリース向けに最適化したビルドを実行することができる。

```sh
# デバッグ用にビルド
$ cargo build

# リリース用にビルド
$ cargo build --release
```

`cargo run` は、Cargoが管理するプロジェクトをビルドし、実行を行う[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ cargo run
```

`cargo check` は、Cargoが管理するプロジェクトが[コンパイル](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイル)可能かどうかを確かめる[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。 `cargo build` を使用した場合よりも早く確認できるため、開発に便利。

```sh
$cargo check
```

### rustc

**rustc**は、[Rust](#rust)の[コンパイラ](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイラ)。ビルドツールとして[Cargo](#cargo)を利用しているプロジェクトでは、内部で自動的にrustcが実行されている。クロスコンパイルが可能で、[Windows](../../../../computer/software/_/chapters/operating_system.md#windows)、[Linux](../../../../computer/software/_/chapters/operating_system.md#linux)、[macOS](../../../../computer/software/_/chapters/operating_system.md#macos)向けの[実行ファイル](../../../../computer/software/_/chapters/file_system.md#実行ファイル)他、[Android](../../../../computer/software/_/chapters/operating_system.md#android)や[iOS](../../../../computer/software/_/chapters/operating_system.md#ios)で動作する[ライブラリ](../../../../computer/software/_/chapters/package.md#ライブラリ)をホストマシンで出力することができる。

[Rust](#rust)で書かれた[プログラム](../../../_/chapters/programming.md#プログラム)を指定することで、それを[コンパイル](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイル)することができる。

```sh
$ rustc <path>
```

### rustdoc

**rustdoc**は、[Rust](#rust)のドキュメンテーションツールで、[ソースコード](../../../_/chapters/programming.md#ソースコード)中のドキュメンテーションコメントを整形してHTMLを生成する。また、ドキュメンテーションコメント中に書かれた[Rust](#rust)[プログラム](../../../_/chapters/programming.md#プログラム)が正常に動作するかをテストすることができる。

[Rust](#rust)で書かれた[プログラム](../../../_/chapters/programming.md#プログラム)を指定することで、その[ソースコード](../../../_/chapters/programming.md#ソースコード)に対するHTMLドキュメントを生成することができる。

```sh
$ rustdoc <path>
```

[Cargo](#cargo)を用いたプロジェクトでは、次の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)でrustdocを使用することができる。

```sh
$ cargo doc
```
