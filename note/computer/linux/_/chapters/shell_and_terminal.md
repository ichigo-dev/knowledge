# 『シェルとターミナル』ノート

（最終更新： 2023-05-22）


## 目次

1. [シェル](#シェル)
	1. [ログインシェル](#ログインシェル)
	1. [プロンプト](#プロンプト)
	1. [sh](#sh)
	1. [bash](#bash)
	1. [ash](#ash)
	1. [zsh](#zsh)
	1. [csh](#csh)
	1. [tcsh](#tcsh)
1. [シェルの操作](#シェルの操作)
	1. [補間機能](#補間機能)
	1. [インクリメンタルサーチ](#インクリメンタルサーチ)
1. [シェルセッション](#シェルセッション)
1. [ターミナル](#ターミナル)
	1. [ターミナルエミュレータ](#ターミナルエミュレータ)
	1. [ターミナルマルチプレクサ](#ターミナルマルチプレクサ)
1. [エイリアス](#エイリアス)
1. [環境変数](#環境変数)
	1. [シェル変数](#シェル変数)
	1. [PATH](#path)
1. [設定ファイルの読み込み](#設定ファイルの読み込み)


## シェル

**シェル**は、[ユーザ](./user_and_permission.md#ユーザ)と[Linux](./linux.md#linux)[カーネル](../../../software/_/chapters/operating_system.md#カーネル)の橋渡しをする[プログラム](../../../../programming/_/chapters/programming.md#プログラム)で、[カーネル](../../../software/_/chapters/operating_system.md#カーネル)の機能を[ユーザ](./user_and_permission.md#ユーザ)に提供するためのインタフェース。シェルはまず、[キーボード](../../../hardware/_/chapters/io_unit.md#キーボード)に入力された[ユーザ](./user_and_permission.md#ユーザ)の[コマンド](./basic_command.md#コマンド)を受け取り、[カーネル](../../../software/_/chapters/operating_system.md#カーネル)に対して処理命令を行う。そして、[カーネル](../../../software/_/chapters/operating_system.md#カーネル)の処理結果を受け取ってそれを出力する。シェルは、このようなやり取りを繰り返すインタラクティブな操作を提供する。

[Linux](./linux.md#linux)[カーネル](../../../software/_/chapters/operating_system.md#カーネル)とシェルのレイヤを分離することにより、[カーネル](../../../software/_/chapters/operating_system.md#カーネル)には手を入れずにシェルのみをユースケースに合わせて変更することができたり、別の[OS](../../../software/_/chapters/operating_system.md#オペレーティングシステム)を利用する際にシェルのみを移植すれば同様のインタフェースが利用できる、といった利点がある。

### ログインシェル

**ログインシェル**は、[ユーザ](./user_and_permission.md#ユーザ)がログイン時に最初に[Linux](./linux.md#linux)によって自動的に起動される[シェル](#シェル)。ログインシェルは、 `SHELL` [環境変数](#環境変数)に指定された[コマンド](./basic_command.md#コマンド)で起動されるものとなる。ログインシェルを変更するには、 `chsh` [コマンド](./basic_command.md#コマンド)を使用する。

```sh
# ログインシェルの確認
$ echo $SHELL
/bin/bash

# 利用できるシェルを一覧表示
$ chsh --list-shells
/bin/sh
/bin/bash
/usr/bin/git-shell
/bin/zsh
/usr/bin/zsh
/bin/rbash
/usr/bin/rbash

# ログインシェルの変更
$ chsh
Changing shell for user.
Password:
New shell [/bin/bash]: /bin/zsh
Shell changed.
```

### プロンプト

**プロンプト**は、[シェル](#シェル)が[ユーザ](./user_and_permission.md#ユーザ)の入力を受け付けている状態を示す記号。多くの場合は `$` や `#` で表わされ、さらにログイン中のユーザ名やホスト名、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)などの付加情報を表示することもできる。プロンプトの表示は、[シェル](#シェル)の設定により[ユーザ](./user_and_permission.md#ユーザ)が任意に変更できる（ `PROMPT` や `RPROMPT` といった環境変数）。

[Ubuntu](./linux.md#debian系)のデフォルトのプロンプトは以下の通り。（ユーザ名が `user` 、ホスト名が `host` の場合）

```sh
user@host:~$
```

プロンプト記号の `$` は[一般ユーザ](./user_and_permission.md#ユーザ)での操作、 `#` は[スーパユーザ](./user_and_permission.md#rootユーザ)での操作を例示しているドキュメントが多い。

また、長い[コマンド](./basic_command.md#コマンド)などの途中で `\` により改行された際には、**セカンダリプロンプト**が表示される。

### sh

**sh**(Bourne Shell)は、多くの[UNIX系OS](../../../software/_/chapters/operating_system.md#unix)で標準的に使われている[シェル](#シェル)のひとつで、古くからあり多くの[シェル](#シェル)がshとの互換性を持っている。

### bash

**bash**(Bourne-Again Shell)は、GNUで開発されている[オープンソース](../../../software/_/chapters/open_source_software.md#オープンソースソフトウェア)の[シェル](#シェル)。[sh](#sh)互換だが様々な機能が追加されており、一時期は[macOS](../../../software/_/chapters/operating_system.md#macos)の[シェル](#シェル)としても採用されていた。多くの[Linux](./linux.md#linux)環境でデフォルトの[シェル](#シェル)として採用されており、利用者が多いため情報を集めやすい。

### ash

**ash**(Almquist Shell)は、軽量かつ高速な[sh](#sh)互換の[シェル](#シェル)で、[Debian系](./linux.md#debian系)の[Linuxディストリビューション](./linux.md#ディストリビューション)で利用されている。

### zsh

**zsh**(Z Shell)は、他の[シェル](#シェル)の便利な機能を持ちつつ独自に進化している[sh](#sh)互換で[オープンソース](../../../software/_/chapters/open_source_software.md#オープンソースソフトウェア)の[シェル](#シェル)。[macOS](../../../software/_/chapters/operating_system.md#macos)のデフォルトの[シェル](#シェル)としても採用されている。

### csh

**csh**(C Shell)は、[BSD](../../../software/_/chapters/operating_system.md#unix)系の[OS](../../../software/_/chapters/operating_system.md#nui)で使用されている[シェル](#シェル)で、[sh](#sh)とは異なり文法が[C言語](../../../../programming/_/chapters/programming_language.md#c言語)風となっている。

### tcsh

**tcsh**(TENEX C Shell)は、[csh](#csh)を拡張した[オープンソース](../../../software/_/chapters/open_source_software.md#オープンソースソフトウェア)の[シェル](#シェル)で、[macOS](../../../software/_/chapters/operating_system.md#macos)で採用されていたこともあったが、[シェルスクリプト](./shell_script.md#シェルスクリプト)の記述には向いていないとされている。


## シェルの操作

[シェル](#シェル)にはデフォルトで様々なキーバインドが設定されており、できるだけホームポジションから手を離さずに様々な操作ができるようになっている。[シェル](#シェル)によっては対応していないものや、別のキーバインドが設定されているものもあるので注意。

| キーマップ   | 内容                                     |
| ------------ | :--------------------------------------- |
| `Ctrl` + `b` | 後方に1文字分移動する                    |
| `Ctrl` + `f` | 前方に1文字分移動する                    |
| `Ctrl` + `a` | 行頭に移動する                           |
| `Ctrl` + `e` | 行末に移動する                           |
| `Alt` + `b`  | 後方に単語1つ分移動する                  |
| `Alt` + `f`  | 前方に単語1つ分移動する                  |
| `Ctrl` + `h` | カーソル位置の後方に1文字削除する        |
| `Ctrl` + `d` | カーソル位置の1文字を削除する            |
| `Ctrl` + `w` | 後方にスペース区切りで1単語分を削除する  |
| `Ctrl` + `k` | カーソル位置から行末までを削除する       |
| `Ctrl` + `u` | カーソル位置から行頭までを削除する       |
| `Ctrl` + `y` | 最後に削除した内容を挿入する             |
| `Ctrl` + `s` | 画面表示をロックする                     |
| `Ctrl` + `q` | 画面表示のロックを解除する               |
| `Ctrl` + `c` | 実行中のコマンドを強制終了する           |
| `Ctrl` + `l` | 画面を消去する                           |
| `Ctrl` + `p` | 1つ前のコマンド履歴に移動する            |
| `Ctrl` + `n` | 1つ後のコマンド履歴に移動する            |
| `Ctrl` + `r` | コマンド履歴をインクリメンタルサーチする |

### 補間機能

多くの[シェル](#シェル)では、 `Tab` キーを押下することで補間機能が利用できる。途中まで入力した[コマンド](./basic_command.md#コマンド)や、[パス](../../../software/_/chapters/file_system.md#パス)の候補を[シェル](#シェル)が自動で入力してくれる。また、複数の候補がある場合には、その候補を画面上に出力してくれる。

### インクリメンタルサーチ

**インクリメンタルサーチ**は、キーボードから1文字入力するごとに[コマンド](./basic_command.md#コマンド)履歴を検索する機能。インクリメンタルサーチモード中は、次のようなキーバインドが利用できる。

| キーマップ   | 内容                                               |
| ------------ | :------------------------------------------------- |
| `Ctrl` + `r` | 1つ前の検索結果へ移動                              |
| `Enter`      | 現在の検索結果をそのまま実行                       |
| `Esc`        | 現在の検索結果を表示したまま、コマンドラインに戻る |
| `Ctrl` + `g` | 検索結果を破棄し、プロンプトに戻る                 |


## シェルセッション

**シェルセッション**は、[ユーザ](./user_and_permission.md#ユーザ)がシェルにログインしてから[シェル](#シェル)を閉じるまでの一連の流れ。セッションが開始されると、[シェル](#シェル)は設定を読み込んで[ユーザ](./user_and_permission.md#ユーザ)に[プロンプト](#プロンプト)を返し、[コマンド](./basic_command.md#コマンド)を入力できる状態となる。セッションは、[ユーザ](./user_and_permission.md#ユーザ)がログアウトするか、システムが再起動されると終了する。


## ターミナル

**ターミナル**（**端末**）は、[ユーザ](./user_and_permission.md#ユーザ)が[コンピュータ](../../../_/chapters/computer.md#コンピュータ)へ入出力する際に利用する[ハードウェア](../../../_/chapters/computer.md#ハードウェア)を指す言葉。[入力装置](../../../hardware/_/chapters/hardware.md#入力装置)にはキーボード、[出力装置](../../../hardware/_/chapters/hardware.md#出力装置)にはディスプレイが主に用いられる。

[ターミナルエミューレタ](#ターミナルエミュレータ)のことを単にターミナルと呼ぶ場合もある。

### ターミナルエミュレータ

**ターミナルエミュレータ**は、[ターミナル](#ターミナル)を[ソフトウェア](../../../software/_/chapters/software.md#ソフトウェア)によって再現したもの。入出力画面を提供するインタフェースで、[シェル](#シェル)はターミナルエミュレータ上で動作する。代表的なターミナルエミュレータとしては、[Windows](../../../software/_/chapters/operating_system.md#windows)のTeraTermやRLogin、[macOS](../../../software/_/chapters/operating_system.md#macos)のiTerm2、各環境で動作するAlacrittyなどがある。

遠隔の[コンピュータ](../../../_/chapters/computer.md#コンピュータ)を操作する場合、接続元ではターミナルエミュレータが、接続先では[シェル](#シェル)が動作することになる。

### ターミナルマルチプレクサ

**ターミナルマルチプレクサ**は、1つの[ターミナル](#ターミナル)を複数の仮想[ターミナル](#ターミナル)に分割して、同時に複数の[ターミナル](#ターミナル)を実行できるようにするツール。特に有名なターミナルマルチプレクサとしては、 `tmux` がある。リモートアクセスなどにより[Linux](./linux.md#linux)を利用する場合、コネクションが切断されたときに前のセッションを復元することができたり、複数のコネクションを張ることなくマルチターミナルで作業ができるといった利点がある。

### TTY

**TTY**(Teletypewriter)は、物理的な端末デバイスを表す用語で、キーボードと画面からなる[ターミナル](#ターミナル)の接続を表す[デバイスファイル](./file.md#デバイスファイル)。

### PTY

**PTY**(Pseudo terminal)は、仮想的な端末デバイスを表す用語で、[ターミナルエミュレータ](#ターミナルエミュレータ)や[SSH](../../../../network/_/chapters/application_layer.md#ssh)などの[アプリケーション](../../../software/_/chapters/software.md#応用ソフトウェア)で使用される。**PTS**(Pseudo terminal slave)と呼ぶ場合もある。

## エイリアス

**エイリアス**は、既存の[コマンド](./basic_command.md#コマンド)に対して別名をつけて実行できるようにする機能。エイリアスの設定には**alias**[コマンド](./basic_command.md#コマンド)を使用する。

例えば以下のようなエイリアスを設定すると、 `ls` [コマンド](./basic_command.md#コマンド)を実行したときに `-F` [オプション](./basic_command.md#オプション)が自動で付与されるようになる。

```sh
$ alias ls='ls -F'
```

ある[コマンド](./basic_command.md#コマンド)が、本当に[コマンド](./basic_command.md#コマンド)であるかエイリアスであるかを確認するには、**type**[コマンド](./basic_command.md#コマンド)を使用する。

```sh
$ type ls
ls is an alias for ls -F
```

また、エイリアスを削除するには**unalias**[コマンド](./basic_command.md#コマンド)を使用する。

```sh
$ unalias ls
```

**command**[コマンド](./basic_command.md#コマンド)を使用したり、[コマンド](./basic_command.md#コマンド)の先頭に `\` を付与することで、一時的にエイリアスを無視して元の[コマンド](./basic_command.md#コマンド)を実行することもできる。

```sh
$ command ls
$ \ls
```


## 環境変数

**環境変数**は、[Linux](./linux.md#linux)においてシステム全体に影響を与える[変数](../../../../programming/_/chapters/variable.md#変数)のことで、主に[シェル](#シェル)の振る舞いを制御するために使用される。環境変数は[プロセス](./process_and_job.md#プロセス)が実行されるときに[プロセス](./process_and_job.md#プロセス)に渡され、その[プロセス](./process_and_job.md#プロセス)の動作を変更することができる。環境変数は以下のような場合に利用される。

- [ユーザ](./user_and_permission.md#ユーザ)の設定やシステムの設定を制御する
- [プログラム](../../../../programming/_/chapters/programming.md#プログラム)が必要とする[パス](../../../software/_/chapters/file_system.md#パス)や[ライブラリ](../../../software/_/chapters/package.md#ライブラリ)の場所を指定する
- 各[ユーザ](./user_and_permission.md#ユーザ)の[シェル](#シェル)の動作をカスタマイズする

以下の[コマンド](./basic_command.md#コマンド)を実行すると、 `MY_VAR` という[変数](../../../../programming/_/chapters/variable.md#変数)に `avlue` という値を設定することができる。

```Sh
$ export MY_VAR=value
```

また、環境変数を確認するには、以下の[コマンド](./basic_command.md#コマンド)を実行する。

```sh
$ echo $MY_VAR
```

環境変数の設定は[シェルセッション](#シェルセッション)が終了すると消えるので、永続化したい場合は[シェル](#シェル)の設定ファイル内などで定義しておくとよい。

### シェル変数

**シェル変数**は、[シェル](#シェル)内でのみ使用される[変数](../../../../programming/_/chapters/variable.md#変数)で、その[シェル](#シェル)自体とその子[プロセス](./process_and_job.md#プロセス)でのみ使用される。シェル変数は[環境変数](#環境変数)とは異なり、他の[プログラム](../../../../programming/_/chapters/programming.md#プログラム)には影響を与えない。

シェル変数を設定するには、以下の例のようにシェル変数名と値を等号で結ぶ。

```sh
$ MY_SHELL_VAR=value
```

### PATH

**PATH**は、[シェル](#シェル)が[コマンド](./basic_command.md#コマンド)を実行する際に検索する[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)のリストを含む[環境変数](#環境変数)。 `PATH` には、[シェル](#シェル)が[コマンド](./basic_command.md#コマンド)を検索する順序で、コロンで区切られた[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)のリストが格納される。

例えば、 `PATH` が `/usr/local/bin:/usr/bin:/bin` に設定されている場合、[シェル](#シェル)はまず `/usr/local/bin` を検索し、次に `/usr/bin` を検索し、最後に `/bin` を検索する。

`PATH` 環境変数を修正することで、独自定義の[コマンド](./basic_command.md#コマンド)をどこからでも実行できるようにしたりすることができる。


## 設定ファイルの読み込み

**source**は、指定された[シェルスクリプト](./shell_script.md#シェルスクリプト)を現在の[シェルセッション](#シェルセッション)で実行し、その[シェルスクリプト](./shell_script.md#シェルスクリプト)内で定義された[変数](../../../../programming/_/chapters/variable.md#変数)や[関数](../../../../programming/_/chapters/function.md#関数)、設定等を有効にする[コマンド](./basic_command.md#コマンド)。

例えば次の[コマンド](./basic_command.md#コマンド)を実行すると、[ホームディレクトリ](../../../software/_/chapters/file_system.md#ホームディレクトリ)にある[bash](#bash)の設定ファイルである `.bashrc` を現在の[シェル](#シェル)に読み込む。

```sh
$ source ~/.bashrc
```
