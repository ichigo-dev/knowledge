# 『ストレージ』ノート

（最終更新： 2023-05-07）


## 目次

1. [EBS](#ebs)
	1. [ボリュームタイプ](#ボリュームタイプ)
	1. [バースト性能](#バースト性能)
	1. [EBSマルチアタッチ](#ebsマルチアタッチ)
1. [EFS](#efs)
	1. [パフォーマンスモード](#パフォーマンスモード)
	1. [スループットモード](#スループットモード)
1. [S3](#s3)
	1. [バケット](#バケット)
	1. [オブジェクト](#オブジェクト)
	1. [メタデータ](#メタデータ)
	1. [ストレージクラス](#ストレージクラス)
	1. [静的コンテンツホスティング機能](#静的コンテンツホスティング機能)
	1. [署名付きURL](#署名付きurl)
	1. [署名付きURL](#署名付きurl)
1. [S3 Glacier](#s3-glacier)
	1. [ボールト](#ボールト)
	1. [アーカイブ](#アーカイブ)
	1. [インベントリ](#インベントリ)
	1. [ジョブ](#ジョブ)
	1. [S3 Glacier Select](#s3-glacier-select)
	1. [ボールトロック](#ボールとロック)
1. [Storage Gateway](#storage-gateway)
	1. [ファイルゲートウェイ](#ファイルゲートウェイ)
	1. [ボリュームゲートウェイ](#ボリュームゲートウェイ)
	1. [テープゲートウェイ](#テープゲートウェイ)
1. [FSx](#fsx)
	1. [FSx for windowsファイルサーバ](#fsx-for-windowsファイルサーバ)
	1. [FSx for Lustre](#fsx-for-lustre)


## EBS

**EBS**(**Amazon Elastic Block Store**)は、[AWS](./aws.md#aws)が提供するブロックストレージサービス。[EC2](./computing.md#ec2)の[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)領域として利用したり、追加ボリュームとして複数のEBSを[EC2](./computing.md#ec2)にアタッチすることもできる。[RDS](./database.md#rds)のデータ保存用にも用いられる。

EBSは[EC2](./computing.md#ec2)と1対1で対応するサービスであり、複数の[EC2](./computing.md#ec2)にアタッチする[マルチアタッチ](#ebsマルチアタッチ)機能もあるが制約が多い。また、EBSは同じ[AZ](./aws.md#az)内にある[EC2](./computing.md#ec2)にしかアタッチすることはできず、別の[AZ](./aws.md#az)の[EC2](./computing.md#ec2)にアタッチしたい場合は、EBSのスナップショットを取得して新しいEBSボリュームを[EC2](./computing.md#ec2)と同じ[AZ](./aws.md#az)内に作る必要がある。

### ボリュームタイプ

[EBS](#ebs)には、[SSD](../../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ssd)と[HDD](../../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)それぞれにボリュームタイプの種類があり、それに加えて旧世代のHDDストレージタイプとしてマグネティックタイプがある。各タイプの性能を最大限発揮するためには、[EBS最適化インスタンスオプション](./computing.md#ebs最適化オプション)のついた[EC2](./computing.md#ec2)を利用することが推奨される。

- **汎用SSD**(gp2,gp3): [EBS](#ebs)の中で最も一般的なボリュームタイプ。性能の指標としてIOPS（1秒あたりに処理できるI/Oアクセス数）を用い、容量に応じたベースライン性能がある。容量が少ないボリュームには、一時的なIOPSの上昇に対応できるようにバースト機能が用意されている。
- **プロビジョンドIOPS SSD**(io1): [EBS](#ebs)の中で最も高性能なボリュームタイプ。[RDS](./database.md#rds)や[EC2インスタンス](./computing.md#ec2)で[データベース](../../../../development/database/_/chapters/basic_knowledge_of_database.md#データベース)サーバを構成する場合など、高いIOPS性能が求められる場合や、高い[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)が必要なユースケースに適している。
- **スループット最適化HDD**(st1): [HDD](../../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)をベースとした[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)重視のボリュームタイプ。ログデータに対する処理やバッチ処理のインプット用[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)など、大容量[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を高速に読み取るようなユースケースに適している。性能指標として[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)を用いている。
- **Cold HDD**(sc1): 性能は高くはないが最も低コストなボリュームタイプ。利用頻度が少なく、アクセス時の性能も求められないデータにシーケンシャルアクセス（端から順番にアクセスする、⇔ ランダムアクセス）するようなユースケース、あるいはアーカイブ領域の用途に適している。

### バースト性能

**バースト性能**は、一時的な処理量の増加へどれほど対応できるかを表す、ストレージサービスの性能指標のひとつ。あくまでベースライン性能を中心に設計しておき、バースト性能に頼った[システム](../../../../system/_/chapters/system.md#システム)とならないようにする。

### EBSマルチアタッチ

**EBSマルチアタッチ**は、複数の[EC2インスタンス](./computing.md#ec2)に同一の[EBS](#ebs)をアタッチできるという機能。同一の[AZ](./aws.md#az)のインスタンスからのみアタッチ可能で、書き込みの排他制御を利用者自身で検討する必要がある。


## EFS

**EFS**(**Amazon Elastic File System**)は、容量無制限で複数の[EC2インスタンス](./computing.md#ec2)から同時にアクセスできるファイルストレージサービス。クライアントからEFSへの接続は、一般的な**NFS**(Network File System)[プロトコル](../../../_/chapters/network_architecture.md#プロトコル)をサポートしている。**amazon-efs-utils**ツールを使うと、EFSへの[マウント](../../../../computer/software/_/chapters/file_system.md#マウント)に関する推奨オプションが含まれていたり、[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)にトラブルが発生した場合のトラブルシューティングに役立つログが記録できる。EFSはファイルが作成されると3か所以上の[AZ](./aws.md#az)に保存される分散[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)を構成する。

### パフォーマンスモード

[EFS](#efs)には、**汎用パフォーマンスモード**と**最大I/Oパフォーマンスモード**がある。通常は汎用パフォーマンスモード、数百～数千台といったクライアントから同時に[EFS](#efs)にアクセスするようなユースケースでは最大I/Oパフォーマンスモードを用いる。

どちらのモードを用いるかの指標として、[CloudWatch](./management_and_governance.md#cloudwatch)の**PercentIOLimit**というメトリクスが参考になる。汎用パフォーマンスモードでPercentIOLimitが長時間高い状態であれば、最大I/Oパフォーマンスモードに変更することを検討するとよい。

パフォーマンスモードは後から変更できないので注意する。

### スループットモード

[EFS](#efs)には、**バーストスループットモード**と**プロビジョニングスループットモード**がある。

バーストスループットモードは、[EFS](#efs)に保存されているデータ容量に応じてベースラインとなる[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)が設定されている。一時的な[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)の上昇にも耐えられるようなバースト機能を持ったモードとなっている。

プロビジョニングスループットモードは、バーストスループットモードで設定されているベースライン[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)やバースト時の最大[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)では性能が足りない場合に、任意の[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)値を指定することができるモード。[Web](../../../_/chapters/web.md#web)配信コンテンツやアプリケーションデータなどの頻繁なアクセスが見込まれる場合に有用。

どちらのモードを用いるかの指標として、[CloudWatch](./management_and_governance.md#cloudwatch)の**BurstCreditBalance**というメトリクスが参考になる。クレジットバランスをすべて使い切ってしまったり、常に減少傾向である場合にはプロビジョニングスループットモードを選択する。

スループットモードはあとから変更できる。


## S3

**S3**(**Amazon Simple Storage Service**)は、高い耐久性を持つ容量無制限のオブジェクトストレージサービス。S3の各[オブジェクト](#オブジェクト)には、[REST](../../../_/chapters/web.md#rest)や[SOAP](../../../_/chapters/web.md#soap)といった[HTTP](../../../_/chapters/application_layer.md#http)ベースの[Web API](../../../_/chapters/web.md#web-api)を使ってアクセスする。柔軟性に優れており、アイデア次第で様々な利用方法がある。

- データバックアップ
- ビックデータ解析用のデータレイク
- ETL(Extract/Transform/Load)の中間ファイル保存
- [Auto Scaling](./computing.md#auto-scaling)構成された[EC2インスタンス](./computing.md#ec2)やコンテナからのログ転送先
- 静的コンテンツのホスティング
- Key-Value型の[データベース](../../../../development/database/_/chapters/basic_knowledge_of_database.md#データベース)

### バケット

**バケット**は、[S3](#s3)において[オブジェクト](#オブジェクト)を保存するための領域。バケット名は[AWS](./aws.md#aws)内で一意である必要がある。

### オブジェクト

**オブジェクト**は、[S3](#s3)において保存されるデータそのもの。各オブジェクトにはキーが付与され、「バケット名+キー名+バージョンID」で必ず一意になる[URL](../../../_/chapters/web.md#url)が作成される。

### メタデータ

**メタデータ**は、[S3](#s3)において[オブジェクト](#オブジェクト)を管理するための情報。[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)で必要な情報をユーザ定義メタデータとして保持することもできる。

### ストレージクラス

[S3](#s3)には用途に応じてランク分けされたストレージクラスがある。[可用性](../../../../system/_/chapters/system_performance_evaluation.md#可用性)の性能にはSLA(Service Level Agreement)が用いられている。

- **S3標準** : デフォルトのストレージクラス。
	- 耐久性： 99.999999999%（イレブンナイン）
	- 可用性： 99.99%
- **S3標準 - 低頻度アクセス** : 格納コストが安価なストレージクラス。データの読み出し容量に対する従量課金が行われる。
	- 耐久性： 99.999999999%（イレブンナイン）
	- 可用性： 99.9%
- **S3 1ゾーン - IA** : 単一の[AZ](./aws.md#az)内のみでデータを複製するストレージクラス。
	- 耐久性： 99.999999999%（イレブンナイン）
	- 可用性： 99.5%
- **S3 Intelligent - Tiering** : 参照頻度の高低を明確に決めることができないデータを扱う場合に有効なストレージクラス。S3標準とS3標準 - 低頻度アクセスの2層構成となっている。
	- 耐久性： 99.999999999%（イレブンナイン）
	- 可用性： 99.9%
- [S3 Glacier](#s3-glacier) : ほとんど参照されないアーカイブ目的のデータを保存するストレージクラス。新規作成時に選択することはできず、ライフサイクル管理機能によって利用可能となる。データへのアクセスには事前にリクエストが必要で、アクセスできるようになるまで時間がかかる。
	- 耐久性： 99.999999999%（イレブンナイン）
	- 可用性： 99.99%
- **S3 Glacier Deep Archive** : [S3 Glacier](#s3-glacier)同様アーカイブ用のストレージクラス。さらにアクセス頻度が低いデータを想定している。
	- 耐久性： 99.999999999%（イレブンナイン）
	- 可用性： 99.99%

### 静的コンテンツホスティング機能

**静的コンテンツホスティング機能**は、[S3](#s3)を[CDN](../../../../system/_/chapters/system_architecture.md#cdn)として静的なコンテンツの配信を行う機能。

### 署名付きURL

**署名付きURL**は、アクセスを許可したい[オブジェクト](#オブジェクト)に対して期限を指定して[URL](../../../_/chapters/web.md#url)を発行する機能。[バケット](#バケット)や[オブジェクト](#オブジェクト)のアクセス制御を変更することなく特定の[オブジェクト](#オブジェクト)に一時的にアクセスを許可したい場合に有効。署名付きURLを知っている人はだれでも[オブジェクト](#オブジェクト)にアクセスできる。


## S3 Glacier

**Amazon S3 Glacier**は、[S3](#s3)と同様にイレブンナインの耐久性を持ちながらもコストを抑えた、アーカイブストレージサービス。データの取り出しには時間がかかるため、アクセス頻度が低いデータなど限られたユースケースにのみ使用可能。

### ボールト

**ボールト**は、[S3 Glacier](#s3-glacier)においてアーカイブを保存するための領域で、[S3](#s3)の[バケット](#バケット)に相当する。

### アーカイブ

**アーカイブ**は、[S3 Glacier](#s3-glacier)に格納されるデータのことで、[S3](#s3)の[オブジェクト](#オブジェクト)に相当する。

### インベントリ

**インベントリ**は、各[ボールト](#ボールト)に保存されているアーカイブの情報を収集し、1日1回の頻度で更新される。リアルタイムで状況を確認したい場合は、マネジメントコンソールで確認するか、ListVaults APIを実行する。

### ジョブ

**ジョブ**は、アーカイブやインベントリに対して検索をかけたり、データをダウンロードするといった要求に対して処理を実行し、処理の状況を管理する機能。

### S3 Glacier Select

**S3 Glacier Select**は、アーカイブデータに対して[SQL](../../../../development/database/_/chapters/sql.md#sql)を実行して、条件に合ったデータを抽出する機能。

### ボールトロック

**ボールトロック**は、[S3 Glacier](#s3-glacier)のアーカイブが削除されないように制御する機能。ボールトロックポリシーを設定することで、ユーザのアーカイブ削除を拒否する。


## Storage Gateway

**AWS Storage Gateway**は、[オンプレミス](../../../../system/_/chapters/system_architecture.md#オンプレミス)にあるデータをクラウドへ連携するための受け口を提供するためのサービス。データの保存先としては[S3](#s3)や[S3 Glacier](#s3-glacier)などの耐久性が高く低コストなストレージが利用されることが多い。

### ファイルゲートウェイ

**ファイルゲートウェイ**は、[S3](#s3)をクライアントサーバからNFS[マウント](../../../../computer/software/_/chapters/file_system.md#マウント)して、[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)のように扱うことができる[ゲートウェイ](../../../_/chapters/network_architecture.md#ゲートウェイ)。作成した[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)は、非同期（ほぼリアルタイム）で[S3](#s3)にアップロードされる。

### ボリュームゲートウェイ

**ボリュームゲートウェイ**は、各[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を[オブジェクト](#オブジェクト)として管理するのではなく、[S3](#s3)のデータ保存領域全体を1つのボリュームとして管理する。クライアントサーバからの[ゲートウェイ](../../../_/chapters/network_architecture.md#ゲートウェイ)への接続方式は、[iSCSI](../../../../computer/hardware/_/chapters/bus.md#scsi)接続になる。

- **キャッシュ型ボリューム** : 頻繁に利用するデータは[Storage Gateway](#storage-gateway)内のキャッシュディスク（[オンプレミス](../../../../system/_/chapters/system_architecture.md#オンプレミス)）に保存して、すべてのデータを保存するストレージ（プライマリストレージ）として[S3](#s3)を利用する。[オンプレミス](../../../../system/_/chapters/system_architecture.md#オンプレミス)のキャッシュボリュームに頻繁に使用するデータを、アップロードバッファボリュームに[S3](#s3)にアップロードするデータを保管しておく。
- **保管型ボリューム** : すべてのデータを保存するストレージとしてローカルストレージを利用し、データを定期的にスナップショット形式で[S3](#s3)へ転送する。[オンプレミス](../../../../system/_/chapters/system_architecture.md#オンプレミス)のデータを定期的にバックアップする用途に適している。

### テープゲートウェイ

**テープゲートウェイ**は、テープデバイスの代替として[S3](#s3)や[S3 Glacier](#s3-glacier)にデータをバックアップするタイプの[ゲートウェイ](../../../_/chapters/network_architecture.md#ゲートウェイ)。すでにバックアップにテープデバイスを利用している場合は、[Storage Gateway](#storage-gateway)への移行が可能。


## FSx

**Amazon FSx**は、フルマネージドなファイルストレージで、[Windows](../../../../computer/software/_/chapters/operating_system.md#windows)向けの[Amazon FSx for Windowsファイルサーバ](#fsx-for-windows)と、ハイパフォーマンスコンピューティング向けの[Amazon FSx for Lustre](./fsx-for-lustre)がある。

### FSx for Windowsファイルサーバ

**FSx for Windowsファイルサーバ**は、[Windows](../../../../computer/software/_/chapters/operating_system.md#windows)用のフルマネージドなファイルサーバ。[Windows](../../../../computer/software/_/chapters/operating_system.md#windows)で利用できるユーザクォータ、エンドユーザファイルの復元、Microsoft Active Directory統合などの幅広い機能が利用可能。単一のサブネットにエンドポイントとなる[ENI](./networking_and_content_delivery.md#eni)を配置し、SMB[プロトコル](../../../_/chapters/network_architecture.md#プロトコル)を介してアクセス可能。

### FSx for Lustre

**FSx for Lustre**は、フルマネージドな分散[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)で、作成時に[S3](#s3)の[バケット](#バケット)と関連付けされる。[S3](#s3)上の[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)をインデックスし、あたかも自前の[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)のように見せる。高速なデータアクセスが必要なハイパフォーマンスコンピューティングで利用され、[機械学習](../../../../artificial_intelligence/_/chapters/machine_learning.md#機械学習)やビッグデータ処理に使われる。
