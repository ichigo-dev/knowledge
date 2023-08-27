# 『履歴の記録』ノート

（最終更新： 2023-08-20）


## 目次

1. [Gitの各領域](#gitの各領域)
	1. [ワークツリー](#ワークツリー)
	1. [ステージ](#ステージ)
	1. [ローカルリポジトリ](#ローカルリポジトリ)
	1. [リモートリポジトリ](#リモートリポジトリ)
1. [状態の確認](#状態の確認)
	1. [省略表記](#省略表記)
1. [ステージング](#ステージング)
	1. [ハンク](#ハンク)
	1. [インタラクティブセッション](#インタラクティブセッション)
1. [ファイルの復元](#ファイルの復元)
	1. [ステージから復元](#ステージから復元)
1. [コミット](#コミット)
	1. [コミットハッシュ](#コミットハッシュ)
	1. [コミットメッセージ](#コミットメッセージ)
	1. [コミットの修正](#コミットの修正)


## Gitの各領域

### ワークツリー

**ワークツリー**（**ワーキングディレクトリ**）は、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の編集作業を行う[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)。この領域での変更内容はまだ[リポジトリ](./create_repository.md#リポジトリ)に記録されていない状態となる。

### ステージ

**ステージ**（**インデックス**）は、変更履歴を作成するための中間領域。[コミット](#コミット)に含めたい[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)は一時的にステージに移動する必要がある。

### ローカルリポジトリ

**ローカルリポジトリ**は、実際に[コミット](#コミット)履歴が記録される領域。[コミット](#コミット)しただけでは[リモートリポジトリ](#リモートリポジトリ)には反映されず、あくまで自身のローカル環境でのみ履歴として追加される。また、複数の[リモートリポジトリ](#リモートリポジトリ)と紐づけることも可能。基本的には[ノンベアリポジトリ](./create_repository.md#ノンベアリポジトリ)で運用される。

### リモートリポジトリ

**リモートリポジトリ**は、各ユーザの[ローカルリポジトリ](#ローカルリポジトリ)の変更を集約し、全体の[コミット](#コミット)履歴を管理する領域。[インターネット](../../../../network/_/chapters/network.md#インターネット)や[サーバ](../../../../computer/_/chapters/computer.md#サーバ)上に用意する[リポジトリ](./create_repository.md#リポジトリ)で、開発者間で変更内容を共有したり、開発した[プログラム](../../../../programming/_/chapters/programming.md#プログラム)を[OSS](../../../../computer/software/_/chapters/open_source_software.md#オープンソースソフトウェア)として公開するために用いられる。基本的には[ベアリポジトリ](./create_repository.md#ベアリポジトリ)で運用される。


## 状態の確認

**git status**は、[ワークツリー](#ワークツリー)、[ステージ](#ステージ)にある[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ git status
```

特に変更がない場合は次のような出力となる。

```sh
$ git status
On branch main
Your branch is up to date with 'origin/main'.

nothing to commit, working tree clean
```

新しく[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を追加した場合には次のような出力となる。

```sh
On branch main
Your branch is up to date with 'origin/main'.

Untracked files:
  (use "git add <file>..." to include in what will be committed)
        example.txt

no changes added to commit (use "git add" and/or "git commit -a")
```

[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)に変更を加えた場合は次のような出力となる。

```sh
On branch main
Your branch is up to date with 'origin/main'.

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   example.txt

no changes added to commit (use "git add" and/or "git commit -a")
```

[ステージエリア](#ステージ)に[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)が存在する場合は次のような出力となる。

```sh
On branch main
Your branch is up to date with 'origin/main'.

Changes to be committed:
  (use "git restore --staged <file>..." to unstage)
        modified:   example1.txt
        new file:   example2.txt
```

### 省略表記

`git status` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `-s` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、出力をコンパクトにすることができる。

```sh
M  example1.txt
 M example2.txt
?? example3.txt
```


## ステージング

**ステージング**は、履歴として記録したい[ワークツリー](#ワークツリー)の変更内容を一時的に[ステージエリア](#ステージ)に登録する操作。ステージングしただけでは変更は履歴に記録されないので注意が必要。

ステージングには、**git add**[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いる。

```sh
# ファイル、ディレクトリのパスを指定してステージに移動
$ git add <path>

# 全ての変更をステージに移動
$ git add .
```

### ハンク

**ハンク**(Hank)は、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の差分ブロック。

### インタラクティブセッション

`git add` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `-p` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、インタラクティブな[ステージング](#ステージング)セッションが開始される。このモードでは、順番に変更内容の[ハンク](#ハンク)が表示され、 `y` を選択するとその[ハンク](#ハンク)を[ステージ](#ステージ)に移動し、 `n` を選択するとその[ハンク](#ハンク)を無視する。また、 `s` を選択すると[ハンク](#ハンク)がさらに細かく分解され、 `e` を選択するとその[ハンク](#ハンク)を手作業で編集でき、 `q` を選択するとインタラクティブセッションを終了する。

```sh
$ git add -p
diff --git a/example.txt b/example.txt
index xxxxxxx..xxxxxxx 000000
--- a/example.txt
+++ b/example.txt
@@ -1,0 +1,1 @@
+Hello, Git!

(1/1) Stage this hunk [y,n,q,a,d,j,J,g,/,e,?]?
```


## ファイルの復元

**git restore**は、指定した[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の変更内容を取り消し、元の状態を復元する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# ファイル、ディレクトリのパスを指定して復元
$ git restore <path>

# 全ての変更を取り消し
$ git restore .
```

### ステージから復元

`git restore` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--staged` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、[ステージ](#ステージ)の変更内容を[ワークツリー](#ワークツリー)に戻すことができる。

```sh
# ファイル、ディレクトリのパスを指定してステージからワークツリーに移動
$ git restore <path> --staged

# 全ての変更をステージからワークツリーに移動
$ git restore . --staged
```


## コミット

**コミット**は、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)や[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)の変更を履歴に記録する操作、あるいはその変更履歴。[リポジトリ](./create_repository.md#リポジトリ)の変更履歴は**リビジョン**と呼ぶ場合もある。

[Git](./git.md#git)では、コミットごとに差分のみ記録するのではなく、毎回すべての[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の**スナップショット**（ある時点での[ソースコード](../../../../programming/_/chapters/programming.md#ソースコード)や[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)、[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)などの状態を抜き出したもの）を作成している。差分のみを記録していると、古いリビジョンの[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を復元するために履歴を全てたどる必要があるが、スナップショットを保存しておくことで高速に再現することができる。

コミットには、**git commit**[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いる。

```sh
# 変更をコミット（エディタが開かれてコミットメッセージの入力に移る）
$ git commit

# コミットメッセージをインラインで指定してコミット
$ git commit -m "<message>"
```

### コミットハッシュ

**コミットハッシュ**は、[コミット](#コミット)ごとに生成される、40桁の文字列。コミットハッシュを指定することで、[コミット](#コミット)の履歴から特定の[コミット](#コミット)を絞り込むことができる。

### コミットハッシュのエイリアス

[コミット](#コミット)履歴のうち特別な意味を持つ[コミット](#コミット)に対しては、[コミットハッシュ](#コミットハッシュ)のエイリアスが指定されている。

| エイリアス                          | 概要                                   |
|-------------------------------------|----------------------------------------|
| `HEAD` , `@`                        | 現在のHEADコミット                     |
| `HEAD^` , `HEAD~` , `@^` , `@~`     | 1つ前のコミット                        |
| `HEAD^^` , `HEAD~~` , `@^^` , `@~~` | 2つ前のコミット                        |
| `HEAD~{n}` , `@~{n}`                | n個前のコミット                        |
| `HEAD^{n}` , `@^{n}`                | 親コミットが複数ある場合のコミット指定 |

### コミットメッセージ

**コミットメッセージ**は、[コミット](#コミット)時に履歴に対してつけるコメントで、作業内容などを記録しておくことができる。コミットメッセージの書き方には、開発チームごとに文化やルールなどが存在するが、次のようなフォーマットで記述するのが一般的。

- 1行目に[コミット](#コミット)の概要を50文字以内（[マルチバイト](../../../../basics/information_theory/_/chapters/character_representation.md#マルチバイト文字)文字なら25文字）で記述
- 2行目は空行
- 3行目以降に[コミット](#コミット)の詳細説明を記述
- コミットメッセージを英語で記述する場合、現在系の文章を使用
- [コミット](#コミット)の概要には、適切なプレフィックスや絵文字をつける

### コミットの修正

`git commit` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--amend` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定することで、直前の[コミット](#コミット)の[コミットメッセージ](#コミットメッセージ)を修正できる。

```sh
# 直前のコミットメッセージの修正
$ git commit --amend

# コミットメッセージをインラインで指定して修正
$ git commit --amend -m "<new-message>"
```

また、直前の[コミット](#コミット)に[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を追加したい場合は、追加したい[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を[ステージング](#ステージング)し、 `git commit` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)に `--amend` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)と `--no-edit` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定する。ただし、この方法で直前に[コミット](#コミット)した[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の編集や削除はできない。

```sh
# 対象ファイルをステージング
$ git add <path>

# 直前のコミットにステージのファイルを追加
$ git commit --amend --no-edit
```

この方法では、直前の[コミット](#コミット)しか修正することができず、可能な操作も限られている。また、[リモートリポジトリ](#リモートリポジトリ)と同期済みの[コミット](#コミット)を修正してはいけない（ `--amend` を用いると、実際には元の[コミット](#コミット)とは別の[コミット](#コミット)として履歴が作られるため）。
