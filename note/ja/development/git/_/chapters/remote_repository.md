# 『リモートリポジトリ』ノート

（最終更新： 2023-05-13）


## 目次

1. [リモートリポジトリ](#リモートリポジトリ)
	1. [エイリアス](#エイリアス)
	1. [origin](#origin)
1. [リモートリポジトリとの接続を一覧表示](#リモートリポジトリとの接続を一覧表示)
1. [リモートリポジトリとの接続を追加](#リモートリポジトリとの接続を追加)
1. [リモートリポジトリとの接続を削除](#リモートリポジトリとの接続を削除)
1. [リモートリポジトリとの接続を編集](#リモートリポジトリとの接続を編集)


## リモートリポジトリ

`git remote` は、紐づく[リモートリポジトリ](./record_history.md#リモートリポジトリ)の確認や、新たな接続の追加、既存の接続の削除などを行う[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)には、複数の[リモートリポジトリ](./record_history.md#リモートリポジトリ)を紐づけることができ、それぞれの[リポジトリ](./create_repository.md#リポジトリ)と相互に同期ができる。

[リモートリポジトリ](./record_history.md#リモートリポジトリ)のエントリリストは、 `.git/config` に記載されている。

### エイリアス

**エイリアス**（**ショートカット**）は、紐づく[リモートリポジトリ](./record_history.md#リモートリポジトリ)の[URL](../../../../network/_/chapters/web.md#url)に対してつけることができる任意の名前。

### origin

`origin` は、[クローン](./create_repository.md#クローン)元の[リモートリポジトリ](./record_history.md#リモートリポジトリ)を指す[エイリアス](#エイリアス)で、[クローン](./create_repository.md#クローン)を行った際に[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)に自動的にエントリが追加される。


## リモートリポジトリとの接続を一覧表示

`git remote` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いると、[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)に紐づく[リモートリポジトリ](./record_history.md#リモートリポジトリ)を確認できる。また、 `-v` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を追加することで詳細を確認することができる。

```sh
# リモートリポジトリとの接続を一覧表示
$ git remote

# リモートリポジトリとの接続の詳細を一覧表示
$ git remote -v
```


## リモートリポジトリとの接続を追加

`git remote add` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いることで、[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)に新たな[リモートリポジトリ](./record_history.md#リモートリポジトリ)のエントリを追加できる。エントリの追加の際には、任意の[エイリアス](#エイリアス)と[リモートリポジトリ](./record_history.md#リモートリポジトリ)の[URL](../../../../network/_/chapters/web.md#url)を指定する必要がある。

```sh
# リモートリポジトリとの接続を追加
$ git remote add <alias> <uri>
```


## リモートリポジトリとの接続を削除

`git remote rm` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いることで、[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)に紐づく[リモートリポジトリ](./record_history.md#リモートリポジトリ)のエントリを削除できる。削除対象の[エイリアス](#エイリアス)を指定して接続を削除する。

```git
# リモートリポジトリとの接続を削除
$ git remote rm <alias>
```


## リモートリポジトリとの接続を編集

`git remote rename` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いることで、[リモートリポジトリ](./record_history.md#リモートリポジトリ)のエントリの[エイリアス](#エイリアス)を変更できる。また、[リモートリポジトリ](./record_history.md#リモートリポジトリ)の[URL](../../../../network/_/chapters/web.md#url)を変更するには、 `git remote set-url` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いる。

```git
# リモートリポジトリとの接続を編集
$ git remote rename <old-alias> <new-alias>
$ git remote set-url <alias> <new-url>
```
