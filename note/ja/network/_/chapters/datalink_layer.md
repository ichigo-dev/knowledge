# 『データリンク層』ノート

（最終更新： 2023-06-27）


## 目次

1. [データリンク](#データリンク)
	1. [フレーム](#フレーム)
	1. [セグメント](#セグメント)
	1. [MTU](#mtu)
1. [イーサネットのフレームフォーマット](#イーサネットのフレームフォーマット)
1. [VLAN](#vlan)
1. [PPP](#ppp)
	1. [PPPoE](#pppoe)
1. [ATM](#atm)
1. [POS](#pos)
1. [VPN](#vpn)
	1. [トンネリング](#トンネリング)
	1. [カプセル化](#カプセル化)
	1. [広域イーサネット](#広域イーサネット)
1. [その他のチェックポイント](#その他のチェックポイント)


## データリンク

**データリンク**では、[物理層](./network_architecture.md#物理層)の通信媒体の電圧の高低や光の点滅を、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)で処理できるように `0` と `1` の羅列に変換し、それを[フレーム](#フレーム)という意味のあるかたまりとして相手に届ける。

データリンクは、区間内の[ノード](./network.md#ノード)を結ぶ[プロトコル](./network_architecture.md#プロトコル)を持つ。

### フレーム

**フレーム**は、[データリンク層](./communication_protocol.md#データリンク層)におけるデータのまとまり。

### セグメント

**セグメント**（**サブネットワーク**）は、[LAN](./network.md#lan)が構成する物理的に同じ信号が届く範囲の[ネットワーク](./network.md#ネットワーク)。

### MTU

**MTU**（Maximum Transmission Unit: 最大転送単位）は、[データリンク](#データリンク)において一度に送信できる[パケット](./network.md#パケット)の大きさのことで、機器によって異なる。


### イーサネットのフレームフォーマット

[イーサネット](./network.md#イーサネット)の[フレーム](#フレーム)の先頭には、[フレーム](#フレーム)本体の開始位置を表すために、 `1` と `0` を交互に並べた**プリアンブル**と呼ばれるフィールドが付けられる。IEEE仕様の[フレーム](#フレーム)では、プリアンブルの末尾は**SFD**(Start Frame Delimiter)という `11` のフィールドで終わり、それ以降が[イーサネット](./network.md#イーサネット)[フレーム](#フレーム)本体として扱われる。

最後の**FCS**(Frame Check Sequence)は、[フレーム](#フレーム)が壊れていないかを確かめるためのフィールド。


## VLAN

**VLAN**（Virtual LAN: 仮想LAN）は、1つの[スイッチ](./network_architecture.md#スイッチ)に接続されている[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)を、論理的に複数の[ネットワーク](./network.md#ネットワーク)に分ける仕組み。部署ごとに接続するVLANを分けたり、セキュリティ対策レベルを分けたVLANを隔離して用いるといった使い方ができる。


## PPP

**PPP**(Point-to-Point Protocol)は、一対一で[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)を接続するための[プロトコル](./network_architecture.md#プロトコル)。従来は電話回線や専用回線などを利用していたが、近年は[インターネット](./network.md#インターネット)接続を利用した[PPPoE](#pppoe)としての利用が増えてきた。

### PPPoE

**PPPoE**(PPP over Ethernet)は、[イーサネット](./network.md#イーサネット)を経由して[PPP](#ppp)接続を行う方法。


## ATM

**ATM**(Asynchronous Transfer Mode)は、[コネクション型](./network.md#コネクション型)の[データリンク](#データリンク)で、「ヘッダ5オクテット + データ48オクテット」の**セル**と呼ばれる単位でデータを処理する。ATMは**AAL**(ATM Adaption Layer)とともに利用される。

ATMはセルが1つでも失われると最大192個のセルを再送しなければいけなくなるという問題がある。


## POS

**POS**(Packet over SDH/SONET)は、[デジタル](../../../basics/information_theory/_/chapters/coding_theory.md#デジタル)信号を光ファイバーでやり取りするための規格。**SDH**(Synchronous Digital Hierarchy)は電話回線や専用線などで信頼性の高い光伝送[ネットワーク](./network.md#ネットワーク)として広く利用されている。


## VPN

**VPN**(Virtual Private Network)は、[インターネット](./network.md#インターネット)のような公開された回線上で、安全に[パケット](./network.md#パケット)をやり取りするための専用回線を擬似的に設ける技術。

### トンネリング

**トンネリング**は、不特定多数が使うような回線上で、データを互いにやり取りするための隔絶された[コネクション](./network.md#コネクション)を設ける技術。

### カプセル化

**カプセル化**は、[VPN](#vpn)上でやり取りされるデータを暗号化し、外部から隠蔽するための技術。

### 広域イーサネット

**広域イーサネット**は、[通信事業者](./network.md#キャリア)が独自に用意した閉域網を利用する[VPN](#vpn)接続サービス。


## その他のチェックポイント

- [WiMAX](https://ja.wikipedia.org/wiki/WiMAX)
- [Bluetooth](https://ja.wikipedia.org/wiki/Bluetooth)
- [ZigBee](https://ja.wikipedia.org/wiki/ZigBee)
- [LPWA(Low Power, Wide Area)](https://ja.wikipedia.org/wiki/LPWA_(%E7%84%A1%E7%B7%9A))
- [LCP(Link Control Protocol)](https://ja.wikipedia.org/wiki/Link_Control_Protocol)
- [NCP(Network Control Protocol)](https://ja.wikipedia.org/wiki/Network_Control_Protocol)
- [PAP(Password Authentication Protocol)](https://ja.wikipedia.org/wiki/Password_Authentication_Protocol)
- [CHAP(Challenge Handshake Authentication Protocol)](https://ja.wikipedia.org/wiki/Challenge-Handshake_Authentication_Protocol)
- [ファイバーチャネル](https://ja.wikipedia.org/wiki/%E3%83%95%E3%82%A1%E3%82%A4%E3%83%90%E3%83%BC%E3%83%81%E3%83%A3%E3%83%8D%E3%83%AB)
- [iSCSI](https://ja.wikipedia.org/wiki/ISCSI)
- [InfiniBand](https://ja.wikipedia.org/wiki/InfiniBand)
- [HDMI](https://ja.wikipedia.org/wiki/HDMI)
- [DOCSIS](https://ja.wikipedia.org/wiki/Data_Over_Cable_Service_Interface_Specifications)
- [高速PLC](https://ja.wikipedia.org/wiki/%E9%9B%BB%E5%8A%9B%E7%B7%9A%E6%90%AC%E9%80%81%E9%80%9A%E4%BF%A1)
- [アナログ電話回線](https://ja.wikipedia.org/wiki/%E9%9B%BB%E8%A9%B1%E5%9B%9E%E7%B7%9A)
- [モバイル通信サービス](https://ja.wikipedia.org/wiki/%E7%A7%BB%E5%8B%95%E4%BD%93%E9%80%9A%E4%BF%A1)
- [ADSL](https://ja.wikipedia.org/wiki/ADSL)
- [FTTH](https://ja.wikipedia.org/wiki/FTTH)
    - [ダークファイバー](https://ja.wikipedia.org/wiki/%E3%83%80%E3%83%BC%E3%82%AF%E3%83%95%E3%82%A1%E3%82%A4%E3%83%90)
- [ケーブルテレビ](https://ja.wikipedia.org/wiki/%E3%82%B1%E3%83%BC%E3%83%96%E3%83%AB%E3%83%86%E3%83%AC%E3%83%93)
- [専用回線](https://ja.wikipedia.org/wiki/%E5%B0%82%E7%94%A8%E7%B7%9A)
- [公衆無線LAN](https://ja.wikipedia.org/wiki/%E5%85%AC%E8%A1%86%E7%84%A1%E7%B7%9ALAN)
    - [ホットスポット](https://ja.wikipedia.org/wiki/%E3%83%9B%E3%83%83%E3%83%88%E3%82%B9%E3%83%9D%E3%83%83%E3%83%88_(NTT))
