# 『制御フロー』ノート

（最終更新： 2023-03-03）


## 目次

1. [制御フロー](#制御フロー)
	1. [ネスト](#ネスト)
1. [ブロック](#ブロック)
	1. [スコープ](#スコープ)
	1. [グローバル](#グローバル)
	1. [ローカル](#ローカル)
1. [条件分岐](#条件分岐)
	1. [if文](#if文)
	1. [switch文](#switch文)
1. [反復](#反復)
	1. [break文](#break文)
	1. [continue文](#continue文)
	1. [while文](#while文)
	1. [do while文](#do-while文)
	1. [for文](#for文)
	1. [for in文](#for-in文)
	1. [loop文](#loop文)


## 制御フロー

**制御フロー**は、[プログラム](./programming.md#プログラム)の処理の流れのこと。制御フローを記述するための[プログラミング言語](./programming.md#プログラミング言語)の[文法](./programming.md#文法)を、**制御構文**という。

[プログラム](./programming.md#プログラム)は通常、上から下へと順次実行されていくが、制御構文を用いることで、処理の流れを[分岐](#条件分岐)させたり、[繰り返し](#反復)同じ処理をさせたりすることができる。

### ネスト

**ネスト**（**入れ子**）は、[制御構文](#制御フロー)中でさらに別の[制御構文](#制御フロー)を記述する構造。


## ブロック

**ブロック**は、[ソースコード](./programming.md#ソースコード)中で処理をまとめたもの。多くの[プログラミング言語](./programming.md#プログラミング言語)ではブロックは `{}` で囲まれた範囲となる。[Python](./programming_language.md#python)のようにインデントにより処理ブロックを表現するものもある。

ブロックは入れ子にすることもできる。

### スコープ

**スコープ**は、[変数](./variable.md#変数)や[関数](./function.md#関数)などの[識別子](./programming.md#識別子)を参照できる範囲。[変数](./variable.md#変数)などが[宣言](./variable.md#宣言)される位置などによって決定される。スコープは、[宣言](./variable.md#宣言)された場所から[宣言](./variable.md#宣言)された[ブロック](#ブロック)の終わりまでとなる。

```javascript
// JavaScript

{
    // 変数xを初期化
    let x = 1;

    {
        // 変数yを初期化
        let y = 3;

        // 変数x, yのスコープ内なので、問題ない
        console.log(x);
        console.log(y);

        x = x + y;
    }

    // 変数xのスコープ内なので問題ない
    console.log(x);

    // 変数yのスコープ外なのでエラーとなる
    console.log(y);
}
```

### グローバル

**グローバル**とは、[ソースコード](./programming.md#ソースコード)全体が[スコープ](#スコープ)となっているデータ（[変数](./variable.md#変数)）のこと。グローバルなデータには、全ての[関数](./function.md#関数)から直接アクセスすることができる。更新されるタイミングがわかりにくかったり、予想外のデータとなってしまうことから[バグ](./programming.md#バグ)を生みやすいため、扱いには注意が必要。

### ローカル

**ローカル**とは、そのデータが[宣言](./variable.md#宣言)された[関数](./function.md#関数)内でしか[参照](./variable.md#参照)できないデータ（[スコープ](#スコープ)が[関数](./function.md#関数)内に閉じられている[変数](./variable.md#変数)）のこと。

## 条件分岐

**条件分岐**は、与えられた条件式の結果によって実行する処理[ブロック](#ブロック)を切り替えるような構文。

### if文

`if` 文は、与えられた条件式を評価し、それが `true` （言語によっては `true` に類するもの（[整数型](./data_type.md#整数型)の `0` 以外や[文字列型](./data_type.md#文字列型)で空文字でない場合等）が含まれる場合もある）であれば、直後の処理[ブロック](#ブロック)を実行する。

```c
// C言語

#include <stdio.h>

int main()
{
    int input_num;
    printf("> ");
    scanf("%d", &input_num);

    if( input_num > 100 )
    {
        printf("Input number is greater than 100.\n");
    }

    return 0;
}
```

```php
<?php

// PHP

$input_num = $_POST["input_num"];

if( $input_num > 100 )
{
    echo("Input number is greater than 100.\n");
}

?>
```

`if else` 文は、与えられた条件式を評価して、それが `true` であれば `if` の処理[ブロック](#ブロック)を、 `true` でなければ `else` の処理[ブロック](#ブロック)を実行する。また `else if` 文を用いることで、多段の[条件分岐](#条件分岐)行うことも可能である。 `else` 文を省略した場合、条件に当てはまらない時は何も処理が実行されない。

```c
// C言語

#include <stdio.h>

int main()
{
    // if else文の例
    int input_num;
    printf("> ");
    scanf("%d", &input_num);

    if( input_num > 100 )
    {
        printf("Input number is greater than 100.\n");
    }
    else
    {
        printf("Input number is less than or equal to 100.\n");
    }

    // 多段の条件分岐の例
    int favorite_month;
    printf("Input your favorite month > ");
    scanf("%d", &favorite_month);

    if( 3 <= favorite_month && favorite_month < 6 )
    {
        printf("Your favorite season is spring.\n");
    }
    else if( 6 <= favorite_month && favorite_month < 9 )
    {
        printf("Your favorite season is summer.\n");
    }
    else if( 9 <= favorite_month && favorite_month < 12 )
    {
        printf("Your favorite season is autumn.\n");
    }
    else if( favorite_month == 12 && 1 <= favorite_month && favorite_month < 3 )
    {
        printf("Your favorite season is winter.\n");

    }

    return 0;
}
```

```php
<?php

// PHP

// if else文の例
$input_num = $_POST["input_num"];

if( $input_num > 100 )
{
    echo("Input number is greater than 100.\n");
}
else
{
    echo("Input number is less than or equal to 100.\n");
}

// 多段の条件分岐の例
$favorite_month = $_POST["favorite_month"];

if( 3 <= $favorite_month && $favorite_month < 6 )
{
    echo("Your favorite season is spring.");
}
else if( 6 <= $favorite_month && $favorite_month < 9 )
{
    echo("Your favorite season is summer.");
}
else if( 9 <= $favorite_month && $favorite_month < 12 )
{
    echo("Your favorite season is autumn.\n");
}
else if( $favorite_month == 12 && 1 <= $favorite_month && $favorite_month < 3 )
{
    echo("Your favorite season is winter.\n");
}

?>
```

### switch文

`switch` 文は、与えられた式を評価した結果に応じて処理を分岐する構文。与えられた式を `case` 節にある値と比較し、等価であればその節の処理[ブロック](#ブロック)を実行する。また、 `default` 節が用意されている場合は、全ての条件に対してその処理[ブロック](#ブロック)が実行される。

`case` の処理[ブロック](#ブロック)の最後に `break` 文を記述した場合は、その処理[ブロック](#ブロック)の実行後に即座に `switch` 文を抜ける。一方で `break` 文を省略した場合には、条件に合致した処理[ブロック](#ブロック)を実行した後に、次に条件に合致するものがあればその処理[ブロック](#ブロック)を続けて実行する。 `default` 節がある `swtich` 文においては、意図しない処理[ブロック](#ブロック)が実行されてしまわないように適切に `break` 文を用いる必要がある。

```c
// C言語

#include <stdio.h>

int main()
{
    int input_num;
    printf("> ");
    scanf("%d", &input_num);

    switch( input_num )
    {
        case 1:
        {
            printf("Input number is 1.\n");
            break;
        }
        case 2:
        {
            printf("Input number is 2.\n");
            break;
        }
        case 3:
        {
            printf("Input number is 3.\n");
            break;
        }
        default:
        {
            printf("Input number is some other value.\n");
        }
    }
}
```

```php
<?php

// PHP

$input_num = $_POST["input_num"];

switch( $input_num )
{
    case 1:
    {
        echo("Input number is 1.\n");
        break;
    }
    case 2:
    {
        echo("Input number is 2.\n");
        break;
    }
    case 3:
    {
        echo("Input number is 3.\n");
        break;
    }
    default:
    {
        echo("Input number is some other value.\n");
    }
}

?>
```


## 反復

**反復**（**ループ**）は、条件に応じて特定の処理[ブロック](#ブロック)を複数回実行するための構文。

### break文

`break` 文は、現在実行中の[ループ](#反復)処理[ブロック](#ブロック)の[反復](#反復)処理を中断し、処理を次に進める[制御構文](#制御フロー)。 `swtich` 文でも利用される。[ループ](#反復)文を[ネスト](#ネスト)した多重[ループ](#反復)においては、 `break` 文から見て最も内側の（近い）[ループ](#反復)から抜け出す。

### continue文

`continue` 文は、現在実行中の[ループ](#反復)処理[ブロック](#ブロック)を中断して、次の[ループ](#反復)を実行する（[ループ](#反復)処理[ブロック](#ブロック)の先頭に戻る）ための[制御構文](#制御フロー)。[ループ](#反復)文を[ネスト](#ネスト)した多重[ループ](#反復)においては、 `continue` 文から見て最も内側の（近い）[ループ](#反復)の処理を継続する。

### while文

`while` 文は、与えられた条件式を評価して、それが `true` （言語によっては `true` に類するもの（[整数型](./data_type.md#整数型)の `0` 以外や[文字列型](./data_type.md#文字列型)で空文字でない場合等）が含まれる場合もある）である間、直後の処理[ブロック](#ブロック)を実行する。

```c
// C言語

#include <stdio.h>

int main()
{
    int input_num;
    printf("> ");
    scanf("%d", &input_num);

    while( input_num > 0 )
    {
        printf("%d\n", input_num);
        input_num--;
    }

    return 0;
}
```

```php
<?php

// PHP

$input_num = $_POST["input_num"];

while( $input_num > 0 )
{
    echo($input_num);
    $input_num--;
}

?>
```

### do while文

`do while` 文は、 `while` 文と似ているが、処理[ブロック](#ブロック)の最初ではなく最後に条件式の評価を行う。そのため、最初の[ループ](#反復)が必ず実行される（ `while` 文では、最初から条件式が `false` であった場合には一度も実行されない）。

```c
// C言語

#include <stdio.h>

int main()
{
    int input_num;
    printf("> ");
    scanf("%d", &input_num);

    do
    {
        printf("%d\n", input_num);
        input_num--;
    } while( input_num > 0 )

    return 0;
}
```

```php
<?php

// PHP

$input_num = $_POST["input_num"];

do
{
    echo($input_num);
    $input_num--;
} while( $input_num > 0 )

?>
```

### for文

`for` 文は、[ループ](#反復)の回数を数える**カウンタ変数**を用いて、カウンタ変数に関する条件式を評価しながら[反復](#反復)処理を行う。[ループ](#反復)開始前のカウンタ変数の初期化式、カウンタ変数に関する条件式、1回のループ終了時のカウンタ変数の更新式を与えることで、ループの回数が決定される。 `while` 文を用いても同様の処理は実現できるが、 `for` 文を用いるとより簡潔に記述することができる。一般的な `for` 文の書式は以下の通り。

```
for( 初期化式; 条件式; 更新式 )
{
    // 実行する処理ブロック
}
```

**初期化式**には、カウンタ変数の初期化を行う処理を記述する。**条件式**には、 [ループ](#反復)を[反復](#反復)する条件を記述し、結果が `true` となる間処理を繰り返す。**更新式**には、[ループ](#反復)が1回終わったときの更新処理を記述する。

```c
// C言語

int main()
{
    for( int i=0; i<20; i++ )
    {
        printf("%d\n", i);
    }

    return 0;
}
```

```php
<?php

// PHP

for( $i=0; $i<20; $i++ )
{
    echo($i);
}

?>
```

### for in文

`for in` 文は、[イテレート](./data_type.md#イテレータ)可能な値に対して使用できる[反復](#反復)処理であり、[コンテナ型](./data_type.md#コンテナ型)のデータの中身を順に取り出したい場合などに用いられる。[プログラミング言語](./programming.md#プログラミング言語)によってはサポートしていないものや、別のキーワードで実装されている場合もある（[JavaScript](./programming_language.md#javascript)など）。

```python
# Python

nums = [10, 20, 30]

for num in nums;
    print("Number: {0}".format(num))
```

```php
<?php

// PHP

$nums = [10, 20, 30];

foreach( $nums as $num )
{
    echo("Number: " . $num);
}

?>
```

```javascript
// JavaScript

let nums = [10, 20, 30];

for( let num of nums )
{
    console.log("Number: " + num);
}
```

また、[オブジェクト](./object_oriented.md#オブジェクト)を[イテレート](./data_type.md#イテレータ)する場合の[制御構文](#制御フロー)を用意している[プログラミング言語](./programming.md#プログラミング言語)もある。

```php
<?php

// PHP

$profile = [
    "name" => "ichigo",
    "email" => "dev.honda.ichigo@gmail.com",
    "gender" => "male",
];

foreach( $profile as $key => $val )
{
    echo($key . ": " . $val);
}

?>
```

```javascript
// JavaScript

let profile = [
    name : "ichigo",
    email : "dev.honda.ichigo@gmail.com",
    gender : "male",
];

for( let key in profile )
{
    console.log(key + ": " + profile[key]);
}
```

### loop文

`loop` 文は、処理[ブロック](#ブロック)を**無限ループ**で実行する。一般的には、[ループ](#反復)内である条件において `break` 文を実行するように制御することで、途中で[ループ](#反復)を離脱するような実装とする場合が多い。処理を空回ししたい特殊な場合（[OS](../../../computer/software/_/chapters/operation_system.md#オペレーティングシステム)のhaltなど）にも用いられる。通常の[アプリケーション](../../../computer/software/_/chapters/basic_knowledge_of_software.md#応用ソフトウェア)において無限[ループ](#反復)が発生することは[バグ](./programming.md#バグ)である場合が多いため、注意して使用する。
