# 『アルゴリズム』

（最終更新： 2023-02-04）


## 目次

1. [アルゴリズム](#アルゴリズム)
1. [背理法](#背理法)
1. [探索アルゴリズム](#探索アルゴリズム)
	1. [全探索](#全探索)
	1. [ビット全探索](#ビット全探索)
	1. [線形探索](#線形探索)
	1. [二分探索](#二分探索)


## アルゴリズム

**アルゴリズム**は、問題を解くための手順のことで、特にコンピュータプログラムについて使われることが多い。適切なアルゴリズムを用いると、処理時間（時間計算量）が大幅に短縮できたり、メモリなどの処理に必要となる資源（空間計算量）を削減できたりする。アルゴリズムを学ぶことで、良いプログラムを作ることができるようになる。


## 背理法

**背理法**は、真であると証明したい命題を偽であると仮定し、その矛盾を導くことによって、命題が偽であるという仮定が誤り（=命題が真）であることを論理付ける方法。


## 探索アルゴリズム

### 全探索

**全探索**は、あり得るすべてのパターンをしらみつぶしに調べるアルゴリズム。最もシンプルなアルゴリズムのひとつで、問題を考える際にはまず全探索をしても現実的な時間で実行が終わるのかどうかを検討することが大切。

### ビット全探索

**ビット全探索**は、ビット演算を利用することで全探索を行う方法。ビット全探索では、部分集合を全パターン列挙することができるため、組み合わせを全探索する際のテクニックとして使える。

### 線形探索

**線形探索**は、配列から検索したい値を見つけ出すためのアルゴリズムのひとつで、全ての要素を順番に検索したい値と比較する。

### 二分探索

**二分探索**は、昇順あるいは降順に並べられた配列から検索したい値を見つけ出すためのアルゴリズムのひとつで、中央の要素と検索したい値との大小比較を行い、検索したい値が中央の要素よりも前にあるか後ろにあるかを判断しながら絞り込んでいく。ソートされていない配列や、大小関係の定義されていない要素には使用できない。