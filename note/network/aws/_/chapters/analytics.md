# 『分析』ノート

（最終更新： 2023-05-06）


## 目次

1. [EMR](#emr)
1. [Kinesis](#kinesis)
1. [Data Pipeline](#data-pipeline)
1. [Glue](#glue)


## EMR

**EMR**(**Amazon Elastic MapReduce**)は、Apache HadoopやApache Sparkなどの[オープンソースソフトウェア](../../../../computer/software/_/chapters/open_source_software.md#オープンソースソフトウェア)を利用した、ビッグデータの分析が可能な[AWS](./aws.md#aws)のサービス。マスタノード、コアノード、タスクノードから構成される[分散処理基盤](../../../../system/_/chapters/system_processing_model.md#分散処理)からなり、マスタノードによってジョブを振り分け、コアノードとタスクノードでジョブを実行する。コアノードは、データを保持する**HDFS**(Hadoop Distributed File System)という領域を持つ。


## Kinesis

**Amazon Kinesis**は、[AWS](./aws.md#aws)が提供する[ストリーミング](../../../../computer/software/_/chapters/multimedia.md#ストリーミング)処理[プラットフォーム](../../../../computer/software/_/chapters/software.md#プラットフォーム)。センサやログなどのデータを、リアルタイム/準リアルタイムで処理するData StreamsとData Firehose、動画を処理するVideo Streams、収集したデータを可視化・分析するData Analyticsといった機能がある。


## Data Pipeline

**AWS Data Pipeline**は、データ処理やデータ移動を支援するサービス。パイプラインを設定すると、[オンプレミス](../../../../system/_/chapters/system_architecture.md#オンプレミス)や[AWS](./aws.md#aws)上の特定の場所に定期的にアクセスし、必要に応じてデータを変換して[S3](./storage.md#s3)、[RDS](./database.md#rds)、[DynamoDB](./database.md#dynamodb)などの[AWS](./aws.md#aws)の各種サービスに転送することができる。


## Glue

**AWS Glue**は、データレイクやデータウェアハウスとセットで使われることが多い、サーバレス型のELTツール。ビッグデータの解析などに使われることが多く、[S3](./storage.md#s3)のデータを管理して[Redshift](./database.md#redshift)などに変換して格納するといった用途によく利用される。

データソースのデータを探索するクローラ機能と、それをメタデータとして管理するデータカタログ機能がある。また、データの変換はジョブという単位で管理され、変換処理はPythonやSparkによって自分で実装することができる。
