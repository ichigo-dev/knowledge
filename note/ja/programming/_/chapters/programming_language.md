# 『プログラミング言語の種類』

（最終更新： 2023-03-03）


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

**C言語**は、1972年に開発された[汎用プログラミング言語](./programming.md#汎用プログラミング言語)で、[高水準言語](./programming.md#高水準言語)ではあるが[ハードウェア](../../../computer/hardware/_/chapters/basic_knowledge_of_hardware.md#ハードウェア)寄りの記述が可能な[低水準言語](./programming.md#低水準言語)のような特徴も持っている。[コンパイラ言語](./programming.md#コンパイラ言語)・[静的型付け言語](./programming.md#静的型付け言語)に分類される。[OS](../../../computer/software/_/chapters/operation_system.md#オペレーティングシステム-1)や[デバイスドライバ](../../../computer/hardware/_/chapters/io_unit.md#デバイスドライバ)など、低レイヤを中心にあらゆる分野で利用されており、[プログラミング言語](./programming.md#プログラミング言語)の中でも実行速度は最速の部類となる。また、習得難易度は[スクリプト言語](./programming.md#スクリプト言語)に比べて高く、限られた[ハードウェア](../../../computer/hardware/_/chapters/basic_knowledge_of_hardware.md#ハードウェア)資産で効率的に実行できる[プログラム](./programming.md#プログラム)を要するケースに適している。

[メモリ管理](./programming.md#プログラムのメモリ管理)に[ガベージコレクション](./programming.md#ガベージコレクション)を用いず、[プログラマ](./programming.md#プログラマ)が[メモリ管理](./programming.md#プログラムのメモリ管理)の責務を負うため、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)に関する様々な[エラー](./programming.md#エラー)を回避して[プログラム](./programming.md#プログラム)を記述する必要がある。

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

**C++** は、[C言語](#c言語)の機能や特徴を継承しつつ表現力を向上させた[汎用プログラミング言語](./programming.md#汎用プログラミング言語)で、複数の[プログラミングパラダイム](./programming.md#プログラミングパラダイム)が組み合わされている。[オブジェクト指向](./object_oriented.md#オブジェクト指向-1)やテンプレートといった[C言語](#c言語)にはない機能を有しており、柔軟性に優れている。[C言語](#c言語)のように[ハードウェア](../../../computer/hardware/_/chapters/basic_knowledge_of_hardware.md#ハードウェア)を直接扱うような低レイヤ向けの言語としても、複雑な[アプリケーションソフトウェア](../../../computer/software/_/chapters/basic_knowledge_of_software.md#応用ソフトウェア)を開発するような高レイヤ向けの言語としても活躍している。習得難易度は比較的高く、パフォーマンスが求められる場合や[ハードウェア](../../../computer/hardware/_/chapters/basic_knowledge_of_hardware.md#ハードウェア)資源を効率的に利用したい場合に適している。

[C言語](#c言語)と同様、[メモリ管理](./programming.md#プログラムのメモリ管理)に[ガベージコレクション](./programming.md#ガベージコレクション)を用いず、[プログラマ](./programming.md#プログラマ)が[メモリ管理](./programming.md#プログラムのメモリ管理)の責務を負う。

### C#

**C#** は、[C言語](#c言語)や[C++](#c-1)に影響を受けMicrosoftが開発した、[汎用プログラミング言語](./programming.md#汎用プログラミング言語)。[Windows](../../../computer/software/_/chapters/operation_system.md#windows)との相性がよく、**.NET Framework**を用いた[Windows](../../../computer/software/_/chapters/operation_system.md#windows)[アプリケーション](../../../computer/software/_/chapters/basic_knowledge_of_software.md#応用ソフトウェア)開発などに適している。また、ゲーム開発エンジンとして有名な**Unity**とも相性が良い。

[C言語](#c言語)や[C++](#c-1)とは異なり、[ガベージコレクション](./programming.md#ガベージコレクション)により[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)を管理できるため、習得難易度は比較的低い。

### Objective-C

**Objective-C**は、[C言語](#c言語)をベースにして**Smalltalk**（[オブジェクト指向](./object_oriented.md#オブジェクト指向-1)の起源ともいえる[プログラミング言語](./programming.md#プログラミング言語)）の機能を取り込んだ[プログラミング言語](./programming.md#プログラミング言語)。Appple社の[macOS](../../../computer/software/_/chapters/operation_system.md#macos)や[iOS](../../../computer/software/_/chapters/operation_system.md#ios)向けの[ソフトウェア](../../../computer/software/_/chapters/basic_knowledge_of_software.md#ソフトウェア)開発における標準言語。近年では[MacOS](../../../computer/software/_/chapters/operation_system.md#macos)、[iOS](../../../computer/software/_/chapters/operation_system.md#ios)[アプリ](../../../computer/software/_/chapters/basic_knowledge_of_software.md#応用ソフトウェア)の開発用言語は[Swift](#swift)に置き換えられつつある。


## Java

**Java**は、[C言語](#c言語)に影響を受けた[汎用プログラミング言語](./programming.md#汎用プログラミング言語)で、[静的型付け言語](./programming.md#静的型付け言語)に分類され、[メモリ管理](./programming.md#プログラムのメモリ管理)には[ガベージコレクション](./programming.md#ガベージコレクション)を用いる。様々な[プラットフォーム](../../../computer/software/_/chapters/basic_knowledge_of_software.md#プラットフォーム)で実行できるように[JITコンパイラ](./programming.md#jitコンパイラ)方式を用いており、Java[プログラム](./programming.md#プログラム)を実行するためのソフトウェアを**JVM**（Java仮想マシン）という。また、[オブジェクト指向](./object_oriented.md#オブジェクト指向-1)を取り入れた言語でもある。非常に人気が高く、世界で最も使用されている[プログラミング言語](./programming.md#プログラミング言語)のひとつとなっている。

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

**Kotlin**は、[Android](../../../computer/software/_/chapters/operation_system.md#android)[アプリ](../../../computer/software/_/chapters/basic_knowledge_of_software.md#応用ソフトウェア)を開発するための比較的新しい[プログラミング言語](./programming.md#プログラミング言語)。[Java](#java)を簡潔に安全に記述できるように改良されており、[JVM](#java)上で動作する。


## Swift

**Swift**は、Apple製品の[アプリケーション](../../../computer/software/_/chapters/basic_knowledge_of_software.md#応用ソフトウェア)を開発するためにつくられた[プログラミング言語](./programming.md#プログラミング言語)。[Objective-C](#objective-c)に代わる新たな言語として設計されており、シンプルで直感的に[プログラミング](./programming.md#プログラミング)ができるとして人気が高い。


## Python

**Python**は、[インタプリタ](./programming.md#インタプリタ言語)方式の[汎用プログラミング言語](./programming.md#汎用プログラミング言語)。[動的型付け言語](./programming.md#動的型付け言語)に分類され、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ-1)の管理には[ガベージコレクション](./programming.md#ガベージコレクション)を用いる。可読性が高く記述が用意であるため、[プログラミング](./programming.md#プログラミング)の入門やスピード感のある開発に適している。[機械学習](../../../artificial_intelligence/_/chapters/machine_learning.md#機械学習-1)分野の[ライブラリ](../../../computer/software/_/chapters/package.md#ライブラリ)が充実しているほか、[Web](../../../network/_/chapters/web.md#web-1)開発にも用いられる。

```python
input_num = int(input("> "))

if ( input_num % 2 ) == 0:
    print("It is an even number : {0}".format(input_num))
else:
    print("It is an even number : {0}".format(input_num))
```


## PHP

**PHP**は、動的な[Web](../../../network/_/chapters/web.md#web-1)サイトを作成するためのツールから派生した[スクリプト言語](./programming.md#スクリプト言語)。[Web](../../../network/_/chapters/web.md#web-1)開発に特化しており、HTMLに埋め込むような記法を用いることができるという特徴がある。そのほかにも、学習コストが低い点や[データベース](../../../development/database/_/chapters/basic_knowledge_of_database.md#データベース)アクセスを容易に行えるという強みもある。

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

**JavaScript**は、一般的な[Webブラウザ](../../../network/_/chapters/web.md#webブラウザ)上で実行される[プログラミング言語](./programming.md#プログラミング言語)。[インタプリタ言語](./programming.md#インタプリタ言語)・[動的型付け言語](./programming.md#動的型付け言語)に分類される。[Web](../../../network/_/chapters/web.md#web-1)ページ上のコンテンツに動きを与える目的でよく用いられており、近年では[サーバ](../../../network/_/chapters/web.md#webサーバ)サイドにおける実行環境の登場や、**SPA**(Single Page Application)の普及により活躍の幅が広がっている。習得難易度は比較的低く、[ブラウザ](../../../network/_/chapters/web.md#webブラウザ)さえあれば実行環境が整うという手軽さもメリットのひとつである。

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
