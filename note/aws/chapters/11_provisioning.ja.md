# プロビジョニングサービス


## 目次

1. [Elastic Beanstalk](#elastic-beanstalk)
	1. [定番の構成](#定番の構成)
	1. [Elastic Beanstalkの利用方法](#elastic-beanstalkの利用方法)
1. [OpsWorks](#opsworks)
	1. [OpsWorksスタック](#opsworksスタック)
	1. [OpsWorks for Chef Automate](#opsworks-for-chef-automate)
1. [CloudFormation](#cloudformation)
	1. [CloudFormationスタック](#cloudformationスタック)
	1. [CloudFormationテンプレート](#cloudformationテンプレート)


## Elastic Beanstalk

**AWS Elastic Beanstalk**はインフラ構成を自動構築するサービス。アーキテクチャのパターンは限られるが、AWSのベストプラクティスに沿った構成を簡単に構築できる。

また、アプリケーションデプロイのサポート機能もある。

### 定番の構成

**Webサーバ構成**（ELB + Auto Scaling + EC2）、**Batchワーカ構成**（SQS + Auto Scaling + EC2）は定番の構成である。プラットフォームとしてJavaやRuby、PHP、Pythonといった言語を選択することができ、最初からサンプルアプリケーションをデプロイすることもできる。

### Elastic Beanstalkの利用方法

Elastic Beanstalkはマネジメントコンソールから利用することができるが、**AWS Toolkit for Eclipse**や**AWS CLI**、各言語のSDKを使って環境を構築することも可能。

Elastic Beanstalk専用の**EB CLI**というクライアントツールも提供されている。


## OpsWorks

**AWS OpsWorks**は**Chef**を利用した構成管理サービス。Chefは**レシピ**と呼ばれるファイルにサーバの構成を定義し、ミドルウェアのインストールや設定を自動化するツール。**Chef Clientローカル方式**では、各サーバがレシピを持ち、自分自身にレシピを適用する。**Chef Server/Client方式**では、マスタサーバを用意し、レシピそのものや各サーバへの適用状況を管理する。

### OpsWorksスタック

Chef Clientローカル方式でChefを利用する場合は、**OpsWorksスタック**を利用する。

**スタック**はOpsWorksのトップエンティティで、スタック単位の構成情報をJSON形式で保持する。スタックの下に複数のレイヤを定義できる。

**レイヤ**はインスタンスの特性ごとに用意され、Chefレシピをマッピングする対象となる。レイヤを指定してEC2インスタンスを起動することで、インスタンスに対してレシピが適用される。

レイヤ内で起動したEC2インスタンスには**OpsWorksエージェント**がインストールされ、このOpsWorksエージェントがOpsWorksからレシピを取得して自身のインスタンスに適用する。

### OpsWorks for Chef Automate

Chef Server/Client方式でChefを利用する場合は、**Chef Automate**という機能を用いる。Chef Automateでは、レシピが適用される各インスタンスを**クライアント**と呼び、それらとは別にクライアントを管理する**マスタサーバ**が存在するアーキテクチャを採用している。

**OpsWorks for Chef Automate**を利用すると、Chef Automateサーバが自動構築される。


## CloudFormation

**AWS CloudFormation**はAWSリソースを自動構築するサービスで、同じ環境を複数用意したり、複数の環境に同じ修正を横展開する作業を効率化することができる。構築されたAWSリソースは**スタック**と呼ばれ、スタックは**テンプレート**という設計図に基づいて構築される。

### CloudFormationスタック

CloudFormationで構築されたAWSリソースは、**CloudFormationスタック**と呼ばれる。

### CloudFormationテンプレート

**CloudFormationテンプレート**はスタックの設計図で、JSON形式かYAML形式で記述する。

テンプレートはいくつかの**セクション**にわかれている。**Resourcesセクション**では、構築するAWSリソースの設計を書く。**Parametersセクション**では、実行時に値を選択する項目を定義する（変数として定義しておくことで、テンプレートの柔軟性が増す）。**Mappingsセクション**では、実行環境によって変わる変数などをMap形式で定義できる。テンプレート内では**擬似パラメータ**と呼ばれるAWSアカウントに紐づいて事前定義されるパラメータ（ `AWS::Region` など）を用いることができる。

テンプレートを作成する際には、**組み込み関数**を利用することもできる。**Ref**関数は、AWSリソースの値や設定されたパラメータの値を取得する関数。**FindMap**関数は、Mappingセクションで定義したMap型の変数を取得する際に用いる。また、リストからインデックスを指定して値を取得する**Select**関数や、他のCloudFormationスタックの値を取得する**ImportValue**関数などの組み込み関数が用意されている。
