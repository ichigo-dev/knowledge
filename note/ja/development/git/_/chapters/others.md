# 『その他の機能』ノート

（最終更新： 2023-05-13）


## 目次

1. [ワークツリーの退避](#ワークツリーの退避)
	1. [スタッシュの一覧表示](#スタッシュの一覧表示)
	1. [スタッシュの復元](#スタッシュの復元)
	1. [スタッシュの確認](#スタッシュの確認)
	1. [スタッシュの削除](#スタッシュの削除)
1. [タグ](#タグ)
	1. [タグの一覧表示](#タグの一覧表示)
	1. [タグの作成](#タグの作成)
	1. [タグの削除](#タグの削除)
	1. [タグの共有](#タグの共有)


## ワークツリーの退避

`git stash` は、[ワークツリー](./record_history.md#ワークツリー)に加えた変更を一時的に退避するための[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[ワークツリー](./record_history.md#ワークツリー)の変更内容を**スタッシュ**領域に退避しておき、緊急の作業等を行った後にスタッシュの内容を復元する、といった使い方ができる。また、[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)にコメントをつけることで、複数のスタッシュが管理しやすくなる。

```sh
# ワークツリーをスタッシュに退避
$ git stash

# ワークツリーをコメント付きでスタッシュに退避
$ git stash "<comment>"
```

### スタッシュの一覧表示

`git stash list` は、[スタッシュ](#ワークツリーの退避)に退避した変更内容を一覧表示する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ git stash list
```

### スタッシュの復元

`git stash apply` や `git stash pop` は、[スタッシュ](#ワークツリーの退避)に退避した変更内容を[ワークツリー](./record_history.md#ワークツリー)に反映する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。 `git stash apply` は[スタッシュ](#ワークツリーの退避)の内容を維持したまま[ワークツリー](./record_history.md#ワークツリー)に反映し、 `git stash pop` は[スタッシュ](#ワークツリーの退避)の内容を[ワークツリー](./record_history.md#ワークツリー)に反映した後に破棄される。また、 `git stash list` で確認できる[スタッシュ](#ワークツリーの退避)の識別子を指定して[スタッシュ](#ワークツリーの退避)を復元することもできる。


```sh
# 直前のスタッシュを復元
$ git stash apply
$ git stash pop

# 指定のスタッシュを復元
$ git stash apply <stash>
$ git stash pop <stash>
```

### スタッシュの確認

`git stash show` は、[スタッシュ](#ワークツリーの退避)の変更[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定しなければ直前の[スタッシュ](#ワークツリーの退避)の変更[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を表示し、[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)に[スタッシュ](#ワークツリーの退避)の識別子を指定すると任意の[スタッシュ](#ワークツリーの退避)の変更[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を表示できる。また、 `git stash show` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `-p` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[スタッシュ](#ワークツリーの退避)の変更内容を確認できる。

```sh
# 直前のスタッシュの変更ファイルを確認
$ git stash show

# 指定のスタッシュの変更ファイルを確認
$ git stash show <stash>

# 直前のスタッシュの変更を確認
$ git stash show -p

# 指定のスタッシュの変更を確認
$ git stash show <stash> -p
```

### スタッシュの削除

`git stash drop` は、[スタッシュ](#ワークツリーの退避)を削除する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)として[スタッシュ](#ワークツリーの退避)の識別子を指定することで、任意の[スタッシュ](#ワークツリーの退避)を削除できる。また、 `git stash clear` を用いることで全ての[スタッシュ](#ワークツリーの退避)を一括で削除できる。


```sh
# 指定のスタッシュを削除
$ git stash drop <stash>

# スタッシュを一括削除
$ git stash clear
```


## タグ

**タグ**は、特定の[コミット](./record_history.md#コミット)をマークすることで参照しやすくするための機能。**軽量タグ**では、ある特定の[コミット](./record_history.md#コミット)に対して名前を付けることができる。**注釈付きタグ**では、タグ名に加えて作成者や作成日、コメントといった情報を付与することができる。タグは[コミットハッシュ](./record_history.md#コミットハッシュ)のエイリアスとしても利用できる。

### タグの一覧表示

`git tag` は、登録されている[タグ](#タグ)の一覧を表示する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ git tag
```

### タグの作成

`git tag` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)の[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)として任意の[タグ](#タグ)名を与えることで、 `HEAD` に対して[軽量タグ](#タグ)を作成できる。さらに、 `-a` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで[注釈付きタグ](#タグ)を作成したり、[タグ](#タグ)名の後に[コミットハッシュ](./record_history.md#コミットハッシュ)を指定することで任意の[コミット](./record_history.md#コミット)に[タグ](#タグ)をつけることもできる。

```sh
# 軽量タグの作成
$ git tag <tagname>

# 注釈付きタグの作成
$ git tag -a <tagname>

# コメントをインラインで指定して注釈付きタグを作成
$ git tag -a <tagname> -m <comment>

# 過去のコミットに対してタグを作成
$ git tag <tagname> <commit-hash>
$ git tag -a <tagname> <commit-hash>
$ git tag -a <tagname> <commit-hash> -m <comment>
```

### タグの削除

`git tag` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `-d` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、任意の[タグ](#タグ)を削除できる。

```sh
$ git tag -d <tagname>
```

### タグの共有

`git push` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に[引数](../../../../computer/linux/_/chapters/basic_command.md#オプション)としてリモート名と[タグ](#タグ)名を指定することで、[リモートリポジトリ](./record_history.md#リモートリポジトリ)に対して[タグ](#タグ)を共有することができる。
