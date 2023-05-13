# 『リポジトリの同期』ノート

（最終更新： 2023-05-13）


## 目次

1. [プッシュ](#プッシュ)
1. [フェッチ](#フェッチ)
1. [プル](#プル)


## プッシュ

**プッシュ**(Push)は、自身の[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)の[コミット](./record_history.md#コミット)履歴を、[リモートリポジトリ](./record_history.md#リモートリポジトリ)にアップロードする操作。

プッシュは、 `git push` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いて、[リモートリポジトリ](./record_history.md#リモートリポジトリ)と[ブランチ](./branch.md#ブランチ)を指定して行う。

```sh
# リモートリポジトリ、ブランチを指定してプッシュ
$ git push <remote> <branch>

# 具体例
$ git push origin main
```

`-f` または `--force` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[リモートリポジトリ](./record_history.md#リモートリポジトリ)の[コミット](./record_history.md#コミット)履歴を強制的に上書きすることもできるが、基本的には使わないようにした方がよい。


## フェッチ

**フェッチ**(Fetch)は、[リモートリポジトリ](./record_history.md#リモートリポジトリ)の[コミット](./record_history.md#コミット)履歴を、[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)の[トピックブランチ](./branch.md#ブランチ)にダウンロードする操作。

フェッチは、 `git fetch` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いて、[リモートリポジトリ](./record_history.md#リモートリポジトリ)と[ブランチ](./branch.md#ブランチ)を指定して行う。フェッチを行うと、変更は即座に同名の[ブランチ](./branch.md#ブランチ)に反映されるわけではなく、 `<remote>/<branch>` という名前の[トピックブランチ](./branch.md#ブランチ)に取り込まれる。任意の[ブランチ](./branch.md#ブランチ)に変更を取り込みたい場合、[マージ](./branch.md#マージ)を行う必要がある。

```sh
# リモートリポジトリ、ブランチを指定してフェッチ
$ git fetch <remote> <branch>

# 具体例
# git fetch origin main
```


## プル

**プル**(Pull)は、[リモートリポジトリ](./record_history.md#リモートリポジトリ)の[コミット](./record_history.md#コミット)履歴を、[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)の同名[ブランチ](./branch.md#ブランチ)に即座にダウンロードする操作。

プルは、 `git pull` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いて、[リモートリポジトリ](./record_history.md#リモートリポジトリ)と[ブランチ](./branch.md#ブランチ)を指定して行う。プルは[フェッチ](#フェッチ)と[マージ](./branch.md#マージ)を組み合わせた操作で、[リモートリポジトリ](./record_history.md#リモートリポジトリ)の変更を `<remote>/<branch>` という名前の[トピックブランチ](./branch.md#ブランチ)に一時的に取り込み、[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)の同名の[ブランチ](./branch.md#ブランチ)に[マージ](./branch.md#マージ)する。

```sh
# リモートリポジトリ、ブランチを指定してプル
$ git pull <remote> <branch>

# 具体例
# git pull origin main
```
