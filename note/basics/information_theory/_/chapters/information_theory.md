# 『情報理論』ノート

（最終更新： 2023-03-03）


## 目次

1. [情報量](#情報量)
	1. [生起確率](#生起確率)
	1. [自己エントロピー](#自己エントロピー)
	1. [エントロピー](#エントロピー)


## 情報量

**情報量**は、ある[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#事象)がどれほど起こりにくいかを表す尺度。起こりやすい[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#事象)であるほど持っている情報は少ないと考えられ、一方で起こりにくい[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#事象)は多くの情報を持っていると考えられる。

### 生起確率

**生起確率**は、ある[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#事象)が発生する[確率](../../../applied_mathematics/_/chapters/probability_and_statistics.md#確率)。

### 自己エントロピー

**自己エントロピー**（**選択情報量**）は、ある[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#事象)が起こるときの[情報量](#情報量)。[対数](../../../applied_mathematics/_/chapters/numerical_calculation.md#対数)の底を $2$ とする（[2進数](../../../discrete_mathematics/_/chapters/radix.md#2進数)で情報を表現することを考えた場合）と、自己エントロピーとはその情報を何[ビット](../../../_/chapters/computer_and_number.md#ビット)で表現できるのかを表す。

ある[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#事象) $E$ の[生起確率](#生起確率)を $P(E)$ とすると、自己エントロピー $I(E)$ は以下の式で求められる。

```math
I(E) = \log_2{\frac{1}{P(E)}} = - \log_2{P(E)}
```

### エントロピー

**エントロピー**（**平均情報量**）は、系全体で考えた場合の[情報量](#情報量)の平均。平均情報量は、その系の情報を[圧縮](./coding_theory.md#圧縮)するときの限界となる。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)
