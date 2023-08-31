# 『コンピュータと数』ノート

（最終更新： 2023-08-31）


## 目次

1. [コンピュータとデータ](#コンピュータとデータ)
	1. [バイナリ](#バイナリ)
	1. [ビット](#ビット)
	1. [バイト](#バイト)
	1. [ワード](#ワード)
	1. [MSB](#msb)
	1. [LSB](#lsb)


## コンピュータとデータ

[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)は、情報を処理するために設計された電子機器の総称で、スイッチのON/OFFや電圧の高低により信号を伝達している。これらの信号を $0$ と $1$ に対応させる（ONなら $1$ でOFFなら $0$ 、 $5$ V（電圧高）なら $1$ で $0$ V（電圧低）なら $0$ など）ことで、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)内部では情報を[ビット](#ビット)列として扱っている。[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)が扱う情報は、数字であろうと文字であろうと命令であろうと、全てこのような[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)の羅列で表現される。

[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)で様々な情報を表現しようとすると桁数が大きくなってしまうため、1桁でより多くの情報を表せるように[8進数](../../discrete_mathematics/_/chapters/radix.md#8進数)や[16進数](../../discrete_mathematics/_/chapters/radix.md#16進数)でデータを表現することも[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)の世界では一般的。これらの[基数](../../discrete_mathematics/_/chapters/radix.md#基数)が用いられることが多いのは、[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)との[相互変換](../../discrete_mathematics/_/chapters/radix.md#基数変換)が容易なためである。また、一般的に数は[10進数](../../discrete_mathematics/_/chapters/radix.md#10進数)で扱われるため、[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)や[8進数](../../discrete_mathematics/_/chapters/radix.md#8進数)や[16進数](../../discrete_mathematics/_/chapters/radix.md#16進数)と[10進数](../../discrete_mathematics/_/chapters/radix.md#10進数)の[相互変換](../../discrete_mathematics/_/chapters/radix.md#基数変換)もよく行われる。

### バイナリ

**バイナリ**は、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)が扱うデータの一種で、[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)で表現されたデータ（[ビット](#ビット)列）を指す。

[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)において[プログラム](../../../programming/_/chapters/programming.md#プログラム)やデータは、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)内にバイナリ形式で格納されている。[プログラム](../../../programming/_/chapters/programming.md#プログラム)を実行するには、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)がバイナリを解釈して命令を処理する必要がある。同様に、データもバイナリ形式で格納され、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)がその内容を解釈して利用する。

### ビット

**ビット**(bit)は、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)内部で扱われるデータの最小単位で、 $0$ か $1$ のどちらかの値を持つ。ビットという言葉は"Binary digit"からきており、情報を格納するための最小単位となる。[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)内のすべての情報はビット列（[バイナリ](#バイナリ)）で表現されており、ビット単位の[論理演算](../../discrete_mathematics/_/chapters/logical_operation.md#論理演算)の組み合わせにより様々な処理を行っている。

### バイト

**バイト**(byte)は、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)内部で扱われるデータの単位のひとつで、8[ビット](#ビット)を1つにまとめたもの。1バイト（=8[ビット](#ビット)）は256通りの情報を表現することができる。[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)が扱う[ファイル](../../../computer/software/_/chapters/file_system.md#ファイル)のサイズは一般的にバイト単位で表現される。

### ワード

**ワード**は、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)内部で扱われるデータの単位のひとつで、通常は16[ビット](#ビット)（2[バイト](#バイト)）か32[ビット](#ビット)（4[バイト](#バイト)）、あるいは64[ビット](#ビット)（8[バイト](#バイト)）の長さを持つ。[CPU](../../../computer/hardware/_/chapters/processor.md#cpu)や[レジスタ](../../../computer/hardware/_/chapters/processor.md#レジスタ)、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)などでデータを扱う際の基本単位となる。

ワードの長さ（**ワードサイズ**）は、[CPU](../../../computer/hardware/_/chapters/processor.md#cpu)の[アーキテクチャ](../../../computer/hardware/_/chapters/processor.md#cpuアーキテクチャ)や設計によって異なる。16[ビット](#ビット)のワードを使用する[CPU](../../../computer/hardware/_/chapters/processor.md#cpu)は16ビットCPU、32[ビット](#ビット)のワードを使用する[CPU](../../../computer/hardware/_/chapters/processor.md#cpu)は32ビットCPUのように呼ばれ、現在では64[ビット](#ビット)の[CPU](../../../computer/hardware/_/chapters/processor.md#cpu)が主流となっている。ワードサイズが大きい[CPU](../../../computer/hardware/_/chapters/processor.md#cpu)ほど一度に扱えるデータ量が大きくなるため、より高速な処理が可能となる。

[プログラミング](../../../programming/_/chapters/programming.md#プログラミング)における[基本データ型](../../../programming/_/chapters/data_type.md#プリミティブ型)も、一般的には[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)のワードサイズに合わせた大きさとなっている。例えば、[C言語](../../../programming/_/chapters/programming_language.md#c言語)では `int` [型](../../../programming/_/chapters/data_type.md#型)は32[ビット](#ビット)、 `short` [型](../../../programming/_/chapters/data_type.md#型)は16[ビット](#ビット)の情報を格納するための[型](../../../programming/_/chapters/data_type.md#型)となっている。

### MSB

**MSB**（**最上位ビット**: Most Significant Bit）は、[ビット](#ビット)列において一番左の[ビット](#ビット)。

[2進数](../../discrete_mathematics/_/chapters/radix.md#2進数)の正負の数の表現においては、MSBが $0$ の場合その数は正の数として、 $1$ の場合は負の数として扱われる。

また、[ビッグエンディアン方式](../../information_theory/_/chapters/coding_theory.md#ビッグエンディアン)のデータ表現においては、MSBがデータの先頭となる。

### LSB

**LSB**（**最下位ビット**: Least Significant Bit）は、[ビット](#ビット)列において一番右の[ビット](#ビット)。

[リトルエンディアン方式](../../information_theory/_/chapters/coding_theory.md#リトルエンディアン)のデータ表現においては、LSBがデータの先頭となる。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
