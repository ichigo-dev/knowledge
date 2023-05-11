# 『デバロッパツール』ノート

（最終更新： 2023-05-07）


## 目次

1. [CodeCommit](#codecommit)
1. [CodeBuild](#codebuild)
1. [CodeDeploy](#codedeploy)
1. [CodePipeline](#codepipeline)


## CodeCommit

**CodeCommit**は、[Git](../../../../development/git/_/chapters/git.md#git)[リポジトリ](../../../../development/git/_/chapters/create_repository.md#リポジトリ)を提供するマネージドサービス。[IAM](./security_and_identity.md#iam)ユーザを利用して権限管理を行うことができたり、他の[AWS](./aws.md#aws)サービスとシームレスに連携できたり、[プルリクエスト](../../../../development/git/_/chapters/branch.md#プルリクエスト)機能があるといった特徴がある。


## CodeBuild

**CodeBuild**は、[ソースコード](../../../../programming/_/chapters/programming.md#ソースコード)の[コンパイル](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイラ)/ビルド環境を提供するマネージドサービス。ビルドの定義はbuildspec.ymlに記載する。

ビルド環境のランタイムとして、[Java](../../../../programming/_/chapters/programming_language.md#java)や[PHP](../../../../programming/_/chapters/programming_language.md#php)、[Python](../../../../programming/_/chapters/programming_language.md#python)、Ruby、Node.jsなどを標準でサポートしており、個人で用意した[Docker](../../../../development/docker/_/chapters/docker.md#docker)[イメージ](../../../../development/docker/_/chapters/image.md#イメージ)を利用することもできる。

また、料金が従量課金型である点もひとつの特徴である。


## CodeDeploy

**CodeDeploy**は、ビルド済みの[モジュール](../../../../computer/software/_/chapters/package.md#モジュール)（アーティファクト）を[サーバ](../../../../computer/_/chapters/computer.md#サーバ)へデプロイする工程を自動化するサービス。また、新しい[モジュール](../../../../computer/software/_/chapters/package.md#モジュール)に不具合が見つかったという場合に備えて、ロールバックの機能もある。

デプロイの定義はappspec.ymlに定義する。また、デプロイ方式についても任意に決定することができる。


## CodePipeline

**CodePipeline**は、[ソースコード](../../../../programming/_/chapters/programming.md#ソースコード)が変更されてから検証環境にデプロイするまでの流れを自動化するサービス。また、リリースの承認プロセスをパイプラインの途中に挟むことも可能で、チーム開発の権限管理にも対応している。
