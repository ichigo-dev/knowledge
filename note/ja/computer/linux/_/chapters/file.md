# 『ファイル』ノート

（最終更新： 2023-03-26）


## 目次

1. [Linuxとファイル](#linuxとファイル)
1. [FHS](#fhs)
1. [リンク](#リンク)
	1. [シンボリックリンク](#シンボリックリンク)
	1. [ハードリンク](#ハードリンク)
1. [ファイルディスクリプタ](#ファイルディスクリプタ)


## Linuxとファイル

[Linux](./linux.md#linux)では、システムを構成するすべての要素が[ファイル](../../../software/_/chapters/file_system.md#ファイル)として表現される。[ハードディスク](../../../hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)やキーボード、プリンタといった[入出力装置](../../../hardware/_/chapters/io_unit.md)はそれぞれ[ファイル](../../../software/_/chapters/file_system.md#ファイル)が割り当てられており、その[ファイル](../../../software/_/chapters/file_system.md#ファイル)を介して機器を操作する。また、[Linux](./linux.md#linux)[カーネル](../../../software/_/chapters/operating_system.md#カーネル)も1つの[ファイル](../../../software/_/chapters/file_system.md#ファイル)として表現されている。

[Linux](./linux.md#linux)では[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)自体も[ファイル](../../../software/_/chapters/file_system.md#ファイル)として扱われる。


## FHS

**FHS**(Filesystem Hierarchy Standard)は、[Linux](./linux.md#linux)の[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)構成の標準化仕様。[ディストリビューション](./linux.md#ディストリビューション)によって[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)構成に多少の違いはあるが、基本的にはFHSに準拠している。

| ディレクトリ    | 説明                                                                                                  |
| --------------- | :---------------------------------------------------------------------------------------------------- |
| `/`             | ファイルシステムのルートディレクトリ                                                                  |
| `├ bin/`        | 一般ユーザ向けの基本コマンドの実行ファイルを格納するディレクトリ                                      |
| `├ boot/`       | ブートローダ関連のファイルを格納するディレクトリ                                                      |
| `├ dev/`        | デバイスファイル（コンピュータに接続された周辺機器に関するファイル）を格納するディレクトリ            |
| `├ etc/`        | 各種プログラムの設定ファイルを格納したディレクトリ                                                    |
| `├ home/`       | ユーザごとのホームディレクトリ群を格納したディレクトリ                                                |
| `├ lib/`        | `/bin` や `/sbin` にある実行ファイルに紐づけられた共有ライブラリを格納したディレクトリ                |
| `├ lost+found/` | 予期しないシャットダウンなどにより破損したファイルなどの断片が格納されるディレクトリ                  |
| `├ media/`      | 外部メディアが接続された際にOSによって自動的にマウンティングされるディレクトリを格納したディレクトリ  |
| `├ mnt/`        | 一時的なファイルシステムのマウントなどに用いるディレクトリで、ユーザが手動でマウントを行う            |
| `├ opt/`        | パッケージマネージャを使わずにインストールしたアプリケーションなどが格納されるディレクトリ            |
| `├ proc/`       | カーネルやプロセスの状態に関する情報を格納するディレクトリ                                            |
| `├ root/`       | rootユーザのホームディレクトリ                                                                        |
| `├ run/`        | 実行中のプログラムのPIDファイルなどを格納するディレクトリ                                             |
| `├ sbin/`       | システム管理用のコマンドの実行ファイルを格納するディレクトリ                                          |
| `├ srv/`        | サービスを提供する際のデータを格納するディレクトリ                                                    |
| `├ sys/`        | デバイスやドライバについての情報を格納するディレクトリ                                                |
| `├ tmp/`        | 一時ファイルの置き場として利用されるディレクトリ                                                      |
| `├ usr/`        | システム以外の全ユーザが共通して利用するプログラムやライブラリを格納するディレクトリ                  |
| `│ ├ bin/`      | ユーザが利用するコマンドを格納するディレクトリ                                                        |
| `│ ├ include/`  | ユーザ空間のソースコードをコンパイルするのに必要なヘッダファイルを格納するディレクトリ                |
| `│ ├ lib/`      | `/usr/bin` にある実行ファイルに紐づけられた共有ライブラリを格納したディレクトリ                       |
| `│ ├ share/`    | アイコンやフォントなど、アーキテクチャに依存しない共有う可能なファイルを格納したディレクトリ          |
| `│ ├ src/`      | ソースコードを格納するためのディレクトリ                                                              |
| `│ └ local/`    | ホスト固有のローカルデータを格納する第三階層（ `bin` や `lib` 、 `share` などを持つ）                 |
| `└ var/`        | ログやメールなどの可変ファイルを格納する為のディレクトリ                                              |
| `  ├ cache/`    | キャッシュファイルを格納するディレクトリ                                                              |
| `  ├ log/`      | 各種ログファイルを格納するディレクトリ                                                                |
| `  ├ mail/`     | ユーザのメールボックスとして利用されるディレクトリ                                                    |
| `  ├ spool/`    | 処理待ち状態のタスクのスプールを格納するディレクトリ                                                  |
| `  ├ tmp/`      | 一時ファイルの置き場として利用されるディレクトリ                                                      |
| `  └ www/`      | 慣習的にWebコンテンツを格納するために利用されるディレクトリ                                           |


## リンク

**リンク**は、[ファイル](../../../software/_/chapters/file_system.md#ファイル)に別名をつける機能。リンクを利用することで、長い[パス](../../../software/_/chapters/file_system.md#パス)名を省略したり、複数[バージョン](../../../software/_/chapters/package.md#バージョン)の[プログラム](../../../../programming/_/chapters/programming.md#プログラム)を共存させたりすることができる。

### シンボリックリンク

**シンボリックリンク**（**ソフトリンク**）は、[リンク](#リンク)先の[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)の[パス](../../../software/_/chapters/file_system.md#パス)を記憶する方法で、[リンク](#リンク)先の[ファイル](../../../software/_/chapters/file_system.md#ファイル)が削除されると無効な[リンク](../../../software/_/chapters/file_system.md#リンク)となる。[ディレクトリ](../../../software/_/chapters/file_system.md#ディレクトリ)に対する[リンク](#リンク)や、別の[ファイルシステム](../../../software/_/chapters/file_system.md#ファイルシステム)に対する[リンク](#リンク)を作成することができる。[Windows](../../../software/_/chapters/operating_system.md#windows)における**ショートカット**や、[macOS](../../../software/_/chapters/operating_system.md#macos)における**エイリアス**と似た機能。

### ハードリンク

**ハードリンク**は、[ファイル](../../../software/_/chapters/file_system.md#ファイル)の実体を直接指し示して共有する[リンク](#リンク)。[ファイルシステム](../../../software/_/chapters/file_system.md#ファイルシステム)上では[ファイル](../../../software/_/chapters/file_system.md#ファイル)のメタデータを**iノード**で管理しており、**iノード番号**という固有の番号で実体のデータと紐付けられている。ハードリンクは元の[ファイル](../../../software/_/chapters/file_system.md#ファイル)と同じiノード番号を指す参照カウントとなっているため、[リンク](#リンク)先の[ファイル](../../../software/_/chapters/file_system.md#ファイル)が削除されても実体にアクセスできる。


## ファイルディスクリプタ

**ファイルディスクリプタ**は、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)がアクセスする[ファイル](../../../software/_/chapters/file_system.md#ファイル)や[標準入出力](./stdio_and_pipeline.md#標準入出力)などを[OS](../../../software/_/chapters/operating_system.md#オペレーティングシステム)が識別するために用いる識別子。 $0$ から順番に整数の値が割り当てられる。

ファイルディスクリプタには、識別子とともに[ファイル](../../../software/_/chapters/file_system.md#ファイル)名、[ファイル](../../../software/_/chapters/file_system.md#ファイル)サイズ、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)が操作中の[ファイル](../../../software/_/chapters/file_system.md#ファイル)内の位置、作成日時、更新日時などの情報が含まれている。

通常は、 $0$ に[標準入力](./stdio_and_pipeline.md#標準入力)、 $1$ に[標準出力](./stdio_and_pipeline.md#標準出力)、 $2$ に[標準エラー出力](./stdio_and_pipeline.md#標準エラー出力)が[OS](../../../software/_/chapters/operating_system.md#オペレーティングシステム)によって最初に用意されるため、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)が[ファイル](../../../software/_/chapters/file_system.md#ファイル)をオープンすると $3$ から順番にディスクリプタが割り当てられる。
