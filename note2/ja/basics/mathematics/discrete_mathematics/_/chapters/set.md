# 『集合』

（最終更新： 2023-01-09）


## 目次

1. [集合](#集合)
	1. [代表的な集合](#代表的な集合)
	1. [集合の大きさ](#集合の大きさ)
	1. [ベン図](#ベン図)
1. [部分集合](#部分集合)
1. [補集合](#補集合)
1. [和集合と積集合](#和集合と積集合)
	1. [差集合と対称差集合](#差集合と対称差集合)
1. [ド・モルガンの法則](#ド・モルガンの法則)


## 集合

**集合**は単一または複数のものの集まりのことで、一定のルールや条件によってグループ化されている。集合に含まれるもののことを**要素**という。

### 代表的な集合

数に関する[集合](#集合)の表記として一般的に以下のものがよく用いられる。

$$
\mathbb{N} : 自然数の集合 \\
\mathbb{N_0} : 0を含めた自然数の集合 \\
\mathbb{Z} : 整数の集合 \\
\mathbb{Q} : 有理数の集合 \\
\mathbb{R} : 実数の集合
$$

### 集合の大きさ

[集合](#集合)が持つ[要素](#集合)の数を**大きさ**（**サイズ**）という。大きさが0の[集合](#集合)を**空集合**という。大きさが有限の[集合](#集合)を**有限集合**、無限の[集合](#集合)を**無限集合**という。

### ベン図

**ベン図**は複数の[集合](#集合)の関係や[集合](#集合)の範囲を図として表現したもの。ベン図中では各[集合](#集合)を円などのひとつの閉曲線で表す。


## 部分集合

2つの[集合](#集合) $A, B$ において $A$ の全ての[要素](#集合)が $B$ の[要素](#集合)であるような場合、 $A$ は $B$ の**部分集合**であるという。

2つの[集合](#集合)の[要素](#集合)が全て同じであるとき、 $A$ は $B$ の部分集合と同時に $B$ も $A$ の部分集合となる。

$A$ が $B$ の部分集合で $A$ と $B$ が同じ集合ではない時、 $A$ は $B$ の**真部分集合**であるという。


## 補集合

考える対象となる全体の[集合](#集合)を**全体集合**という。全体集合のうち、[部分集合](#部分集合) $A$ に属さない[要素](#集合)の[集合](#集合)を $A$ の**補集合**という。


## 和集合と積集合

2つの[集合](#集合)において、少なくともどちらか片方に属している[要素](#集合)の[集合](#集合)を**和集合**という。

2つの[集合](#集合)において、両方の[集合](#集合)に属している[要素](#集合)の[集合](#集合)を**積集合**（**共通集合**）という。

### 差集合と対称差集合

2つの[集合](#集合) $A, B$ において、 $A$ から $B$ を除いた[集合](#集合)を $A$ と $B$ の**差集合**という。また $A$ と $B$ のどちらか一方にだけ属する[要素](#集合)の[集合](#集合)を $A$ と $B$ の**対称差集合**という。


## ド・モルガンの法則

$U$ を[全体集合](#部分集合)、 $A, B$ を $U$ の[部分集合](#部分集合)としたとき、次の法則が成り立つ。この法則を**ド・モルガンの法則**という。

$$
\overline{A \cup B} = \overline{A} \cap \overline{B} \\
\overline{A \cap B} = \overline{A} \cup \overline{B}
$$


## 参考文献

- [山本真基. 離散数学 テキスト. 成蹊大学, 2022, 91](https://www.ci.seikei.ac.jp/yamamoto/lecture/dm/text.pdf)
