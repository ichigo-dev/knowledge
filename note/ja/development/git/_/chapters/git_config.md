# 『Gitの設定』ノート

（最終更新： 2023-05-12）


## 目次

1. [Gitの設定](#gitの設定)
	1. [設定のレベル](#設定のレベル)
	1. [セクション](#セクション)
	1. [キー](#キー)
1. [設定の確認](#設定の確認)
1. [設定の変更](#設定の変更)


## Gitの設定

[Git](./git.md#git)は設定によって、[コミット](./record_history.md#コミット)時に記録されるユーザ名やメールアドレス、[リモートリポジトリ](./record_history.md#リモートリポジトリ)との接続に関するオプションなどを変更することができる。

設定は、[ホームディレクトリ](../../../../computer/software/_/chapters/file_system.md#ホームディレクトリ)や各[リポジトリ](./create_repository.md#リポジトリ)の `.git` [ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)にある設定[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)に記述することで適用される。また、 `git config` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)によって設定の確認や変更が行える。

### 設定のレベル

[Git](./git.md#git)の設定にはレベルがあり、異なるレベルに同じ設定が登録されている場合には `local > global > system` の順に優先して適用される。

| レベル | 適用範囲                           | 設定ファイルの場所(Linux)               | オプション |
|--------|------------------------------------|-----------------------------------------|------------|
| system | システム上の全ユーザの全リポジトリ | `/etc/gitconfig`                        | `--system` |
| global | ログイン中のユーザの全リポジトリ   | `~/.gitconfig` , `~/.config/git/config` | `--global` |
| local  | 各リポジトリ内                     | `<path>/.git/config`                    | `--local`  |

### セクション

**セクション**は、いくつかの設定値をまとめたグループ。

### キー

**キー**は、[セクション](#セクション)内の個別の設定値。


## 設定の確認

`git config` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--list` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、登録されている設定値の一覧を確認できる。また、各[レベル](#設定のレベル)に対応した[オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)をつけることで、その[レベル](#設定のレベル)の設定値の一覧を確認することもできる。

```sh
# 設定値の一覧を確認
$ git config --list

# globalレベルの設定値の一覧を確認
$ git config --list --global
```

また、[セクション](#セクション)と[キー](#キー)を指定することで、個別の設定項目の値を確認することもできる。

```sh
# 個別の設定項目の値を確認
$ git <section>.<key>

# 具体例
$ git user.email
```


## 設定の変更

`git config` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に変更したい項目の[セクション](#セクション)と[キー](#キー)、登録したい値を与えることで、設定を変更できる。

```sh
# 設定値の変更
$ git config <section>.<key> <value>

# 具体例
$ git config user.email "example@example.com"
```
