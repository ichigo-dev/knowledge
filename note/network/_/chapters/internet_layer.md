# 『インターネット層』ノート

（最終更新： 2023-09-23）


## 目次

1. [IP](#ip)
1. [ルーティング](#ルーティング)
	1. [ホップ](#ホップ)
	1. [ルーティングテーブル](#ルーティングテーブル)
	1. [ホップバイホップルーティング](#ホップバイホップルーティング)
	1. [デフォルトルート](#デフォルトルート)
	1. [スタティックルーティング](#スタティックルーティング)
	1. [ダイナミックルーティング](#ダイナミックルーティング)
	1. [ネットワークACL](#ネットワークacl)
1. [フラグメンテーション](#フラグメンテーション)
	1. [経路MTU探索](#経路mtu探索)
1. [IPv4](#ipv4)
	1. [IPv4のフレームフォーマット](#ipv4のフレームフォーマット)
1. [IPv6](#ipv6)
	1. [IPv6のフレームフォーマット](#ipv4のフレームフォーマット)
1. [DNS](#dns)
	1. [ドメイン名](#ドメイン名)
	1. [ホスト名](#ホスト名)
	1. [FQDN](#fqdn)
	1. [ネームサーバ](#ネームサーバ)
	1. [ラウンドロビンDNS](#ラウンドロビンdns)
	1. [リゾルバ](#リゾルバ)
	1. [Aレコード](#aレコード)
	1. [NSレコード](#nsレコード)
	1. [MXレコード](#mxレコード)
1. [ARP](#arp)
	1. [RARP](#rarp)
	1. [GARP](#garp)
	1. [代理ARP](#代理arp)
1. [ICMP](#icmp)
	1. [ICMPv6](#icmpv6)
1. [DHCP](#dhcp)
1. [NAT](#nat)
	1. [NAPT](#napt)
	1. [NAT64/DNS64](#nat64dns64)
	1. [CGN](#cgn)
	1. [NAT越え](#nat越え)
1. [IPトンネリング](#ipトンネリング)
1. [VRRP](#vrrp)
1. [IGMP](#igmp)
	1. [スヌーピング](#スヌーピング)
1. [IPエニーキャスト](#ipエニーキャスト)
1. [QoS](#qos)
	1. [ふくそう](#ふくそう)
	1. [IntServ](#intserv)
	1. [DiffServ](#diffserv)
	1. [ECN](#ecn)
1. [Mobile IP](#mobile-ip)


## IP

**IP**(Internet Protocol)は、エンドノード間の通信を実現する役割を果たす[インターネット層](./communication_protocol.md#インターネット層)の[プロトコル](./network_architecture.md#プロトコル)。IPは[コネクションレス型](./network.md#コネクションレス型)の通信で、[パケット](./network.md#パケット)を送信する前に[コネクション](./network.md#コネクション)の確立を行わない。[パケット](./network.md#パケット)を宛先まで送り届けるために最大限努力を行うことから、**最善努力型（ベストエフォート）のサービス**と呼ばれている。接続の信頼性を高める役目を担うのはIPの上位層である[TCP](./transport_layer.md#tcp)で、これは[コネクション型](./network.md#コネクション型)である。


## ルーティング

**ルーティング**（**経路制御**）は、宛先[IPアドレス](./address_on_network.md#ipアドレス)の[ホスト](./network.md#ホスト)まで[パケット](./network.md#パケット)を届ける仕組み。[ルータ](./network_architecture.md#ルータ)は正しい方向へ[パケット](./network.md#パケット)を転送するために、[ルーティングテーブル](#ルーティングテーブル)を参照して[パケット](./network.md#パケット)の転送先を決定する。

### ホップ

**ホップ**は、[TCP/IP](./communication_protocol.md#tcpip)において、[ネットワーク](./network.md#ネットワーク)の1区間を[IP](#ip)[パケット](./network.md#パケット)が移動すること。この1区間のことを**1ホップ**（**ワンホップ**）という。

### ルーティングテーブル

**ルーティングテーブル**（**経路制御表**）は、[IPアドレス](./address_on_network.md#ipアドレス)と対応する[ホスト](./network.md#ホスト)や[ルータ](./network_architecture.md#ルータ)の情報が書かれた表。[IP](#ip)では、ルーティングテーブルを元にして次の[ホップ](#ホップ)を決める。

### ホップバイホップルーティング

**ホップバイホップルーティング**は、[IP](#ip)において用いられている[ルーティング](#ルーティング)の方式で、最終目的地に行くまでの経路を各中継点（[ホップ](#ホップ)）で決定する方法。このため、[パケット](./network.md#パケット)は行き当たりばったりに最終目的地に近づいていく。

### デフォルトルート

**デフォルトルート**は、[ルーティングテーブル](#ルーティングテーブル)に経路情報がない宛先[IP](#ip)との通信の際に選ばれる、次の[ホップ](#ホップ)。

### スタティックルーティング

**スタティックルーティング**（**静的経路制御**）は、[ルーティングテーブル](#ルーティングテーブル)を[ネットワーク](./network.md#ネットワーク)管理者が手動で作成する方法。

### ダイナミックルーティング

**ダイナミックルーティング**（**動的経路制御**）は、他の[ルータ](./network_architecture.md#ルータ)との通信の際に、経路情報を交換することで、自動的に[ルーティングテーブル](#ルーティングテーブル)を作成する方法。[IP](#ip)自体には経路情報を交換する機能は備わっていないので、ダイナミックルーティングを行う場合には[ルーティングプロトコル](./routing_protocol.md#ルーティングプロトコル)が必要となる。

### ネットワークACL

**ネットワークACL**は、[システム](../../../system/_/chapters/system.md#システム)や[ファイル](../../../computer/software/_/chapters/file_system.md#ファイル)、[ネットワーク](./network.md#ネットワーク)上のリソースなどへのアクセス可否の設定をリストとして列挙したもの。宛先と送信元の[IPアドレス](./address_on_network.md#ipアドレス)および[ポート番号](./address_on_network.md#ポート番号)を条件とした上で、その条件に合致した通信の可否をACLとして設定する。


## フラグメンテーション

**フラグメンテーション**（**分割処理**）は、[データリンク](./datalink_layer.md#データリンク)の[MTU](./datalink_layer.md#mtu)に合わせて[パケット](./network.md#パケット)を分割する処理。[パケット](./network.md#パケット)は[ルータ](./network_architecture.md#ルータ)を経由するたびに必要に応じてフラグメンテーションされ、宛先[ホスト](./network.md#ホスト)にて復元される。経路の途中でフラグメンテーションが発生すると、分割されたデータの一部が失われただけで、全体の情報を再送する必要が出てきてしまうため、あらかじめ送信元[ホスト](./network.md#ホスト)で経路全体の[MTU](./datalink_layer.md#mtu)に[パケット](./network.md#パケット)を分割して送信することが多い。

### 経路MTU探索

**経路MTU探索**は、[ICMP](#icmp)の[到達不能メッセージ](#icmp)を利用して、送信元の[ホスト](./network.md#ホスト)から宛先[ホスト](./network.md#ホスト)までの経路の[MTU](./datalink_layer.md#mtu)を探索する処理。これにより、[フラグメンテーション](#フラグメンテーション)の発生を防ぎながらも、最大効率で[パケット](./network.md#パケット)を送信することができる。


## IPv4

**IPv4**は、[インターネット](./network.md#インターネット)の普及とともに広く使われるようになった[IP](#ip)の仕様で、[IPアドレス](./address_on_network.md#ipアドレス)を32[ビット](../../../basics/_/chapters/computer_and_number.md#ビット)で表現する。[インターネット](./network.md#インターネット)の急速な普及によって[IPアドレス](./address_on_network.md#ipアドレス)が不足したため、[IPv6](#ipv6)への置き替えが進んでいる。

### IPv4のフレームフォーマット

**DSCPフィールド**は、初期の頃はサービスタイプ(TOS: Type Of Service)として定義されていた部分で、[DiffServ](#diffserv)と呼ばれる品質制御に利用される。

**ECNフィールド**も同様にサービスタイプに置き換わって利用されているフィールドで、[ネットワーク](./network.md#ネットワーク)が[ふくそう](#ふくそう)していることを通知するために用いられる。


## IPv6

**IPv6**は、[IPv4](#ipv4)アドレスの枯渇問題を解決するために標準化された[IP](#ip)の仕様。[IPv4](#ipv4)の4倍の16オクテット（128[ビット](../../../basics/_/chapters/computer_and_number.md#ビット)）の長さを持ち、[IPv4](#ipv4)との互換性を保つ努力が行われている。IPv6の特徴や目的は以下の通りである。

- [IPアドレス](./address_on_network.md#ipアドレス)の拡大と経路制御表の集約
- パフォーマンスの向上
- [プラグアンドプレイ](../../../computer/hardware/_/chapters/bus.md#プラグアンドプレイ)機能を必須にする
- 認証機能や暗号化機能の採用
- [マルチキャスト](./network.md#マルチキャスト)、[Mobile IP](#mobile-ip)の機能を拡張機能として提供

### IPv6のフレームフォーマット


## DNS

**DNS**(Domain Name System)は、[ドメイン名](#ドメイン名)と[IPアドレス](./network.md#ipアドレス)の対応を管理する[システム](../../../system/_/chapters/system.md#システム)。[ドメイン名](#ドメイン名)から対応する[IPアドレス](./address_on_network.md#ipアドレス)を検索して、見つかった[IPアドレス](./address_on_network.md#ipアドレス)を返却する。

### ドメイン名

**ドメイン名**は、[インターネット](./network.md#インターネット)上の特定の[ネットワーク](./network.md#ネットワーク)を指す識別子。[ネットワーク](./network.md#ネットワーク)だけでなく、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)を指し示す場合もあり、この場合は[ホスト名](#ホスト名)や[FQDN](#fqdn)と同様の意味で用いられる。

### ホスト名

**ホスト名**は、[ネットワーク](./network.md#ネットワーク)内の特定の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)を指す識別子。

### FQDN

**FQDN**(Fully Qualified Domain Name)は、[ドメイン名](#ドメイン名)と[ホスト名](#ホスト名)を組み合わせた識別子で、[インターネット](./network.md#インターネット)上の特定の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)を表す。FQDNは[IPアドレス](./address_on_network.md#ipアドレス)と対応させることができる。FQDNのことを単に[ドメイン名](#ドメイン名)ということもある。

### ネームサーバ

**ネームサーバ**は、[ドメイン名](#ドメイン名)を管理する[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)を動作させる[ホスト](./network.md#ホスト)のことで、そのネームサーバが設置された階層（ゾーン）の[ドメイン](#ドメイン名)に関する情報を管理している。**ルートネームサーバ**は、ネームサーバの階層のルート部分に設置されている[サーバ](../../../computer/_/chapters/computer.md#サーバ)。

### ラウンドロビンDNS

**ラウンドロビンDNS**は、1つの[ドメイン名](#ドメイン名)に複数の[IPアドレス](./address_on_network.md#ipアドレス)を割り当てることができる[DNS](#dns)。[IPアドレス](./address_on_network.md#ipアドレス)の問い合わせが行われると、同じ[ドメイン名](#ドメイン名)に登録された複数のレコードのうち毎回異なる値を返す。この動作はサービスの[負荷分散](../../../system/_/chapters/system_processing_model.md#負荷分散)などに利用される。ただし、ラウンドロビンDNSを利用した[負荷分散](../../../system/_/chapters/system_processing_model.md#負荷分散)では、[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)の数だけ[グローバルIPアドレス](./address_on_network.md#グローバルipアドレス)が必要になり、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)側の[プロキシサーバ](../../../system/security/_/chapters/security_technology.md#プロキシ)のキャッシュ利用により均等に負荷が分散されない可能性や、[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)ダウン時に対応できないといった問題がある。

### リゾルバ

**リゾルバ**は、[DNS](#dns)に問い合わせをするための[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)。リゾルバは、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)内の `hosts` という[ファイル](../../../computer/software/_/chapters/file_system.md#ファイル)を参照したり、外部の[ネームサーバ](#ネームサーバ)に問い合わせることで名前解決する。リゾルバの[DNS](#dns)に対する問い合わせを**クエリ**という。

### Aレコード

**Aレコード**は、[DNS](#dns)で管理される、[ドメイン名](#ドメイン名)と[IPアドレス](./address_on_network.md#ipアドレス)の対応レコード。

### NSレコード

**NSレコード**は、[DNS](#dns)で管理される、上位や下位の[ネームサーバ](#ネームサーバ)の[IPアドレス](./address_on_network.md#ipアドレス)のレコード。

### MXレコード

**MXレコード**は、[DNS](#dns)で管理される、メールアドレスとそのメールを受信する[メールサーバ](./application_layer.md#電子メール)の[ホスト名](#ホスト名)の対応レコード。


## ARP

**ARP**(Address Resolution Protocol)は、アドレス解決のための[プロトコル](./network_architecture.md#プロトコル)。宛先[IPアドレス](./address_on_network.md#ipアドレス)を手掛かりにして、次に[パケット](./network.md#パケット)を受け取るべき機器の[MACアドレス](./address_on_network.md#macアドレス)を知りたいときに利用する。次の[ホップ](#ホップ)の[ルータ](./network_architecture.md#ルータ)の[MACアドレス](./address_on_network.md#macアドレス)を調べるために使用される。

ARPでは、最初にARP要求[パケット](./network.md#パケット)を[ネットワーク](./network.md#ネットワーク)に[ブロードキャスト](./network.md#ブロードキャスト)する。この[パケット](./network.md#パケット)には知りたい[ホスト](./network.md#ホスト)の[IPアドレス](./address_on_network.md#ipアドレス)が含まれており、該当する[ホスト](./network.md#ホスト)はARP応答[パケット](./network.md#パケット)を返信する。

[IPv6](#ipv6)では、ARPの代わりに[ICMPv6](#icmpv6)の[近接探索メッセージ](#icmpv6)が利用されている。

### RARP

**RARP**(Reverse ARP)は、[ARP](#arp)の反対で、[MACアドレス](./address_on_network.md#macアドレス)から[IPアドレス](./address_on_network.md#ipアドレス)を知りたい場合に使われる。RARPを使うにはRARPサーバを用意する必要がある。

### GARP

**GARP**(Gratuitous ARP)は、自分の[IPアドレス](./address_on_network.md#ipアドレス)に対する[MACアドレス](./address_on_network.md#macアドレス)を知りたい場合に使われる。[IPアドレス](./address_on_network.md#ipアドレス)の重複を確認したり、スイッチングハブなどの[MACアドレス](./address_on_network.md#macアドレス)学習テーブルを更新させる働きがある。

### 代理ARP

**代理ARP**(Proxy ARP)は、[ネットワーク](./network.md#ネットワーク)内の機器からの[ARP](#arp)要求を、本来の機器に代わって返答する、[ルータ](./network_architecture.md#ルータ)などの機能のひとつ。通常の[ARP](#arp)は、同一[セグメント](./datalink_layer.md#セグメント)内で[IP](#ip)[パケット](./network.md#パケット)を配送するときに使われるが、代理ARPは、[ルーティングテーブル](#ルーティングテーブル)を使わずに[IP](#ip)[パケット](./network.md#パケット)を別の[セグメント](./datalink_layer.md#セグメント)に送りたい場合に使われる。


## ICMP

**ICMP**(Internet Control Message Protocol)は、[ネットワーク](./network.md#ネットワーク)が正常に動作しているかの確認や、異常が発生したときのトラブルシューティング（障害対策）を行う[プロトコル](./network_architecture.md#プロトコル)。ICMPには、[IP](#ip)[パケット](./network.md#パケット)が目的の[ホスト](./network.md#ホスト)まで届いたかどうかを確認する機能や、[IP](#ip)[パケット](./network.md#パケット)が破棄されたときにその原因を通知する機能がある。

- ICMP 到達不能メッセージ
- ICMP リダイレクトメッセージ
- ICMP 時間経過メッセージ
- ICMP エコーメッセージ
- ICMP ルーター探索メッセージ
- ICMP 拡張エコーメッセージ

### ICMPv6

**ICMPv6**は、[IPv6](#ipv6)用の[ICMP](#icmp)で、[IPv6](#ipv6)の通信を行う上でなくてはならない[プロトコル](./network_architecture.md#プロトコル)。[ICMP](#icmp)は[IPv4](#ipv4)においては補助的な役割でしかなかったが、ICMPv6はその役割がより重要なものとなっている。

特に近隣探索では、近隣探索メッセージを[ブロードキャスト](./network.md#ブロードキャスト)に対して送信し、近隣告知メッセージで[MACアドレス](./address_on_network.md#macアドレス)を通知する。


## DHCP

**DHCP**(Dynamic Host Configuration Protocol)は、[IPアドレス](./address_on_network.md#ipアドレス)などの[ネットワーク](./network.md#ネットワーク)への接続に必要な情報を一括管理する[プロトコル](./network_architecture.md#プロトコル)。特に、スマートフォンやラップトップPCなどの移動を伴う端末では、DHCPを用いることにより[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)の[プラグアンドプレイ](../../../computer/hardware/_/chapters/bus.md#プラグアンドプレイ)を実現している。

また、大規模な[ネットワーク](./network.md#ネットワーク)ではたくさんのDHCPを管理することになるため、これらを一元管理するために**DHCPリレーエージェント**を用いる。


## NAT

**NAT**(Network Address Translator)は、[グローバルIPアドレス](./address_on_network.md#グローバルipアドレス)と[プライベートIPアドレス](./address_on_network.md#プライベートipアドレス)を一対一で変換する機能。ローカルな[ネットワーク](./network.md#ネットワーク)で[プライベートIPアドレス](./address_on_network.md#プライベートipアドレス)を使用している機器が[インターネット](./network.md#インターネット)へ接続するときに、[プライベートIPアドレス](./address_on_network.md#プライベートipアドレス)を[グローバルIPアドレス](./address_on_network.md#グローバルipアドレス)に変換する。[インターネット](./network.md#インターネット)に同時に接続できる台数は、割り当てられている[グローバルIPアドレス](./address_on_network.md#グローバルipアドレス)の個数までとなる。

NATには、次のような問題点がある。

- NATの外側から内側の[サーバ](../../../computer/_/chapters/computer.md#サーバ)に接続することはできない
- 変換テーブルの作成や変換処理に[オーバヘッド](../../../system/_/chapters/system_performance_evaluation.md#オーバヘッド)が生じる
- 通信中にNATが異常動作して再起動した場合、すべての[TCP](./transport_layer.md#tcp)[コネクション](./network.md#コネクション)がリセットされる

### NAPT

**NAPT**（Network Address Ports Translator、**IPマスカレード**）は、[グローバルIPアドレス](./address_on_network.md#グローバルipアドレス)1つに対して複数の[プライベートIPアドレス](./address_on_network.md#プライベートipアドレス)を対応させる機能。[IPアドレス](./address_on_network.md#ipアドレス)と[ポート番号](./address_on_network.md#ポート番号)の組み合わせにより通信を識別するため、1つの[グローバルIPアドレス](./address_on_network.md#グローバルipアドレス)で複数の機器が同時に[インターネット](./network.md#インターネット)に接続できる。モバイルルータやスマートフォンのテザリングは、NAPTを利用している。

現在では、NAPTのことを単に[NAT](#nat)、[NAT](#nat)のことをベーシックNATと呼ぶことが多い。

### NAT64/DNS64

**NAT64/DNS64**は、[DNS](#dns)と[NAT](#nat)が連携して動作することで、[IPv6](#ipv6)環境から[IPv4](#ipv4)環境への通信を実現するための技術。

### CGN

**CGN**(Carrier Grade NAT)は、[ISP](./network.md#isp)レベルで[NAT](#nat)を行う技術。**LSN**(Large Scale NAT)と呼ばれることもある。

### NAT越え

**NAT越え**(NAT traversal)は、[IPv6](#ipv6)を使用する、[NAT](#nat)環境を前提とした[アプリケーション](../../../computer/software/_/chapters/software.md#応用ソフトウェア)を利用する、といった方法によって[NAT](#nat)があっても[NAT](#nat)の外側と内側が通信できるようにする技術。

### DSR

**DSR**(Direct Server Return)は、[NAT](#nat)とは異なり、[IPアドレス](./address_on_network.md#ipアドレス)の書き換えを行わずに[パケット](./network.md#パケット)をルーティングする技術。[トランスポート層](./network_architecture.md#トランスポート層)における[ロードバランサ](../../../system/_/chapters/system_processing_model.md#ロードバランサ)などに利用される場合が多く、[IPアドレス](./address_on_network.md#ipアドレス)の書き換えを行わない分高速でトラフィックを制御できる一方で、転送先の[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が[グローバルIPアドレス](./address_on_network.md#グローバルipアドレス)を処理できなければならないという制限がある。

## IPトンネリング

**IPトンネリング**は、[IPv4](#ipv4)[パケット](./network.md#パケット)全体を1つのデータとして扱い、その前に[IPv6](#ipv6)ヘッダを付与することで、[IPv4](#ipv4)環境同士の通信の間に[IPv6](#ipv6)環境の[ネットワーク](./network.md#ネットワーク)が介在するさせる技術。

IPトンネリングを使用すると、追加されるヘッダの分だけ[MTU](./datalink_layer.md#mtu)が小さくなるため、**ジャンボフレーム**（1500[バイト](../../../basics/_/chapters/computer_and_number.md#バイト)以上のペイロードを持つ[Ethernet](./network.md#イーサネット)[フレーム](./network.md#フレーム)）の利用などの工夫が必要となる。


## VRRP

**VRRP**(Virtual Router Redundancy Protocol)は、デフォルト[ルータ](./network_architecture.md#ルータ)の故障やメンテナンス時にも[ネットワーク](./network.md#ネットワーク)が利用できるように、複数の[ルータ](./network_architecture.md#ルータ)による[冗長化](../../../system/_/chapters/system_architecture.md#冗長化)を行う仕組み。スマートフォンや[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)は、デフォルト[ルータ](./network_architecture.md#ルータ)（デフォルト[ゲートウェイ](./network_architecture.md#ゲートウェイ)）を経由して社内[LAN](./network.md#lan)や[インターネット](./network.md#インターネット)を利用する環境が一般的なので、VRRPによる[冗長化](../../../system/_/chapters/system_architecture.md#冗長化)が重要となる。

VRRPでは複数の[ルータ](./network_architecture.md#ルータ)をまとめて運用し、その中の1つをマスタールータ、別の[ルータ](./network_architecture.md#ルータ)をバックアップルータとして扱う。マスタールータは定期的に**VRRPパケット**を[マルチキャスト](./network.md#マルチキャスト)を使って送信する。バックアップルータがこのVRRPパケットを一定期間受け取れなかったとき、マスタールータが故障したと判断してマスタールータを切り替える。

複数台のバックアップルータを使用する場合は、各バックアップルータに**プライオリティ**という数値を設定しておき、障害時はこの値が大きいものが優先してマスタールータに昇格する。**プリエンプティブモード**が有効である場合、既存のマスタールータよりも高いプライオリティを持ったバックアップルータが起動すると[フェールオーバ](../../../system/_/chapters/reliability_design.md#フェールオーバ)が発生する。


## IGMP

**IGMP**(Internet Group Management Protocol)は、IPマルチキャストの通信において通信相手がいるかどうかを確認するための[プロトコル](./network_architecture.md#プロトコル)。[マルチキャスト](./network.md#マルチキャスト)では[コネクションレス](./network.md#コネクションレス型)の[UDP](./transport_layer.md#udp)を用いるため、受信者がいなくても[ネットワーク](./network.md#ネットワーク)を使用し続けてしまい、無駄が多くなる。これを防ぐためにIGMPが利用される。

また、[IPv6](#ipv6)においては**MLD**(Multicast Listener Discovery)が用いて受信者の確認を行う。

### スヌーピング

**IGMP(MLD)スヌーピング**では、スイッチングハブが通過する[パケット](./network.md#パケット)を覗き見して、どの[ポート](./address_on_network.md#ポート番号)にどのアドレスの[マルチキャスト](./network.md#マルチキャスト)[フレーム](./network.md#フレーム)を送ればよいかを知り、無関係な[ポート](./address_on_network.md#ポート番号)には[マルチキャスト](./network.md#マルチキャスト)[フレーム](./network.md#フレーム)を流さないようにする。


## IPエニーキャスト

**IPエニーキャスト**は、同じサービスを提供する[サーバ](../../../computer/_/chapters/computer.md#サーバ)に同じ[IPアドレス](./address_on_network.md#ipアドレス)を付け、クライアントの最寄りの[サーバ](../../../computer/_/chapters/computer.md#サーバ)と通信できるようにする方法。代表例として、[DNS](#dns)ルートサーバーが挙げられる。

IPエニーキャストでは、1つ目の[パケット](./network.md#パケット)と2つ目の[パケット](./network.md#パケット)が同じ[ホスト](./network.md#ホスト)に届くという保証がない。そのため、最初の1[パケット](./network.md#パケット)のみ[エニーキャスト](./network.md#エニーキャスト)を用いて、それ以降は[ユニキャスト](./network.md#ユニキャスト)を使うといった処理が行われる。


## QoS

**QoS**(Quality of Service)は、[ネットワーク](./network.md#ネットワーク)上のサービスの品質。

### ふくそう

**ふくそう**（輻輳）は、ベストエフォートの通信において、通信回線が混雑すると性能が極端に低下するという問題。

### IntServ

**IntServ**は、**RSVP**(Resource Reservation Protocol)エンドツーエンドできめ細かい優先制御を提供するための仕組み。必要な時にだけフローのセットアップを行って通信品質を制御する。RSVPは、[パケット](./network.md#パケット)を受信する側から送信する側に向けて制御[パケット](./network.md#パケット)を流し、その間に存在する[ルータ](./network_architecture.md#ルータ)に品質制御のための設定を行う。

### DiffServ

**DiffServ**は、特定の[ネットワーク](./network.md#ネットワーク)内で大雑把に通信品質を制御することが目的で、DiffServ[ドメイン](#ドメイン名)の境界にある[ルータ](./network_architecture.md#ルータ)によって[IP](#ip)[パケット](./network.md#パケット)の[DSCPフィールド](#ipv4のフレームフォーマット)を書き換えることによって制御を行う。

### ECN

**ECN**(Explicit Congestion Notification)は、[ふくそう](#ふくそう)通知機能を実現するための仕組み。[ふくそう](#ふくそう)が起きている場合には、[ホスト](./network.md#ホスト)はデータの送信量を減らす必要がある。


## Mobile IP

**Mobile IP**は、[ホスト](./network.md#ホスト)が接続している[サブネット](./network.md#サブネットワーク)が変わっても、[IPアドレス](./address_on_network.md#ipアドレス)が変らないようにする技術。

- 移動ホスト(MH: Mobile Host)
- ホームネットワーク、ホームアドレス
- 気付けアドレス(CoA: Care-of Address)
- ホームエージェント(HA: Home Agent)
- 外部エージェント(FA: Foreign Agent)
