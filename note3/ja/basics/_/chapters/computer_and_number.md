# 『コンピュータと数』

（最終更新： 2023-01-11）


## 目次

1. [コンピュータとデータ](#コンピュータとデータ)
	1. [データの単位](#データの単位)
	1. [MSBとLSB](#msbとlsb)


## コンピュータとデータ

コンピュータは多くの電子回路から構成されており、スイッチのON/OFFや電圧の高低により信号を伝達している。これらの信号を0と1に対応させる（ONなら1でOFFなら0、5V（電圧高）なら1で0V（電圧低）なら0など）ことで、コンピュータ内部では情報を[2進数](./discrete_mathematics/_/chapters/radix.md#2進数)として扱っている。コンピュータが扱う情報は、数字であろうと文字であろうと命令であろうと、全て[2進数](./discrete_mathematics/_/chapters/radix.md#2進数)の羅列で表現される。

[2進数](./discrete_mathematics/_/chapters/radix.md#2進数)で様々な情報を表現しようとすると桁数が大きくなってしまうため、1桁でより多くの情報を表せるように[8進数](./discrete_mathematics/_/chapters/radix.md#8進数)や[16進数](./discrete_mathematics/_/chapters/radix.md#16進数)もコンピュータの世界でよく使用される。これらの[基数](./discrete_mathematics/_/chapters/radix.md#基数)が用いられることが多いのは、[2進数](./discrete_mathematics/_/chapters/radix.md#2進数)との相互変換が容易なためである。また、一般的に数は[10進数](./discrete_mathematics/_/chapters/radix.md#10進数)で扱われるため、[2進数](./discrete_mathematics/_/chapters/radix.md#2進数)や[8進数](./discrete_mathematics/_/chapters/radix.md#8進数)、[16進数](./discrete_mathematics/_/chapters/radix.md#16進数)と[10進数](./discrete_mathematics/_/chapters/radix.md#10進数)の相互変換もよく行われる。

### データの単位

コンピュータ内部で扱われるデータの最小単位は[2進数](./discrete_mathematics/_/chapters/radix.md#2進数)で表された1桁で、これを**ビット**(bit)という。

複数のビットを集めたデータ量の単位を**バイト**(byte)といい、一般的には1バイトは8ビットと等価である。1バイトは256通りの情報を表現することができる。

複数のビットを集めたデータ量の単位としては他にも**ワード**があり、これはコンピュータ内部で情報をCPUやメモリによって処理する際の単位を表している。ワードサイズはコンピュータやOSによって異なるが、4バイトや8バイトが一般的。

### MSBとLSB

[ビット](#データの単位)列において一番左の[ビット](#データの単位)のことを**最上位ビット**(**MSB**: Most Significant Bit)、一番右側の[ビット](#データの単位)のことを**最下位ビット**(**LSB**: Least Significant Bit)という。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
