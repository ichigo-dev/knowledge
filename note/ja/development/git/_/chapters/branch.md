# 『ブランチ』ノート

（最終更新： 2023-03-03）


## 目次

1. [ブランチ](#ブランチ)
	1. [ブランチの確認](#ブランチの確認)
	1. [ブランチの作成](#ブランチの作成)
	1. [ブランチの名前変更](#ブランチの名前変更)
	1. [ブランチの削除](#ブランチの削除)
	1. [ブランチの切り替え](#ブランチの切り替え)
1. [マージ](#マージ)
	1. [Fast-forwardマージ](#fast-forwardマージ)
	1. [3wayマージ](#3wayマージ)
	1. [コンフリクト](#コンフリクト)
	1. [リベース](#リベース)
1. [プルリクエスト](#プルリクエスト)


## ブランチ

**ブランチ**は、[リポジトリ](./create_repository.md#リポジトリ)の[コミット](./record_history.md#コミット)履歴を分岐させる機能、あるいは枝分かれした流れ。一般的には管理の本流となるブランチを `master` ( `main` )とし、分岐させるブランチのことを**トピックブランチ**という。複数人が各自[バグ](../../../../programming/_/chapters/programming.md#バグ)修正や新機能の開発を並行して行うような場合に、各々がトピックブランチ上で作業を進めることで、他のユーザの作業に影響を受けることなく独立した状態を保つことができる。

### ブランチの確認

`git branch` は、[リポジトリ](./create_repository.md#リポジトリ)上の[ブランチ](#ブランチ)の一覧を表示する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定しない場合は[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)の[ブランチ](#ブランチ)を、 `-r` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定すると[リモートリポジトリ](./record_history.md#リモートリポジトリ)の[ブランチ](#ブランチ)を確認できる。また、 `-a` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[ローカルリポジトリ](./record_history.md#ローカルリポジトリ)と[リモートリポジトリ](./record_history.md#リモートリポジトリ)の両方を一覧表示できる。

```sh
# ローカルリポジトリのブランチを一覧表示
$ git branch

# リモートリポジトリのブランチを一覧表示
$ git branch -r

# ローカルリポジトリとリモートリポジトリのブランチを一覧表示
$ git branch -a
```

### ブランチの作成

`git branch` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)の[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)として[ブランチ](#ブランチ)名を指定することで、新しい[ブランチ](#ブランチ)を作成できる。

```sh
$ git branch <name>
```

### ブランチの名前変更

`git branch` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `-m` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[ブランチ](#ブランチ)名を変更できる。

```sh
$ git branch -m <old-name> <new-name>
```

### ブランチの削除

`git branch` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `-d` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[ブランチ](#ブランチ)を削除できる。

```sh
$ git branch -d <name>
```

また、[リモートリポジトリ](./record_history.md#リモートリポジトリ)の[ブランチ](#ブランチ)を削除したい場合は、 `git push` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--delete` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定する。

```sh
$ git push <remote> --delete <branch>
```

### ブランチの切り替え

`git swtich` は、指定した[ブランチ](#ブランチ)への切り替えを行う[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。切り換え先の `HEAD` の内容が[ワークツリー](./record_history.md#ワークツリー)に展開される。 `git swtich` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `-c` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、新規[ブランチ](#ブランチ)の作成も同時に行える。

```sh
# 指定のブランチに切り替え
$ git switch <branch>

# 新規ブランチを作成して、そのままそのブランチに切り替え
$ git switch <branch> -c
```

また、1つ前の[ブランチ](#ブランチ)を `-` で指定することもできる。

```sh
# 1つ前のブランチに切り替え
$ git switch -
```


## マージ

**マージ**は、ある[ブランチ](#ブランチ)の `HEAD` を別の[ブランチ](#ブランチ)に取り込む操作。[トピックブランチ](#ブランチ)の変更を本流となる[ブランチ](#ブランチ)に統合したり、本流となる[ブランチ](#ブランチ)の変更点を[トピックブランチ](#ブランチ)に取り込むといった使い方をする。

マージには `git merge` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いる。まずは `HEAD` をマージ先の[ブランチ](#ブランチ)に切り替え、 `git merge` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)の[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)にマージしたい[ブランチ](#ブランチ)名を指定する。

```sh
# 現在のブランチに指定のブランチをマージ
$ git merge <branch>

# 具体例
$ git switch main
$ git merge develop
```

### Fast-forwardマージ

**Fast-forwardマージ**（**早送りマージ**）は、[マージ](#マージ)先の[ブランチ](#ブランチ)の `HEAD` から[マージ](#マージ)したい[ブランチ](#ブランチ)の `HEAD` に向かって1本の直線的なパスのみが通っている場合に適用される戦略（[マージ](#マージ)したい[ブランチ](#ブランチ)以外の[ブランチ](#ブランチ)で[コミット](./record_history.md#コミット)が行われていない状態での[マージ](#マージ)）。この場合は、実際には差分の統合が行われているわけではなく、[マージ](#マージ)先の `HEAD` を[マージ](#マージ)したい[ブランチ](#ブランチ)の `HEAD` に移動する。これにより、実質的に全ての履歴が統合され、[マージ](#マージ)したい[ブランチ](#ブランチ)からアクセス可能であった全ての[コミット](./record_history.md#コミット)が[マージ](#マージ)先からも利用できるようになる。

### 3wayマージ

**3wayマージ**（**三方向マージ**）は、[マージ](#マージ)先の[ブランチ](#ブランチ)と[マージ](#マージ)したい[ブランチ](#ブランチ)の両方で[コミット](./record_history.md#コミット)が行われており、単純な[Fast-forwardマージ](#fast-forwardマージ)が実行できない場合に適用される戦略。この[マージ](#マージ)方法では、2つの[ブランチ](#ブランチ)の `HEAD` を統合するような新たな[マージ](#マージ)[コミット](./record_history.md#コミット)が作成される。

### コンフリクト

**コンフリクト**は、[3wayマージ](#3wayマージ)の際に同じ[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の同じ行に対する変更があった場合や、片方の[ブランチ](#ブランチ)で削除された[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)に対してもう片方の[ブランチ](#ブランチ)で編集を加えた場合などに、どちらの変更を優先すれば良いかを自動的に判断できなくなり[マージ](../../../../computer/software/_/chapters/file_system.md#ファイル)に失敗する現象。コンフリクトが発生した場合、コンフリクトが発生した[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の該当箇所に以下のようなマークが追加される。

```sh
<<<<<<< HEAD
// マージ先のブランチの変更
aaa
=======
// マージしたいブランチの変更
bbb
>>>>>>> <branch>
```

この部分を統合したい形に編集して保存し、[コミット](./record_history.md#コミット)を作成すると、コンフリクトしていた[マージ](#マージ)も完了した状態となる。また、 `git merge` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--abort` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[マージ](#マージ)を中止することもできる。

```sh
$ git merge --abort
```

### リベース

**リベース**は、[Fast-forwardマージ](#fast-forwardマージ)ができないような2つの[ブランチ](#ブランチ)を、[マージ](#マージ)[コミット](./record_history.md#コミット)を作成せずに統合する方法。統合元の[ブランチ](#ブランチ)の[コミット](./record_history.md#コミット)履歴の先頭に、統合先の[ブランチ](#ブランチ)の[コミット](./record_history.md#コミット)履歴の差分を全て直線的に追加する。これにより不要な[マージ](#マージ)[コミット](./record_history.md#コミット)が除去され、履歴を汚さずに済む。ただし、リベースを行うと統合元の[ブランチ](#ブランチ)の[コミット](./record_history.md#コミット)を打ち消してしまう可能性があるので、本流となる[ブランチ](#ブランチ)を自身が作業する[トピックブランチ](#ブランチ)に統合する場合などに限って利用すると良い。

リベースには `git rebase` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いる。まずは `HEAD` を統合先の[ブランチ](#ブランチ)に切り替え、 `git merge` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)の[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)に統合したい[ブランチ](#ブランチ)名を指定する。

```sh
# 現在のブランチに指定のブランチをリベース
$ git rebase <branch>

# 具体例
$ git switch my-work-branch
$ git rebase main
```

また、 `git rebase` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `-i` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、インタラクティブなリベースセッションを開始することができる。

```sh
# インタラクティブなリベースセッションを開始
$ git rebase main -i
```


## プルリクエスト

**プルリクエスト**は、[ブランチ](#ブランチ)の[マージ](#マージ)を他のユーザに通知し、[マージ](#マージ)を実行してよいか確認する機能。プルリクエストは[Git](./git.md#git)自身の機能ではなく、[GitHub](./git.md#gitホスティングサービス)などの[Gitホスティングサービス](./git.md#gitホスティングサービス)により提供されているため、環境によっては利用できない場合もある。

実装者は[Gitホスティングサービス](./git.md#gitホスティングサービス)上で[マージ](#マージ)したい[ブランチ](#ブランチ)を指定してプルリクエストを発行し、確認者が[ソースコード](../../../../programming/_/chapters/programming.md#ソースコード)のレビューを行う。確認者がプルリクエストを承認すると実際に[マージ](#マージ)が実行される。プルリクエストを活用することで、このようなコードレビューのフローが実施しやすくなる。
