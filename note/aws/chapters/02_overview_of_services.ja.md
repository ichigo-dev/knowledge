# サービス概要

（最終更新： 2023/1/1）


## 目次

1. [コンピューティングサービス](#コンピューティングサービス)
1. [運用支援サービス](#運用支援サービス)
1. [ストレージサービス](#ストレージサービス)
1. [データベースサービス](#データベースサービス)
1. [セキュリティとアイデンティティ](#セキュリティとアイデンティティ)
	1. [AWSアカウントの種類](#awsアカウントの種類)
	1. [AWSアカウント](#awsアカウント)
	1. [IAMポリシー](#iamポリシー)
	1. [IAMユーザとIAMグループ](#iamユーザとiamグループ)
	1. [IAMロール](#iamロール)
1. [アプリケーションサービス](#アプリケーションサービス)
1. [デバロッパツール](#デバロッパツール)
	1. [Codeシリーズ](#codeシリーズ)


## コンピューティングサービス

**コンピューティングサービス**は、アプリケーションを稼働させるインフラストラクチャサービスで、システムアーキテクチャの中核を担う。

[EC2](./04_computing.ja.md#ec2)は仮想サーバを提供するコンピューティングサービスで、いわゆる[IaaS（Infrastructure as a Service）](../../internet/chapters/01_basic_knowledge_of_network.ja.md#クラウドの利用)型のサービス。[Elastic Load Balancing（ELB）](./04_computing.ja.md#elb)や[Auto Scaling](./04_computing.ja.md#auto-scaling)と組み合わせることで、[負荷分散](./01_basic_knowledge_of_aws.ja.md#負荷分散)を動的に行うことができる。

[ECS](./04_computing.ja.md#ecs)はDockerコンテナの実行環境を提供するサービスである。

[Lambda](./04_computing.ja.md#lambda)はサーバを用意しなくてもプログラムを実行できる環境を提供するサービスである。**サーバレスアーキテクチャ**の中心ともいえるサービスで、拡張性やコスト効率の面でメリットがある。サーバのセットアップやメンテナンスの必要がないため、アプリケーションの開発に集中できる。


## 運用支援サービス

[コンピューティングサービス](#コンピューティングサービス)は[AWS](./01_basic_knowledge_of_aws.ja.md#aws)上にシステムを構築する上で中核となるサービスだが、[AWS](./01_basic_knowledge_of_aws.ja.md#aws)にはシステムが世の中に公開されてからの運用フェーズを支援するサービスも存在する。

[Amazon CloudWatch](./05_management_and_governance.ja.md#cloudwatch)は[AWS](./01_basic_knowledge_of_aws.ja.md#aws)リソースの状態を定期的に取得し、問題があればそれを通知するサービスである。また、独自のトリガーが発生した場合の処理を行う機能も提供されている。

[AWS CloudTrail](./05_management_and_governance.ja.md#cloudtrail)は[AWS](./01_basic_knowledge_of_aws.ja.md#aws)リソースの作成や、マネジメントコンソールへのログインなどの操作を記録するサービスである。


## ストレージサービス

機械学習やIT技術の革新に伴い、データ量の増加やデータの取り扱い方法の多様化が進んでおり、ストレージサービスの需要が増している。ストレージサービスは大きく分けてブロックストレージ、ファイルストレージ、オブジェクトストレージに分類できる。

**ブロックストレージ**はデータを物理的なディスクでブロック単位で管理するストレージ。記憶領域をボリュームと呼ばれる単位に分割し、ブロックという単位で呼び出す。データベースや仮想サーバのイメージ保存領域のように、頻繁に更新されたり高速なアクセスが必要となる場合に使われる。[Amazon EBS](./06_storage.ja.md#ebs)がこれにあたる。

**ファイルストレージ**はブロックストレージ上にファイルシステムを構成して、データをファイル単位で管理するストレージ。データをファイルとフォルダ（ディレクトリ）といった階層で管理する。データ共有やデータの保存といった目的で使用される。[Amazon EFS](./06_storage.ja.md#efs)や[FSx](./06_storage.ja.md#fsx)がこれにあたる。

**オブジェクトストレージ**はファイルに任意のメタデータを追加してオブジェクトとして管理するストレージ。ファイルストレージとは違いフラットな構造で管理されるため、拡張性が高い。ファイルの内容をストレージ内で直接操作することはできず、作成済みのデータに対する、[HTTP（HTTPS）](../../internet/chapters/09_application_layer.ja.md#http)経由の追加・削除・参照といった操作ができる。更新頻度の少ないデータや大容量のマルチメディアコンテンツを保存する用途で使われる。[Amazon S3](./06_storage.ja.md#s3)や[Amazon S3 Glacier](./06_storage.ja.md#s3-glacier)がこれにあたる。

|                      | ブロックストレージ                 | ファイルストレージ             | オブジェクトストレージ                     |
| -------------------- | ---------------------------------- | ------------------------------ | ------------------------------------------ |
| 管理単位             | ブロック                           | ファイル                       | オブジェクト                               |
| データライフサイクル | 追加・更新・削除                   | 追加・更新・削除               | 追加・削除                                 |
| プロトコル           | SATA、SCSI、FC                     | CIFS、NFS                      | HTTP（HTTPS）                              |
| メタデータ           | 固定情報のみ                       | 固定情報のみ                   | カスタマイズ可能                           |
| ユースケース         | データベース、トランザクションログ | ファイル共有、データアーカイブ | マルチメディアコンテンツ、データアーカイブ |


## データベースサービス

システムの構成要素としてなくてはならないデータベースは、大きく分けてRDBとNoSQLというアーキテクチャに分類できる。

**RDB**（Relational Database）ではデータを表（テーブル）形式で表現し、各テーブルの関係を定義・関連付けすることでデータを管理する。RDBの操作には**SQL**（Structured Query Language）を使用する。[Amazon RDS](./07_database.ja.md#rds)と[Amazon Redshift](./07_database.ja.md#redshift)がこれにあたる。一般的なRDBMSとしてはOracle、Microsoft SQL Server、MySQL、PostgreSQLなどが挙げられる。

**NoSQL**（Not Only SQL）はSQLを使わないデータベースアーキテクチャの総称。様々なデータモデルが存在し、柔軟でスキーマレスなデータモデル、水平スケーラビリティ、分散アーキテクチャ、高速な処理、などそれぞれに特徴がある。[Amazon DynamoDB](./07_database.ja.md#dynamodb)、[Amazon ElastiCache](./07_database.ja.md#elasticache)、[Amazon Neptune](./07_database.ja.md#amazon-neptune)、[Amazon DocumentDB](./07_database.ja.md#amazon-documentdb)、[Amazon Keyspaces](./07_database.ja.md#amazon-keyspaces)などがこれにあたる。主なソフトウェアとしてRedis、Memcached（Key-Valueストア）、Cassandra、HBase（カラム指向データベース）、MongoDB、CouchDB（ドキュメント指向データベース）、Neo4j、Titan（グラフ指向データベース）などが挙げられる。


## セキュリティとアイデンティティ

### AWSアカウントの種類

**AWSアカウント**は[AWS](./01_basic_knowledge_of_aws.ja.md#aws)へサインアップするときに作成されるアカウントで、**ルートユーザ**とも呼ばれる。

**IAMユーザ**は[AWS](./01_basic_knowledge_of_aws.ja.md#aws)を利用する各利用者向けのアカウントであり、AWSアカウントでログインしてIAMユーザを作成する必要がある。

複数のAWSアカウントを管理するための**AWS Organizations**（組織アカウント）もあり、[AWS](./01_basic_knowledge_of_aws.ja.md#aws)サービスの請求を一括決済することが可能となっている。また、各アウントごとに利用できるサービスを制限する**サービスコントロールポリシー**（**SCP**）も利用できる。

### AWSアカウント

[AWSアカウント](#awsアカウントの種類)は[ルートユーザ](#awsアカウントの種類)とも呼ばれ、[AWS](./01_basic_knowledge_of_aws.ja.md#aws)の全サービスに対してネットワーク上のどこからでも操作できる権限を持つ。できる限り[ルートユーザ](#awsアカウントの種類)は使わず、[IAMユーザ](#awsアカウントの種類)を利用する方がよい。また、[AWSアカウント](#awsアカウントの種類)は**多要素認証**（MFA: Multi-Factor Authentication）により[セキュリティ](./01_basic_knowledge_of_aws.ja.md#rasとrasis)を強固にしておくようにするとよい。

### IAMポリシー

**IAMポリシー**は**Action**（どのサービスの）、**Resource**（どういう機能や範囲を）、**Effect**（許可/拒否）という3つのルールに基づいて様々な権限を設定する。[IAM](./08_security_and_identity.ja.md#iam)ではユーザやグループ、ロールに付与する権限をオブジェクトとして管理することが可能で、これを**ポリシー**と呼ぶ。

**インラインポリシー**は対象ごとに作成・付与するポリシーで複数のユーザ・グループに同種の権限を付与するには向いていない。

**管理ポリシー**は1つのポリシーを複数のユーザやグループに適用できる。**AWS管理ポリシー**は[AWS](./01_basic_knowledge_of_aws.ja.md#aws)側が用意しているポリシーで、管理者権限やPowerUser、サービスごとのポリシーなどがある。**カスタマー管理ポリシー**はユーザ自身が管理するポリシーで、インラインポリシーと同じように設定できる。

### IAMユーザとIAMグループ

[ルートユーザ](#awsアカウントの種類)は[IAMユーザ](#awsアカウントの種類)やグループに対して操作を制限することができる。[IAMユーザ](#awsアカウントの種類)の管理は[セキュリティ](./01_basic_knowledge_of_aws.ja.md#rasとrasis)の要になるため、注意して権限を適切に制御する。

**ユーザ**とは[AWS](./01_basic_knowledge_of_aws.ja.md#aws)を利用するために各利用者に1つずつ与えられる認証情報（ID）である。ユーザIDとパスワードを用いた認証は、Webコンソールにログインするときに使用する。アクセスキーとシークレットアクセスキーを用いた認証は、CLIやAPIから[AWS](./01_basic_knowledge_of_aws.ja.md#aws)のリソースにアクセスする場合に使用する。

**グループ**とは同じ権限を持ったユーザの集まりのこと。グループはグループ内のユーザの権限を管理することができる。

### IAMロール

**IAMロール**は永続的な権限を保持するユーザとは異なり、一時的に[AWS](./01_basic_knowledge_of_aws.ja.md#aws)リソースへのアクセス権限を付与する場合に使用する。

- [EC2](./04_computing.ja.md#ec2)インスタンス上で稼働するアプリケーションに一時的に[AWS](./01_basic_knowledge_of_aws.ja.md#aws)のリソースへアクセスする権限を与えたい場合
- 複数の[AWSアカウント](#awsアカウントの種類)間のリソースを1つの[IAMユーザ](#iamユーザとiamグループ)で操作したい場合（**クロスアカウントアクセス**）
- 社内のAD（Active Directory）サーバに登録されているアカウントを使用して[AWS](./01_basic_knowledge_of_aws.ja.md#aws)リソースにアクセスしたい場合（**IDフェデレーション**）
- FacebookやGoogleアカウントを使用して[AWS](./01_basic_knowledge_of_aws.ja.md#aws)リソースにアクセスしたい場合（**Web ID フェデレーション**）

「アプリケーションに対して[SES](./09_application_integration.ja.md#ses)への一時的なアクセス権限を[EC2](./04_computing.ja.md#ec2)インスタンスから取得し、[SES](./09_application_integration.ja.md#ses)からメールを送信する」といったパターンがよく用いられている。


## アプリケーションサービス

[AWS](./01_basic_knowledge_of_aws.ja.md#aws)は[EC2](./04_computing.ja.md#ec2)や[RDS](./07_database.ja.md#rds)、[VPC](./03_networking_and_content_delivery.ja.md#vpc)といった[IaaS](../../internet/chapters/01_basic_knowledge_of_network.ja.md#クラウドの利用)や[PaaS](../../internet/chapters/01_basic_knowledge_of_network.ja.md#クラウドの利用)のみならず、[SaaS](../../internet/chapters/01_basic_knowledge_of_network.ja.md#クラウドの利用)と呼ばれるようなアプリケーションサービスも展開している。[AWS](./01_basic_knowledge_of_aws.ja.md#aws)のアプリケーションサービスは、[AWS](./01_basic_knowledge_of_aws.ja.md#aws)が提供するサーバリソースの上に構築されており、サーバとアプリケーションのメンテナンスは[AWS](./01_basic_knowledge_of_aws.ja.md#aws)が行う。


## デバロッパツール

アプリケーションは通常、リリース後も継続的な不具合調整や機能追加などを行っていく。そのためソースコードをビルドしたり、ユニットテストを走らせたりといった開発プロセスの自動化が重要となる。このような考え方は**継続インテグレーション**（**CI**）や**継続的デリバリー**（**CD**）と呼ばれる。[AWS](./01_basic_knowledge_of_aws.ja.md#aws)では[Codeシリーズ](#codeシリーズ)という**CI/CD環境**をマネージドサービスとして提供している。

### Codeシリーズ

- [CodeCommit](./10_developer_tools.ja.md#codecommit)はソースコードを管理するGitリポジトリサービス。
- [CodeBuild](./10_developer_tools.ja.md#codebuild)はソースコードのビルド/テスティングサービス。
- [CodeDeploy](./10_developer_tools.ja.md#codedeploy)はビルドされたモジュール（アーティファクト）のデプロイサービス。
- [CodePipeline](./10_developer_tools.ja.md#codepipeline)は上記3つのサービスを束ねて、一連の開発プロセスを自動化するサービス。
- **CodeStar**は上記4つのサービスを利用したCI/CD環境を自動構築するサービス。
