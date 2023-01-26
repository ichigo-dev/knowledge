# 『制御フロー』

（最終更新： 2023-01-27）


## 目次

1. [制御フロー](#制御フロー)
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


## 制御フロー

**制御フロー**は、プログラムの処理の流れのこと。制御フローを記述するためのプログラミング言語の文法を、**制御構文**という。

プログラムは通常、上から下へと順次実行されていくが、制御構文を用いることで、処理の流れを分岐させたり、繰り返し同じ処理をさせたりすることができる。


## ブロック

**ブロック**は、ソースコード中で処理をまとめたもの。多くのプログラミング言語ではブロックは `{}` で囲まれた範囲となる。Pythonのようにインデントにより処理ブロックを表現するものもある。

ブロックは入れ子にすることもできる。

### スコープ

**スコープ**は、変数や関数などの識別子を参照できる範囲。変数などが宣言される位置などによって決定される。スコープは、宣言された場所から宣言されたブロックの終わりまでとなる。

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

**グローバル**とは、ソースコード全体がスコープとなっているデータのこと。グローバルなデータには、全ての関数から直接アクセスすることができる。更新されるタイミングがわかりにくかったり、予想外のデータとなってしまうことからバグを生みやすいため、扱いには注意が必要。

### ローカル

**ローカル**とは、そのデータが宣言された関数内でしか参照できないデータ（スコープが関数内に閉じられているデータ）のこと。


## 条件分岐

**条件分岐**は、与えられた条件式の結果によって実行する処理ブロックを切り替えるような構文。

### if文

`if` 文は、与えられた条件式を評価し、それが `true` （言語によっては `true` に類するもの（整数型の0以外や文字列型で空文字でない場合等）が含まれる場合もある）であれば、直後の処理ブロックを実行する。

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

`if else` 文は、与えられた条件式を評価して、それが `true` であれば `if` の処理ブロックを、 `true` でなければ `else` の処理ブロックを実行する。また `else if` 文を用いることで、多段の条件分岐行うことも可能である。 `else` 文を省略した場合、条件に当てはまらない時は何も処理が実行されない。

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

`switch` 文は、与えられた式を評価した結果に応じて処理を分岐する構文。与えられた式を `case` 節にある値と比較し、等価であればその節の処理ブロックを実行する。また、 `default` 節が用意されている場合は、全ての条件に対してその処理ブロックが実行される。

`case` の処理ブロックの最後に `break` 文を記述した場合は、その処理ブロックの実行後に即座に `switch` 文を抜ける。一方で `break` 文を省略した場合には、条件に合致した処理ブロックを実行した後に、次に条件に合致するものがあればその処理ブロックを続けて実行する。 `default` 節がある `swtich` 文等においては、意図しない処理ブロックが実行されてしまわないように適切に `break` 文を用いる必要がある。

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

**反復**（**ループ**）は、条件に応じて特定の処理ブロックを複数回実行するための構文。

### break文

`break` 文は、現在実行中のループ処理ブロックの反復処理を中断し、処理を次に進める制御構文。 `swtich` 文でも利用される。ループ文をネストした多重ループにおいては、 `break` 文から見て最も内側の（近い）ループから抜け出す。

### continue文

`continue` 文は、現在実行中のループ処理ブロックを中断して、次のループを実行する（ループ処理ブロックの先頭に戻る）ための制御構文。ループ文をネストした多重ループにおいては、 `continue` 文から見て最も内側の（近い）ループの処理を継続する。
