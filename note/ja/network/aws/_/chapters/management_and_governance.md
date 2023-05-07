# 『マネジメントとガバナンス』ノート

（最終更新： 2023-05-07）


## 目次

1. [CloudWatch](#cloudwatch)
	1. [CloudWatch Logs](#cloudwatch-logs)
1. [CloudTrail](#cloudtrail)
	1. [管理イベント](#管理イベント)
	1. [データイベント](#データイベント)


## CloudWatch

**Amazon CloudWatch**は、運用監視を支援するマネージドサービスであり、[システム](../../../../system/_/chapters/system.md#システム)の安定運用をサポートする。各[AWS](./aws.md#aws)リソースの状態（メトリクス）を定期的に取得し、既定値を超えた場合にアラートを送信することができる。[AWS](./aws.md#aws)があらかじめ定義している標準メトリクスと、利用者が独自に定義することができるカスタムメトリクスを使用できる。

### CloudWatch Logs

**CloudWatch Logs**は、アプリケーションログやApacheログなどのログをモニタリングするサービス。エージェントを介してログを収集し、アラームを設定することもできる。


## CloudTrail

**AWS CloudTrail**は、[AWS](./aws.md#aws)に関する操作ログを自動的に取得するサービス。誰が、いつ、どのような操作をしたか、といった監査ログを残すことができる。

[CloudWatch Logs](#cloudwatch-logs)と連携することで、事前に不正な操作を登録しておき、そのような操作が行われたときに通知するように設定することもできる。

### 管理イベント

**管理イベント**は、マネジメントコンソールへのログインや[EC2インスタンス](./computing.md#ec2)の作成、[S3](./storage.md#s3)[バケット](./storage.md#バケット)の作成などといったイベント。

### データイベント

**データイベント**は、[S3](./storage.md#s3)[バケット](./storage.md#バケット)上のデータの操作、[Lambda関数](./computing.md#lambda)の実行などといったイベント。[CloudTrail](#cloudtrail)において、[管理イベント](#管理イベント)の取得はデフォルトで有効であるが、データイベントの取得はデフォルトで無効となっているので注意。
