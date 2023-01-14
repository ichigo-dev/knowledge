# 『情報理論』

（最終更新： 2023-01-14）


## 目次

1. [情報量](#情報量)
	1. [生起確率](#生起確率)
	1. [情報量の算出](#情報量の算出)


## 情報量

**情報量**（**エントロピー**）とは、ある[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#確率)がどれほど起こりにくいかを表す尺度。起こりやすい[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#確率)であるほど持っている情報は少ないと考えられ、一方で起こりにくい[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#確率)は多くの情報を持っていると考えられる。

### 生起確率

**生起確率**とは、ある[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#確率)が発生する[確率](../../../applied_mathematics/_/chapters/probability_and_statistics.md#確率)のこと。

### 情報量の算出

ある[事象](../../../applied_mathematics/_/chapters/probability_and_statistics.md#確率) $E$ の[生起確率](#生起確率)を $P(E)$ とすると、[情報量](#情報量) $I(E)$ は以下の式で求められる。

```math
I(E) = \log{\fraq{1}{P(E)}} = - \log{P(E)}
```


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
