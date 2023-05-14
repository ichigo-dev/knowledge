# 『AWSの基礎知識』ノート

（最終更新： 2023-05-05）


## 目次

1. [AWS](#aws)
	1. [リージョン](#リージョン)
	1. [AZ](#az)
	1. [マルチAZ](#マルチaz)


## AWS

**AWS**(Amazon Web Service)は、Amazonが提供する[クラウドコンピューティング](../../../../system/_/chapters/system_architecture.md#クラウドコンピューティング)サービスで、[インターネット](../../../_/chapters/network.md#インターネット)を介して、[サーバ](../../../../computer/_/chapters/computer.md#サーバ)やストレージ、[データベース](../../../../development/database/_/chapters/database.md#データベース)、[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)などを利用することができる。従来の[オンプレミス](../../../../system/_/chapters/system_architecture.md#オンプレミス)よりも低いコストでサービスを運用できたり、障害への対策や負荷分散が手軽に行えるといった利点がある。

[クラウドコンピューティング](../../../../system/_/chapters/system_architecture.md#クラウドコンピューティング)サービスの役割は、[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)や通信機器を設置することに特化した[データセンタ](../../../_/chapters/network.md#データセンタ)の[ハードウェア](../../../../computer/hardware/_/chapters/hardware.md#ハードウェア)資源を、[インターネット](../../../_/chapters/network.md#インターネット)を介してユーザに提供することである。AWSは単一の[データセンタ](../../../_/chapters/network.md#データセンタ)ではなく、複数の地域にまたがった[データセンタ](../../../_/chapters/network.md#データセンタ)から構成されている。

### リージョン

**リージョン**は、[AWS](#aws)の[データセンタ](../../../_/chapters/network.md#データセンタ)が設置されている地域。[AWS](#aws)は世界各国に多数のリージョンを保有している。各地域のリージョンは相互に高速回線で接続されている。

### AZ

**AZ**（**アベイラビリティゾーン**、**ゾーン**）は、1つの[リージョン](#リージョン)内で物理的に切り離された、冗長的な設備を備えた[データセンタ](../../../_/chapters/network.md#データセンタ)。1つの[リージョン](#リージョン)内には必ず複数のAZが用意されており、自然災害などでひとつのAZが機能停止に陥ったとしても、他のAZを利用しているサービスは引き続き運用することができる。

### マルチAZ

**マルチAZ**は、[システム](../../../../system/_/chapters/system.md#システム)のインフラを構築する際に、同じ役割を持った[サーバ](../../../../computer/_/chapters/computer.md#サーバ)を複数の[AZ](#az)に用意することで[冗長化](../../../../system/_/chapters/system_architecture.md#冗長化)する手法。マルチAZ構成を意識することで、高い[耐障害性](../../../../system/_/chapters/reliability_design.md#フォールトトレランス)や[可用性](../../../../system/_/chapters/system_performance_evaluation.md#可用性)を実現できる。
