# 『Cassandra』ノート

（最終更新： 2023-11-02）


## 目次

1. [Cassandra](#cassandra)
1. [データモデル](#データモデル)


## Cassandra

**Cassandra**は、[NoSQL](./nosql.md#nosql)の[ワイドカラムモデル](./nosql.md#ワイドカラムモデル)の[KVS](./nosql.md#kvs)に分類されるプロダクト。[RDB](../../../_/chapters/database.md#リレーショナルデータベース)のデータモデルは構造化された厳格なものであるのに対して、Cassandraはより柔軟で流動的な[データモデル](#データモデル)を備えており、[アジャイル](../../../../management/_/chapters/development_method.md#アジャイル開発)な開発手法での利用に適している。また、他の[NoSQL](./nosql.md#nosql)とは異なり、[スキーマレス](./nosql.md#スキーマレス)ではない[データベース](../../../_/chapters/database.md#データベース)となっている。その他、Cassandraは以下のような特徴を持つ。

- 大規模でスケーラブルなアーキテクチャ: マスターレス設計を備えることで、[スケールアウト](../../../../../system/_/chapters/system_performance_evaluation.md#スケールアウト)を容易にしている
- すべてのノードがアクティブ: [分散](../../../../../system/_/chapters/system_processing_model.md#分散処理)して配置されたすべてのノードが、書き込みと読み取りの両方の対象となる
- 線形スケールするパフォーマンス: ノードの数に応じて線形にパフォーマンスを強化することができる（1つのノードで200,000/秒トランザクションが扱えるとすると、2つのノードでは400,000/秒トランザクション、4つのノードでは800,000/秒トランザクションが処理できる）
- 高い可用性: Cassandraは高い[冗長性](../../../../../system/_/chapters/system_architecture.md#冗長化)を備え、[単一障害点](../../../../../system/_/chapters/system_architecture.md#単一障害点)を持たない
- 障害検知とリカバリ: 障害のあるノードをかんたんにリストアしたり置き換えたりできる
- 強力なデータ保護: コミットログを利用した設計により、トランザクションの整合性を保証する
- トランザクションのサポート: [強整合性](../../../../../system/_/chapters/system_processing_model.md#acid特性)あるいは[結果整合性](../../../../../system/_/chapters/system_processing_model.md#acid特性)でのトランザクションをサポートする
