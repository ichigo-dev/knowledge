# 『CLI』ノート

（最終更新： 2023-10-06）


## 目次

1. [初期化](#初期化)
1. [計画](#計画)
1. [適用](#適用)
1. [破棄](#破棄)


## 初期化

**terrafrom init**は、[Terraform](./terraform.md#terraform)環境で用いるプロバイダ情報を取得してセットアップする[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。


## 計画

**terraform plan**は、[Terraform](./terraform.md#terraform)の[ルートモジュール](./configuration_language.md#ルートモジュール)を読み込んで、実行後にどのようなリソースの作成や削除が発生するかを表示する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。


## 適用

**terraform apply**は、[Terraform](./terraform.md#terraform)の[ルートモジュール](./configuration_language.md#ルートモジュール)を読み込んで、リソースの作成や削除を実行する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。


## 破棄

**terraform destory**は、[Terraform](./terraform.md#terraform)の[ルートモジュール](./configuration_language.md#ルートモジュール)を読み込んで、生成されたリソースを破棄する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。
