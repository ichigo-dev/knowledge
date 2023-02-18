# 基本コマンド


## 目次

1. [リポジトリの作成](#リポジトリの作成)
	1. [初期化](#初期化)
	1. [クローン](#クローン)
1. [変更内容の記録](#変更内容の記録)
	1. [ステージング](#ステージング)
	1. [コミット](#コミット)
1. [リポジトリの同期](#リポジトリの同期)
	1. [紐づけ](#紐づけ)
	1. [プッシュ](#プッシュ)
	1. [プル](#プル)
	1. [フェッチ](#フェッチ)
1. [状態の確認](#状態の確認)
	1. [ステータス](#ステータス)
	1. [コミット履歴](#コミット履歴)
	1. [コミット詳細](#コミット詳細)
1. [差分の比較](#差分の比較)
1. [変更の退避](#変更の退避)
1. [変更の破棄](#変更の破棄)
1. [ブランチ](#ブランチ)
	1. [ブランチ操作](#ブランチ操作)
	1. [ブランチの切り替え](#ブランチの切り替え)
	1. [ブランチの統合](#ブランチの統合)
1. [タグ](#タグ)
1. [チェックアウト](#チェックアウト)
1. [コミットの取り消し](#コミットの取り消し)


## リポジトリの作成

Gitリポジトリを作成して変更履歴の管理を始めるには、自身のコンピュータの管理対象としたいディレクトリをリポジトリとして初期化する方法と、既に存在するリポジトリをサーバからクローンする方法がある。

### 初期化

`init` は自身のコンピュータ上のディレクトリをGitリポジトリとして初期化するコマンド。

```sh
# 新たにプロジェクトディレクトリを作成
$ cd <path>
$ mkdir <name>
$ cd <name>

# Gitリポジトリとして初期化
$ git init
Initialized empty Git repository in /path/to/working_directory/<name>/.git/
```

### クローン

`clone` は既存のリモートリポジトリをローカルリポジトリに複製するコマンド。

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


## 変更内容の記録

変更内容をリポジトリに記録するには、ワーキングディレクトリの変更箇所のうちコミットの対象としたいものをステージングエリアに追加してコミットを作成する。

### ステージング

`add` はワーキングディレクトリの変更をステージングエリアに移動するコマンド。

```sh
# ファイル、ディレクトリのパス指定でステージングエリアに移動
$ git add <path>

# すべての変更をステージングエリアに移動
$ git add .
```

### コミット

`commit` はステージングエリアの変更をローカルリポジトリに記録するコマンド。

```sh
# 変更をコミット（エディタが開かれてコミットメッセージの入力に移る）
$ git commit

# コミットメッセージ付きで変更をコミット
$ git commit -m "<message>"

# 直前のコミットメッセージを変更
$ git commit --amend -m "<message>"
```


## リポジトリの同期

ローカルリポジトリとリモートリポジトリはそれぞれ独立して履歴が管理されているため、定期的にそれぞれの変更内容を同期する必要がある。

#### 紐づけ

`remote` はローカルリポジトリとリモートリポジトリの紐づけに関するコマンド。

```sh
# 紐づいているリモートリポジトリを確認
$ git remote
$ git remote -v

# リモートリポジトリとの紐づけを追加
$ git remote add <name> <uri>

# リモートリポジトリとの紐づけを解除
$ git remote remove <name>

# リモートリポジトリとの紐づけをリネーム
$ git remote rename <old-name> <new-name>
```

### プッシュ

`push` はローカルリポジトリのコミット履歴をリモートリポジトリに反映するコマンド。

```sh
# リモートリポジトリ、ブランチ指定でプッシュ
$ git push <remote> <branch>
```

### プル

`pull` はリモートリポジトリのコミット履歴をローカルリポジトリに取り込み、ワーキングディレクトリとステージングエリアにも反映するコマンド。フェッチとマージを組み合わせた動作となる。

```sh
# リモートリポジトリ、ブランチ指定でプル
$ git pull <remote> <branch>
```

### フェッチ

`fetch` はリモートリポジトリのコミット履歴をローカルリポジトリのトピックブランチに取り込むコマンド。

```sh
# リモートリポジトリの変更を<remote>/<branch>という名前のブランチに取り込む
$ git fetch <remote> <branch>
```


## 状態の確認

Gitにはワーキングディレクトリとステージングエリアとコミット履歴の状態を確認するためのコマンドが用意されている。

### ステータス

`status` は現在のワーキングディレクトリとステージングエリアの状態を確認するコマンド。

```sh
# 変更のあったファイルを一覧表示
$ git status
```

### コミット履歴

`log` はコミット履歴を表示するコマンド。

```sh
# 現在のブランチのコミット履歴を表示
$ git log

# ブランチ、ファイル名指定でコミット履歴を表示
$ git log <branch>
$ git log <file>

# コミットメッセージとコミット内容で検索
$ git log -S "<search>"

# コミットメッセージで検索
$ git log --grep="<search>"
```

### コミット詳細

`show` はコミットのハッシュを指定して変更内容を参照するコマンド。

```sh
# コミットハッシュ(or HEAD)を指定して変更内容を参照
$ git show <hash>
```


## 差分の比較

`diff` はリポジトリの2つのデータセットの差分を確認するためのコマンド。比較の対象としてはコミットやブランチ、ファイルなどを指定することができる。

```sh
# ワーキングディレクトリとステージングエリアの差分を比較
$ git diff

# ステージングエリアとHEADの差分を比較
$ git diff --staged

# ワーキングディレクトリとHEADの差分を比較
$ git diff HEAD

# ワーキングディレクトリと指定のコミットの差分を比較
$ git diff <hash>

# ステージングエリアと指定のコミットの差分を比較
$ git diff <hash> --staged

# 2つのコミットの差分を比較
$ git diff <hash1> <hash2>

# 2つのブランチの差分を比較
$ git diff <branch1> <branch2>

# 差分をゲージで比較
$ git diff --stat

# ファイルパス指定で差分を比較
$ git diff <path>
```


## 変更の退避

`stash` はコミット前の変更を一時的に退避するためのコマンド。ワーキングディレクトリをスタッシュに退避しておき、一時的な作業を行った後にスタッシュの内容を復元したい場合などに用いる。

```sh
# 作業内容をスタッシュに退避
$ git stash

# 作業内容をコメント付きでスタッシュに退避
$ git stash "<message>"

# スタッシュの一覧を表示
$ git stash list

# スタッシュから変更内容を復元（スタッシュに残す）
$ git stash apply
$ git stash apply <stash>

# スタッシュから変更内容を復元（スタッシュから削除）
$ git stash pop
$ git stash pop <stash>

# スタッシュの差分を確認
$ git stash show
$ git stash show -p

# スタッシュの削除
$ git stash drop <stash>
$ git stash clear
```


## 変更の破棄

`restore` はワーキングディレクトリやステージングエリアの変更内容を破棄するコマンド。

```sh
# ファイル、ディレクトリのパス指定でワーキングディレクトリの変更内容を破棄
$ git restore <path>

# ワーキングディレクトリの変更内容を全て破棄
$ git restore .

# ファイル、ディレクトリのパス指定でステージングエリアの変更内容を破棄
$ git restore --staged <path>

# ステージングエリアの変更内容を全て破棄
$ git restore --staged .
```


## ブランチ

Gitのブランチは非常に軽量で、他のバージョン管理システムと比較してもブランチの機能を利用する場面が多い。

### ブランチ操作

`branch` はリポジトリのブランチに対する操作を行うコマンド。ブランチの切り替えには `switch` を用いる。

```sh
# ローカルリポジトリのブランチを一覧表示
$ git branch

# リモートリポジトリのブランチを一覧表示
$ git branch -a

# ローカルリポジトリとリモートリポジトリのブランチを一覧表示
$ git branch -r

# 新しいブランチを作成
$ git branch <name>

# ブランチの名前を変更
$ git branch -m <old-name> <new-name>

# ブランチの削除
$ git branch -d <name>

# リモートリポジトリのブランチを削除
$ git push <remote> --delete <branch>
```

### ブランチの切り替え

`switch` はブランチの切り替えを行うコマンド。ブランチの切り替えにはもともと `checkout` が用いられていたが、バージョン2.23.0で `switch` が追加された。

```sh
# 指定のブランチに切り替え
$ git switch <branch>

# ブランチを作成してそのブランチに切り換える
$ git switch -c <branch>
```

### ブランチの統合

`merge` はあるブランチを別のブランチに合流させるためのコマンド。

```sh
# 現在のブランチに指定のブランチを統合
$ git merge <branch>
```


## タグ

`tag` は特定のコミットにタグ付けを行うことで参照しやすくするためのコマンド。**軽量タグ**はあるコミットに対して名前を付けるために利用され、**注釈付きタグ**ではタグ名に加えて作成者や作成日、コメントといった情報を付与することができる。また、タグは明示的に `push` しなければリモートリポジトリに共有されないため注意。

```sh
# タグの一覧を表示
$ git tag

# 軽量タグの作成
$ git tag <tagname>

# 注釈付きタグの作成
$ git tag -a <tagname> -m "<message>"

# 過去のコミットに対するタグの作成
$ git tag -a <tagname> <hash>

# タグの削除
$ git tag -d <tagname>
```


## チェックアウト

`checkout` は現在のワーキングディレクトリを別のスナップショットを取得するコマンド。コミットのハッシュやブランチを指定してスナップショットを取得することができる。

```sh
# コミット指定でスナップショットを取得
$ git checkout <hash>

# ブランチ指定でスナップショットを取得
$ git checkout <branch>
```


## コミットの取り消し

`reset` は誤ったコミットを作成してしまった場合などにそのコミットを取り消すコマンド。このコマンドではリモートリポジトリに反映した変更は取り消せないため注意。

```sh
# 直前のコミットを取り消し（編集内容はワーキングディレクトリとステージングエリアに残る）
$ git reset --soft HEAD^

# 直前のコミットを取り消し（編集内容はワーキングディレクトリに残る）
$ git reset --mixed HEAD^

# 直前のコミットを取り消し（編集内容は消去）
$ git reset --hard HEAD^

# 指定のコミットまで取り消し
$ git reset <hash>
```

リモートリポジトリのコミット履歴に対する変更は他のメンバーにも影響を与えるため行ってはいけない。誤って `push` してしまったコミットを取り消したい場合には、 `revert` によってコミットを打ち消すような新しいコミットを作成する必要がある。

```sh
# 指定のコミットを打ち消す新しいコミットを作成
$ git revert <hash>
```