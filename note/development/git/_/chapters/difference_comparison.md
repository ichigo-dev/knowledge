# 『差分比較』ノート

（最終更新： 2023-08-20）


## 目次

1. [差分の比較](#差分の比較)
	1. [差分の強調表示](#差分の強調表示)
	1. [変更ファイル名のみ表示](#変更ファイル名のみ表示)
1. [コミット間の差分](#コミット間の差分)
1. [ファイル間の差分](#ファイル間の差分)
1. [ブランチ間の差分](#ブランチ間の差分)
1. [特定のコミットの確認](#特定のコミットの確認)


## 差分の比較

**git diff**は、2つの入力データセットを比較して、その差分を出力する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定しなかった場合は、 `HEAD` と[ワークツリー](./record_history.md#ワークツリー)の差分を出力する。

```sh
$ git diff
```

### 差分の強調表示

`git diff` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--color-words` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、差分を強調表示することができる。

```sh
$ git diff --color-words
```

### 変更ファイル名のみ表示

`git diff` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--name-only` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、差分のある[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)名のみを表示することができる。

```sh
$ git diff --name-only
```


## コミット間の差分

`git diff` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定しなかった場合、[ステージ](./record_history.md#ステージ)と[ワークツリー](./record_history.md#ワークツリー)の差分が出力される。また、 `git diff` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--staged` （または `--cached` ）[オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、 `HEAD` と[ステージ](./record_history.md#ステージ)の差分を出力できる。

```sh
# ステージとワークツリーの差分
$ git diff

# HEADとステージの差分
$ git diff --staged
$ git diff --cached
```

[コミットハッシュ](./record_history.md#コミットハッシュ)を指定することで、[ワークツリー](./record_history.md#ワークツリー)と指定の[コミット](./record_history.md#コミット)や、2つの[コミット](./record_history.md#コミット)間の差分を出力できる。

```sh
# ワークツリーと指定コミットの差分
$ git diff <commit-hash>

# 2つのコミット間の差分
$ git diff <commit-hash-1> <commit-hash-2>
$ git diff <commit-hash-1>..<commit-hash-2>
```


## ファイル間の差分

`git diff` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)の最後に[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)として[ファイルパス](../../../../computer/software/_/chapters/file_system.md#パス)を指定することで、特定の[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の差分のみを確できる。

```sh
$ git diff <path>
```

また、 `git diff` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--no-index` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定し、2つの[ファイルパス](../../../../computer/software/_/chapters/file_system.md#パス)を与えることで、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)間の差分も出力できる。

```sh
$ git diff --no-index <path-1> <path-2>
```

[コミット](./record_history.md#コミット)の特定[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を指定したい場合は、 `<commit-hash>:<path>` のように記述すれば良い。


## ブランチ間の差分

`git diff` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)の[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)として[ブランチ](./branch.md#ブランチ)名を指定した場合、現在の[ブランチ](./branch.md#ブランチ)と指定の[ブランチ](./branch.md#ブランチ)の差分を出力できる。また、2つの[ブランチ](./branch.md#ブランチ)名を指定することで、指定した[ブランチ](./branch.md#ブランチ)間の差分を出力できる。

```sh
# 現在のブランチと指定ブランチの差分
$ git diff <branch>

# 2つの指定ブランチの差分
$ git diff <branch-1> <branch-2>
```


## 特定のコミットの確認

**git show**は、[コミットハッシュ](./record_history.md#コミットハッシュ)を指定することで、指定した[コミット](./record_history.md#コミット)と直前の[コミット](./record_history.md#コミット)を `git diff` した結果を出力できる。

```sh
# HEADとHEAD~1のdiffを出力
$ git show
$ git show HEAD

# 特定のコミットとその1つ前のコミットのdiffを出力
$ git show <commit-hash>
```

また、[ファイルパス](../../../../computer/software/_/chapters/file_system.md#パス)を指定することで、指定の[コミット](./record_history.md#コミット)時点での[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の内容を確認できる。

```sh
$ git show <commit-hash>:<path>
```
