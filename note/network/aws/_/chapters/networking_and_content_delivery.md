# 『ネットワーキングとコンテンツ配信』ノート

（最終更新： 2023-05-07）


## 目次

1. [VPC](#vpc)
	1. [セキュリティグループ](#セキュリティグループ)
	1. [インバウンドルール](#インバウンドルール)
	1. [アウトバウンドルール](#アウトバウンドルール)
	1. [ENI](#eni)
	1. [インターネットゲートウェイ](#インターネットゲートウェイ)
	1. [インタフェースエンドポイント](#インタフェースエンドポイント)
	1. [Gateway Load Balancerエンドポイント](#gateway-load-balancerエンドポイント)
	1. [ゲートウェイエンドポイント](#ゲートウェイエンドポイント)
1. [Direct Connect](#direct-connect)
1. [PrivateLink](#privatelink)
1. [CloudFront](#cloudfront)
1. [Route 53](#route-53)
	1. [ホストゾーン](#ホストゾーン)
	1. [Aliasレコード](#aliasレコード)
	1. [ルーティングポリシー](#ルーティングポリシー)
1. [ELB](#elb)
	1. [ALB](#alb)
	1. [NLB](#nlb)
	1. [CLB](#clb)
	1. [GWLB](#gwlb)


## VPC

**VPC**(Amazon Virtual Private Cloud)は、[AWS](./aws.md#aws)のネットワーキングサービスの中心であり、プライベートな仮想ネットワークを[AWS](./aws.md#aws)内に作成することができるサービス。自由な[IPアドレス](../../../_/chapters/address_on_network.md#ipアドレス)のアサインや、[サブネット](../../../_/chapters/address_on_network.md#サブネットワーク)の作成、[ルーティングテーブル](../../../_/chapters/internet_layer.md#ルーティングテーブル)や[ゲートウェイ](../../../_/chapters/network_architecture.md#ゲートウェイ)の設定など、仮想ネットワークを完全に制御することができる。

[S3](./storage.md#s3)や[CloudWatch](./management_and_governance.md#cloudwatch)、[DynamoDB](./database.md#dynamodb)など、VPCに入れることができないサービスも多数あるので注意。

[ネットワークACL](../../../_/chapters/internet_layer.md#ネットワークacl)により、1つ以上の[サブネット](../../../_/chapters/address_on_network.md#サブネットワーク)に出入りするトラフィックを制御するファイアウォールを設定することができる。[サブネット](../../../_/chapters/address_on_network.md#サブネットワーク)単位で[インバウンドルール](#インバウンドルール)と[アウトバウンドルール](#アウトバウンドルール)の許可・拒否設定をすることができる。ステートレスで、1つの通信で受信と送信のそれぞれに対して設定が必要。

### セキュリティグループ

**セキュリティグループ**は、[VPC](#vpc)内のリソースに対するトラフィックを制御するファイアウォール機能。[EC2](./computing.md#ec2)や[ELB](#elb)、[RDS](./database.md#rds)などのインスタンス単位で[インバウンドルール](#インバウンドルール)と[アウトバウンドルール](#アウトバウンドルール)の許可設定をすることができる。[プロトコル](../../../_/chapters/network_architecture.md#プロトコル)や[ポート](../../../_/chapters/address_on_network.md#ポート番号)範囲、[IPアドレス](../../../_/chapters/address_on_network.md#ipアドレス)、他のセキュリティグループなどを指定して通信を制御する。ステーフルで、通信の復路は動的に許可される。

### インバウンドルール

**インバウンドルール**は、[ネットワーク](../../../_/chapters/network.md#ネットワーク)や[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)に対して外部からアクセスする際の規約。

### アウトバウンドルール

**アウトバウンドルール**は、[ネットワーク](../../../_/chapters/network.md#ネットワーク)や機器から外部に向かって通信する際の規約。

### ENI

**ENI**(Elastic Network Interface)は、[VPC](#vpc)内での仮想ネットワークインタフェースカード([NIC](../../../_/chapters/network.md#nic))を表す論理コンポーネント。インスタンスにアタッチしたりデタッチしたりすることで、[IPアドレス](../../../_/chapters/address_on_network.md#ipアドレス)や[MACアドレス](../../../_/chapters/address_on_network.md#macアドレス)といった[ネットワーク](../../../_/chapters/network.md#ネットワーク)の属性を操作することができる。

### インターネットゲートウェイ

**インターネットゲートウェイ**(**IGW**)は、[VPC](#vpc)を[インターネット](../../../_/chapters/network.md#インターネット)に接続するためのサービス。ひとつの[VPC](#vpc)に対してひとつアタッチすることができ、[VPC](#vpc)から[インターネット](../../../_/chapters/network.md#インターネット)へアクセスすることができるようになる。

### インタフェースエンドポイント

**インタフェースエンドポイント**は、[PrivateLink](#privatelink)を用いて、[VPC](#vpc)と他のサービスをプライベートに接続するためのサービス。同じ[リージョン](./aws.md#リージョン)内のサービスにしか接続できない。

### Gateway Load Balancerエンドポイント

**Gateway Load Balancerエンドポイント**は、[PrivateLink](#privatelink)を用いて、[VPC](#vpc)とロードバランサ（[GWLB](#gwlb)）を接続し、ロードバランサを経由して目的のサービスに接続する。ロードバランサを用いるため、スケールが容易となる。

### ゲートウェイエンドポイント

**ゲートウェイエンドポイント**は、[VPC](#vpc)の[ルーティングテーブル](../../../_/chapters/internet_layer.md#ルーティングテーブル)にターゲットの[グローバルIPアドレス](../../../_/chapters/address_on_network.md#グローバルipアドレス)を追加することでプライベートに接続するサービス。[S3](./storage.md#s3)と[DynamoDB](./database.md#dynamodb)にのみ対応しており、同じ[リージョン](./aws.md#リージョン)内のサービスにしか接続できない。


## Direct Connect

**AWS Direct COnnect**は、[AWS](./aws.md#aws)の[クラウドサービス](../../../../system/_/chapters/system_architecture.md#クラウドコンピューティング)と[オンプレミス](../../../../system/_/chapters/system_architecture.md#オンプレミス)の[サーバ](../../../../computer/_/chapters/computer.md#サーバ)などを専用線でプライベート接続するためのサービス。[インターネット](../../../_/chapters/network.md#インターネット)を経由せず接続することができるので、高いセキュリティレベルを保持したまま安定した速度でデータの送受信が行える。


## PrivateLink

**AWS PrivateLink**は、トラフィックをパブリック[インターネット](../../../_/chapters/network.md#インターネット)に公開することなく、[VPC](#vpc)、[AWS](./aws.md#aws)のサービス、および[オンプレミス](../../../../system/_/chapters/system_architecture.md#オンプレミス)[ネットワーク](../../../_/chapters/network.md#ネットワーク)間のプライベート接続を実現するサービス。異なる[VPC](#vpc)同士の接続や、[VPC](#vpc)とPrivateLinkに対応したサービスとの接続などに用いられる（[インタフェースエンドポイント](#インタフェースエンドポイント)）。それぞれのサービスに[ENI](#eni)を追加するため、[グローバルIPアドレス](../../../_/chapters/address_on_network.md#グローバルipアドレス)ではなく[プライベートIPアドレス](../../../_/chapters/address_on_network.md#プライベートipアドレス)を用いて接続が可能。


## CloudFront

**AWS CloudFront**は、静的なコンテンツをキャッシュしておき、低[レイテンシ](../../../../system/_/chapters/system_performance_evaluation.md#レイテンシ)でユーザにコンテンツを配信する[CDN](../../../../system/_/chapters/system_architecture.md#cdn)サービス。世界中にエッジサーバが存在しており、あらゆる場所からのアクセスに対しても対応できる。また、オリジンサービスの負荷を軽減にもつながる。


## Route 53

**Amazon Route 53**は、[ドメイン](../../../_/chapters/internet_layer.md#ドメイン名)の登録・管理や[ルーティング](../../../_/chapters/internet_layer.md#ルーティング)を行う、フルマネージドな権威[DNS](../../../_/chapters/internet_layer.md#dns)サービス。

### ホストゾーン

**ホストゾーン**は、[Route 53](#route-53)における[レコード](../../../_/chapters/internet_layer.md#aレコード)情報（[ドメイン名](../../../_/chapters/internet_layer.md#ドメイン名)と[IPアドレス](../../../_/chapters/address_on_network.md#ipアドレス)の対応）を管理するコンテナ。

### Aliasレコード

**Aliasレコード**は、[Route 53](#route-53)特有のレコードで、[CloudFront](#cloudfront)や[ELB](#elb)、[S3](./storage.md#s3)などの[AWS](./aws.md#aws)リソース[FQDN](../../../_/chapters/internet_layer.md#fqdn)、Zone Apex（最上位ドメイン）を指定できる。

### ルーティングポリシー

**ルーティングポリシー**は、[ゾーン](#ホストゾーン)情報の名前解決の問い合わせに対する応答の方針。[Route 53](#route-53)では次のようなポリシーを指定することができる。

- **シンプルルーティングポリシー**: 特別なルーティングポリシーを使わない1対1の[ルーティング](../../../_/chapters/internet_layer.md#ルーティング)。
- **フェイルオーバルーティングポリシー**: アクティブ/スタンバイ方式で、アクティブ側への[ヘルスチェック](../../../../system/_/chapters/reliability_design.md#ヘルスチェック)が失敗したときにスタンバイ側の[システム](../../../../system/_/chapters/system.md#システム)へ[ルーティング](../../../_/chapters/internet_layer.md#ルーティング)する。本番[システム](../../../../system/_/chapters/system.md#システム)障害時にSorryサーバ（Webサイトやサービスが停止していることを知らせる[サーバ](../../../../computer/_/chapters/computer.md#サーバ)）の[IPアドレス](../../../_/chapters/address_on_network.md#ipアドレス)をセカンダリレコードとして登録しておくなどの使い方。
- **位置情報ルーティングポリシー**: ユーザの位置情報に基づいてトラフィックを[ルーティング](../../../_/chapters/internet_layer.md#ルーティング)する。
- **地理的近接性ルーティングポリシー**: リソースの場所に基づいてトラフィックを[ルーティング](../../../_/chapters/internet_layer.md#ルーティング)する。
- **レイテンシルーティングポリシー**: 遅延が最も少ない[サーバ](../../../../computer/_/chapters/computer.md#サーバ)に[リクエスト](../../../../system/_/chapters/system_processing_model.md#リクエスト)を[ルーティング](../../../_/chapters/internet_layer.md#ルーティング)する。
- **複数値回答ルーティングポリシー**: 1つのレコードに異なる[IPアドレス](../../../_/chapters/address_on_network.md#ipアドレス)を複数登録して、ランダムに返却された[IPアドレス](../../../_/chapters/address_on_network.md#ipアドレス)に接続する。[ヘルスチェック](../../../../system/_/chapters/reliability_design.md#ヘルスチェック)がNGになった[サーバ](../../../../computer/_/chapters/computer.md#サーバ)の[IPアドレス](../../../_/chapters/address_on_network.md#ipアドレス)は返却されない。
- **加重ルーティングポリシー**: 指定した比率で複数のリソースにトラフィックを[ルーティング](../../../_/chapters/internet_layer.md#ルーティング)する。


## ELB

**ELB**(**Elastic Load Balancing**)は、[AWS](./aws.md#aws)で提供されているロードバランササービス。

### ALB

**ALB**(**Application Load Balancer**)は、[L7レイヤ](../../../_/chapters/network_architecture.md#アプリケーション層)での負荷分散を行うロードバランサ。[IPアドレス](../../../_/chapters/address_on_network.md#ipアドレス)、[EC2インスタンス](./computing.md#ec2)、[Lambda関数](./computing.md#lambda)などをターゲットとして指定することができ、[プロトコル](../../../_/chapters/network_architecture.md#プロトコル)としては[HTTP](../../../_/chapters/application_layer.md#http)やHTTPS、gRPCをサポートしている。[Web](../../../_/chapters/web.md#web)サービスに最適なロードバランサ。

### NLB

**NLB**(**Network Load Balancer**)は、[L4レイヤ](../../../_/chapters/network_architecture.md#トランスポート層)での負荷分散を行うロードバランサ。[IPアドレス](../../../_/chapters/address_on_network.md#ipアドレス)、[EC2インスタンス](./computing.md#ec2)、[ALB](#alb)などをターゲットとして指定することができ、[プロトコル](../../../_/chapters/network_architecture.md#プロトコル)としては[TCP](../../../_/chapters/transport_layer.md#tcp)や[UDP](../../../_/chapters/transport_layer.md#udp)、TLSをサポートしている。[HTTP](../../../_/chapters/application_layer.md#http)[プロトコル](../../../_/chapters/network_architecture.md#プロトコル)以外を扱いたい場合に最適なロードバランサ。

### CLB

**CLB**(**Classic Load Balancer**)は、[L4](../../../_/chapters/network_architecture.md#トランスポート層)-[L7](../../../_/chapters/network_architecture.md#アプリケーション層)レイヤでの負荷分散を行うロードバランサ。クラシックな[EC2インスタンス](./computing.md#ec2)を対象としているため、新しいサービスを構築する場合は、用途に合わせて[ALB](#alb)か[NLB](#nlb)を用いることが推奨されている。

### GWLB

**GWLB**(**Gateway Load Balancer**)は、[VPC](#vpc)の[Gateway Load Balancerエンドポイント](#gateway-load-balancerエンドポイント)に用いられるロードバランサ。[VPC](#vpc)でサードパーティのセキュリティ製品を利用したい場合などに最適。
