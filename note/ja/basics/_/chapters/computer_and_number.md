# 『コンピュータと数』

（最終更新： 2023-02-18）


## 目次

1. [コンピュータとデータ](#コンピュータとデータ)
	1. [ビット](#ビット)
	1. [バイト](#バイト)
	1. [ワード](#ワード)
	1. [MSB](#msb)
	1. [LSB](#lsb)


## コンピュータとデータ

コンピュータは多くの電子回路から構成されており、スイッチのON/OFFや電圧の高低により信号を伝達している。これらの信号を0と1に対応させる（ONなら1でOFFなら0、5V（電圧高）なら1で0V（電圧低）なら0など）ことで、コンピュータ内部では情報を[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)として扱っている。コンピュータが扱う情報は、数字であろうと文字であろうと命令であろうと、全て[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)の羅列で表現される。

[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)で様々な情報を表現しようとすると桁数が大きくなってしまうため、1桁でより多くの情報を表せるように[8進数](../../discrete_mathematics/_/chapters/radix.md#8進数)や[16進数](../../discrete_mathematics/_/chapters/radix.md#16進数)を使用することもコンピュータの世界では一般的である。これらの[基数](../../discrete_mathematics/_/chapters/radix.md#基数)が用いられることが多いのは、[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)との[相互変換](../../discrete_mathematics/_/chapters/radix.md#基数変換)が容易なためである。また、一般的に数は[10進数](../../discrete_mathematics/_/chapters/radix.md#2進数)で扱われるため、[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)や[8進数](../../discrete_mathematics/_/chapters/radix.md#8進数)、[16進数](../../discrete_mathematics/_/chapters/radix.md#16進数)と[10進数](../../discrete_mathematics/_/chapters/radix.md#10進数)の[相互変換](../../discrete_mathematics/_/chapters/radix.md#基数変換)もよく行われる。

### ビット

**ビット**(bit)は、コンピュータ内部で扱われるデータの最小単位で、0か1の[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)で表現される。

### バイト

**バイト**(byte)は、複数の[ビット](#ビット)を集めたデータ量の単位で、一般的には1バイトは8[ビット](#ビット)と等価である。1バイト（=8[ビット](#ビット)）は256通りの情報を表現することもできる。

### ワード

**ワード**は、複数の[ビット](#ビット)を集めたデータ量の単位で、コンピュータ内部で情報を[CPU](../../../computer/hardware/_/chapters/processor.md#cpu)や[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)によって処理する際の単位を表している。ワードサイズはコンピュータや[OS](../../../computer/software/_/chapters/operation_system.md#オペレーティングシステム)によって異なるが、4[バイト](#バイト)や8[バイト](#バイト)が一般的。

### MSB

**MSB**（**最上位ビット**: Most Significant Bit）は、[ビット](#ビット)列において一番左の[ビット](#ビット)のこと。

### LSB

**LSB**（**最下位ビット**: Least Significant Bit）は、[ビット](#ビット)列において一番右の[ビット](#ビット)のこと。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
