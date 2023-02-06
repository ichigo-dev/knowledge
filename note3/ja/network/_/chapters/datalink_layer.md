# 『データリンク層』

（最終更新： 2023-02-06）


## 目次

1. [データリンク](#データリンク)
	1. [フレーム](#フレーム)
	1. [セグメント](#セグメント)
	1. [MTU](#mtu)
1. [イーサネットのフレームフォーマット](#イーサネットのフレームフォーマット)
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

**データリンク**では、物理層の通信媒体の電圧の高低や光の点滅を、コンピュータで処理できるように0と1の羅列に変換し、それをフレームという意味のあるかたまりとして相手に届ける。

データリンクは、区間内のノードを結ぶプロトコルを持つ。

### フレーム

**フレーム**は、データリンク層におけるデータのまとまり。

### セグメント

**セグメント**は、LANが構成する物理的に同じ信号が届く範囲のネットワーク。

### MTU

**MTU**（Maximum Transmission Unit: 最大転送単位）は、データリンクにおいて一度に送信できるパケットの大きさのことで、機器によって異なる。


### イーサネットのフレームフォーマット

イーサネットのフレームの先頭には、フレーム本体の開始位置を表すために、1と0を交互に並べた**プリアンブル**と呼ばれるフィールドが付けられる。IEEE仕様のフレームでは、プリアンブルの末尾は**SFD**(Start Frame Delimiter)という「11」のフィールドで終わり、それ以降がイーサネットフレーム本体として扱われる。

最後の**FCS**(Frame Check Sequence)は、フレームが壊れていないかを確かめるためのフィールド。


## PPP

**PPP**(Point-to-Point Protocol)は、一対一でコンピュータを接続するためのプロトコル。従来は電話回線や専用回線などを利用していたが、近年はインターネット接続を利用したPPPoEとしての利用が増えてきた。

### PPPoE

**PPPoE**(PPP over Ethernet)は、イーサネットを経由してPPP接続を行う方法。


## ATM

**ATM**(Asynchronous Transfer Mode)は、コネクション型のデータリンクで、「ヘッダ5オクテット + データ48オクテット」の**セル**と呼ばれる単位でデータを処理する。ATMは**AAL**(ATM Adaption Layer)とともに利用される。

ATMはセルが1つでも失われると最大192個のセルを再送しなければいけなくなるという問題がある。


## POS

**POS**(Packet over SDH/SONET)は、デジタル信号を光ファイバーでやり取りするための規格。**SDH**(Synchronous Digital Hierarchy)は電話回線や専用線などで信頼性の高い光伝送ネットワークとして広く利用されている。


## VPN

**VPN**(Virtual Private Network)は、インターネットのような公開された回線上で、安全にパケットをやり取りするための専用回線を擬似的に設ける技術。

### トンネリング

**トンネリング**は、不特定多数が使うような回線上で、データを互いにやり取りするための隔絶されたコネクションを設けること。

### カプセル化

**カプセル化**は、VPN上でやり取りされるデータを暗号化し、外部から隠蔽するための技術。

### 広域イーサネット

**広域イーサネット**は、通信事業者が独自に用意した閉域網を利用するVPN接続サービス。


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
