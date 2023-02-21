# 『伝送理論』


（最終更新： 2023-02-21）


## 目次

1. [伝送路](#伝送路)
	1. [単方向](#単方向)
	1. [半二重](#半二重)
	1. [全二重](#全二重)
1. [誤り検出と訂正](#誤り検出と訂正)
	1. [パリティ](#パリティ)
	1. [ハミング符号](#ハミング符号)
	1. [CRC](#crc)


## 伝送路

**伝送路**は、電気信号により情報を伝送するための媒体のこと。

### 単方向

**単方向**の[伝送路](#伝送路)は、決まった方向にしか通信できない。

### 半二重

**半二重**の[伝送路](#伝送路)は、双方向の通信が可能であるが、送信と受信が同時に行えない。

### 全二重

**全二重**の[伝送路](#伝送路)は、双方向の通信が可能であり、送信と受信を同時に行うことができる。


## 誤り検出と訂正

機器同士が通信を行う際には、回線状況などにより伝送データの送信誤りが起こる場合がある。このような誤りを検出したり、受信側で誤りを訂正するための手法がいくつかある。

### バースト誤り

**バースト誤り**は、通信回線の混線やケーブルの不具合などにより、一度にまとめて起こる誤り。

### パリティ

**パリティ**は、数字の並びの合計値が偶数であるか奇数であるかによって伝送誤りを検出する方法。この方法では、データの最後に**パリティビット**を付加する。[ビット](../../../_/chapters/computer_and_number.md#ビット)列の $1$ の数が偶数になるように付加されたものを**偶数パリティ**、奇数になるように付加されたものを**奇数パリティ**という。

パリティは方法によっては誤り訂正も行うことができる。分割された伝送データの最後に毎回パリティビット（**垂直パリティ**）を付加し、全てのデータの伝送が終わった後にデータ全体を横断的に見たときのパリティビット列（**水平パリティ**）を伝送する。垂直パリティと水平パリティの組み合わせにより、どの[ビット](../../../_/chapters/computer_and_number.md#ビット)で誤りがあるかを判断することができ、その[ビット](../../../_/chapters/computer_and_number.md#ビット)を反転することで誤りを訂正する。

### ハミング符号

**ハミング符号**は、データにいくつかの冗長[ビット](../../../_/chapters/computer_and_number.md#ビット)を付加することで、1[ビット](../../../_/chapters/computer_and_number.md#ビット)の誤りを検出し、それを訂正する方法。

ハミング符号では、ある整数 $m$ に対して、符号化するデータの[ビット](../../../_/chapters/computer_and_number.md#ビット)数 $k$ と、**符号語**の長さ $n$ は次のようになる。

```math
\begin{array}{ccc}
k & = & n - m \
n & = & 2^m - 1
\end{array}
```

ハミング符号では最初に、 $m$ [行](../../../applied_mathematics/_/chapters/numerical_calculation.md#行列) $n$ [列](../../../applied_mathematics/_/chapters/numerical_calculation.md#行列)の**検査行列** $H$ を求める。 $m = 3$ （ $n = 7$ ）の場合、 次のような検査行列となる。

```math
H =
\left[
\begin{array}{ccccccc}
1 & 0 & 1 & 1 & 1 & 0 & 0 \\
1 & 1 & 0 & 1 & 0 & 1 & 0 \\
0 & 1 & 1 & 1 & 0 & 0 & 1
\end{array}
\right]
```

検査行列は、全ての[列要素](../../../applied_mathematics/_/chapters/numerical_calculation.md#行列)がゼロではなく、それぞれが相違となるような[ビット](../../../_/chapters/computer_and_number.md#ビット)列を並べたものとなる。[列](../../../applied_mathematics/_/chapters/numerical_calculation.md#行列)の並べ方は任意で、上記の例に限らない。

次に、 $HG^{T} = GH^{T} = 0$ を満たすような**生成行列** $G$ を求める。前述の検査行列に対する生成行列は次のようになる。

```math
G =
\left[
\begin{array}{ccccccc}
1 & 0 & 0 & 0 & 1 & 1 & 0 \\
0 & 1 & 0 & 0 & 0 & 1 & 1 \\
0 & 0 & 1 & 0 & 1 & 0 & 1 \\
0 & 0 & 0 & 1 & 1 & 1 & 1
\end{array}
\right]
```

そして、送信したい情報と生成行列の[積](../../../applied_mathematics/_/chapters/numerical_calculation.md#行列の積)をとった結果得られる答えが、ハミング符号化後の符号語となる。送信したいデータを $\left[ \begin{array}{cccc} 1 & 0 & 1 & 1 \right]$ とすると、符号語は次のようになる。

```math
[1 0 1 1]

\cdot

\left[
\begin{array}{ccccccc}
1 & 0 & 0 & 0 & 1 & 1 & 0 \\
0 & 1 & 0 & 0 & 0 & 1 & 1 \\
0 & 0 & 1 & 0 & 1 & 0 & 1 \\
0 & 0 & 0 & 1 & 1 & 1 & 1
\end{array}
\right]

=

\left[
\begin{array}{ccccccc}
1 & 0 & 1 & 1 & 1 & 0 & 0
\end{array}
\right]
```

受信側では、**受信語** $Y$ に対して次のような関係が成り立つ。ここで、 $x$ は複合後のデータとし、送信時の誤りは発生していないものとする。

```math
\begin{array}{ccc}
Y    & = & xG \
YH^T & = & xGH^T \
YH^T & = & 0
\end{array}
```

送信時の誤りが発生していた場合、次のような関係が成り立つ。ここで、 $e_i$ は**誤りベクトル**とする。

```math
\begin{array}{ccc}
Y    & = & xG \oplus e_i \
YH^T & = & (xG \oplus e_i)H^T \
YH^T & = & xGH^T \oplus e_i H^T \
YH^T & = & e_i H^T
\end{array}
```

これらの関係より、受信語 $Y$ と検査行列の[転置行列](../../../applied_mathematics/_/chapters/numerical_calculation.md#転置行列)の[積](../../../applied_mathematics/_/chapters/numerical_calculation.md#行列の積)が $0$ であった場合には誤りはなく、非 $0$ であった場合には、 $e_i^T$ に対応する検査行列の[列](../../../applied_mathematics/_/chapters/numerical_calculation.md#行列)が誤っているということがわかる。例えば、受信語が `1111100` であった場合は次のような誤りベクトルが得られる。

```math
\left[
\begin{array}{ccccccc}
1 & 1 & 1 & 1 & 1 & 0 & 0
\end{array}
\right]

\cdot

\left[
\begin{array}{ccccccc}
1 & 0 & 1 & 1 & 1 & 0 & 0 \\
1 & 1 & 0 & 1 & 0 & 1 & 0 \\
0 & 1 & 1 & 1 & 0 & 0 & 1
\end{array}
\right]^T

=

\left[
\begin{array}{ccc}
0 & 1 & 1
\end{array}
\right]
```

この場合、誤りベクトルの[転置行列](../../../applied_mathematics/_/chapters/numerical_calculation.md#転置行列)は検査行列の2[列](../../../applied_mathematics/_/chapters/numerical_calculation.md#行列)目と一致するので、受信語の2[ビット](../../../_/chapters/computer_and_number.md#ビット)目が誤っている、すなわち正しい受信語は `1011100` であることがわかる。

### CRC

**CRC**(Cyclic Redundancy Check)は、連続する誤り（[バースト誤り](#バースト誤り)）を検出するための誤り制御の仕組み。送信する基となるデータを、あらかじめ決められた多項式で除算し、その余りをCRCとする。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)
