# 『基数』

（最終更新： 2023-01-10）


## 目次

1. [位取り記数法](#位取り記数法)
1. [基数](#基数)
	1. [基数の表記](#基数の表記)
	1. [10進数](#10進数)
	1. [2進数](#2進数)
	1. [8進数](#8進数)
	1. [16進数](#16進数)
	1. [位取り記数法の対応表](#位取り記数法の対応表)
1. [基数変換](#基数変換)
	1. [n進数から10進数への変換](#10進数からn進数への変換)
	1. [10進数からn進数への変換](#10進数からn進数への変換)
	1. [2進数と8進数や16進数の相互変換](#2進数と8進数や16進数の相互変換)


## 位取り記数法

**位取り記数法**とは、あらかじめ決められたn種類の記号（数字）により数を表現する方法のこと。一般的には、記号としてアラビア数字やアルファベットを使用するが、それ以外のもの（漢数字など）を用いてもよい。

> [基数](#基数)となる自然数 $n$ に対して、
> 
> ```math
> 0, 1, \cdots, n-1
> ```
> 
> の数字に任意の記号を対応させる。これを、
>
> ```math
> a_m a_{m-1} \cdots a_0.b_1 b_2 \cdots b_k
> ```
>
> という数値列で表現すると、この[基数](#基数)で表される数字列は、[10進数](#10進数)で次の数字を表す。
>
> ```math
> a_m \times n^m + a_{m-1} \times n^{m-1} + \cdots a_1 \times n + a_0 + \frac{b_1}{n} + \frac{b_2}{n^2} +\cdots + \frac{b_k}{n^k}
> ```


## 基数

**基数**は、[位取り記数法](#位取り記数法)で数を表現する際に各桁の重み付けの基本となる数のことで、位が上がるたびに何倍になるかを表す。基数が $n$ のときの[位取り記数法](#位取り記数法)のことを**n進数**（**n進法**）という。

[位取り記数法](#位取り記数法)で表された数の各桁には基数による**重み**がかかっており、n進数においては1桁目から順番に $n^0, n^1, n^2 \cdots$ という重みになる。ある桁の1つ上の桁の重みはn倍となり、1つ下の桁の重みは1/n倍となる。

### 基数の表記

[位取り記数法](#位取り記数法)では、[基数](#基数)が異なると、表記上は同じ数であっても実際に表す大きさが異なる。そのため、数の右側に[基数](#基数)を次のような形式で明記する場合がある。

```math
123.456_{10} \\
123.456_{(10)} \\
(123.456)_{10} \\
(123.456)_{(10)}
```

### 10進数

**10進数**は最も身近な[位取り記数法](#位取り記数法)で、0から9までの10つの数字を用いて数を表す。1つの桁は10になると位が上がる。ある桁の1つ上の桁は[重み](#基数)が10倍になり、1つ下の桁は[重み](#基数)が1/10倍になる。

$123.456_{10}$ は次のように表現できる。

```math
1 \times 10^2 + 2 \times 10^1 + 3 \times 10^0 + 4 \times 10^{-1} + 5 \times 10^{-2} + 6 \times 10^{-3} = 123.456
```

### 2進数

**2進数**はコンピュータ内部でデータを表現するために使用されている[位取り記数法](#位取り記数法)で、0と1の2つの数字を用いて数を表す。1つの桁は2になると位が上がる。ある桁の1つ上の桁は[重み](#基数)が2倍になり、1つ下の桁は[重み](#基数)が1/2倍になる。

2進数n桁で表現できる数は $2^n$ 通りとなる。1桁なら2通り、2桁なら4通り、3桁なら8通り、8桁なら256通りといった具合。

2進数で表されたデータ形式のことを**バイナリ**という。

$101.001_2$は[10進数](#10進数)で次の数を表している。

```math
1 \times 2^2 + 0 \times 2^1 + 1 \times 2^0 + 0 \times 2^{-1} + 0 \times 2^{-2} + 1 \times 2^{-3} = 5.125
```

### 8進数

**8進数**はコンピュータ内部のデータを表現するために使用されることのある[位取り記数法](#位取り記数法)で、0から8までの8つの数字を用いて数を表す。1つの桁は8になると位が上がる。ある桁の1つ上の桁は[重み](#基数)が8倍になり、1つ下の桁は[重み](#基数)が1/8倍になる。

[2進数](#2進数)の3桁を1つにまとめたものと等価となり、[2進数](#2進数)との相互変換が容易である。

$123.456_8$は[10進数](#10進数)で次の数を表している。

```math
1 \times 8^2 + 2 \times 8^1 + 3 \times 8^0 + 4 \times 8^{-1} + 5 \times 8^{-2} + 6 \times 8^{-3} \simeq 83.590
```

### 16進数

**16進数**はコンピュータ内部のデータを表現するために使用されることのある[位取り記数法](#位取り記数法)で、0から9までの10つの数字とAからFまでのアルファベットを用いて数を表す。1つの桁は16になると位が上がる。ある桁の1つ上の桁は[重み](#基数)が16倍になり、1つ下の桁は[重み](#基数)が1/16倍になる。

[2進数](#2進数)の4桁を1つにまとめたものと等価となり、[2進数](#2進数)との相互変換が容易である。

$123.ABC_{16}$は[10進数](#10進数)で次の数を表している。

```math
1 \times 16^2 + 2 \times 16^1 + 3 \times 16^0 + 10 \times 16^{-1} + 11 \times 16^{-2} + 12 \times 16^{-3} \simeq 291.671
```

### 位取り記数法の対応表

|  2進数 |  8進数 | 10進数 | 16進数 |  2進数 |  8進数 | 10進数 | 16進数 |
|--------|--------|--------|--------|--------|--------|--------|--------|
|      0 |      0 |      0 |      0 |  10001 |     21 |     17 |     11 |
|      1 |      1 |      1 |      1 |  10010 |     22 |     18 |     12 |
|     10 |      2 |      2 |      2 |  10011 |     23 |     19 |     13 |
|     11 |      3 |      3 |      3 |  10100 |     24 |     20 |     14 |
|    100 |      4 |      4 |      4 |  10101 |     25 |     21 |     15 |
|    101 |      5 |      5 |      5 |  10110 |     26 |     22 |     16 |
|    110 |      6 |      6 |      6 |  10111 |     27 |     23 |     17 |
|    111 |      7 |      7 |      7 |  11000 |     30 |     24 |     18 |
|   1000 |     10 |      8 |      8 |  11001 |     31 |     25 |     19 |
|   1001 |     11 |      9 |      9 |  11010 |     32 |     26 |     1A |
|   1010 |     12 |     10 |      A |  11011 |     33 |     27 |     1B |
|   1011 |     13 |     11 |      B |  11100 |     34 |     28 |     1C |
|   1100 |     14 |     12 |      C |  11101 |     35 |     29 |     1D |
|   1101 |     15 |     13 |      D |  11110 |     36 |     30 |     1E |
|   1110 |     16 |     14 |      E |  11111 |     37 |     31 |     1F |
|   1111 |     17 |     15 |      F | 100000 |     40 |     32 |     20 |
|  10000 |     20 |     16 |     10 | 100001 |     41 |     33 |     21 |


## 基数変換

ある[基数](#基数)で表された数を別の[基数](#基数)で表現することを、**基数変換**という。一般的には数を[10進数](#10進数)で表現することが多いため、[10進数](#10進数)と[n進数](#基数)との変換が重要となる。

### n進数から10進数への変換

[n進数](#基数)で表された数の各桁に[重み](#基数)をかけて足し合わせると、[10進数](#10進数)に変換することができる。変換式の定義は[位取り記数法](#位取り記数法)に、具体的な変換の例は[2進数](#2進数)、[8進数](#8進数)、[16進数](#16進数)にそれぞれ示す。

### 10進数からn進数への変換

まずは[10進数](#10進数)の値を整数部と小数部に分ける。

整数部については、[10進数](#10進数)で表された数を任意の[基数](#基数)nで繰り返し割り続け、それ以上割れなくなった時点で最後の商と各回の余りを逆順に並べる。

小数部については、[10進数](#10進数)で表された数に任意の[基数](#基数)nをかけて整数部を取るという操作を続け、小数部が0になった時点で各回の整数部の値を順番に並べる。ただし小数部は無限小数となる場合がある。

整数部と小数部で得られた数値列がそれぞれ変換後の[基数](#基数)における整数部と小数部となる。

### 2進数と8進数や16進数の相互変換

[8進数](#8進数)、[16進数](#16進数)はそれぞれ[2進数](#2進数)の3桁、4桁と等価であるため、[2進数](#2進数)の数を3桁あるいは4桁ずつ区切って読み替えることで容易に変換することができる。

[8進数](#8進数)や[16進数](#16進数)の値を[2進数](#2進数)に変換する場合も、各桁を[2進数](#2進数)に変換して並べるだけでよい。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
