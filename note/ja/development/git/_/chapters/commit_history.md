# 『コミット履歴』ノート

（最終更新： 2023-05-13）


## 目次

1. [履歴の確認](#履歴の確認)
	1. [コミット内容の確認](#コミット内容の確認)
	1. [コミットの統計情報の確認](#コミットの統計情報の確認)
	1. [変更ファイルの確認](#変更ファイルの確認)
	1. [コミットの相対日時の確認](#コミットの相対日時の確認)
	1. [ブランチとマージの履歴](#ブランチとマージの履歴)
	1. [履歴のフォーマット](#履歴のフォーマット)
	1. [履歴表示の制限](#履歴表示の制限)
1. [コミットの打ち消し](#コミットの打ち消し)
1. [コミットの取り消し](#コミットの取り消し)
1. [Gitの操作履歴](#gitの操作履歴)


## 履歴の確認

`git log` は、[コミット](./record_history.md#コミット)履歴を確認するための[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ git log
commit xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx (HEAD -> main, origin/main, origin/HEAD)
Author: name <example@example.com>
Date:   Thu Feb 2 00:00:00 2023

    feat: Add new utility functions

commit xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
Author: name <example@example.com>
Date:   Wed Feb 1 00:00:00 2023

    Initial commit
```

### コミット内容の確認

`git log` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `-p` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[コミット](./record_history.md#コミット)の変更内容を確認できる。

```sh
# コミット履歴の変更内容を表示
$ git log -p

# コミット履歴の変更内容を2件表示
$ git log -p -2
```

### コミットの統計情報の確認

`git log` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--stat` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)あるいは `--shortstat` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[コミット](./record_history.md#コミット)履歴の各[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)に対する変更の統計情報（変更された[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の数、追加・削除された行数）を確認できる。

```sh
$ git log --stat
commit xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx (HEAD -> main, origin/main, origin/HEAD)
Author: name <example@example.com>
Date:   Thu Feb 1 00:00:00 2023

    Example commit

 example1.txt                                                  | 10 +++++++++
 example2.txt                                                  | 12 ++++++++---

$ git log --shortstat
commit xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx (HEAD -> main, origin/main, origin/HEAD)
Author: name <example@example.com>
Date:   Thu Feb 1 00:00:00 2023

    Example commit

 2 files changed, 19 insertions(+), 3 deletions(-)
```

### 変更ファイルの確認

`git log` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--name-only` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)あるいは `--name-status` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[コミット](./record_history.md#コミット)履歴とその時の変更[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を確認できる 。

```sh
# 変更したファイルを一覧表示
$ git log --name-only

# 変更したファイルと変更情報（追加、修正、削除）を一覧表示
$ git log --name-status
```

### コミットの相対日時の確認

`git log` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--relative-date` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[コミット](./record_history.md#コミット)履歴に表示される[コミット](./record_history.md#コミット)日時を相対フォーマット（ `2 weeks ago` など）で表示できる。

### ブランチとマージの履歴

`git log` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--graph` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[ブランチ](./branch.md#ブランチ)と[マージ](./branch.md#マージ)の履歴をアスキーアートで表示できる。

```sh
$ git log --graph
```

### 履歴のフォーマット

`git log` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--pretty` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[コミット](./record_history.md#コミット)履歴の出力を任意のフォーマットに置き換えることができる。

```sh
# コミット履歴を1行で表示
$ git log --pretty=oneline

# コミット履歴を任意のフォーマットで表示
$ git log --pretty=format:"%h - %an, %ar : %s"
```

### 履歴表示の制限

`git log` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に数字を[オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)として指定することで、表示する履歴の数を限定できる 。

```sh
# 2件の履歴を表示
$ git log -2
```

他にも、[コミット](./record_history.md#コミット)履歴の絞り込みには次のような[オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)が用意されている。

| オプション                | 概要                                                                     |
|---------------------------|--------------------------------------------------------------------------|
| `--since` , `--after`     | 指定した日付以降のコミットを絞り込む                                     |
| `--until` , `--before`    | 指定した日付以前のコミットを絞り込む                                     |
| `--author` , `--commiter` | 指定した対象のコミットを絞り込む                                         |
| `--grep`                  | 指定した文字列がコミットメッセージに含まれるコミットを絞り込む           |
| `-S`                      | 指定した文字列がコミットメッセージや変更内容に含まれるコミットを絞り込む |


## コミットの打ち消し

`git revert` は、任意の[コミット](./record_history.md#コミット)を打ち消すための[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[コミット](./record_history.md#コミット)履歴を削除するのではなく、特定の[コミット](./record_history.md#コミット)を打ち消すような新しい[コミット](./record_history.md#コミット)を作成する。[コミット](./record_history.md#コミット)履歴を破壊しないため、[リモートリポジトリ](./record_history.md#リモートリポジトリ)と同期済みの[コミット](./record_history.md#コミット)を取り消しても安全。また、この[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)では特定の[コミット](./record_history.md#コミット)のみを打ち消すため、指定した[コミット](./record_history.md#コミット)以降のすべての変更内容を元に戻すわけではない。

```sh
# 任意のコミットを打消し
$ git revert <commit-hash>

# 具体例
$ git revert HEAD
$ git revert HEAD~3
```


## コミットの取り消し

`git reset` は、[コミット](./record_history.md#コミット)を取り消すための[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[ブランチ](./branch.md#ブランチ)上の `HEAD` の位置を移動し、[オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)に応じて[ステージ](./record_history.md#ステージ)と[ワークツリー](./record_history.md#ワークツリー)の状態も変更する。

| オプション | 概要                                                                                      |
|------------|-------------------------------------------------------------------------------------------|
| `--hard`   | `HEAD` を指定の位置に移動し、ステージとワークツリーをその時の状態に戻す                   |
| `--mixed`  | `HEAD` を指定の位置に移動し、ステージをその時の状態に戻す（ワークツリーの変更内容は維持） |
| `--soft`   | `HEAD` を指定の位置に移動する（ステージとワークツリーの変更内容は維持）                   |

```sh
# 任意のコミットまで変更内容を取り消し
$ git reset <commit-hash>

# 具体例
$ git reset HEAD
$ git reset HEAD~3
```


## Gitの操作履歴

`git reflog` は、[Git](./git.md#git)の操作履歴を確認するための[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[Git](./git.md#git)では操作履歴に対してもハッシュがついており、 `git reset` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)にハッシュあるいは `HEAD@{0}` のようなエイリアスを指定することで、その操作を取り消すとこができる。誤って必要な[ブランチ](./branch.md#ブランチ)を消してしまったり、[ワークツリー](./record_history.md#ワークツリー)の変更内容を破棄してしまった場合などに、その操作を取り消すことで復旧できる。

```sh
$ git reflog
xxxxxxx (HEAD -> main, origin/main, origin/HEAD) HEAD@{0}: commit: example commit
xxxxxxx HEAD@{1}: pull origin main: Fast-forward
xxxxxxx HEAD@{2}: checkout: moving from exampole to main
```
