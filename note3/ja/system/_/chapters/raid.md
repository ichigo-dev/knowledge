# 『RAID』

（最終更新： 2023-01-21）


## 目次

1. [RAID](#raid)
	1. [RAID0](#raid0)
	1. [RAID1](#raid1)
	1. [RAID0+1、RAID1+0](#raid0+1、raid1+0)
	1. [RAID3](#raid3)
	1. [RAID4](#raid4)
	1. [RAID5](#raid5)
	1. [RAID6](#raid6)
1. [ストレージの信頼性向上](#ストレージの信頼性向上)
	1. [DAS](#das)
	1. [SAN](#san)
	1. [NAS](#nas)


## RAID

**RAID**(Redundant Arrays of Inexpensive Disks)は、複数台のハードディスクを接続して全体でひとつの記憶装置として扱う仕組み。複数台のディスクを組み合わせることによって、信頼性や性能が向上する。

### RAID0

**RAID0**は、複数のハードディスクにデータを分散することで高速化したもの。これを**ストライピング**といい、性能は上がるものの信頼性は1台のディスクに比べて低下する。

### RAID1

**RAID1**は、複数のハードディスクに同時に同じデータを書き込むことで信頼性を向上させたもの。これを**ミラーリング**といい、2台のディスクはお互いに完全なバックアップとなっているため、性能は特に上がらない。

### RAID0+1、RAID1+0

**RAID0+1**や**RAID1+0**は、4台のディスクを用いてRAID0とRAID1を組み合わせることで性能と信頼性の両方を向上させる技術。RAID0+1はストライピングされたディスクをミラーリングし、RAID1+0はミラーリングされたディスクをストライピングする。

### RAID3

**RAID3**は、複数のディスクのうち1台を誤り訂正用のパリティディスクにし、誤りが発生した場合に復元する。パリティディスクにほかのディスクの偶数パリティを計算したものを格納しておく。データの復元はビットごとに行う。

### RAID4

**RAID4**は、複数のディスクのうち1台を誤り訂正用のパリティディスクにし、誤りが発生した場合に復元する。パリティディスクにほかのディスクの偶数パリティを計算したものを格納しておく。データの復元はブロックごとにまとめて行う。

### RAID5

**RAID5**は、データへのアクセス効率を上げるためにパリティをブロックごとに分散し、通常時にもすべてのディスクを用いるようにした方式。RAID3やRAID4と比べて信頼性は同等だが、性能面では優れている。ひとつのディスクが故障しても、他のディスクの排他的論理和を計算することで復元できる。

### RAID6

**RAID6**は、冗長データを2種類作成することで、2台のディスクが故障しても支障をきたさないようにした方式。RAID3、RAID4、RAID5はパリティ用のディスクも含めて最低でもディスクが3台必要となるが、RAID6では4台必要となる。



## ストレージの信頼性向上

### DAS

**DAS**(Direct Attached Storage)は、サーバにストレージを直接接続する従来の方式のこと。SANやNASと区別するためにDASと呼ばれる。

### SAN

**SAN**(Storage Area Network)は、サーバとストレージを接続するために専用のネットワークを使用する方法。ファイバチャネルやIPネットワークを使って、あたかも内蔵したストレージのように使用することができる。**ファイバチャネル**とは、主にストレージネットワーク用に使用される高速ネットワークを構築する技術のひとつ。

### NAS

**NAS**(Network Attached Storage)は、ファイルを格納するサーバをネットワークに直接接続することで、外部からファイルを利用できるようにする方法。複数のサーバやクライアントがストレージを共有できるため、資源を効率的に活用することができる。また、物理的なストレージの数が減らせることから、バックアップも取りやすくなる。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)