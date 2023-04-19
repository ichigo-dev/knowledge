# 『RAID』ノート

（最終更新： 2023-04-19）


## 目次

1. [RAID](#raid)
	1. [RAID0](#raid0)
	1. [RAID1](#raid1)
	1. [RAID0+1、RAID1+0](#raid01raid10)
	1. [RAID3](#raid3)
	1. [RAID4](#raid4)
	1. [RAID5](#raid5)
	1. [RAID6](#raid6)
1. [ストレージの信頼性向上](#ストレージの信頼性向上)
	1. [DAS](#das)
	1. [SAN](#san)
	1. [NAS](#nas)


## RAID

**RAID**(Redundant Arrays of Inexpensive Disks)は、複数台の[ハードディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)を接続して全体でひとつの[記憶装置](../../../computer/hardware/_/chapters/hardware.md#記憶装置)として扱う[冗長化](./system_architecture.md#冗長化)の仕組み。複数台の[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)を組み合わせることによって、[信頼性](./system_performance_evaluation.md#信頼性)や性能が向上する。

### RAID0

**RAID0**は、複数の[ハードディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)にデータを分散することで高速化する[RAID](#raid)の方式。これを**ストライピング**といい、性能は上がるものの[信頼性](./system_performance_evaluation.md#信頼性)は1台の[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)に比べて低下する。

### RAID1

**RAID1**は、複数の[ハードディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)に同時に同じデータを書き込むことで[信頼性](./system_performance_evaluation.md#信頼性)を向上させた[RAID](#raid)の方式。これを**ミラーリング**といい、2台の[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)はお互いに完全なバックアップとなっているため、性能は特に上がらない。

### RAID0+1、RAID1+0

**RAID0+1**や**RAID1+0**は、4台の[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)を用いて[RAID0](#raid0)と[RAID1](#raid1)を組み合わせることで性能と[信頼性](./system_performance_evaluation.md#信頼性)の両方を向上させる[RAID](#raid)の方式。RAID0+1は[ストライピング](#raid0)された[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)を[ミラーリング](#raid1)し、RAID1+0は[ミラーリング](#raid1)された[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)を[ストライピング](#raid0)する。

### RAID3

**RAID3**は、複数の[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)のうち1台を[誤り訂正](../../../basics/communication_theory/_/chapters/transmission_theory.md#誤り訂正)用の[パリティ](../../../basics/communication_theory/_/chapters/transmission_theory.md#パリティ)[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)にし、誤りが発生した場合に復元する[RAID](#raid)の方式。[パリティ](../../../basics/communication_theory/_/chapters/transmission_theory.md#パリティ)[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)にほかの[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)の偶数[パリティ](../../../basics/communication_theory/_/chapters/transmission_theory.md#パリティ)を計算したものを格納しておく。データの復元は[ビット](../../../basics/_/chapters/computer_and_number.md#ビット)ごとに行う。

### RAID4

**RAID4**は、複数の[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)のうち1台を[誤り訂正](../../../basics/communication_theory/_/chapters/transmission_theory.md#誤り訂正)用の[パリティ](../../../basics/communication_theory/_/chapters/transmission_theory.md#パリティ)[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)にし、誤りが発生した場合に復元する[RAID](#raid)の方式。[パリティ](../../../basics/communication_theory/_/chapters/transmission_theory.md#パリティ)[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)にほかの[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)の偶数[パリティ](../../../basics/communication_theory/_/chapters/transmission_theory.md#パリティ)を計算したものを格納しておく。データの復元はブロックごとにまとめて行う。

### RAID5

**RAID5**は、データへのアクセス効率を上げるために[パリティ](../../../basics/communication_theory/_/chapters/transmission_theory.md#パリティ)をブロックごとに分散し、通常時にもすべての[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)を用いるようにした[RAID](#raid)の方式。[RAID3](#raid3)や[RAID4](#raid4)と比べて[信頼性](./system_performance_evaluation.md#信頼性)は同等だが、性能面では優れている。ひとつの[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)が故障しても、他の[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)の[排他的論理和](../../../basics/discrete_mathematics/_/chapters/logical_operation.md#xor演算)を計算することで復元できる。

### RAID6

**RAID6**は、冗長データを2種類作成することで、2台の[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)が故障しても支障をきたさないようにした[RAID](#raid)の方式。[RAID3](#raid3)、[RAID4](#raid4)、[RAID5](#raid5)は[パリティ](../../../basics/communication_theory/_/chapters/transmission_theory.md#パリティ)用の[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)も含めて最低でも[ディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)が3台必要となるが、RAID6では4台必要となる。



## ストレージの信頼性向上

### DAS

**DAS**(Direct Attached Storage)は、[サーバ](./system_processing_model.md#クライアントサーバシステム)にストレージを直接接続する従来の方式。[SAN](#san)や[NAS](#nas)と区別するためにDASと呼ばれる。

### SAN

**SAN**(Storage Area Network)は、[サーバ](./system_processing_model.md#クライアントサーバシステム)とストレージを接続するために専用の[ネットワーク](../../../network/_/chapters/network.md#ネットワーク)を使用する方法。ファイバチャネルや[IP](../../../network/_/chapters/internet_layer.md#ip)[ネットワーク](../../../network/_/chapters/network.md#ネットワーク)を使って、あたかも内蔵したストレージのように使用することができる。**ファイバチャネル**とは、主にストレージ[ネットワーク](../../../network/_/chapters/network.md#ネットワーク)用に使用される高速[ネットワーク](../../../network/_/chapters/network.md#ネットワーク)を構築する技術のひとつ。

### NAS

**NAS**(Network Attached Storage)は、[ファイル](../../../computer/software/_/chapters/file_system.md#ファイル)を格納する[サーバ](./system_processing_model.md#クライアントサーバシステム)を[ネットワーク](../../../network/_/chapters/network.md#ネットワーク)に直接接続することで、外部から[ファイル](../../../computer/software/_/chapters/file_system.md#ファイル)を利用できるようにする方法。複数の[サーバ](./system_processing_model.md#クライアントサーバシステム)や[クライアント](./system_processing_model.md#クライアントサーバシステム)がストレージを共有できるため、資源を効率的に活用することができる。また、物理的なストレージの数が減らせることから、バックアップも取りやすくなる。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)
