# 『プログラミング言語の種類』

（最終更新： 2023-01-24）


## 目次

1. [C言語](#c言語)
	1. [C++](#c)
	1. [C#](#c-1)
	1. [Objective-C](#objective-c)
1. [Java](#java)
	1. [Kotlin](#kotlin)
1. [Swift](#swift)
1. [Python](#python)
1. [PHP](#php)
1. [JavaScript](#javascript)
	1. [TypeScript](#typescript)
1. [Rust](#Rust)
1. [Go](#Go)


## C言語

**C言語**は、1972年に開発された汎用プログラミング言語で、高水準言語ではあるがハードウェア寄りの記述が可能な低水準言語のような特徴も持っている。コンパイラ言語・静的型付け言語に分類される。OSやデバイスドライバなど、低レイヤを中心にあらゆる分野で利用されており、プログラミング言語野中でも実行速度は最速の部類となる。また、習得難易度はスクリプト言語に比べて高く、限られたハードウェア資産で効率的に実行できるプログラムを要するケースに適している。

メモリ管理にガベージコレクションを用いず、プログラマがメモリ管理の責務を負うため、メモリに関する様々なエラーを回避してプログラムを記述する必要がある。

```c
#include <stdio.h>

int main()
{
    int input_num;
    printf("> ");
    scanf("%d", &input_num);

    if( input_num % 2 === 0 )
    {
        printf("It is an even number : %d\n", input_num);
    }
    else
    {
        printf("It is an even number : %d\n", input_num);
    }

    return 0;
}
```

### C++

**C++** は、C言語の機能や特徴を継承しつつ表現力を向上させた汎用プログラミング言語で、複数のプログラミングパラダイムが組み合わされている。オブジェクト指向やテンプレートといったC言語にはない機能を有しており、柔軟性に優れている。C言語のようにハードウェアを直接扱うような低レイヤ向けの言語としても、複雑なアプリケーションソフトウェアを開発するような高レイヤ向けの言語としても活躍している。習得難易度は比較的高く、パフォーマンスが求められる場合やハードウェア資源を効率的に利用したい場合に適している。

C言語と同様、メモリ管理にガベージコレクションを用いず、プログラマがメモリ管理の責務を負う。

### C#

**C#**は、C言語やC++に影響を受け、Microsoftが開発した汎用プログラミング言語。Windowsとの相性がよく、**.NET Framework**を用いたWindowsアプリケーション開発などに適している。また、ゲーム開発エンジンとして有名な**Unity**とも相性が良い。

C言語やC++とは異なり、ガベージコレクションによりメモリを管理できるため、習得難易度は比較的低い。

### Objective-C

**Objective-C**は、C言語をベースにして**Smalltalk**（オブジェクト指向の起源ともいえるプログラミング言語）の機能を取り込んだプログラミング言語。Appple社のmacOSやiOS向けのソフトウェア開発における標準言語。近年ではiOSアプリの開発用言語はSwiftに置き換えられつつある。


## Java

**Java**はC言語に影響を受けた汎用プログラミング言語で、静的型付け言語に分類され、メモリ管理にはガベージコレクションを用いる。様々なプラットフォームで実行できるようにJITコンパイラ方式を用いており、Javaプログラムを実行するためのソフトウェアを**JVM**（Java仮想マシン）という。また、オブジェクト指向を取り入れた言語でもある。非常に人気が高く、世界で最も使用されているプログラミング言語のひとつとなっている。

```java
import java.util.Scanner;

public class Oddeven
{
    public static void main( String[] args )

    {
        Scanner scanner = new Scanner(System.in);
        System.out.print("> ");
        int input_num = scanner.nextInt();


        if( input_num % 2 == 0 )
        {
            System.out.println("It is an even number : " + input_num);
        }
        else
        {
            System.out.println("It is an odd number : " + input_num);

        }


        scanner.close();
    }
}
```

### Kotlin

**Kotlin**は、Androidアプリを開発するための比較的新しいプログラミング言語。Javaを簡潔に安全に記述できるように改良されており、JVM上で動作する。


## Swift

**Swift**は、Apple製品のアプリケーションを開発するためにつくられたプログラミング言語。Objective-Cに代わる新たな言語として設計されており、シンプルで直感的にプログラミングができるとして人気が高い。


## Python

**Python**は、インタプリタ方式の汎用プログラミング言語。動的型付け言語に分類され、メモリの管理にはガベージコレクションを用いる。可読性が高く記述が用意であるため、プログラミングの入門やスピード感のある開発に適している。機械学習分野のライブラリが充実しているほか、Web開発にも用いられる。

```python
input_num = int(input("> "))

if ( input_num % 2 ) == 0:
    print("It is an even number : {0}".format(input_num))
else:
    print("It is an even number : {0}".format(input_num))
```


## PHP

**PHP**は、動的なWebサイトを作成するためのツールから派生したスクリプト言語。Web開発に特化しており、HTMLに埋め込むような記法を用いることができるという特徴がある。そのほかにも、学習コストが低い点やデータベースアクセスを容易に行えるという強みもある。

```php
<?php

$input_num = $_POST["input_num"];

if( $input_num % 2 === 0 )
{
    echo("It is an even number : " . $input_num);
}
else
{
    echo("It is an odd number : " . $input_num);
}

?>
```


## JavaScript

**JavaScript**は、一般的なWebブラウザ上で実行されるプログラミング言語。インタプリタ言語・動的型付け言語に分類される。Webページ上のコンテンツに動きを与える目的でよく用いられており、近年ではサーバサイドにおける実行環境の登場や、**SPA**(Single Page Application)の普及により活躍の幅が広がっている。習得難易度は比較的低く、ブラウザさえあれば実行環境が整うという手軽さもメリットのひとつである。

```javascript
let input_num = document.getElementById('#input_num').value;

if( input_num % 2 === 0 )
{
    console.log("It is an even number : " + input_num);
}
else
{
    console.log("It is an even number : " + input_num);
}
```

### TypeScript

**TypeScript**は、Microsoftによって開発されたJavaScriptのスーパーセット。大規模アプリケーション開発のために設計されており、静的型付け言語のような型宣言などの機能が搭載されている。実行時は、JavaScriptにトランスコンパイルしたものをブラウザ上で動作させる。


## Rust

**Rust**は、C言語やC++に代わるシステムプログラミング言語を目指すプログラミング言語。静的型付け言語・コンパイラ言語に分類される。また、ガベージコレクションを用いない独特のメモリ管理（**ボローチェッカー**による参照の有効性の検証）によって、メモリ安全性を確保している。パフォーマンスや信頼性の高さからLinuxのカーネルプログラムにも採用されたり、GoogleやFacebookといった企業でも一部取り入れられたりと、注目の集まる言語となっている。開発者に最も愛されているプログラミング言語としても知られるが、習得難易度はC言語やC++と同程度かそれ以上に高い。


## Go

**Go**は、高負荷がかかるシステムなどには必須の並行プログラミングがシンプルに記述できるように、Googleが開発したプログラミング言語。静的型付け言語・コンパイラ言語に分類され、メモリ管理にはガベージコレクションを用いる。Webアプリケーションやモバイルアプリなどの開発に適しており、標準ライブラリが豊富であるという特徴がある。実行速度はC言語やRustには劣るものの、同じ分野で活躍するPHPなどの言語に比べると圧倒的に高速である。