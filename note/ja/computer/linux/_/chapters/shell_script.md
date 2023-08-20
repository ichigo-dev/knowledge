# 『シェルスクリプト』ノート

（最終更新： 2023-08-20）


## 目次

1. [シェルスクリプト](#シェルスクリプト)
	1. [シバン](#シバン)
	1. [シェルスクリプトの実行](#シェルスクリプトの実行)
1. [シェルスクリプトの書き方](#シェルスクリプトの書き方)
	1. [コメント](#コメント)
	1. [変数](#変数)
	1. [クォーティング](#クォーティング)
	1. [コマンド置換](#コマンド置換)
	1. [位置パラメータ](#位置パラメータ)


## シェルスクリプト

**シェルスクリプト**は、[OS](../../../software/_/chapters/operating_system.md#オペレーティングシステム)に対して実行したい一連の[コマンド](./basic_command.md#コマンド)を並べ、まとめて実行できるようにした[ファイル](../../../software/_/chapters/file_system.md#ファイル)、あるいはそれを記述するための[スクリプト言語](../../../../programming/_/chapters/programming.md#スクリプト言語)。[シェル](./shell_and_terminal.md#シェル)では通常、[コマンド](./basic_command.md#コマンド)の実行と結果の表示を対話的に繰り返すが、シェルスクリプトを使用すると、[コマンド](./basic_command.md#コマンド)の打ち間違いを減らしたり、再利用性や再現性を高めることができる。

シェルスクリプトは他の[プログラミング言語](../../../../programming/_/chapters/programming.md#プログラミング言語)とは書き方や性質が異なる部分が多いので注意する。

シェルスクリプトが記述された[ファイル](../../../software/_/chapters/file_system.md#ファイル)は、[拡張子](../../../software/_/chapters/file_system.md#拡張子)を `.sh` とするのが一般的で、[実行権限](./user_and_permission.md#権限)を付けておく必要がある。

例えば、以下のシェルスクリプトは、[ホームディレクトリ](../../../software/_/chapters/file_system.md#ホームディレクトリ)の合計[ディスク](../../../hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)容量を出力する。

```sh
#!/bin/bash
du -h ~ | tail -n 1
```

[ファイル](../../../software/_/chapters/file_system.md#ファイル)に[実行権限](./user_and_permission.md#権限)を付け、[ファイル](../../../software/_/chapters/file_system.md#ファイル)名の前に `./` をつけることで、このシェルスクリプトを実行できる。

```sh
$ chmod +x homesize.sh
$ ./homesize.sh
```

また、 `source` [コマンド](./basic_command.md#コマンド)を用いて実行することもできる。

### シバン

**シバン**(Shebang)は、[シェルスクリプト](#シェルスクリプト)[ファイル](../../../software/_/chapters/file_system.md#ファイル)をどの[シェル](./shell_and_terminal.md#シェル)で実行するかを宣言する部分で、先頭行に記述する。 `#!` に続けて、 `/bin/bash` のように使用する[シェル](./shell_and_terminal.md#シェル)の[パス](../../../software/_/chapters/file_system.md#パス)を記述する。

例えば、以下のようなシバンが記述された[シェルスクリプト](#シェルスクリプト)を実行すると、[bash](./shell_and_terminal.md#bash)[シェル](./shell_and_terminal.md#シェル)を使用して[スクリプト](#シェルスクリプト)を実行する。

```sh
#!/bin/bash
```

### シェルスクリプトの実行

**source**は、指定した[ファイル](../../../software/_/chapters/file_system.md#ファイル)を現在の[シェル](./shell_and_terminal.md#シェル)で実行する[コマンド](./basic_command.md#コマンド)。 `./` を用いる場合とは異なり、現在の[シェル](./shell_and_terminal.md#シェル)にそのまま[コマンド](./basic_command.md#コマンド)を流し込むかたちとなるので、[ファイル](../../../software/_/chapters/file_system.md#ファイル)に[シバン](#シバン)を記述する必要はない。また、[ファイル](../../../software/_/chapters/file_system.md#ファイル)自体に[実行権限](./user_and_permission.md#権限)を付与しなくてもよい。

以下は、 `homesize.sh` という[シェルスクリプト](#シェルスクリプト)を `source` を用いてカレントシェルで実行する例。

```sh
$ source homesize.sh
```

`source` [コマンド](./basic_command.md#コマンド)はカレントシェルにより[コマンド](./basic_command.md#コマンド)を実行した場合と同じ動作になるため、カレントシェルの環境の影響を受けたり、実行後にカレントシェルに影響を及ぼすことに注意が必要。


## シェルスクリプトの書き方

### コメント

[シェルスクリプト](#シェルスクリプト)内では、 `#` の後ろすべてが[コメント](../../../../programming/_/chapters/programming.md#コメント)として扱われる。[プログラム](../../../../programming/_/chapters/programming.md#プログラム)の内容に関する補足を記述したり、一時的に無効化したい行を[コメントアウト](../../../../programming/_/chapters/programming.md#コメント)したりする目的で用いられる。

```sh
#!/bin/bash

echo "Example Script"

# ルートディレクトリに移動
cd /

ls -l    # ファイルの一覧を表示
```

### 変数

[シェルスクリプト](#シェルスクリプト)では、[シェル変数](./shell_and_terminal.md#シェル変数)や[環境変数](./shell_and_terminal.md#環境変数)といった[変数](../../../../programming/_/chapters/variable.md#変数)を用いることができる。[変数](../../../../programming/_/chapters/variable.md#変数)に値を[代入](../../../../programming/_/chapters/variable.md#代入)するには、 `<変数名>=<値>` という形式で記述する。 `=` の前後にはスペースを入れないように注意する。

```sh
#!/bin/bash

var="value"
```

また、[変数](../../../../programming/_/chapters/variable.md#変数)の値を[参照](../../../../programming/_/chapters/variable.md#参照)する際には、[変数](../../../../programming/_/chapters/variable.md#変数)名の前に `$` をつける。

```sh
#!/bin/bash

work_dir="~/workspace"
cd $work_dir
```

[変数](../../../../programming/_/chapters/variable.md#変数)名と別の文字列をつなげて書きたい場合は、[変数](../../../../programming/_/chapters/variable.md#変数)名を `{}` で囲んで記述する。

```sh
#!/bin/bash

backup_file="~/test.txt"
cp $backup_file ${backup_file}.bak
```

一般的には、[環境変数](./shell_and_terminal.md#環境変数)は[アッパーケース](../../../../programming/_/chapters/coding_rule.md#アッパーケース)、[シェル変数](./shell_and_terminal.md#シェル変数)は[スネークケース](../../../../programming/_/chapters/coding_rule.md#スネークケース)で定義する場合が多い。

### 関数

[シェルスクリプト](#シェルスクリプト)では、次のようにして[関数](../../../../programming/_/chapters/function.md#関数)を定義し、呼び出すことができる。また、 `function` は省略できる。引数を用いる場合は、[位置パラメータ](#位置パラメータ)を用いる。

```sh
function hoge()
{
    echo "Hello, $1"
}

hoge taro
hoge hanako
```

### クォーティング

[シェルスクリプト](#シェルスクリプト)中で文字列をクォートで囲むと、スペースも含めて1つの単語としてみなされるようになる。一般的な[プログラミング言語](../../../../programming/_/chapters/programming.md#プログラミング言語)とは異なり、すべての文字列をクォートで囲む必要はない。

シングルクォートを用いた場合は変数展開が無効となり、ダブルクォートを用いた場合は変数展開が行われる。また、ダブルクォートを用いた場合でも、 `$` の前に `\` を記述してエスケープすることで変数展開を防ぐことができる。

```sh
#!/bin/bash

user_name=Malia
echo 'Hello, $user_name'    # 変数展開されない
echo "Hello, $user_name"    # 変数展開される
echo "Hello, \$user_name"   # 変数展開されない
```

### コマンド置換

**コマンド置換**は、[コマンド](./basic_command.md#コマンド)の結果を文字列として取得する機能。 `$()` という形式が用いられ、[コマンド](./basic_command.md#コマンド)の出力結果を[シェルスクリプト](#シェルスクリプト)内で利用したい場合に用いられる。

```sh
#!/bin/bash

$filename=$(date '+%Y-%m-%d')
touch $filename
```

### 位置パラメータ

**位置パラメータ**は、[シェルスクリプト](#シェルスクリプト)からコマンドライン[引数](../../../../programming/_/chapters/function.md#引数)を扱うための[シェル変数](./shell_and_terminal.md#シェル変数)。

例えば、以下のように[シェルスクリプト](#シェルスクリプト)を実行したとする。

```sh
$ ./backup.sh ./src ./src.bak
```

このとき、[シェルスクリプト](#シェルスクリプト)の中ではこれらの[引数](../../../../programming/_/chapters/function.md#引数)を次のように受け取ることができる。

```sh
#!/bin/bash

$backup_from=$1
$backup_to=$2
cp -r $backup_from $backup_to
```

[引数](../../../../programming/_/chapters/function.md#引数)は1つ目から順番に、 `$$1, $2, $3...` という[変数](../../../../programming/_/chapters/variable.md#変数)名に割り当てられる。また、 `$0` には実行された[シェルスクリプト](#シェルスクリプト)名が格納される。さらに、[引数](../../../../programming/_/chapters/function.md#引数)の個数を表す特別なパラメータとして `$#` が、全ての[引数](../../../../programming/_/chapters/function.md#引数)をまとめたものとして `$@` あるいは `$*` が用いられる。

| 変数            | 内容                                                                                             |
| --------------- | :----------------------------------------------------------------------------------------------- |
| `$0`            | 実行時のシェルスクリプト名                                                                       |
| `$1, $2, $3...` | コマンドライン引数の値（位置パラメータ）                                                         |
| `$#`            | 位置パラメータの個数                                                                             |
| `$@`            | 全ての位置パラメータで、ダブルクォートで囲むとそれぞれの位置パラメータがダブルクォートで囲まれる |
| `$*`            | 全ての位置パラメータで、ダブルクォートで囲むと全体が一つの文字列としてダブルクォートで囲まれる   |

### コマンドラインからの入力の受け取り

**read**は、ユーザに[プロンプト](./shell_and_terminal.md#プロンプト)を返して、コマンドラインからの入力を待ち受ける状態とする[コマンド](./basic_command.md#コマンド)。

```sh
#!/bin/bash

read name
echo "Hello, $name"
```
