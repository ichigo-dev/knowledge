# 『Git』ノート

（最終更新： 2023-05-12）


## 目次

1. [バージョン管理システム](#バージョン管理システム)
	1. [集中管理方式](#集中管理方式)
	1. [分散管理方式](#分散管理方式)
1. [Git](#git)
	1. [作業の流れ](#作業の流れ)
1. [Gitのインストール](#gitのインストール)
	1. [初期設定](#初期設定)
	1. [GUIソフトウェア](#guiソフトウェア)
1. [Gitホスティングサービス](#gitホスティングサービス)


## バージョン管理システム

**バージョン管理システム**(**VCS**: Version Control System)は、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の変更履歴を管理するための[システム](../../../../system/_/chapters/system.md#システム)。[プログラム](../../../../programming/_/chapters/programming.md#プログラム)の[ソースコード](../../../../programming/_/chapters/programming.md#ソースコード)やドキュメントなどの電子ファイルは、通常は段階を経て編集されていく。バージョン管理システムを用いることで、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の編集の過程を履歴として記録し、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の過去の状態を確認したり、その時の状態に戻したりすることができるようになる。また複数人で同じ[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を編集したいような場合に、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の同じ行を変更してしまうことによる競合などの問題が発生する可能性があるが、バージョン管理システムはこのような状況を解決する仕組みも持っているものも多い。バージョン管理システムは[プログラム](../../../../programming/_/chapters/programming.md#プログラム)の開発にとどまらず、電子データを扱う全てのケースで活用できるツールである。

### 集中管理方式

**集中管理方式**（**集中型**）の[バージョン管理システム](#バージョン管理システム)は、バージョン管理のための専用[サーバ](../../../../computer/_/chapters/computer.md#サーバ)に[リポジトリ](./create_repository.md#リポジトリ)を用意しておき、その[サーバ](../../../../computer/_/chapters/computer.md#サーバ)上で履歴を一元管理する。ユーザは[サーバ](../../../../computer/_/chapters/computer.md#サーバ)から自身の[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)に[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)のコピーをダウンロードし、変更を加えたらその変更を[サーバ](../../../../computer/_/chapters/computer.md#サーバ)上の[リポジトリ](./create_repository.md#リポジトリ)に記録する。

ユーザは[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)に繋がっていない状態だと、変更を記録することができない。また、[サーバ](../../../../computer/_/chapters/computer.md#サーバ)が[単一障害点](../../../../system/_/chapters/system_architecture.md#単一障害点)となっており、[サーバ](../../../../computer/_/chapters/computer.md#サーバ)上の[リポジトリ](./create_repository.md#リポジトリ)が破損するとデータが復旧できない可能性がある。

集中管理方式の[バージョン管理システム](#バージョン管理システム)としては、**CVS**や**SVN**が代表的。

### 分散管理方式

**分散管理方式**（**分散型**）の[バージョン管理システム](#バージョン管理システム)は、全ユーザの変更を集約する中央[サーバ](../../../../computer/_/chapters/computer.md#サーバ)に[リポジトリ](./create_repository.md#リポジトリ)を設置し、個々の作業用[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)にはその[リポジトリ](./create_repository.md#リポジトリ)の複製（[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)）を作成する。変更履歴の管理は各々の[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)上で行われ、[リモートリポジトリ](./record_history.md#リモートリポジトリ)と同期することで変更内容を共有する。

ユーザは[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)に接続されていなくても[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)上でほとんどの操作を行うことができる。また、各[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)上に[リポジトリ](./create_repository.md#リポジトリ)の複製が存在するため、中央[サーバ](../../../../computer/_/chapters/computer.md#サーバ)の[リポジトリ](./create_repository.md#リポジトリ)が破損した場合の復旧が容易である。

分散管理方式の[バージョン管理システム](#バージョン管理システム)としては、[Git](#git)や**Mercurial**が代表的。


## Git

**Git**は、開発におけるデファクトスタンダードとなっている[分散型](#分散管理方式)[バージョン管理システム](#バージョン管理システム)。もともとは[Linux](../../../../computer/software/_/chapters/operating_system.md#linux)の開発のためにLinus Torvaldsによって実装され、巨大な[ソースコード](../../../../programming/_/chapters/programming.md#ソースコード)であっても変更点の抽出や[リポジトリ](./create_repository.md#リポジトリ)操作が高速にできるように工夫されている。

### 作業の流れ

[Git](#git)を用いたバージョン管理の一般的な流れは次の通り。

1. 中央[サーバ](../../../../computer/_/chapters/computer.md#サーバ)に全体の履歴を集約する[リポジトリ](./create_repository.md#リポジトリ)を作成する
1. [リモートリポジトリ](./record_history.md#リモートリポジトリ)の複製を[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)として各ユーザの作業[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)に作成する
1. ローカル環境にて[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の修正や追加、削除などを行い、[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)に変更履歴を記録する
1. [ローカルリポジトリ](./record_history.md#ローカルリポジトリ)の変更履歴を[リモートリポジトリ](./record_history.md#リモートリポジトリ)に反映する
1. 他のユーザが[リモートリポジトリ](./record_history.md#リモートリポジトリ)に加えた変更内容を[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)に反映し、最新の状態にする
1. 3～5を繰り返し、バージョン管理を行う


## Gitのインストール

[Git](#git)が既に自身の環境にインストールされているかを確認するには、[ターミナル](../../../../computer/linux/_/chapters/shell_and_terminal.md#ターミナル)[ソフトウェア](../../../../computer/software/_/chapters/software.md応用ソフトウェア)（Macのターミナル、Windowsのコマンドプロンプト等）を起動し、次の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sh
$ git --version
git version 2.40.1
```

上の例のように、[Git](#git)の[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)情報が表示されれば、[Git](#git)は既に使用できる状態となっている。[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)が存在しない旨のエラーが表示される場合は、[Git](#git)をインストールする必要がある。

- [Gitのインストール](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

上記サイトより、自身の環境の[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)に応じたインストール手順を実施し、改めて `git --version` を実行して[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)情報が出力されることを確認する。

### 初期設定

[Git](#git)を使い始めるには、最初に[コミット](./record_history.md#コミット)時に履歴に登録されるユーザ名とメールアドレスを設定する必要がある。[Git](#git)の設定変更には、 `git config` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いる。

```sh
# ユーザ名を設定
$ git config --global user.name <name>

# メールアドレスを設定
$ git config --global user.email <email>
```

また、[コミット](./record_history.md#コミット)時などに使用するエディタを変更したい場合は、次のように設定する。

```sh
$ git config --global core.editor <editor>
```

初期[ブランチ](./branch.md#ブランチ)名を変更したい場合の設定は次のように行う。

```sh
$ git config --global init.defaultBranch <name>
```

設定した内容を確認するには、 `git config` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--list` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)をつける。

```sh
$ git config --list
```

### GUIソフトウェア

[Git](#git)[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)は、[ターミナル](../../../../computer/linux/_/chapters/shell_and_terminal.md#ターミナル)[ソフトウェア](../../../../computer/software/_/chapters/software.md応用ソフトウェア)上で[CLI](../../../../computer/software/_/chapters/software.md#cui)として利用できる。[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)による操作が苦手な場合は、[Git](#git)用の[GUI](../../../../computer/software/_/chapters/software.md#gui)[ソフトウェア](../../../../computer/software/_/chapters/software.md応用ソフトウェア)をインストールすることで直感的な操作が可能となる。

代表的な[Git](#git)の[GUI](../../../../computer/software/_/chapters/software.md#gui)クライアント[ソフトウェア](../../../../computer/software/_/chapters/software.md応用ソフトウェア)は以下の通り。

- [SourceTree](https://www.sourcetreeapp.com/)
- [Git Fork](https://git-fork.com/)
- [GitKraken](https://www.gitkraken.com/)


## Gitホスティングサービス

**Gitホスティングサービス**は、クラウド上で[Git](#git)の環境が整えられた[サーバ](../../../../computer/_/chapters/computer.md#サーバ)をサービスとして提供した[SaaS](../../../../system/_/chapters/system_architecture.md#saas)。Gitホスティングサービスを利用することで、自身で[サーバ](../../../../computer/_/chapters/computer.md#サーバ)を用意することなく手軽に[ソースコード](../../../../programming/_/chapters/programming.md#ソースコード)を共有するための[リモートリポジトリ](./record_history.md#リモートリポジトリ)を用意することができる。

代表的なGitホスティングサービスとしては**GitHub**や**GitLab**、**BitBucket**などがある。

- [GitHub](https://github.co.jp/)
- [GitLab](https://about.gitlab.com/)
- [BitBucket](https://bitbucket.org/)
