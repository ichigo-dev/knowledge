# 『リポジトリの作成』ノート

（最終更新： 2023-05-13）


## 目次

1. [リポジトリ](#リポジトリ)
	1. [ベアリポジトリ](#ベアリポジトリ)
	1. [ノンベアリポジトリ](#ノンベアリポジトリ)
1. [リポジトリの初期化](#リポジトリの初期化)
1. [クローン](#クローン)
1. [フォーク](#フォーク)


## リポジトリ

**リポジトリ**は、[Git](./git.md#git)の変更履歴を管理する[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)、あるいはデータ構造。リポジトリの配下の[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)は、履歴管理の対象となる。

開発チームや一般に公開されている[サーバ](../../../../computer/_/chapters/computer.md#サーバ)上のリポジトリを[リモートリポジトリ](./record_history.md#リモートリポジトリ)、ユーザの作業用[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)にコピーしたリポジトリを[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)という。

### ベアリポジトリ

**ベアリポジトリ**は、[ワークツリー](./record_history.md#ワークツリー)を持たない[リポジトリ](#リポジトリ)で、変更履歴の管理のみを行う。

### ノンベアリポジトリ

**ノンベアリポジトリ**は、[ワークツリー](./record_history.md#ワークツリー)を持つ、ユーザが作業を行う[リポジトリ](#リポジトリ)。


## リポジトリの初期化

`git init` は、[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)内で[Git](./git.md#git)の管理対象としたい任意の[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)を、[リポジトリ](#リポジトリ)として初期化するには[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# 新たにプロジェクト用のディレクトリを作成
$ cd <path>
$ mkdir <project-name>
$ cd <project-name>

# Gitリポジトリとして初期化
$ git init
Initialized empty Git repository in /path/to/working_directory/<project-name>/.git/
```

初期化を行うと、[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)配下に `.git` [ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)が作成される。


## クローン

**クローン**は、[リモートリポジトリ](./record_history.md#リモートリポジトリ)の複製をローカルに作成する操作。基本的にはプロジェクトに加わった際に、最初に1度だけ実行する操作であり、それ以降の[リモートリポジトリ](./record_history.md#リモートリポジトリ)からの差分の取得には[フェッチ](./repository_sync.md#フェッチ)や[プル](./repository_sync.md#プル)を用いる。

クローン元となった[リモートリポジトリ](./record_history.md#リモートリポジトリ)は、[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)に `origin` という[エイリアス](./remote_repository.md#エイリアス)で登録される。

[リポジトリ](#リポジトリ)のクローンには、 `git clone` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いる。

```sh
# クローンしたいディレクトリに移動
$ cd <path>

# Gitリポジトリをクローン
$ git clone <uri>

# Gitリポジトリを名前付きでクローン
$ git clone <uri> <name>

# クローンの例
$ git clone https://github.com/ichigo-dev/knowledge.git
```


## フォーク

**フォーク**は、[リモートリポジトリ](./record_history.md#リモートリポジトリ)の複製を別の[リモートリポジトリ](./record_history.md#リモートリポジトリ)として作成する操作。[クローン](#クローン)はあるプロジェクトに参加する場合などに用いるのに対し、フォークはあるプロジェクトを元に別のプロジェクトを作成したい場合に用いる。

実際にはワークフローの違いだけであり、新規に作成した[リモートリポジトリ](./record_history.md#リモートリポジトリ)に対して `git clone` した[リポジトリ](#リポジトリ)を[プッシュ](./repository_sync.md#プッシュ)することで、[リポジトリ](#リポジトリ)をフォークできる。
