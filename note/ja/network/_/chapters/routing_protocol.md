# 『ルーティングプロトコル』

（最終更新： 2023-02-10）


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

**ルーティングプロトコル**は、ダイナミックルーティングにより自動的にルーティングテーブルの情報を更新するためのプロトコル。

### 自律システム

**自律システム**(**AS**: Autonomous System)は、ルーティングに関するルールを決めて、それを基に運用する範囲のこと。**経路制御ドメイン**(Routing Domain)ともいう。同一の決まりや考え方（ポリシー）によってルーティングを管理する単位を指す。

### EGP

**EGP**(Exterior Gateway Protocol)は、AS間のルーティングに利用されるプロトコル。

### IGP

**IGP**(Interior Gateway Protocol)は、ドメイン内のルーティングに利用されるプロトコル。


## 経路制御アルゴリズム

### メトリック

**メトリック**は、ルーティングの際に参考にされる、距離やコストといった転送の判断に用いられる指標のこと。

### 距離ベクトル型

**距離ベクトル型**(Distance-Vector)のアルゴリズムでは、物理的な距離と方向によってネットワークやホストの位置を決定する。メトリックとしては通過するルータの数（ホップ数）が用いられる。

### リンク状態型

**リンク状態型**(Link-State)のアルゴリズムでは、ルータがネットワーク全体の接続状態を理解してルーティングテーブルを作成する。全てのルータが同じ情報を持ち、安定したルーティングを行うことができる。

### 経路ベクトル型

**経路ベクトル型**(Path Vector)のアルゴリズムでは、通過する経路のリストによってルーティングを行う。


## RIP

**RIP**は、距離ベクトル型のルーティングプロトコルで、経路制御情報を定期的（30秒周期）にネットワーク上にブロードキャストする。メトリックの単位は通過するルータの数（ホップ数）で、できる限り少ないホップ数で目的のIPアドレスに到達できるように制御される。

### 無限カウント

**無限カウント**(Counting to Infinity)は、ルータが接続切れなどの影響により、過去に伝えた情報を逆に教えられ、それをお互いに伝えあってしまう問題。

### スプリットホライズン

**スプリットホライズン**(Split Horizon)は、無限カウントを解決するために、経路情報を教えられたインタフェースには経路情報を流さない方法。

しかし、ループのあるネットワークにおいては、スプリットホライズンを用いても無限カウントが発生してしまうため、ポイズンリバースやトリガードアップデートを用いる必要がある。

### ポイズンリバース

**ポイズンリバース**(Poisoned Reverse)は経路が切れたとき、その情報を流さないのではなく、通信不能であることを表す距離16を流す方法。

### トリガードアップデート

**トリガードアップデート**(Triggered Update)は、情報が変化したとき一定時間（30秒）の経過を待たずにすぐに伝える方法。

### RIP2

**RIP2**は、RIPを改良したプロトコルで、マルチキャスト対応やサブネットマスク対応などがされており、かなり実用的になっている。また、1つのネットワーク上で論理的に独立した複数のRIPが使えるようになっており、認証キーにも対応している。


## OSPF

**OSPF**は、リンク状態型のルーティングプロトコルで、ループのあるネットワークでも安定したルーティングを行うことができる。

RIPではホップ数が最も少なくなる方向に経路を設定するが、OSPFでは各リンクに重みをつけることができ、この重みが小さくなるように経路を選択する。この重みのことを**コスト**といい、メトリックとして用いる。

### 隣接ルータ

**隣接ルータ**(Neighboring Router)は、OSPFにおいて同一リンクに接続されているルータ。

### 指名ルータ

**指名ルータ**(Designated Router)は、OSPFにおいて隣接ルータの中でも経路情報を交換するように設定されたルータ。

### HELLOプロトコル

**HELLOプロトコル**は、OSPFにおいて接続の確認を行うプロトコル。

### リンク状態更新パケット

**リンク状態更新パケット**(Link State Update Packet)は、OSPFにおいて接続が切れたり回復したりといった接続状態の変化があった場合に送信されるパケット。

更新情報が送られてきたルータは、リンク状態データベースを作成して、それを基にルーティングテーブルを作成する。

### リンク状態データベース

**リンク状態データベース**は、OSPFにおいて接続されているルータやネットワークの情報を格納したもの。これを基にルーティングテーブルが作成される。

### ネットワークLSA

**ネットワークLSA**は、リンク状態更新パケットによって伝えられる情報のひとつで、ネットワークにどのルータが接続されているかを表した情報。

### ルータLSA

**ルータLSA**は、リンク状態更新パケットによって伝えられる情報のひとつで、ルータがどのネットワークに接続されているかを表した情報。

### ダイクストラ法

**ダイクストラ法**は、OSPFの最短経路を求めるためのアルゴリズムで、ルーティングテーブルを作成する際に用いられる。

### エリア

**エリア**は、OSPFにおいて計算の負荷軽減のために用いられた概念で、ネットワーク同士やホスト同士をまとめてグループ化したもの。

### バックボーンエリア

**バックボーンエリア**は、OSPFの各エリアが接続されている、根幹となるエリア。

### エリア境界ルータ

**エリア境界ルータ**は、エリアとバックボーンエリアを結ぶルータ。

### 内部ルータ

**内部ルータ**は、エリア内のルータ。

### バックボーンルータ

**バックボーンルータ**は、バックボーンエリアにのみ接続されているルータ。

### AS境界ルータ

**AS境界ルータ**は、外部のエリアと接続されているルータ。

### スタブエリア

**スタブエリア**は、エリア境界ルータを1つしか持たないエリアで、経路情報を減らすことができる。


## BGP

**BGP**は、組織間のネットワークを接続するときに利用される経路ベクトル型のプロトコルで、EGPに分類される。

ISPや地域ネットワークなどの組織を束ねるネットワーク集団を1つの自律システムとして扱い、それぞれの自律システムに16ビットの**AS番号**を割り当てる。

BGPにより経路制御情報を交換するルータを**BGPスピーカ**という。目的とするネットワークアドレスにパケットを送った場合に、そこに到達するまでのAS番号のリスト（**AS経路リスト**）が作られる。BGPのメトリックの単位は、通過するASの数となる。


## MPLS

**MPLS**は、代表的なラベルスイッチング機能。

### ラベルスイッチング

**ラベルスイッチング**は、IPパケットに**ラベル**を設定し、そのラベルに基づいて転送を行う技術。

ラベルをつけてフォワーディングする**Push**、ラベルを付け替えてフォワーディングする**Swap**、ラベルを外してフォワーディングする**Pop**を基本動作とする。

宛先が同じであるパケット(**FFC**: Forwarding Equivalence Class)は、どれもラベルによって決まる同一の経路を通る。個の経路を**LSP**(Label Switching Path)という。

### LSR

**LSR**(Label Switching Router)は、ラベルスイッチング機能を持つルータ。特に、外部のネットワークとの接続部分にあたるエッジのLSRを**LER**(Label Edge Router)という。