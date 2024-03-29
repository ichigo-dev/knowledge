# 『IC』ノート

（最終更新： 2023-04-03）


## 目次

1. [IC](#ic)
	1. [カスタムIC](#カスタムic)
	1. [システムLSI](#システムlsi)
	1. [MEMS](#mems)
	1. [ESD破壊](#esd破壊)
	1. [ラッチアップ](#ラッチアップ)
	1. [ストレスマイグレーション](#ストレスマイグレーション)
	1. [エレクトロマイグレーション](#エレクトロマイグレーション)

## IC

**IC**（**集積回路**: Integrated Circuit）は、半導体素子（半導体による電子部品）のひとつで、基板上に様々な部品をまとめた小型の回路。

### LSI

**LSI**（**大規模集積回路**: Large Scale Integration）は、[IC](#ic)よりも集積度を高めた素子。

### VLSI

**VLSI**(Very Large Scale Integration)は、[LSI](#lsi)よりもさらに集積度を高めた素子。

### カスタムIC

**カスタムIC**は、利用者が要求する特定の用途に特化した[IC](#ic)。製造するときに回路設計を決定する**ASIC**(Application Specific Integrated Circuit)や、製造後に回路を変更できる**FPGA**(Field-Programmable Gate Array)がある。FPGAの構成を記述するハードウェア言語として**HDL**(Hardware Description Language)を用いる。

### システムLSI

**システムLSI**は、組込みシステム製品の電子回路を1チップに集約した半導体製品で、その設計手法は**SoC**(System On a Chip)と呼ばれる。

### MEMS

**MEMS**(Micro Electro Mechanical Systems)は、[センサ](../../../../basics/measurement_and_control/_/chapters/control_theory.md#センサ)や[アクチュアエータ](../../../../basics/measurement_and_control/_/chapters/control_theory.md#アクチュエータ)などの電子回路をひとつのシリコン基板に集積化したデバイス。

### ESD破壊

**ESD破壊**は、半導体特有の故障のひとつで、静電気放電(ESD: ElectroStatic Discharge)によってデバイスが劣化・故障すること。人体や装置、デバイスが帯電し、酸化膜や配線などが破壊される。

### ラッチアップ

**ラッチアップ**は、半導体特有の故障のひとつで、期待しない位置に[トランジスタ](../../../_/chapters/computer.md#第2世代)やサイリスタができてしまうことで回路に不具合を生じること。これを寄生トランジスタや寄生サイリスタなどと呼ぶ。

### ストレスマイグレーション

**ストレスマイグレーション**は、機械的な力によって配線が切断されるなどによって、半導体素子が不良になる現象。

### エレクトロマイグレーション

**エレクトロマイグレーション**は、電流が過度に流れることによって配線が切断されるなどによって、半導体素子が不良になる現象。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)
