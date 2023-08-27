# 『文字の表現』ノート

（最終更新： 2023-03-03）


## 目次

1. [文字コード](#文字コード)
	1. [マルチバイト文字](#マルチバイト文字)
	1. [ASCIIコード](#asciiコード)
	1. [EUCコード](#eucコード)
	1. [JISコード](#jisコード)
	1. [シフトJISコード](#シフトjisコード)
	1. [Unicode](#unicode)


## 文字コード

**文字コード**は、[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)内部で文字を[2進数](../../../discrete_mathematics/_/chapters/radix.md#2進数)の[ビット](../../../_/chapters/computer_and_number.md#ビット)パターンに割り当てたもの。文字コードには複数の体系が存在しており、情報をやり取りするためにはコード体系が合致している必要がある。

### マルチバイト文字

**マルチバイト文字**は、[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)上で1文字を複数[バイト](../../../_/chapters/computer_and_number.md#バイト)で表現するような体系。アルファベットや数字、一部の記号は、[ASCIIコード](#asciiコード)により1[バイト](../../../_/chapters/computer_and_number.md#バイト)で表現することができるが、全角カナや漢字などは1[バイト](../../../_/chapters/computer_and_number.md#バイト)では表現しきれないためマルチバイト文字を用いる。

### ASCIIコード

**ASCIIコード**は、1文字を7[ビット](../../../_/chapters/computer_and_number.md#ビット)の符号と誤り検出用の[パリティ](../../../communication_theory/_/chapters/transmission_theory.md#パリティ)[ビット](../../../_/chapters/computer_and_number.md#ビット)1[ビット](../../../_/chapters/computer_and_number.md#ビット)で表現した[文字コード](#文字コード)。基本的にはアルファベットと数字、記号に用いられており、日本語などは文字種が多いため7[ビット](../../../_/chapters/computer_and_number.md#ビット)では多くの文字は表現しきれない。

### EUCコード

**EUCコード**は、[UNIX](../../../../computer/software/_/chapters/operating_system.md#unix)上で2[バイト](../../../_/chapters/computer_and_number.md#バイト)文字と1[バイト](../../../_/chapters/computer_and_number.md#バイト)文字を混在して用いる[文字コード](#文字コード)。

### JISコード

**JISコード**は、漢字やひらがなを含む[文字コード](#文字コード)で、漢字やひらがなは2[バイト](../../../_/chapters/computer_and_number.md#バイト)、英数字や記号は1[バイト](../../../_/chapters/computer_and_number.md#バイト)で表現される。

### シフトJISコード

**シフトJISコード**は、[JISコード](#jisコード)を[シフト](../../../discrete_mathematics/_/chapters/arithmetic_operation_and_precision.md#シフト演算)することで[ASCIIコード](#asciiコード)との混在を可能とした[文字コード](#文字コード)。日本語を扱うパソコンで標準的に利用されている。

### Unicode

**Unicode**は、世界各国の文字体系全てに対応するための[文字コード](#文字コード)。すべての文字を2[バイト](../../../_/chapters/computer_and_number.md#バイト)で表現する。

**UTF-8**、**UTF-16**、**UTF-32**の3つのエンコーディング方式により符号化される。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)
