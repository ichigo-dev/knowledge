# 『制御フロー』

（最終更新： 2023-02-23）


## 目次

1. [制御フロー](#制御フロー-1)
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

### while文

`while` 文は、与えられた条件式を評価して、それが `true` （言語によっては `true` に類するもの（整数型の0以外や文字列型で空文字でない場合等）が含まれる場合もある）である間、直後の処理ブロックを実行する。

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

`do while` 文は、 `while` 文と似ているが、処理ブロックの最初ではなく最後に条件式の評価を行う。そのため、最初のループが必ず実行される（ `while` 文では、最初から条件式が `false` であった場合には一度も実行されない）。

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

`for` 文は、ループの回数を数える**カウンタ変数**を用いて、カウンタ変数に関する条件式を評価しながら反復処理を行う。ループ開始前のカウンタ変数の初期化式、カウンタ変数に関する条件式、1回のループ終了時のカウンタ変数の更新式を与えることで、ループの回数が決定される。 `while` 文を用いても同様の処理は実現できるが、 `for` 文を用いるとより簡潔に記述することができる。一般的な `for` 文の書式は以下の通り。

```
for( 初期化式; 条件式; 更新式 )
{
    // 実行する処理ブロック
}
```

**初期化式**には、カウンタ変数の初期化を行う処理を記述する。**条件式**には、 ループを反復する条件を記述し、結果が `true` となる間処理を繰り返す。**更新式**には、ループが1回終わったときの更新処理を記述する。

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

`for in` 文は、イテレート可能な値に対して使用できる反復処理であり、コンテナ型のデータの中身を順に取り出したい場合などに用いられる。プログラミング言語によってはサポートしていないものや、別のキーワードで実装されている場合もある（JavaScriptなど）。

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

また、オブジェクトをイテレートする場合の制御構文を用意しているプログラミング言語もある。

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

`loop` 文は、処理ブロックを**無限ループ**で実行する。一般的には、ループ内である条件において `break` 文を実行するように制御することで、途中でループを離脱するような実装とする場合が多い。処理を空回ししたい特殊な場合（OSのhaltなど）にも用いられる。通常のアプリケーションにおいて無限ループが発生することはバグである場合が多いため、注意して使用する。
