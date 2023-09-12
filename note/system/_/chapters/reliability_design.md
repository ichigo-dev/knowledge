# 『信頼性設計』ノート

（最終更新： 2023-04-15）


## 目次

1. [フォールトトレランス](#フォールトトレランス)
1. [フォールトアボイダンス](#フォールトアボイダンス)
1. [フェールセーフ](#フェールセーフ)
1. [フェールソフト](#フェールソフト)
1. [フォールトマスキング](#フォールトマスキング)
1. [フールプルーフ](#フールプルーフ)
1. [フェールオーバ](#フェールオーバ)
	1. [ヘルスチェック](#ヘルスチェック)


## フォールトトレランス

**フォールトトレランス**（**耐障害性**: Fault tolerance）は、[システム](./system.md#システム)の一部で障害が発生しても全体でカバーして機能停止を防ぐ設計手法。[単一障害点](./system_architecture.md#単一障害点)を排除することで、耐障害性を向上させることができる。復旧後に復旧前よりも[システム](./system.md#システム)が使いづらくなるようであれば、耐障害性はないと言える。


## フォールトアボイダンス

**フォールトアボイダンス**（**障害回避**: Fault avoidance）は、個々の機能の障害が起こる確率を下げて、全体として[信頼性](./system_performance_evaluation.md#信頼性)を上げるという考え方。


## フェールセーフ

**フェールセーフ**は、[システム](./system.md#システム)に障害が発生したとき、安全な方に制御する方法。障害が新たな障害を生まない制御をしたり、場合によっては処理を停止させる。


## フェールソフト

**フェールソフト**は、[システム](./system.md#システム)に障害が発生したとき、障害が起こった部分を切り離すなどして最低限の[システム](./system.md#システム)の稼働を続ける方法。このとき、機能を限定的にして稼働を続ける操作を**フォールバック**（縮退運転）という。


## フォールトマスキング

**フォールトマスキング**は、機器などに障害が発生した時、その影響が外部に出ないようにする方法。[システム](./system.md#システム)の[冗長化](./system_architecture.md#冗長化)などによって、1台が故障しても全体に影響が出ないようにするなど。


## フールプルーフ

**フールプルーフ**は、利用者が誤った操作を行っても危険な状態にならないようにする設計手法。押せてはいけないボタンを押せないようにしたり、危険な操作に対しては確認を行うなど。


## フェールオーバ

**フェールオーバ**は、障害発生時に運用系の[システム](./system.md#システム)から待機系の[システム](./system.md#システム)に自動的に切り替える機能や設計。[サーバ](./system_processing_model.md#クライアントサーバシステム)をフェールオーバにするには、**仮想IPアドレス**(**VIP**: Virtual IP Address)を用いて障害発生時にVIPの引き継ぎを行う。

### ヘルスチェック

**ヘルスチェック**は、[フェールオーバ](#フェールオーバ)の実現や[システム](./system.md#システム)の監視のために、運用系の[システム](./system.md#システム)に異常が発生していないかを定期的にチェックする仕組みのこと。[IP](../../../network/_/chapters/internet_layer.md#ip)のレイヤでは[ICMP](../../../network/_/chapters/internet_layer.md#icmp)監視、[TCP](../../../network/_/chapters/transport_layer.md#tcp)のレイヤでは[ポート](../../../network/_/chapters/address_on_network.md#ポート番号)監視、[アプリケーション](../../../computer/software/_/chapters/software.md#アプリケーション)のレイヤでは[HTTP](../../../network/_/chapters/application_layer.md#http)[リクエスト](./system_processing_model.md#クライアントサーバシステム)などによるサービス監視が行われる。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)