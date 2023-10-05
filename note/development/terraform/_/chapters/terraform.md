# 『Terraform』ノート

（最終更新： 2023-10-05）


## 目次

1. [IaC](#iac)
1. [Terraform](#terraform)
1. [Terraformのインストール](#terraformのインストール)


## IaC

**IaC**(Infrastructure as Code)は、[システム](../../../../system/_/chapters/system.md#システム)のインフラストラクチャをコードとして管理し、プロビジョニング（構築）、設定、[デプロイメント](../../../management/_/chapters/deployment_strategy.md#デプロイ)を自動化するためのツール。代表的なIaCツールとしては、[Terraform](#terraform)などがある。

このようなツールを利用することで、インフラストラクチャの設定を[バージョン管理](../../../git/_/chapters/git.md#バージョン管理システム)することができる、再利用性を高められる、一貫性や正確性を保てる、といったメリットがある。


## Terraform

**Terraform**は、[AWS](../../../../network/aws/_/chapters/aws.md#aws)やGCP、Azureなどの様々なクラウドプロバイダに対応した[オープンソース](../../../../computer/software/_/chapters/open_source_software.md#オープンソースソフトウェア)の[IaC](#iac)ツール。Terraformでは、Terraformが提供する[API](../../../../network/_/chapters/web.md#web-api)を通じてインフラリソースを作成・管理する。


## Terraformのインストール

[Terraform](#terraform)が既に自身の環境にインストールされているかを確認するには、[ターミナル](../../../../computer/linux/_/chapters/shell_and_terminal.md#ターミナル)[ソフトウェア](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)（Macのターミナル、Windowsのコマンドプロンプト等）を起動し、次の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sh
$ terraterm --version
Terraform v1.5.2
on linux_amd64
+ provider registry.terraform.io/hashicorp/archive v2.4.0
+ provider registry.terraform.io/hashicorp/aws v5.6.2
+ provider registry.terraform.io/hashicorp/null v3.2.1
+ provider registry.terraform.io/hashicorp/tls v4.0.4

Your version of Terraform is out of date! The latest version
is 1.5.7. You can update by downloading from https://www.terraform.io/downloads.html
```

上の例のように、[Terraform](#terraform)の[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)情報が表示されれば、[Terraform](#terraform)は既に使用できる状態となっている。[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)が存在しない旨のエラーが表示される場合は、[Terraform](#terraform)をインストールする必要がある。

- [Terraformのインストール](https://developer.hashicorp.com/terraform/downloads)

上記サイトより、自身の環境の[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)に応じたインストール手順を実施し、改めて `terraform --version` を実行して[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)情報が出力されることを確認する。
