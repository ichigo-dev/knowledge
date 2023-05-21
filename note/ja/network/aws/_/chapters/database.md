# 『データベース』ノート

（最終更新： 2023-05-21）


## 目次

1. [RDS](#rds)
	1. [Aurora](#aurora)
	1. [リードレプリカ](#リードレプリカ)
	1. [バックアップとリストア](#バックアップとリストア)
	1. [RDSのセキュリティ](#rdsのセキュリティ)
	1. [マルチAZ構成](#マルチaz構成)
1. [DynamoDB](#dynamodb)
	1. [高可用性設計](#高可用性設計)
	1. [スループットキャパシティ](#スループットキャパシティ)
	1. [データパーティショニング](#データパーティショニング)
	1. [DynamoDB Streams](#dynamodb-streams)
	1. [Consistent Read](#consistent-read)
	1. [DynamoDB Accelerator](#dynamodb-accelerator)
1. [ElastiCache](#elasticache)
	1. [Memcached](#memcached)
	1. [Redis](#redis)
1. [Redshift](#redshift)
	1. [Redshift Spectrum](#redshift-spectrum)


## RDS

**RDS**(**Amazon Relational Database Service**)は、[AWS](./aws.md#aws)が提供するマネージド[RDB](../../../../development/database/_/chapters/database.md#リレーショナルデータベース)サービス。データベースエンジンは、[Aurora](#aurora)やMySQL、MariaDB、PostgreSQL、Oracle、Microsoft SQL Serverなどから選択できる。RDSのデータ保存用[ストレージ](../../../../computer/hardware/_/chapters/hardware.md#記憶装置)には、[EBS](./storage.md#ebs)を利用する。

### Aurora

**Amazon Aurora**は、[AWS](./aws.md#aws)が開発したデータベースエンジンで、[RDS](#rds)の利用時に選択することができる。DBインスタンスを作成すると同時にDBクラスタが作成される。DBクラスタは、1つ以上のDBインスタンスと、各DBインスタンスから参照するデータ[ストレージ](../../../../computer/hardware/_/chapters/hardware.md#記憶装置)（クラスタボリューム）で構成される。クラスタボリュームは単一の[リージョン](./aws.md#リージョン)内の3つの[AZ](./aws.md#az)にそれぞれ2つのデータコピーで構成され、各[ストレージ](../../../../computer/hardware/_/chapters/hardware.md#記憶装置)間のデータは自動的に同期される。

**Auroraレプリカ**は通常の[リードレプリカ](#リードレプリカ)と違い、Auroraのプライマリインスタンスに障害が発生した場合にレプリカインスタンスがプライマリインスタンスに昇格することで[フェールオーバ](../../../../system/_/chapters/reliability_design.md#フェールオーバ)を実現する。

### リードレプリカ

**リードレプリカ**は、通常の[RDS](#rds)とは別に参照専用のDBインスタンスを作成するサービス。マスタDBの負荷を抑えたり、読み込みが多い[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)におけるDBリソースの[スケールアウト](../../../../system/_/chapters/system_performance_evaluation.md#スケールアウト)を容易に実現することが可能となる。

マスタとリードレプリカのデータ同期は**非同期レプリケーション方式**であるため、参照されるタイミングによってはマスタ側で更新された情報が反映されていない可能性があることに注意が必要。

### バックアップとリストア

**自動バックアップ**は、バックアップウィンドウと保持期間を指定することで、1日1回自動的にバックアップ（DBスナップショット）を取得してくれるサービス。

**手動スナップショット**は、任意のタイミングで[RDB](../../../../development/database/_/chapters/database.md#リレーショナルデータベース)のバックアップを取得できるサービス。

データの**リストア**は、スナップショットから新規の[RDS](#rds)を作成することで簡単に実現できる。

**ポイントインタイムリカバリ**は、自動バックアップのスナップショットを利用して、任意のタイミングの状態の[RDS](#rds)を新規に作成することができるサービス。

### RDSのセキュリティ

[RDS](#rds)は[VPC](./networking_and_content_delivery.md#vpc)に対応しているため、[インターネット](../../../_/chapters/network.md#インターネット)に接続せずとも[VPC](./networking_and_content_delivery.md#vpc)内のサービスから利用できる。[セキュリティグループ](./networking_and_content_delivery.md#セキュリティグループ)による通信要件の制限も可能で、[EC2](./computing.md#ec2)やほかの[AWS](./aws.md#aws)サービスから[RDS](#rds)までの通信もデータベースエンジンが提供するSSLを使った暗号化に対応している。

また、[RDS](#rds)の**暗号化オプション**を有効にすることで、データが保存される[ストレージ](../../../../computer/hardware/_/chapters/hardware.md#記憶装置)やスナップショット、ログなどの[RDS](#rds)に関連するすべてのデータが暗号化された状態で保持される。


## DynamoDB

**Amazon DynamoDB**は、[AWS](./aws.md#aws)が提供するKey-Value型のマネージド[NoSQLデータベース](../../../../development/database/_/chapters/database.md#nosqlデータベース)サービス。[テーブル](../../../../development/database/_/chapters/rdb.md#テーブル)や[インデックス](../../../../development/database/_/chapters/index.md#インデックス)を作成する際に、読み取り・書き込みに必要な[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)を指定してリソースを確保することで、安定した性能を担保する仕組み。[トランザクション](../../../../development/database/_/chapters/transaction.md#トランザクション)機能にも対応している。

以下のようなシステムに適している。

- 高い信頼性と拡張性を必要とする[システム](../../../../system/_/chapters/system.md#システム)
- [スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)が増減するようなピーク帯のある[システム](../../../../system/_/chapters/system.md#システム)
- 大量のデータを蓄積して高速な検索が可能な[システム](../../../../system/_/chapters/system.md#システム)
- 広告やゲームなどのユーザの行動履歴を管理する[システム](../../../../system/_/chapters/system.md#システム)
- [Web](../../../_/chapters/web.md#web)[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の永続的セッションデータベース

### 高可用性設計

[DynamoDB](#dynamodb)はデータが自動的に3つの[AZ](./aws.md#az)に保存される仕組みになっており、[単一障害点](../../../../system/_/chapters/system_architecture.md#単一障害点)を持たない構成であるため、非常に可用性が高いサービスである。

### スループットキャパシティ

**Read Capacity Unit**(**RCU**)は、読み取りのスループットキャパシティを指定する指標。1RCUは、最大4KBの項目に対して、1秒当たり1回の強力な整合性のある読み取り性能、あるいは1秒当たり2回の結果的に整合性のある読み取り性能を担保することを示す。

**Write Capacity Unit**(**WCU**)は、書き込みのスループットキャパシティを指定する指標。1WCUは、最大1KBの項目に対して、1秒当たり1回の書き込み性能を担保することを示す。

[DynamoDB](#dynamodb)では、負荷の状況に応じてスループットキャパシティを自動的に増減することができる。急激な[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)の上昇に即座に対応できるわけではないため、事前にスパイクが発生することがわかっている場合は手動でキャパシティを拡張して対応する。

### データパーティショニング

DynamoDBはデータを**パーティション**という単位で分散保存する。1つのパーティションに保存できる容量や[スループットキャパシティ](#スループットキャパシティ)が決まっているため、データの増加や指定した[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)のサイズによって最適化された状態を保つように自動的にパーティションを拡張する。

### DynamoDB Streams

**DynamoDB Streams**は、[DynamoDB](#dynamodb)に対して行われた直近24時間の追加・更新・削除の変更履歴を保持する機能。

### Consistent Read

**Consistent Read**は、[DynamoDB](#dynamodb)のオプションで、有効にすると参照のリクエストがあった時点よりも前に書き込まれているデータがすべて反映された状態のデータをもとに参照結果を返す。[RCU](#スループットキャパシティ)が2倍消費される点には注意が必要。

### DynamoDB Accelerator

**DynamoDB Accelerator**(**DAX**)は、[DynamoDB](#dynamodb)の前段にキャッシュクラスタを構成する拡張サービス。


## ElastiCache

**Amazon ElastiCache**は、[AWS](./aws.md#aws)が提供するインメモリ型データベースサービス。高頻度で参照するデータや検索に時間がかかるデータセットを[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)上に保持することでパフォーマンスを向上させる。

### Memcached

**Memcached**は、KVS（Key-Valueストア）型インメモリデータベースのデファクトスタンダードとして広く利用されているエンジン。データの永続性機能はないため、メンテナンスや障害による再起動時にすべてのデータが消去される。

### Redis

**Redis**は、KVS型インメモリデータベースで、[Memcached](#memcached)よりも多くのデータが扱え、キャッシュ用としてだけではなくメッセージブローカーやキューを構成する要素としても利用される。ノード間のレプリケーション機能やデータ永続性機能といった可用性も考慮された機能が実装されている。


## Redshift

**Amazon Redshift**は、[AWS](./aws.md#aws)が提供するデータウェアハウス（データの分析に最適化された[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)）向けの[データベース](../../../../development/database/_/chapters/database.md#データベース)サービス。大量のデータから意思決定に役立つ情報を見つけ出すために必要な環境を安価で準備できる。

Redshiftは、複数ノードによる分散並列実行が大きな特徴として挙げられる。1つのRedshiftを構成する複数のノードのまとまりをRedshiftクラスタと呼び、クラスタは1つのリーダノードと複数のコンピュートノードから構成される。いかにコンピュートノードをまたがずに処理を完結させることができるのかがRedshift利用のポイントとなる。

**リーダノード**では、[SQL](../../../../development/database/_/chapters/sql.md#sql)クライアントやBI(Business Intelligence)ツールからの実行[クエリ](../../../../development/database/_/chapters/sql.md#クエリ)を受け付けて、[クエリ](../../../../development/database/_/chapters/sql.md#クエリ)の解析や実行プランの作成を行う。

**コンピュートノード**では、リーダノードからの実行[クエリ](../../../../development/database/_/chapters/sql.md#クエリ)を処理する。各コンピュートノードは[ストレージ](../../../../computer/hardware/_/chapters/hardware.md#記憶装置)とセットになっている。

**ノードスライス**は、Redshiftが分散並列処理をする最小の単位で、コンピュートノードの中でさらにリソースを分割してスライスという単位を構成する。

### Redshift Spectrum

**Redshift Spectrum**は、[S3](./storage.md#s3)に置かれたデータを外部[テーブル](../../../../development/database/_/chapters/rdb.md#テーブル)として定義できるようにし、[Redshift](#redshift)内にデータを取り込むことなく[クエリ](../../../../development/database/_/chapters/sql.md#クエリ)の実行を可能にする拡張サービス。
