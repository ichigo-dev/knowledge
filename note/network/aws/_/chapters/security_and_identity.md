# 『セキュリティとアイデンティティ』ノート

（最終更新： 2023-05-07）


## 目次

1. [IAM](#iam)
	1. [IAMユーザ](#iamユーザ)
	1. [IAMポリシー](#iamポリシー)
	1. [IAMユーザグループ](#iamユーザグループ)
	1. [IAMロール](#iamロール)
1. [KMS](#kms)
	1. [エンベロープ暗号化](#エンベロープ暗号化)
1. [ACM](#acm)
1. [Cognito](#cognito)


## IAM

**IAM**(**AWS Identity and Access Management**)は、[AWS](./aws.md#aws)サービスを利用する際の権限管理を行うサービス。

### IAMユーザ

**IAMユーザ**は、[AWS](./aws.md#aws)リソースの操作をするユーザで、[AWS](./aws.md#aws)のマネジメントコンソールにログインしたり、[AWS](./aws.md#aws)の様々なサービスにアクセスするために用いる。

### IAMポリシー

**IAMポリシー**は、[AWS](./aws.md#aws)サービスにアクセスするための権限で、実行できるアクションやリソース、条件を制御することができる。[IAMユーザ](#iamユーザ)を作成しただけでは、[AWS](./aws.md)サービスに対するアクセス権限が何もないため、[AWS](./aws.md#aws)サービスを利用することができない。IAMポリシーを作成し、[IAMユーザ](#iamユーザ)に付与することで、与えられた権限の範囲内で[AWS](./aws.md#aws)サービスを利用することができる。また、[IAMユーザ](#iamユーザ)だけでなく、[IAMユーザグループ](#iamユーザグループ)や[IAMロール](#iamロール)に対してもIAMポリシーを付与することができる。

### IAMユーザグループ

**IAMユーザグループ**は、[IAMユーザ](#iamユーザ)をグループ化して権限管理を行う機能。共通の権限を与えたい[IAMユーザ](#iamユーザ)に対してまとめて[IAMポリシー](#iamポリシー)を適用することができる。

### IAMロール

**IAMロール**は、[AWS](./aws.md#aws)リソース自体に付与する権限。あるリソースに対して、サービス、リソース、アクションを指定して操作権限を付与することができる。


## KMS

**KMS**(**AWS Key Management Service**)は、データの暗号化/復号化をするためのキーをセキュアに管理できるサービス。鍵管理機能を持つ他、[IAM](#iam)による権限管理といった[AWS](./aws.md#aws)サービスとの連携が容易という利点もある。KMSは、[エンベロープ暗号化](#エンベロープ暗号化)を採用している。

### エンベロープ暗号化

**エンベロープ暗号化**は、データを暗号化するためのキーと、そのキーをさらに暗号化するような2層構造の暗号化方式。


## ACM

**ACM**(AWS Certification Manager)は、[インターネット](../../../_/chapters/network.md#インターネット)における通信をSSL/TLS[プロトコル](../../../_/chapters/network_architecture.md#プロトコル)により暗号化するための証明書の発行や管理を行うサービス。[AWS](./aws.md#aws)自身が認証局となって、ドメイン証明書を発行し、RSAやSHA（暗号化方式）のSSL/TLSサーバ証明書の作成・管理を行うことができる。証明書の有効期限は13か月で、自動で更新するように設定することもできる。


## Cognito

**Amazon Cognito**は、Webアプリケーションおよびモバイルアプリに素早く簡単にユーザーのサインアップ/サインイン、アクセスコントロールの機能を追加できるサービス。ユーザプールを保持し、ユーザのサインアップとサインインのためのインタフェースを提供する。
