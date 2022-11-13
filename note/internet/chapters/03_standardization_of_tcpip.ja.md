# TCP/IPの標準化


## 目次

1. [プロトコルの標準化](#プロトコルの標準化)
	1. [ISOとIETF](#isoとietf)
	1. [インターネットプロトコルスイート](#インターネットプロトコルスイート)
	1. [RFC](#rfc)
	1. [標準化の流れ](#標準化の流れ)


## プロトコルの標準化

### ISOとIETF

**ISO**（International Organization for Standardization、国際標準化機構）は、国際標準として**OSI**（Open System Interconnection）と呼ばれる通信体系を標準化した。

一方で、[インターネット](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#インターネット)で使われている[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)である**TCP/IP**（Transmission Control Protocol / Internet Protocol）は、**IETF**（Internet Engineering Task Force）で提案・標準化作業が行われているプロトコルである。

TCP/IPは「実装すること」を念頭に作業が進められ、その方針から素早く[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)を完成させることができた。そのため、ISOのOSIよりも先に動作する[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)を作り上げることができ、世の中に普及していくこととなった。

### インターネットプロトコルスイート

TCP/IPは、[TCP](/note/internet/chapters/08_transport_layer.ja.md#tcp)と[IP](/note/internet/chapters/07_internet_layer.ja.md#ip)という2つの[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)だけを指すわけではなく、これに付随した[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)群の総称として使われる言葉である。この[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)一式を、**インターネットプロトコルスイート**と呼ぶ。

### RFC

TCP/IPの[プトロコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)は、[IETF](#isoとietf)で議論されて標準化される。標準化しようとする[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)は、**RFC**（Request For Comments）と呼ばれるドキュメントとして管理されている。一度RFCになったものは、その内容を改定することはできない。そのため、同じ[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)でも内容を更新しようとすると、新しいRFCを発行する必要がある。

RFCは[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)の仕様が変更されるたびに番号が変わるのが不便であるという意見もあったため、主要な[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)や標準に対しては、**STD**（Standard）という変化しない番号を割り振った。

また、[インターネット](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#インターネット)のユーザや管理者に向けて有益な情報を提供するために、**FYI**（For Your Information）という番号付けも行われている。

[RFCのホームページ](https://www.rfc-editor.org/)

### 標準化の流れ

TCP/IPの標準化作業は、まず、仕様を煮詰める**インターネットドラフト**（I-D: Internet-Draft）から始まる。そして、標準化したほうが良いと認められると[RFC](#rfc)になり、**提案標準**（Proposed Standard）になる。次に標準の草案である**ドラフト標準**（Draft Standard）になり、最後に**標準**（Standard）となる。
