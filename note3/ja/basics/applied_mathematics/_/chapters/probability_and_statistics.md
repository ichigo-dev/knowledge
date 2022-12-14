# 『確率と統計』

（最終更新： 2023-01-12）


## 目次

1. [順列](#順列)
1. [組合せ](#組合せ)
1. [確率](#確率)
	1. [排反](#排反)
	1. [独立](#独立)
	1. [余事象](#余事象)
	1. [加法定理](#加法定理)
	1. [乗法定理](#乗法定理)
	1. [条件付き確率](#条件付き確率)
	1. [ベイズの定理](#ベイズの定理)
1. [基本統計量](#基本統計量)
	1. [代表値と散布値](#代表値と散布値)
	1. [平均値](#平均値)
	1. [中央値](#中央値)
	1. [最頻値](#最頻値)
	1. [範囲](#範囲)
	1. [分散](#分散)
	1. [標準偏差](#標準偏差)
1. [確率分布](#確率分布)
	1. [確率変数と確率分布](#確率変数と確率分布)
	1. [ベルヌーイ試行](#ベルヌーイ試行)
	1. [二項分布](#二項分布)
	1. [ポアソン分布](#ポアソン分布)
	1. [正規分布](#正規分布)
1. [マルコフ過程](#マルコフ過程)
	1. [マルコフ連鎖](#マルコフ連鎖)
1. [相関関係](#相関関係)
1. [回帰直線](#回帰直線)
	1. [回帰分析](#回帰分析)
	1. [ロジスティック回帰分析](#ロジスティック回帰分析)


## 順列

**順列**は、複数の異なる[要素](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)を持つ[集合](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)から[要素](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)を取り出し、それらを順に並べたもの。

[要素](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)数が $n$ 個の場合、それを並べる並べ方（順列の数）は以下の式で表される。

```math
1 \times 2 \times \cdots (n - 1) \times n = n!
```

また、 $n$ 個の[要素](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)の中から $r$ 個の[要素](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)を取り出して、それを並べる並べ方は以下の式で表される。

```math
\begin{array}{cc}
{}_n P_r = \frac{n!}{(n - r)!} & (0 \leq r \leq n)
\end{array}
```


## 組合せ

**組合せ**は、複数の異なる[要素](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)を持つ[集合](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)からいくつか[要素](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)を取り出すときのパターンの総数。[順列](#順列)とは異なり取り出す順番は意味を持たない。

[要素](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)数が $n$ 個の[集合](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)から $r$ 個の[要素](../../../discrete_mathematics/_/chapters/set_and_proposition.md#集合)を取り出す組み合わせの数は、以下の式で表される。

```math
{}_n C_r = \frac{{}_n P_r}{r!} = \frac{n!}{(n - r)!r!}
```


## 確率

**確率**とは、ある試行に対して得られる全ての結果に対し、特定の事象が起こる頻度（起こりやすさ）のこと。

**試行**とは、同じ条件で何度も繰り返すことができ、その結果が偶然によって決まる実験や観測のこと。試行の結果起こる事柄を**事象**という。

ある試行によって事象 $A$ の起こる確率 $P(A)$ は、事象 $A$ の起こりうる場合の数を $n$ 、全事象の数を $m$ とすると以下の式で表される。

```math
P(A) = \frac{n}{m}
```

### 排反

**排反**とは、ある2つの[事象](#確率)について、両方が同時に起こることがないこと。

### 独立

**独立**とは、ある2つの[事象](#確率)にについて、それぞれが無関係に起こったり起こらなかったりすること。

### 余事象

**余事象**とは、ある[事象](#確率)が起こらない[事象](#確率)のこと。ある[事象](#確率) $A$ が起こらない[確率](#確率) $P(\bar{A})$ （余事象）は次のように求められる。

```math
P(\bar{A}) = 1 - P(A)
```

### 加法定理

**加法定理**とは、ある2つの[事象](#確率) $A, B$ のいずれかが起こるという[事象](#確率)（**和事象**）の[確率](#確率)を求めるための定理。

2つの[事象](#確率)が互いに[排反](#排反)である場合、和事象の確率 $P(A \cup B)$ は、それぞれの[事象](#確率)の[確率](#確率)の和で求められる。

```math
P(A \cup B) = P(A) + P(B)
```

2つの[事象](#確率)が互いに[排反](#排反)でない場合、和事象の[確率](#確率) $P(A \cup B)$ は、それぞれの[事象](#確率)の[確率](#確率)の和から重複する部分（両方が同時に起こる[確率](#確率)）の[確率](#確率)を引くことで求められる。

```math
P(A \cup B) = P(A) + P(B) - P(A \cap B)
```

### 乗法定理

**乗法定理**とは、ある2つの[事象](#確率) $A, B$ の両方が同時に起こるという[事象](#確率)（**積事象**）の[確率](#確率)を求めるための定理。

2つの[事象](#確率)が互いに[独立](#独立)している場合、積事象の確率 $P(A \cap B)$ は、それぞれの[事象](#確率)の[確率](#確率)の積で求められる。

```math
P(A \cap B) = P(A) \times P(B)
```

### 条件付き確率

**条件付き確率**とは、ある[事象](#確率) $B$ が起こるという条件の下で別の[事象](#事象) $A$ の起こる[確率](#確率)のことをいい、 $P(A|B)$ のように表す。

### ベイズの定理

**ベイズの定理**（**原因の確率**）とは、2つの[事象](#確率)が互いに[排反](#排反)であり、そのいずれかの[事象](#確率)によって[事象](#確率) $E$ が発生したとき、その[事象](#確率) $E$ が発生した原因が[事象](#確率) $A$ または $B$ である[確率](#確率) を求めるための定理。

[事象](#確率) $E$ の発生する[確率](#確率)を $P(E)$ 、[事象](#確率) $A$ の発生する[確率](#確率)を $P(A)$ 、[事象](#確率) $A$ が原因で[事象](#確率) $E$ が起こる[確率](#確率)を $P(E|A)$ 、[事象](#確率) $B$ の発生する[確率](#確率)を $P(B)$ 、[事象](#確率) $B$ が原因で[事象](#確率) $E$ が起こる[確率](#確率)を $P(E|B)$ とすると、[事象](#確率) $E$ が起こる原因が $A$ である[確率](#確率) $P(A|E)$ は以下の式で表される。

```math
P(A|E) = \frac{P(A) \times P(E|A)}{P(E)} = \frac{P(A) \times P(E|A)}{P(A) \times P(E|A) + P(B) \times P(E|B)}
```


## 基本統計量

### 代表値と散布値

**代表値**とは、データの特性を表すような代表となる値のことで、[平均値](#平均値)や[中央値](#中央値)、[最頻値](#最頻値)などがある。

**散布値**とは、データのばらつきを表すような指標となる値のことで、[範囲](#範囲)や[分散](#分散)、[標準偏差](#標準偏差)などがある。

### 平均値

**平均値**とは、データの合計をデータの数で割ることで得られる値。

### 中央値

**中央値**（**メジアン**）とは、データを昇順もしくは降順に並べたときに真ん中の順位に位置するデータの値。極端な外れ値が存在するデータに対しては、[平均値](#平均値)ではなく中央値を用いた方が適切な場合がある。

### 最頻値

**最頻値**（**モード**）とは、データ群の中で最も頻繁に出現する値。どの値が出やすいかという傾向を見るのに適した指標。

### 範囲

**範囲**とは、データの最大値から最小値を引いたもの。

### 分散

**分散**とは、データのばらつき具合を評価するための指標。データ数を $n$ 、分散を $s^2$ 、[平均値](#平均値)を $\bar{x}$ とすると、以下の式で表される。なお、各データに対する[平均値](#平均値)との差 $x_i - \bar{x}$ を**偏差**という。

```math
s^2 = \frac{1}{n} \sum^{n}_{i=1}{(x_i - \bar{x})^2}
```

全てのデータが等しいとき、分散は0となる。

### 標準偏差

**標準偏差**とは、[分散](#分散)の平方根をとったもの。標準偏差を $s$ 、[分散](#分散)を $s^2$ とすると、以下の式で表される。[分散](#分散)は各データと[平均値](#平均値)との差の二乗をとるため単位が元のデータと異なるが、標準偏差を求めることで単位が元に戻り分析しやすくなる。

```math
s = \sqrt{s^2}
```


## 確率分布

### 確率変数と確率分布

**確率変数**とは、ある[試行](#確率)の結果によって値がランダムに決まり、とりうる各値について[確率](#確率)が定まるような変数。

**確率分布**とは、確率変数の取りうる値と、その値をとる[確率](#確率)との対応関係のこと。

### 期待値

**期待値**とは、[確率変数](#確率変数と確率分布)の全ての値に[確率](#確率)の重みをつけた加重平均。1回の[試行](#確率)で得られる結果の[平均値](#平均値)のこと。

### ベルヌーイ試行

**ベルヌーイ試行**とは、とりうる結果が2種類（成功か失敗かなど）で、繰り返し行った各[試行](#試行)が[独立](#独立)となるような[試行](#試行)のこと。

ベルヌーイ試行では、[確率変数](#確率変数と確率分布) $X$ がとりうる値の一方を1、もう一方を0とする。[試行](#確率)が成功となる[確率](#確率)を $p$ とすると、成功と失敗の[確率](#確率)はそれぞれ次のように表される。

```math
P(X=1) = p
P(X=0) = 1 - p
```

### 二項分布

**二項分布**とは、[ベルヌーイ試行](#ベルヌーイ試行)を繰り返し行ったときに、ある[事象](#確率)が何回起こるかを示した[確率分布](#確率変数と確率分布)。

成功する確率が $p$ のあるベルヌーイ試行を $n$ 回行って成功する回数 $X$ は以下の式で表され、 $X$ が二項分布に従うとき $X \sim B(n, p)$ と表す。

```math
\begin{array}{cc}
P(X = k) = {}_n C_k p^k (1 - p)^{n-k} & (k = 0, 1, 2, \cdots, n)
\end{array}
```

### ポアソン分布

**ポアソン分布**とは、[試行](#確率)回数が十分に大きく、発生[確率](#確率)が低い[ベルヌーイ試行](#ベルヌーイ試行)を繰り返し行ったときに、ある[事象](#確率)が何回起こるかを示した[確率分布](#確率変数と確率分布)。

成功する確率が $p$ のあるベルヌーイ試行を $n$ 回行ったとき、期待発生回数を $\lambda = np$ とおく。このとき、成功する回数 $X$ は以下の式で表され、 $X$ がポアソン分布に従うとき $X \sim Po(\lambda)$ と表す。

```math
\begin{array}{cc}
P(X = k) = \frac{\lambda^k e^{-\lambda}}{k!} & (k = 0, 1, 2, \cdots)
\end{array}
```

### 正規分布

**正規分布**（**ガウス分布**）とは、[平均値](#平均値)・[中央値](#中央値)・[最頻値](#最頻値)が一致し、これらの値を中心に左右対称にデータが分布しているような[確率分布](#確率分布)。自然現象や社会現象の多くは正規分布に従っており、実験や推定なども正規分布を前提として行うことが多い。

[平均値](#平均値)を $\mu$ 、[分散](#分散)を $s^2$ とおく。このとき、[確率変数](#確率変数と確率分布) $X$ は以下の式で表され、 $X$ が正規分布に従うとき $X \sim N(\mu, s^2)$ と表す。

```math
f(x) = \frac{1}{\sqrt{2 \pi s^2}} \exp(-\frac{(x - \mu)^2}{2 s^2})
```


## マルコフ過程

**マルコフ過程**とは、未来の挙動が現在の値だけで決定され、過去の挙動とは無関係であるという性質を持つ[確率](#確率)過程のこと。

### マルコフ連鎖

**マルコフ連鎖**とは、連鎖的な時間軸での[マルコフ過程](#マルコフ過程)のこと。


## 相関関係

**相関関係**とは、2つの変数の関連性の強さを表すための指標。相関関係を数値で表したものを**相関係数**という。

相関係数 $r$ は $-1 \leq r \req +1$ であり、この値が1に近いほど強い正の相関があるといい、-1に近いほど強い負の相関があるという。


## 回帰直線

### 回帰分析

**回帰分析**は、[相関関係](#相関関係)にある2つの変数の関係を数式で表現し、データを分析・予測する統計的手法。2つの変数のうち、値が分かっている変数を**説明変数**、その値を基に予測したい変数を**目的変数**という。説明変数が1つの場合を**単回帰分析**、2つ以上の場合を**重回帰分析**という。回帰直線は説明変数の各値からの距離の二乗和が最小となるような直線とし、これを**最小二乗法**という。

### ロジスティック回帰分析

**ロジスティック回帰分析**は、1つの**カテゴリ変数**（2値の変数）の成功[確率](#確率)を、複数の[説明変数](#回帰分析)によって予測する解析手法。


## 推定

**推定**とは、データ量が多く全てのデータを調査することが困難な場合において、いくつかのデータを取り出してそれを基に全体の[平均](#平均値)・[分散](#分散)などの傾向を算出すること。全体のデータのことを**母集団**、推定のために取り出したデータのことを**標本**という。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
