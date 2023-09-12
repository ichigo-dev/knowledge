# 『待ち行列理論』ノート

（最終更新： 2023-03-03）


## 目次

1. [待ち行列](#待ち行列)
	1. [ケンドール記法](#ケンドール記法)
	1. [M/M/1モデル](#mm1モデル)


## 待ち行列

**待ち行列**は、ユーザがサービスを受ける際の混雑状況を数理的に表現したもの。待つことなくサービスを受けられる[[確率](./probability_and_statistics.md#確率)や、サービスの平均待ち時間などを考えるために用いられる。

### ケンドール記法

**ケンドール記法**は、[待ち行列](#待ち行列)を考える上で必要となる要素をモデル化して表現したもの。ケンドール記法には以下の情報が含まれている。

- **到着分布** : [待ち行列](#待ち行列)に並びに来る人の到着間隔
- **サービス時間分布** : サービスを行う時間
- **窓口数** : サービスを行う窓口の数

[M/M/1モデル](#mm1モデル)やM/M/mモデル、M/D/1モデルなどがある。

### M/M/1モデル

**M/M/1モデル**は、[待ち行列](#待ち行列)における最もシンプルなモデルで、[到着分布](#ケンドール記法)と[サービス時間分布](#ケンドール記法)はランダム（[ポアソン分布](./probability_and_statistics.md#ポアソン分布)）で、[窓口数](#ケンドール記法)は1つのみを仮定している。このモデルにおける平均待ち時間は次の式で求められる。

```math
\begin{array}
\rm{窓口利用率}   & = & \frac{\rm{仕事をしている時間}}{\rm{全体の時間}} = \frac{\rm{平均サービス時間}}{\rm{平均到着間隔}} \
\rm{平均待ち時間} & = & \frac{\rm{窓口利用率}}{1 - \rm{窓口利用率}} \times \rm{平均サービス時間}
\end{array}
```


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)