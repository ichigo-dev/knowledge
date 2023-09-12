# 『ルーティングプロトコル』ノート

（最終更新： 2023-06-27）


## 目次

1. [ルーティングプロトコル](#ルーティングプロトコル) 
	1. [自律システム](#自律システム) 
	1. [EGP](#egp) 
	1. [IGP](#igp) 
1. [経路制御アルゴリズム](#経路制御アルゴリズム) 
	1. [メトリック](#メトリック) 
	1. [距離ベクトル型](#距離ベクトル型) 
	1. [リンク状態型](#リンク状態型) 
	1. [経路ベクトル型](#経路ベクトル型) 
1. [RIP](#rip) 
	1. [無限カウント](#無限カウント) 
	1. [スプリットホライズン](#スプリットホライズン) 
	1. [ポイズンリバース](#ポイズンリバース) 
	1. [トリガードアップデート](#トリガードアップデート) 
	1. [RIP2](#rip2) 
1. [OSPF](#ospf) 
	1. [隣接ルータ](#隣接ルータ) 
	1. [指名ルータ](#指名ルータ) 
	1. [HELLOプロトコル](#helloプロトコル) 
	1. [リンク状態更新パケット](#リンク状態更新パケット) 
	1. [リンク状態データベース](#リンク状態データベース)
	1. [ネットワークLSA](#ネットワークlsa)
	1. [ルータLSA](#ルータlsa)
	1. [ダイクストラ法](#ダイクストラ法)
	1. [エリア](#エリア)
	1. [バックボーンエリア](#バックボーンエリア)
	1. [エリア境界ルータ](#エリア境界ルータ)
	1. [内部ルータ](#内部ルータ)
	1. [バックボーンルータ](#バックボーンルータ)
	1. [AS境界ルータ](#as境界ルータ)
	1. [スタブエリア](#スタブエリア)
1. [BGP](#bgp)
1. [MPLS](#mpls)
	1. [ラベルスイッチング](#ラベルスイッチング)
	1. [LSR](#lsr)


## ルーティングプロトコル

**ルーティングプロトコル**は、[ダイナミックルーティング](./internet_layer.md#ダイナミックルーティング)により自動的に[ルーティングテーブル](./internet_layer.md#ルーティングテーブル)の情報を更新するための[プロトコル](./network_architecture.md#プロトコル)。

### 自律システム

**自律システム**(**AS**: Autonomous System)は、[ルーティング](./internet_layer.md#ルーティング)に関する同一の考え方や決まり（ポリシー）で運用される[ネットワーク](./network.md#ネットワーク)の集まり。**経路制御ドメイン**(Routing Domain)ともいう。

### EGP

**EGP**(Exterior Gateway Protocol)は、[AS](#自律システム)間の[ルーティング](./internet_layer.md#ルーティング)に利用される[プロトコル](./network_architecture.md#プロトコル)。

### IGP

**IGP**(Interior Gateway Protocol)は、ドメイン内の[ルーティング](./internet_layer.md#ルーティング)に利用される[プロトコル](./network_architecture.md#プロトコル)。


## 経路制御アルゴリズム

### メトリック

**メトリック**は、[ルーティング](./internet_layer.md#ルーティング)の際に参考にされる、距離やコストといった転送の判断に用いられる指標。

### 距離ベクトル型

**距離ベクトル型**(Distance-Vector)の[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)は、物理的な距離と方向によって経由する[ネットワーク](./network.md#ネットワーク)や[ホスト](./network.md#ホスト)の位置を決定する。[メトリック](#メトリック)としては通過する[ルータ](./network_architecture.md#ルータ)の数（[ホップ](./internet_layer.md#ホップ)数）が用いられる。

### リンク状態型

**リンク状態型**(Link-State)の[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)は、[ルータ](./network_architecture.md#ルータ)が[ネットワーク](./network.md#ネットワーク)全体の接続状態を理解して[ルーティングテーブル](./internet_layer.md#ルーティングテーブル)を作成する。全ての[ルータ](./network_architecture.md#ルータ)が同じ情報を持ち、安定した[ルーティング](./internet_layer.md#ルーティング)を行うことができる。

### 経路ベクトル型

**経路ベクトル型**(Path Vector)の[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)は、通過する経路のリストによって[ルーティング](./internet_layer.md#ルーティング)を行う。


## RIP

**RIP**は、[距離ベクトル型](#距離ベクトル型)の[ルーティングプロトコル](#ルーティングプロトコル)で、[経路制御](./internet_layer.md#ルーティング)情報を定期的（30秒周期）に[ネットワーク](./network.md#ネットワーク)上に[ブロードキャスト](./network.md#ブロードキャスト)する。[メトリック](#メトリック)の単位は通過する[ルータ](./network_architecture.md#ルータ)の数（[ホップ](./internet_layer.md#ホップ)数）で、できる限り少ない[ホップ](./internet_layer.md#ホップ)数で目的の[IPアドレス](./address_on_network.md#ipアドレス)に到達できるように制御される。

### 無限カウント

**無限カウント**(Counting to Infinity)は、[ルータ](./network_architecture.md#ルータ)が接続切れなどの影響により、過去に伝えた情報を逆に教えられ、それをお互いに伝えあってしまう問題。

### スプリットホライズン

**スプリットホライズン**(Split Horizon)は、[無限カウント](#無限カウント)を解決するために、経路情報を教えられたインタフェースには経路情報を流さない方法。

しかし、ループのある[ネットワーク](./network.md#ネットワーク)においては、スプリットホライズンを用いても[無限カウント](#無限カウント)が発生してしまうため、[ポイズンリバース](#ポイズンリバース)や[トリガードアップデート](#トリガードアップデート)を用いる必要がある。

### ポイズンリバース

**ポイズンリバース**(Poisoned Reverse)は、経路が切れたときにその情報を流さないのではなく、通信不能であることを表す距離16を流す方法。

### トリガードアップデート

**トリガードアップデート**(Triggered Update)は、情報が変化したとき一定時間（30秒）の経過を待たずにすぐに伝える方法。

### RIP2

**RIP2**は、[RIP](#rip)を改良した[プロトコル](./network_architecture.md#プロトコル)で、[マルチキャスト](./network.md#マルチキャスト)対応や[サブネットマスク](./address_on_network.md#サブネットマスク)対応などがされており、かなり実用的になっている。また、1つの[ネットワーク](./network.md#ネットワーク)上で論理的に独立した複数の[RIP](#rip)が使えるようになっており、認証キーにも対応している。


## OSPF

**OSPF**は、[リンク状態型](#リンク状態型)の[ルーティングプロトコル](#ルーティングプロトコル)で、ループのある[ネットワーク](./network.md#ネットワーク)でも安定した[ルーティング](./internet_layer.md#ルーティング)を行うことができる。

[RIP](#rip)では[ホップ](./internet_layer.md#ホップ)数が最も少なくなる方向に経路を設定するが、OSPFでは各リンクに重みをつけることができ、この重みが小さくなるように経路を選択する。この重みのことを**コスト**といい、[メトリック](#メトリック)として用いる。

### 隣接ルータ

**隣接ルータ**(Neighboring Router)は、[OSPF](#ospf)において同一リンクに接続されている[ルータ](./network_architecture.md#ルータ)。

### 指名ルータ

**指名ルータ**(Designated Router)は、[OSPF](#ospf)において[隣接ルータ](#隣接ルータ)の中でも経路情報を交換するように設定された[ルータ](./network_architecture.md#ルータ)。

### HELLOプロトコル

**HELLOプロトコル**は、[OSPF](#ospf)において接続の確認を行う[プロトコル](./network_architecture.md#プロトコル)。

### リンク状態更新パケット

**リンク状態更新パケット**(Link State Update Packet)は、[OSPF](#ospf)において接続が切れたり回復したりといった接続状態の変化があった場合に送信される[パケット](./network.md#パケット)。

更新情報が送られてきた[ルータ](./network_architecture.md#プロトコル)は、[リンク状態データベース](#リンク状態データベース)を作成して、それをもとに[ルーティングテーブル](./internet_layer.md#ルーティングテーブル)を作成する。

### リンク状態データベース

**リンク状態データベース**は、[OSPF](#ospf)において接続されている[ルータ](./network_architecture.md#ルータ)や[ネットワーク](./network.md#ネットワーク)の情報を格納したもの。これをもとに[ルーティングテーブル](./internet_layer.md#ルーティングテーブル)が作成される。

### ネットワークLSA

**ネットワークLSA**は、[リンク状態更新パケット](#リンク状態更新パケット)によって伝えられる情報のひとつで、[ネットワーク](./network.md#ネットワーク)にどの[ルータ](./network_architecture.md#ルータ)が接続されているかを表した情報。

### ルータLSA

**ルータLSA**は、[リンク状態更新パケット](#リンク状態更新パケット)によって伝えられる情報のひとつで、[ルータ](./network_architecture.md#ルータ)がどの[ネットワーク](./network_architecture.md#ネットワーク)に接続されているかを表した情報。

### ダイクストラ法

**ダイクストラ法**は、[OSPF](#ospf)の最短経路を求めるための[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)で、[ルーティングテーブル](./internet_layer.md#ルーティングテーブル)を作成する際に用いられる。

### エリア

**エリア**は、[OSPF](#ospf)において計算の負荷軽減のために用いられた概念で、[ネットワーク](./network.md#ネットワーク)同士や[ホスト](./network.md#ホスト)同士をまとめてグループ化したもの。

### バックボーンエリア

**バックボーンエリア**は、[OSPF](#ospf)の各[エリア](#エリア)が接続されている、根幹となる[エリア](#エリア)。

### エリア境界ルータ

**エリア境界ルータ**は、[エリア](#エリア)と[バックボーンエリア](#バックボーンエリア)を結ぶ[ルータ](./network_architecture.md#ルータ)。

### 内部ルータ

**内部ルータ**は、[エリア](#エリア)内の[ルータ](./network_architecture.md#ルータ)。

### バックボーンルータ

**バックボーンルータ**は、[バックボーンエリア](#バックボーンエリア)にのみ接続されている[ルータ](./network_architecture.md#ルータ)。

### AS境界ルータ

**AS境界ルータ**は、外部の[エリア](#エリア)と接続されている[ルータ](./network_architecture.md#ルータ)。

### スタブエリア

**スタブエリア**は、[エリア境界ルータ](#エリア境界ルータ)を1つしか持たない[エリア](#エリア)のことで、スタブエリアを用いることで経路情報を減らすことができる。


## BGP

**BGP**は、組織間の[ネットワーク](./network.md#ネットワーク)を接続するときに利用される[経路ベクトル型](#経路ベクトル型)の[プロトコル](./network_architecture.md#プロトコル)で、[EGP](#egp)に分類される。

[ISP](./network.md#isp)や地域[ネットワーク](./network.md#ネットワーク)などの組織を束ねる[ネットワーク](./network.md#ネットワーク)集団を1つの[自律システム](#自律システム)として扱い、それぞれの[自律システム](#自律システム)に16[ビット](../../../basics/_/chapters/computer_and_number.md#ビット)の**AS番号**を割り当てる。

BGPにより[経路制御](./internet_layer.md#ルーティング)情報を交換する[ルータ](./network_architecture.md#ルータ)を**BGPスピーカ**という。目的とするネットワークアドレスに[パケット](./network.md#パケット)を送った場合に、そこに到達するまでのAS番号のリスト（**AS経路リスト**）が作られる。BGPの[メトリック](#メトリック)の単位は、通過する[AS](#自律システム)の数となる。


## MPLS

**MPLS**は、代表的な[ラベルスイッチング](#ラベルスイッチング)機能。

### ラベルスイッチング

**ラベルスイッチング**は、[IP](./internet_layer.md#ip)[パケット](./network.md#パケット)に**ラベル**を設定し、そのラベルに基づいて転送を行う技術。

ラベルをつけてフォワーディングする**Push**、ラベルを付け替えてフォワーディングする**Swap**、ラベルを外してフォワーディングする**Pop**を基本動作とする。

宛先が同じである[パケット](./network.md#パケット)(**FFC**: Forwarding Equivalence Class)は、どれもラベルによって決まる同一の経路を通る。この経路を**LSP**(Label Switching Path)という。

### LSR

**LSR**(Label Switching Router)は、[ラベルスイッチング](#ラベルスイッチング)機能を持つ[ルータ](./network_architecture.md#ルータ)。特に、外部の[ネットワーク](./network.md#ネットワーク)との接続部分にあたる[エッジ](./network.md#エッジ)のLSRを**LER**(Label Edge Router)という。