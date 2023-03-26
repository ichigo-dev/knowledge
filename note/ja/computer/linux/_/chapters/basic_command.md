# 『基本コマンド』ノート

（最終更新： 2023-03-26）


## 目次

1. [コマンド](#コマンド)
	1. [サブコマンド](#サブコマンド)
	1. [オプション](#オプション)
1. [コマンドの使い方](#コマンドの使い方)
	1. [マニュアル](#マニュアル)
	1. [ヘルプオプション](#ヘルプオプション)
1. [ログアウト](#ログアウト)
	1. [シャットダウン](#シャットダウン)
	1. [再起動](#再起動)
	1. [スリープモード](#スリープモード)
1. [ファイル操作](#ファイル操作)
	1. [ファイルの一覧表示](#ファイルの一覧表示)
	1. [ファイルの作成](#ファイルの作成)
	1. [ファイルの削除](#ファイルの削除)
	1. [ファイルのコピー](#ファイルのコピー)
	1. [ファイルの移動](#ファイルの移動)
	1. [ファイルの内容確認](#ファイルの内容確認)
	1. [ファイルのページャ表示](#ファイルのページャ表示)
	1. [ファイルの先頭表示](#ファイルの先頭表示)
	1. [ファイルの末尾表示](#ファイルの末尾表示)
1. [ディレクトリ操作](#ディレクトリ操作)
	1. [カレントディレクトリの確認](#カレントディレクトリの確認)
	1. [ディレクトリの移動](#ディレクトリの移動)
	1. [ディレクトリの作成](#ディレクトリの作成)
	1. [ディレクトリの削除](#ディレクトリの削除)
1. [リンクの作成](#リンクの作成)
1. [ファイルの検索](#ファイルの検索)
	1. [データベースを利用したファイルの検索](#データベースを利用したファイルの検索)
	1. [ファイル内のパターン検索](#ファイル内のパターン検索)
1. [コマンドの検索](#コマンドの検索)


## コマンド

**コマンド**は、[ユーザ](./user_and_permission.md#ユーザ)が[シェル](./shell_and_terminal.md#シェル)に対して与える命令。様々な[オプション](#オプション)を付与するなどして、1行に入力するには長い場合は、 `\` を入力して改行することができる。

### サブコマンド

**サブコマンド**は、様々な機能をまとめた[コマンド](#コマンド)に対して、具体的な操作の種類を指定する命令。第一[コマンド](#コマンド)に続けて指定する。以下の例では、 `systemctl` [コマンド](#コマンド)のサブコマンドとして `start` を指定している。

```sh
$ systemctl start httpd
```

### オプション

**オプション**（**引数**、**フラグ**、**パラメータ**）は、[コマンド](#コマンド)の操作対象や処理内容の詳細を指定する部分。オプションの形式は[コマンド](#コマンド)によって異なるが、 `-` から始まる**ショートオプション**や、 `--` から始まる**ロングオプション**などが一般的。


## コマンドの使い方

[コマンド](#コマンド)の使い方がわからない場合、[コマンド](#コマンド)の[マニュアル](#マニュアル)を確認したり、[ヘルプオプション](#ヘルプオプション)を利用して簡易的な説明を表示することができる。

### マニュアル

`man` は、[Linux](./linux.md#linux)のマニュアルページを表示するために使用される[コマンド](#コマンド)。マニュアルページには、[Linux](./linux.md#linux)システムや[ソフトウェア](../../../software/_/chapters/software.md#ソフトウェア)の機能、[オプション](#オプション)、使用例などが記載されている。 `man` [コマンド](#コマンド)を使うことで、[コマンド](#コマンド)や[ファイル](../../../software/_/chapters/file_system.md#ファイル)、システムの概要を調べることができる。

`man` [コマンド](#コマンド)の書式は以下の通り。

```
man [OPTION]... [COMMAND/FILE]
```

例えば、 `ls` [コマンド](#コマンド)のマニュアルページを表示したい場合は、次のように実行する。

```sh
$ man ls
```

マニュアルページを開いている間は、矢印キーや `Enter/BackSpace` 、 `j/k` でページ内を移動することができ、 `q` キーでマニュアルを閉じてインタラクティブシェルに復帰できる。また、操作がわからなくなった場合には `h` キーを入力することでヘルプを参照することができる。これらの操作が有効なのは、ページャとしてデフォルトの `less` を用いている場合に限られる。

`OPTION` に指定することができる主要な[オプション](#オプション)は以下の通り。

| オプション      | 概要                                                                                     |
| --------------- | :--------------------------------------------------------------------------------------- |
| `-f` `-k` `-i`  | 指定したキーワードに該当するコマンドを検索し、見つかったマニュアルページの一覧を出力する |
| `-a`            | すべてのマニュアルページを表示する                                                       |
| `-P` `--pager`  | マニュアルページの閲覧に用いるページャを指定する（デフォルトでは `less` ）               |

また、マニュアルは種類ごとに以下のようなセクション番号が割り振られており、番号を指定してマニュアルを調べることもできる。

| セクション番号 | 内容                                   |
| -------------- | :------------------------------------- |
| `1`            | Linuxコマンド                          |
| `2`            | システムコール（カーネル関数）         |
| `3`            | ライブラリコール（Cライブラリの関数）  |
| `4`            | スペシャルファイル（デバイスファイル） |
| `5`            | ファイルのフォーマット                 |
| `6`            | ゲームやデモ                           |
| `7`            | マクロ                                 |
| `8`            | その他                                 |
| `9`            | システムコマンドとデーモン             |

### ヘルプオプション

**ヘルプオプション**は、[コマンド](#コマンド)の使い方や[オプション](#オプション)などの情報を表示するための[オプション](#オプション)で、多くの[コマンド](#コマンド)がこの[オプション](#オプション)を備えている。一般的にはヘルプオプションは `-h` 、 `--help` 、 `-?` といった形式で提供される。

ヘルプオプションを使用することで、[コマンド](#コマンド)の詳細な使用方法や[オプション](#オプション)の意味を知ることができる。ヘルプオプションを使用する場合、次のように[コマンド](#コマンド)を入力する。

```sh
$ [COMMAND] --help
```

例えば、 `ls` [コマンド](#コマンド)のヘルプを表示するには、次のように入力する。

```sh
$ ls --help
```


## ログアウト

`exit` は、[Linux](./linux.md#linux)において[シェル](./shell_and_terminal.md#シェル)からログアウトするための[コマンド](#コマンド)。この[コマンド](#コマンド)の実行により現在使用している[シェル](./shell_and_terminal.md#シェル)[プロセス](./process_and_job.md#プロセス)を終了する。

### シャットダウン

`shutdown` は、[Linux](./linux.md#linux)システムをシャットダウンするための[コマンド](#コマンド)。システムをシャットダウンする前に[ユーザ](./user_and_permission.md#ユーザ)に通知を行い、デフォルトではシャットダウンまで1分間の待ち時間が設定されている。

また、 `poweroff` もシャットダウンのための[コマンド](#コマンド)。ただし、 `shutdown` とは異なり、システムを直ちに停止するため、[アプリケーション](../../../software/_/chapters/software.md#応用ソフトウェア)が意図せず停止されてしまう恐れがある。

### 再起動

`reboot` は、[Linux](./linux.md#linux)システムを再起動するための[コマンド](#コマンド)。 `reboot` を実行すると、システムはシャットダウンして再起動する。 `shutdown` コマンドのように再起動時刻を指定して実行することもできる。例えば、5分後にシステムを再起動したい場合、以下の[コマンド](#コマンド)を実行する。

```sh
$ reboot +5
```

### スリープモード

`suspend` は、[Linux](./linux.md#linux)システムをスリープモードにするための[コマンド](#コマンド)。 `suspend` を実行すると、[コンピュータ](../../../_/chapters/computer.md#コンピュータ)の電源はOFFにならず、[CPU](../../../hardware/_/chapters/processor.md#cpu)や[メモリ](../../../hardware/_/chapters/memory.md#メモリ)などの一部の機能が停止して、省エネモードになる。また、この[コマンド](#コマンド)から復帰すると、停止前の状態から作業を再開することができる。


## ファイル操作

### ファイルの一覧表示

`ls` は、[Linux](./linux.md#linux)及び[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)の一覧表示を行う[コマンド](#コマンド)。 `ls` は"list"の略語。[ファイル](../../../software/_/chapters/file_system.md#ファイル)名や[パーミッション](./user_and_permission.md#パーミッション)、[所有者](./user_and_permission.md#所有者)、更新日時、ファイルサイズなどの情報を表示することができる。

[コマンド](#コマンド)の書式は以下の通り。

```
ls [OPTION]... [FILE]...
```

`ls` [コマンド](#コマンド)を単体で使用すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)の[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を一覧表示する。また、 `ls` [コマンド](#コマンド)の主要な[オプション](#オプション)は以下の通り。

| オプション | 概要                                                                                  |
| ---------- | :------------------------------------------------------------------------------------ |
| `-l`       | パーミッションや所有者、グループ、ファイルサイズ、更新日時など、詳細な情報を表示する  |
| `-a`       | 隠しファイルも含めたすべてのファイルを表示する                                        |
| `-h`       | ファイルサイズを人間が読みやすい形式で表示する                                        |
| `-t`       | 更新日時の新しい順に並べ替えて表示する                                                |
| `-r`       | 結果を逆順で表示する                                                                  |
| `-R`       | サブディレクトリ以下のファイルやディレクトリを再帰的に表示する                        |
| `-F`       | ファイル名の後ろにファイルの種類を表す記号を追加して表示する                          |

`-F` [オプション](#オプション)を指定した場合、ファイル名の後ろに次のような記号が追加で表示される。

| 種別               | 記号 |
| ------------------ | ---- |
| 通常ファイル       | なし |
| ディレクトリ       | `/`  |
| 実行可能ファイル   | `*`  |
| シンボリックリンク | `@`  |

### ファイルの作成

`touch` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティグシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、空の[ファイル](../../../software/_/chapters/file_system.md#ファイル)を作成したり、既存の[ファイル](../../../software/_/chapters/file_system.md#ファイル)の更新日時を変更する[コマンド](#コマンド)です。

[コマンド](#コマンド)の書式は以下の通り。

```
touch [OPTION]... FILE...
```

`touch` [コマンド](#コマンド)を単独で使用すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)に空の[ファイル](../../../software/_/chapters/file_system.md#ファイル)を作成する。例えば次の[コマンド](#コマンド)は、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)に `test.txt` という名前のから[ファイル](../../../software/_/chapters/file_system.md#ファイル)を作成する。

```sh
$ touch test.txt
```

また、既存の[ファイル](../../../software/_/chapters/file_system.md#ファイル)の更新日時を変更する場合は、ファイル名を指定して `touch` [コマンド](#コマンド)を実行する。 `touch` [コマンド](#コマンド)の代表的な[オプション](#オプション)は以下の通り。

| オプション | 概要                                                                    |
| ---------- | :---------------------------------------------------------------------- |
| `-c`       | ファイルが存在しない場合に、ファイルを新規作成せずにコマンドを終了する  |
| `-d`       | ファイルの更新日時を指定した日時に変更する                              |
| `-m`       | ファイルの更新日時だけを変更する（最終アクセス日時は更新しない）        |
| `-r`       | 指定したファイルと同じ更新日時にする                                    |

### ファイルの削除

`rm` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#オペレーティングシステム)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を削除するための[コマンド](#コマンド)。 `rm` は"remove"の略語。

[コマンド](#コマンド)の書式は以下の通り。

```
ls [OPTION]... [FILE]...
```

`rm` [コマンド](#コマンド)を単独で使用すると、指定した[ファイル](../../../software/_/chapters/file_system.md#ファイル)を削除する。例えば次の[コマンド](#コマンド)は、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)にある `test.txt` という名前の[ファイル](../../../software/_/chapters/file_system.md#ファイル)を削除する。

```sh
$ rm test.txt
```

`rm` [コマンド](#コマンド)によって削除された[ファイル](../../../software/_/chapters/file_system.md#ファイル)は元に戻すことができないので、注意が必要。 `rm` [コマンド](#コマンド)の代表的な[オプション](#オプション)は以下の通り。

| オプション | 概要                                                                                                  |
| ---------- | :---------------------------------------------------------------------------------------------------- |
| `-r`       | サブディレクトリを削除するためのオプションで、サブディレクトリ内のファイルやディレクトリもすべて削除 |
| `-i`       | 削除時に確認を求める                                                                                  |
| `-f`       | 警告を表示せずに、強制的にファイルを削除する                                                          |

`rm` は強力な[コマンド](#コマンド)であり、誤って重要な[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を削除しないように十分に注意が必要。

### ファイルのコピー

`cp` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)をコピーするための[コマンド](#コマンド)。 `cp` は"copy"の略語。

[コマンド](#コマンド)の書式は以下の通り。

```
cp [OPTION]... [-T] SOURCE DEST
cp [OPTION]... SOURCE... DIRECTORY
cp [OPTION]... -t DIRECTORY SOURCE...
```

`cp` [コマンド](#コマンド)にコピー元[ファイル](../../../software/_/chapters/file_system.md#ファイル)とコピー先[ファイル](../../../software/_/chapters/file_system.md#ファイル)の[パス](../../../software/_/chapters/file_system.md#パス)を指定することで、[ファイル](../../../software/_/chapters/file_system.md#ファイル)をコピーすることができる。例えば次の[コマンド](#コマンド)は、 `/home/user/src/test.txt` という[ファイル](../../../software/_/chapters/file_system.md#ファイル)を `/home/user/dest/test.txt` という[ファイル](../../../software/_/chapters/file_system.md#ファイル)にコピーする。

```sh
cp /home/user/src/test.txt /home/user/dest/test.txt
```

`cp` [コマンド](#コマンド)の代表的な[オプション](#オプション)は以下の通り。

| オプション | 概要                                             |
| ---------- | :----------------------------------------------- |
| `-r`       | 指定したディレクトリの中身を再帰的にコピーする   |
| `-p`       | コピー元ファイルの属性やパーミッションを維持する |
| `-i`       | 上書き確認を求める                               |

`cp` [コマンド](#コマンド)は[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を簡単にコピーすることができるが、コピー先に同名の[ファイル](../../../software/_/chapters/file_system.md#ファイル)が存在する場合には、内容が上書きされるため注意が必要。

### ファイルの移動

`mv` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を移動するための[コマンド](#コマンド)。また、[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)の名前を変更するためにも用いられる。 `mv` は"move"の略語。

[コマンド](#コマンド)の書式は以下の通り。

```
mv [OPTION]... [-T] SOURCE DEST
mv [OPTION]... SOURCE... DIRECTORY
mv [OPTION]... -t DIRECTORY SOURCE...
```

`mv` [コマンド](#コマンド)に移動元[ファイル](../../../software/_/chapters/file_system.md#ファイル)または[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)の[パス](../../../software/_/chapters/file_system.md#パス)と移動先の[パス](../../../software/_/chapters/file_system.md#パス)を指定することで、[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を移動することができる。例えば次の[コマンド](#コマンド)は、 `/home/user/src/test.txt` という[ファイル](../../../software/_/chapters/file_system.md#ファイル)を `/home/user/dest/test.txt` という[ファイル](../../../software/_/chapters/file_system.md#ファイル)に移動する。

```sh
$ mv /home/user/src/test.txt /home/user/dest/test.txt
```

また、 `mv` [コマンド](#コマンド)を使用して[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)をリネームすることもできる。例えば次の[コマンド](#コマンド)は、 `/home/user/src/test.txt` という[ファイル](../../../software/_/chapters/file_system.md#ファイル)を `/home/user/src/test2.txt` という名前に変更する。

```sh
$ mv /home/user/src/test.txt /home/user/src/test2.txt
```

`mv` [コマンド](#コマンド)に `-i` [オプション](#オプション)を指定すると、移動先に同じ名前の[ファイル](../../../software/_/chapters/file_system.md#ファイル)があった場合に上書き確認を行う。

### ファイルの内容確認

`cat` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、テキストファイルの内容を表示するための[コマンド](#コマンド)。 `cat` は"concatenate"の略語。

[コマンド](#コマンド)の書式は以下の通り。

```
cat [OPTION]... [FILE]...
```

基本的に、`cat` [コマンド](#コマンド)は1つ以上の[ファイル](../../../software/_/chapters/file_system.md#ファイル)を引数として受け取り、それらの[ファイル](../../../software/_/chapters/file_system.md#ファイル)の内容を順番に連結して[標準出力](./stdio_and_pipeline.md#標準出力)に表示する。例えば次の[コマンド](#コマンド)を実行すると、[ファイル](../../../software/_/chapters/file_system.md#ファイル) `file1.txt` と `file2.txt` の内容が順番に表示される。

```sh
$ cat test1.txt test2.txt
```

`cat` [コマンド](#コマンド)にファイル名を指定しなかった場合、[標準入力](./stdio_and_pipeline.md#標準出力)を待ち受ける状態となり、受け取った入力をそのまま[標準出力](./stdio_and_pipeline.md#標準出力)に表示する。

`cat` [コマンド](#コマンド)の代表的な[オプション](#オプション)は以下の通り。

| オプション | 概要                                         |
| ---------- | :------------------------------------------- |
| `-n`       | 行番号を表示する                             |
| `-b`       | 行番号を表示する（空行には行番号を付けない） |
| `-s`       | 空行をまとめる                               |
| `-E`       | 各行の末尾に `$` を表示する                  |

`cat` はテキスト[ファイル](../../../software/_/chapters/file_system.md#ファイル)の内容を素早く表示することができるが、大きな[ファイル](../../../software/_/chapters/file_system.md#ファイル)を扱う場合には `less` や `more` といった[ページャコマンド](#ファイルのページャ表示)の方が向いている。

### ファイルのページャ表示

`less` や `more` [コマンド](#コマンド)は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、テキスト[ファイル](../../../software/_/chapters/file_system.md#ファイル)を閲覧するための[コマンド](#コマンド)。 `less` は `more` よりも高度な機能を持っている。

[コマンド](#コマンド)の書式は以下の通り。

```
less [OPTION]... [FILE]...
```

`less` [コマンド](#コマンド)を使用すると、指定した[ファイル](../../../software/_/chapters/file_system.md#ファイル)の内容が先頭から表示される。 `Space` キーを入力すると1ページ分のテキストをスクロールダウンし、 `b` キーを入力すると1ページ分のテキストをスクロールアップする。また、 `q` キーを入力すると `less` [コマンド](#コマンド)を終了する。

また、 `less` [コマンド](#コマンド)は `more` [コマンド](#コマンド)にはない以下のような機能を提供する。

- テキスト内の文字列を検索する機能（ `/` キーを押して、検索したい文字列を入力する）
- [ファイル](../../../software/_/chapters/file_system.md#ファイル)内の行数を表示する（ `=` キーを入力する）
- [ファイル](../../../software/_/chapters/file_system.md#ファイル)内の任意の行に移動する（ `g` キーを入力すると[ファイル](../../../software/_/chapters/file_system.md#ファイル)の先頭に、 `G` キーを入力すると[ファイル](../../../software/_/chapters/file_system.md#ファイル)の末尾にジャンプする）

例えば次の[コマンド](#コマンド)を実行すると、 `less` ページャ内に `/var/log/syslog` [ファイル](../../../software/_/chapters/file_system.md#ファイル)を表示する。

```sh
$ less /var/log/syslog
```

### ファイルの先頭表示

`head` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、テキスト[ファイル](../../../software/_/chapters/file_system.md#ファイル)の先頭から指定された行数分の内容を表示する[コマンド](#コマンド)。 `tail` [コマンド](#コマンド)と同様に、ログ[ファイル](../../../software/_/chapters/file_system.md#ファイル)や大容量テキスト[ファイル](../../../software/_/chapters/file_system.md#ファイル)の処理に便利なツール。

[コマンド](#コマンド)の書式は以下の通り。

```
head [OPTION]... [FILE]...
```

デフォルトでは、 `head` [コマンド](#コマンド)は指定された[ファイル](../../../software/_/chapters/file_system.md#ファイル)の先頭10行を表示する。 `-n` オプションを使用することで、表示する行数を変更することができる。例えば次の[コマンド](#コマンド)を実行すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)にある `test.txt` の先頭5行が表示される。

```sh
$ head -n 5 test.txt
```

また、複数の[ファイル](../../../software/_/chapters/file_system.md#ファイル)を指定して、それぞれの[ファイル](../../../software/_/chapters/file_system.md#ファイル)の先頭を表示することもできる。例えば次の[コマンド](#コマンド)を実行すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)にある `test1.txt` と `test2.txt` の先頭10行がそれぞれ表示される。

```sh
$ head test1.txt test2.txt
```

### ファイルの末尾表示

`tail` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、テキスト[ファイル](../../../software/_/chapters/file_system.md#ファイル)の末尾から指定された行数分の内容を表示するための[コマンド](#コマンド)。主にログ[ファイル](../../../software/_/chapters/file_system.md#ファイル)や大容量のテキスト[ファイル](../../../software/_/chapters/file_system.md#ファイル)の処理や解析に便利なツール。

[コマンド](#コマンド)の書式は以下の通り。

```
tail [OPTION]... [FILE]...
```

デフォルトでは、 `tail` [コマンド](#コマンド)は指定された[ファイル](../../../software/_/chapters/file_system.md#ファイル)の末尾10行を表示する。 `-n` [オプション](#オプション)を使用することで、表示する行数を変更することができる。例えば次の[コマンド](#コマンド)を実行すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)にある `test.txt` の末尾5行が表示される。

```sh
$ tail -n 5 test.txt
```

また、複数の[ファイル](../../../software/_/chapters/file_system.md#ファイル)を指定して、それぞれの[ファイル](../../../software/_/chapters/file_system.md#ファイル)の末尾を表示することもできる。例えば次の[コマンド](#コマンド)を実行すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)にある `test1.txt` と `test2.txt` の末尾10行がそれぞれ表示される。

```sh
$ tail test1.txt test2.txt
```

`-f` [オプション](#オプション)を使用すると、[ファイル](../../../software/_/chapters/file_system.md#ファイル)の変更をリアルタイムに追跡するとこができる。これは、ログ[ファイル](../../../software/_/chapters/file_system.md#ファイル)の解析などにおいて非常に便利な[オプション](#オプション)で、以下の例では `access.log` の変更をリアルタイムに追跡する。

```sh
$ tail -f access.log
```


## ディレクトリ操作

### カレントディレクトリの確認

`pwd` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、現在の作業[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を表示するための[コマンド](#コマンド)。 `pwd` は"print working directory"の略語。

[コマンド](#コマンド)の書式は以下の通り。

```
pwd [OPTION]...
```

基本的に `pwd` は現在の作業[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)の[絶対パス](../../../software/_/chapters/file_system.md#絶対パス)を表示する。例えば、現在の作業[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)が `~/docs` である場合、 `pwd` [コマンド](#コマンド)を実行すると、 `/home/user/docs` という出力が得られる。

`pwd` [コマンド](#コマンド)に `-P` [オプション](#オプション)を指定すると、[シンボリックリンク](./file.md#シンボリックリンク)を解決した[パス](../../../software/_/chapters/file_system.md#パス)を表示する。

### ディレクトリの移動

`cd` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)を変更するための[コマンド](#コマンド)。 `cd` は"change directory"の略語。

[コマンド](#コマンド)の書式は以下の通り。

```
cd [PATH]
```

`cd` に移動先の[パス](../../../software/_/chapters/file_system.md#パス)を指定すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)を指定した[パス](../../../software/_/chapters/file_system.md#パス)に変更する。例えば以下の[コマンド](#コマンド)を実行すると、 `/home/user/docs` [ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)に移動する。

```sh
$ cd /home/user/docs
```

`..` を使用することで、1つ上の[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を指定することができる。例えば以下の[コマンド](#コマンド)を実行すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)から見て1つ上の[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)移動する。

```sh
$ cd ..
```

### ディレクトリの作成

`mkdir` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、新しい[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を作成するための[コマンド](#コマンド)。 `mkdir` は"make directory"の略語。

[コマンド](#コマンド)の書式は以下の通り。

```
mkdir [OPTION]... DIRECTORY...
```

`mkdir` [コマンド](#コマンド)に[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)名を指定すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)に指定した名前の[サブディレクトリ](../../../software/_/chapters/file_system.md#サブディレクトリ)が作成される。例えば以下の[コマンド](#コマンド)を実行すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)に `docs` という名前の新しい[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)が作成される。

```sh
$ mkdir docs
```

`mkdir` [コマンド](#コマンド)は一度に複数の[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を作成することもできる。例えば以下の[コマンド](#コマンド)を実行すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)に `docs` と `music` と `pictures` という3つの新しい[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を同時に作成する。

```sh
$ mkdir docs music pictures
```

### ディレクトリの削除

`rmdir` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、空の[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を削除するための[コマンド](#コマンド)。 `rmdir` は"remove directory"の略語。

[コマンド](#コマンド)の書式は以下の通り。

```
rmdir [OPTION]... DIRECTORY...
```

`rmdir` に[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)名を指定すると、[カレントディレクトリ](../../../software/_/chapters/file_system.md#カレントディレクトリ)にある指定した名前の[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を削除する。例えば以下の[コマンド](#コマンド)を実行すると、 `docs` という名前の空の[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を削除する。

```sh
$ rmdir docs
```


## リンクの作成

`ln` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、[リンク](./file.md#リンク)を作成するために使用される[コマンド](#コマンド)。 `ln` は"link"の略語。

[コマンド](#コマンド)の書式は以下の通り。

```
ln [OPTION]... [-T] TARGET LINK_NAME
ln [OPTION]... TARGET
ln [OPTION]... TARGET... DIRECTORY
ln [OPTION]... -t DIRECTORY TARGET...
```

例えば、 `/home/user/test.txt` という[ファイル](../../../software/_/chapters/file_system.md#ファイル)に対して `/home/user/test_link.txt` という[ハードリンク](./file.md#ハードリンク)を作成するには、次のような[コマンド](#コマンド)を実行する。

```sh
$ ln /home/user/test.txt /home/user/test_link.txt
```

また、 `/home/user/docs` という[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)に対して `/home/user/docs_link` という[シンボリックリンク](./file.md#シンボリックリンク)を作成するには、次のような[コマンド](#コマンド)を実行する。

```sh
$ ln -s /home/user/docs /home/user/docs_link
```

`-s` [オプション](#オプション)を使用することで、[シンボリックリンク](./file.md#シンボリックリンク)を作成することができる。


## ファイルの検索

`find` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、指定された[ディレクトリツリー](../../../software/_/chapters/file_system.md#ツリー構造)内で[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)を検索するための[コマンド](#コマンド)。条件に合致する[ファイル](../../../software/_/chapters/file_system.md#ファイル)を見つけ出して、指定されたアクションを実行することができる。

[コマンド](#コマンド)の書式は以下の通り。

```
find [OPTION]... [PATH] [EXPRESSION]...
```

`PATH` には検索を開始する[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)の[パス](../../../software/_/chapters/file_system.md#パス)を指定し、 `EXPRESSION` には検索条件を指定する。検索条件は、 `-name` や `-type` などの[オプション](#オプション)を指定することができる。例えば次の[コマンド](#コマンド)を実行すると、現在の[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)以下で名前が `test.txt` である[ファイル](../../../software/_/chapters/file_system.md#ファイル)を検索する。

```sh
$ find . -name "test.txt"
```

`find` [コマンド](#コマンド)の主要な[オプション](#オプション)は以下の通り。

| オプション | 概要                                                                                                                                              |
| ---------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| `-name`    | ファイル名が指定したパターンに一致するファイルを検索する                                                                                          |
| `-type`    | 指定したファイルタイプに一致するファイルを検索する（ `-type f` は通常ファイル、 `-type d` はディレクトリ、 `-type l` はシンボリックリンク）      |
| `-mtime`   | ファイルの最終更新日時が指定した日数前（正）または後（負）であるファイルを検索する                                                                |
| `-size`    | ファイルサイズが指定したサイズに一致したファイルを検索する（単位として `c` （バイト）、 `k` （キロバイト）、 `M` （メガバイト）などを指定できる） |
| `-exec`    | 検索されたファイルに対して指定されたコマンドを実行する                                                                                            |

### データベースを利用したファイルの検索

`locate` は、[ファイルシステム](../../../software/_/chapters/file_system.md#ファイルシステム)内で指定した文字列にマッチする[ファイル](../../../software/_/chapters/file_system.md#ファイル)を高速に検索するための[コマンド](#コマンド)。 `find` [コマンド](#コマンド)よりも効率的で、パターンマッチングにも対応している。

[コマンド](#コマンド)の書式は以下の通り。

```
locate [OPTION]... PATTERN
```

例えば以下の[コマンド](#コマンド)を実行すると、 `/etc` [ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)以下の `ssh` という名前を含む[ファイル](../../../software/_/chapters/file_system.md#ファイル)を検索する。

```sh
$ locate /etc/*ssh*
```

`locate` は、[ディスク](../../../hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)をスキャンすることなく、専用の[データベース](../../../../development/database/_/chapters/basic_knowledge_of_database.md#データベース)を利用するため、高速に動作する。ただし、[ファイルパス](../../../software/_/chapters/file_system.md#パス)の[データベース](../../../../development/database/_/chapters/basic_knowledge_of_database.md#データベース)が更新されていない可能性があるため、既に存在しない[ファイル](../../../software/_/chapters/file_system.md#ファイル)が表示されたり、存在する[ファイル](../../../software/_/chapters/file_system.md#ファイル)が表示されない可能性がある。最新の情報を手動で更新するには、 `updatedb` [コマンド](#コマンド)を実行する。主要な[オプション](#オプション)は以下の通り。

| オプション | 概要                           |
| ---------- | :----------------------------- |
| `-i`       | 大文字と小文字を区別しない     |
| `-l [NUM]` | 最大の検索結果数を指定する     |
| `-c`       | 該当するファイルの数を表示する |

パターンに複数のパターンを指定するとOR検索になり、 `-A | --alll` [オプション](#オプション)を指定するとAND検索になる。

### ファイル内のパターン検索

`grep` は、指定したパターンに一致する行を[ファイル](../../../software/_/chapters/file_system.md#ファイル)から検索し、マッチする行を出力するための[コマンド](#コマンド)。

[コマンド](#コマンド)の書式は以下の通り。

```sh
grep [OPTION]... PATTERNS [FILE]...
grep [OPTION]... -e PATTERNS ... [FILE]...
grep [OPTION]... -f PATTERN_FILE ... [FILE]...
```

例えば以下の[コマンド](#コマンド)を実行すると、 `test.txt` [ファイル](../../../software/_/chapters/file_system.md#ファイル)内から `hello` という文字列を検索し、それが含まれる行を表示する。

```sh
grep "hello" test.txt
```

また以下の例では、現在の[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)内の全ての[ファイル](../../../software/_/chapters/file_system.md#ディレクトリ)から、[正規表現](./text_processing_and_regex.md#正規表現)を用いて該当箇所を検索する。

```sh
grep "*.png" *
```

主要な[オプション](#オプション)は以下の通り。

| オプション | 概要                                             |
| ---------- | :----------------------------------------------- |
| `-i`       | 大文字小文字を区別せずに検索する                 |
| `-v`       | パターンと一致しない行を検索する                 |
| `-c`       | パターンと一致する行の数を表示する               |
| `-n`       | パターンと一致する行の行番号を表示する           |
| `-r`       | ディレクトリを再帰的に検索する                   |
| `-l`       | ファイル名のみを表示する                         |
| `-w`       | パターンと完全一致する行のみを検索する           |
| `-E`       | 拡張正規表現を使用する                           |
| `-F`       | パターンを正規表現ではなく、固定文字列として扱う |


## コマンドの検索

`which` は、[Linux](./linux.md#linux)および[UNIX系](../../../software/_/chapters/operating_system.md#unix)[オペレーティングシステム](../../../software/_/chapters/operating_system.md#オペレーティングシステム)で使用される、[コマンド](#コマンド)がどの場所にインストールされているかを特定するための[コマンド](#コマンド)。

[コマンド](#コマンド)の書式は以下の通り。

```
which [OPTION] [--] PROGRAMNAME...
```

例えば以下の[コマンド](#コマンド)を実行すると、 `python` [コマンド](#コマンド)がインストールされている[パス](../../../software/_/chapters/file_system.md#パス)が表示される。

```sh
$ which python
```

デフォルトでは、 `PATH` [環境変数](./shell_and_terminal.md#環境変数)に定義された場所で[コマンド](#コマンド)を検索する。
