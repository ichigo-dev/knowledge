# 制御構文


## 目次

1. [制御フロー](#制御フロー)
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

[プログラム](./01_basic_knowledge_of_programming.ja.md#プログラミングの概要)の処理は基本的に上から下に順次実行されていく。[プログラム](./01_basic_knowledge_of_programming.ja.md#プログラミングの概要)実行の途中で処理の流れを[分岐](#条件分岐)させたり、[繰り返し処理](#反復)を行うような制御のことを**制御フロー**といい、これを実現するために用いる[文法](./01_basic_knowledge_of_programming.ja.md#プログラムの基礎文法)のことを**制御構文**という。


## 条件分岐

**条件分岐**は**条件式**を満たしているか否かによって実行する処理[ブロック](./02_variable.ja.md#ブロックとスコープ)を切り替えるような構文。代表的なものとして `if` 文と`switch` 文がある。

### if文

`if` 文では与えられた[条件式](#条件分岐)を評価して、それが `true` （言語によっては `true` に類するもの（[整数型](./03_data_type.ja.md#整数型)の0以外、[文字列型](./03_data_type.ja.md#文字型)で1文字以上の場合など）を含める場合もある）であれば処理[ブロック](./02_variable.ja.md#ブロックとスコープ)を実行する。

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
        printf("Input number is greater than 100.");
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

`if else` 文では与えられた[条件式](#条件分岐)を評価して、それが `true` であれば `if` の処理[ブロック](./02_variable.ja.md#ブロックとスコープ)を、 `true` 出なければ `else` の処理[ブロック](./02_variable.ja.md#ブロックとスコープ)を実行する。また、 `else if` 文を用いることで多段の[条件分岐](#条件分岐)を行うことも可能である。 `else` 文を省略した場合、条件に当てはまらないデータに対しては何も処理が実行されない。

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

`switch` 文は与えられた式を評価した結果に応じて処理を分岐する構文。与えられた式を `case` 節にある値と比較し、等価であればその節の処理[ブロック](./02_variable.ja.md#ブロックとスコープ)を実行する。また、 `default` 節が用意されている場合は、全ての条件に対してその処理[ブロック](./02_variable.ja.md#ブロックとスコープ)が実行される。

`case` の処理[ブロック](./02_variable.ja.md#ブロックとスコープ)の最後に `break` 文を記述した場合は、その処理[ブロック](./02_variable.ja.md#ブロックとスコープ)の実行後に即座に `switch` 文を抜ける。一方で `break` 文を省略した場合には、条件に合致した処理[ブロック](./02_variable.ja.md#ブロックとスコープ)を実行した後に次に条件に合致するものがあればその処理[ブロック](./02_variable.ja.md#ブロックとスコープ)を続けて実行する。 `default` 節がある `switch` 文などにおいて意図しない処理[ブロック](./02_variable.ja.md#ブロックとスコープ)が実行されてしまわないように注意する必要がある。

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

**反復**（**ループ**）は条件を特定の処理[ブロック](./02_variable.ja.md#ブロックとスコープ)を複数回実行するための構文。代表的なものとして `while` 文や `for` 文などがある。

また反復処理を終了、あるいは継続して次のループを実行するための[制御構文](#制御フロー)として `break文` と `continue` 文がある。

### break文

`break` 文は現在実行中のループ処理[ブロック](./02_variable.ja.md#ブロックとスコープ)の[反復処理](#反復)を中断し、処理を次に進める[制御構文](#制御フロー)。また、 `switch` 文でも使用される。ループ文をネストした多重ループにおいては、 `break` 文から見て最も内側のループから抜け出す。

### continue文

`continue` 文は現在実行中の[反復処理](#反復)において、処理を中断して次のループを実行する（ループ処理[ブロック](./02_variable.ja.md#ブロックとスコープ)の先頭に戻る）[制御構文](#制御フロー)。ループ文をネストした多重ループにおいては、 `continue` 文から見て最も内側のループの処理を継続する。

### while文

`while` 文は与えられた[条件式](#条件分岐)を評価して、それが `true` （言語によっては `true` に類するもの（[整数型](./03_data_type.ja.md#整数型)の0以外、[文字列型](./03_data_type.ja.md#文字型)で1文字以上の場合など）を含める場合もある）である間、処理[ブロック](./02_variable.ja.md#ブロックとスコープ)を反復実行する。

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

`do while` 文は `while` 文と似ているが、処理[ブロック](./02_variable.ja.md#ブロックとスコープ)の最初ではなく最後に[条件式](#条件分岐)の評価を行う。そのため、最初のループが必ず実行される（ `while` 文では[条件式](#条件分岐)が偽である場合は1回も実行されない）。

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

[反復処理](#反復)では基本的に、ループ回数を決定する**カウンタ変数**をループ前に初期化しておき、ループが1回終了する度にカウンタ変数を更新し、カウンタ変数が[条件式](#条件分岐)を満たす間処理を繰り返す。 `while` 文でもこのような構文を用いることは可能であるが、 `for` 文を使うとより簡潔に記述することができる。 `for` 文の一般的な書式は次の通り。

```
for( 初期化処理; 条件式; 更新処理 )
{
    // 実行する処理ブロック
}
```

**初期化処理**はカウンタとして用いる[変数](./02_variable.ja.md#プログラミングにおける変数)の[初期化](./02_variable.ja.md#変数の操作)を行う。**条件式**は `while` 文などと同様で、この[条件式](#条件分岐)が `true` となる間処理を繰り返す。**更新処理**にはループが1回終わったときの更新の処理を記述する。更新処理には、[インクリメント・デクリメント演算子](./04_operation.ja.md#インクリメントとデクリメント)を用いる場合が多い。

カウンタ変数の[識別子](./01_basic_knowledge_of_programming.ja.md#識別子)として、 `i, j, k` などを用いることが慣習となっている場合が多い。

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

`for in` 文は[イテレート](./03_data_type.ja.md#コンテナ型)可能な値に対して使用できる[反復処理](#反復)であり、[コンテナ型](./03_data_type.ja.md#コンテナ型)のデータの中身を順に取り出したい場合などに用いられる。[プログラミング言語](./01_basic_knowledge_of_programming.ja.md#プログラミングの概要)によってはサポートしていないものや、別のキーワードで実装されている場合もある。[イテレータ](./03_data_type.ja.md#コンテナ型)は[コンテナ](./03_data_type.ja.md#コンテナ型)内での要素の[ポインタ](./03_data_type.ja.md#ポインタ型)を表し、[反復処理](#反復)のループが進むにつれて順に次の要素の[ポインタ](./03_data_type.ja.md#ポインタ型)を返す。

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

また、[オブジェクト](./03_data_type.ja.md#オブジェクト型)を[イテレート](./03_data_type.ja.md#コンテナ型)する場合の[制御構文](#制御フロー)を用意している[プログラミング言語](./01_basic_knowledge_of_programming.ja.md#プログラミングの概要)もある。

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

`loop` 文では処理[ブロック](./02_variable.ja.md#ブロックとスコープ)を**無限ループ**で実行する。基本的にはループ内に `break` 文を記述することで、途中でループを離脱するような実装とする場合が多い。通常のアプリケーションにおいて無限ループに陥るのは[バグ](./01_basic_knowledge_of_programming.ja.md#エラー)である場合が多いため、注意して使用する。
