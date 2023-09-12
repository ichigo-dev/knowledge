# 『アルゴリズム』ノート

（最終更新： 2023-03-03）


## 目次

1. [アルゴリズム](#アルゴリズム)
1. [背理法](#背理法)
1. [探索アルゴリズム](#探索アルゴリズム)
	1. [全探索](#全探索)
	1. [ビット全探索](#ビット全探索)
	1. [線形探索](#線形探索)
	1. [二分探索](#二分探索)


## アルゴリズム

**アルゴリズム**は、問題を解くための手順のことで、特に[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)[プログラム](./programming.md#プログラム)について使われることが多い。適切なアルゴリズムを用いると、処理時間（[時間計算量](../../../basics/information_theory/_/chapters/computational_complexity.md#時間計算量)）が大幅に短縮できたり、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)などの処理に必要となる資源（[空間計算量](../../../basics/information_theory/_/chapters/computational_complexity.md#空間計算量)）を削減できたりする。アルゴリズムを学ぶことで、良い[プログラム](./programming.md#プログラム)を作ることができるようになる。


## 背理法

**背理法**は、真であると証明したい[命題](../../../basics/discrete_mathematics/_/chapters/set_and_proposition.md#命題)を偽であると仮定し、その矛盾を導くことによって、[命題](../../../basics/discrete_mathematics/_/chapters/set_and_proposition.md#命題)が偽であるという仮定が誤り（=[命題](../../../basics/discrete_mathematics/_/chapters/set_and_proposition.md#命題)が真）であることを論理付ける方法。


## 探索アルゴリズム

### 全探索

**全探索**は、あり得るすべてのパターンをしらみつぶしに調べる[アルゴリズム](#アルゴリズム)。最もシンプルな[アルゴリズム](#アルゴリズム)のひとつで、問題を考える際にはまず全探索をしても現実的な時間で実行が終わるのかどうかを検討することが大切。

### ビット全探索

**ビット全探索**は、[ビット演算](./operation.md#ビット演算)を利用することで[全探索](#全探索)を行う方法。ビット全探索では、[部分集合](../../../basics/discrete_mathematics/_/chapters/set_and_proposition.md#部分集合)を全パターン列挙することができるため、組み合わせを[全探索](#全探索)する際のテクニックとして使える。

### 線形探索

**線形探索**は、[配列](./data_type.md#配列)から検索したい値を見つけ出すための[アルゴリズム](#アルゴリズム)のひとつで、全ての要素を順番に検索したい値と比較する。

### 二分探索

**二分探索**は、昇順あるいは降順に並べられた[配列](./data_type.md#配列)から検索したい値を見つけ出すための[アルゴリズム](#アルゴリズム)のひとつで、中央の要素と検索したい値との大小比較を行い、検索したい値が中央の要素よりも前にあるか後ろにあるかを判断しながら絞り込んでいく。ソートされていない[配列](./data_type.md#配列)や、大小関係の定義されていない要素には使用できない。