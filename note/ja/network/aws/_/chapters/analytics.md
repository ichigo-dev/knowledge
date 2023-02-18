# 『分析』

（最終更新： 2023-02-18）


## 目次

1. [EMR](#emr)
1. [Kinesis](#kinesis)
1. [Data Pipeline](#data-pipeline)
1. [Glue](#glue)


## EMR

**EMR**(**Amazon Elastic MapReduce**)は、Apache HadoopやApache Sparkなどのオープンソースツールを利用した、ビッグデータの分析が可能なAWSのサービス。マスタノード、コアノード、タスクノードから構成される分散処理基盤からなり、マスタノードによってジョブを振り分け、コアノードとタスクノードでジョブを実行する。コアノードは、データを保持する**HDFS**(Hadoop Distributed File System)という領域を持つ。


## Kinesis

**Amazon Kinesis**は、AWSが提供するストリーミング処理プラットフォーム。センサやログなどのデータを、リアルタイム/準リアルタイムで処理するData StreamsとData Firehose、動画を処理するVideo Streams、収集したデータを可視化・分析するData Analyticsといった機能がある。


## Data Pipeline

**AWS Data Pipeline**は、データ処理やデータ移動を支援するサービス。パイプラインを設定すると、オンプレミスやAWS上の特定の場所に定期的にアクセスし、必要に応じてデータを変換してS3、RDS、DynamoDBなどのAWSの各種サービスに転送することができる。


## Glue

**AWS Glue**は、データレイクやデータウェアハウスとセットで使われることが多い、サーバレス型のELTツール。ビッグデータの解析などに使われることが多く、S3のデータを管理してRedshiftなどに変換して格納するといった用途によく利用される。

データソースのデータを探索するクローラ機能と、それをメタデータとして管理するデータカタログ機能がある。また、データの変換はジョブという単位で管理され、変換処理はPythonやSparkによって自分で実装することができる。
