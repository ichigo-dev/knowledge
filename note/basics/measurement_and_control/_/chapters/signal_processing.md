# 『信号処理』ノート

（最終更新： 2023-03-03）


## 目次

1. [A/D変換](#ad変換)
	1. [標本化](#標本化)
	1. [量子化](#量子化)
	1. [符号化](#符号化)
1. [D/A変換](#da変換)
1. [サンプリング定理](#サンプリング定理)
1. [PCM](#pcm)


## A/D変換

**A/D変換**は、[アナログ](../../../information_theory/_/chapters/coding_theory.md#アナログ)情報を[デジタル](../../../information_theory/_/chapters/coding_theory.md#デジタル)データに変換する処理。

### 標本化

**標本化**は、連続する[アナログ](../../../information_theory/_/chapters/coding_theory.md#アナログ)情報を一定の間隔でサンプリングする処理。

### 量子化

**量子化**は、[標本化](#標本化)したデータを[デジタル](../../../information_theory/_/chapters/coding_theory.md#デジタル)値に変換する処理。

### 符号化

**符号化**は、[量子化](#量子化)したデータを[2進数](../../../discrete_mathematics/_/chapters/radix.md#2進数)に変換する処理。


## D/A変換

**D/A変換**は、[デジタル](../../../information_theory/_/chapters/coding_theory.md#デジタル)データを[アナログ](../../../information_theory/_/chapters/coding_theory.md#アナログ)情報に変換する処理。


## サンプリング定理

**サンプリング定理**（**標本化定理**）は、ある周波数の[アナログ](../../../information_theory/_/chapters/coding_theory.md#アナログ)信号を[デジタル](../../../information_theory/_/chapters/coding_theory.md#デジタル)データに変換するときに、それを[アナログ](../../../information_theory/_/chapters/coding_theory.md#アナログ)信号に復元するためには、その周波数の2倍のサンプリング周波数が必要であるという定理。


## PCM

**PCM**(Pulse Code Modulation)は、音を[標本化](#標本化)し、[量子化](#量子化)、[符号化](#符号化)したデータを格納するために用いられている方式。

単純なPCMでは、[標本化](#標本化)ごとのデータの変化が小さいことが多いので、データの差分を用いて動的にデータを作成することで[圧縮](../../../information_theory/_/chapters/coding_theory.md#圧縮)を行う**ADPCM**(Adaptive Differential PCM)という方式もある。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)
