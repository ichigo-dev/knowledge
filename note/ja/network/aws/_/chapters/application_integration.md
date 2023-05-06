# 『アプリケーション統合』ノート

（最終更新： 2023-05-06）


## 目次

1. [MQ](#mq)
1. [SQS](#sqs)
1. [SNS](#sns)
1. [SES](#ses)


## MQ

**MQ**(**Amazon Messaging Queue**)は、[AWS](./aws.md#aws)の代表的なメッセージングキューイングサービス。[システム](../../../../system/_/chapters/system.md#システム)間でデータをやり取りする際に、一時的にデータを溜め込む仕組みを提供することで、[システム](../../../../system/_/chapters/system.md#システム)の非同期化をサポートする。Apache ActiveMQおよびRabbitMQとよばれる、[オープンソース](../../../../computer/software/_/chapters/open_source_software.md#オープンソースソフトウェア)のメッセージキューイングソフトをベースに構築されている。MQの[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)は、インスタンスタイプによって異なる。


## SQS

**SQS**(**Amazon Simple Queue Service**)は、[AWS](./aws.md#aws)が独自に開発したメッセージキューイングサービス。シンプルな[API](../../../../computer/software/_/chapters/operating_system.md#api)を実行することでデータをため込んでおくことができ、[DynamoDB](./database.md#dynamodb)などの他の[AWS](./aws.md#aws)のサービスとの連携が容易であるという利点がある。SQSは[AWS](./aws.md#aws)によって自動的にスケーリングされるため、ほぼ無制限の[スループット](../../../../system/_/chapters/system_performance_evaluation.md#スループット)が提供される。


## SNS

**SNS**(**Amazon Simple Notification Service**)は、サーバレスな通知システムの構築を可能にするサービス。ユーザや[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の何らかのアクションをトリガーに、[Lambda](./computing.md#lambda)の起動やメッセージ通知、SMSなどによるモバイル通知が行える。


## SES

**SES**(**Amazon Simple Email Service**)は、独自ドメインも利用可能な、[AWS](./aws.md#aws)のメール配信サービス。他の[AWS](./aws.md#aws)サービスとの連携も容易。
