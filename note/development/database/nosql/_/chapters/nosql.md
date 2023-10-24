# 『NoSQL』ノート

（最終更新： 2023-10-24）


## 目次

1. [NoSQL](#nosql)
	1. [BASE特性](#base特性)
	1. [スキーマレス](#スキーマレス)
1. [CAP定理](#capの定理)
1. [KVS](#kvs)
1. [ドキュメントDB](#ドキュメントdb)
1. [グラフDB](#グラフdb)


## NoSQL

**NoSQL**(Not only SQL)は、[RDB](../../../_/chapters/database.md#リレーショナルデータベース)のように汎用的ではないものの、特定のデータモデルに対して高いパフォーマンスを発揮する[データベース](../../../_/chapters/database.md#データベース)。[リレーショナルデータベース](../../../_/chapters/database.md#リレーショナルデータベース)は[ACID特性](../../../../../system/_/chapters/system_processing_model.md#acid特性)に基づいて作られているが、NoSQLは[BASE特性](#base特性)に基づいて作られている。

[RDB](../../../_/chapters/database.md#リレーショナルデータベース)はその性質上、[スケールアウト](../../../../../system/_/chapters/system_performance_evaluation.md#スケールアウト)が難しいため、高速で大量データを扱うことには向いていない。そのため、「高速(Velocity)」で「大量(Volume)」の「[リレーショナルモデル](../../../_/chapters/rdb.md#リレーショナルモデル)でない(Variety)」データを扱う必要がある場合にはNoSQLの活用が検討される。ただし、NoSQLは定義の幅が広く、ものによっては大量データが扱えなかったり、半構造データを扱えないものもあるので、必ずしも全てが要件を満たしているとは限らない。また、NoSQLは[スキーマレス](#スキーマレス)に使用できるものが多い。

NoSQLには、大きく分けると[KVS](#kvs)、[ドキュメントDB](#ドキュメントdb)、[グラフDB](#グラフdb)がある。

### BASE特性

**BASE特性**(Basically Available, Soft-State, Eventual Consistency)は、[ビッグデータ](../../../../../artificial_intelligence/_/chapters/artificial_intelligence.md#ビッグデータ)の扱いに適した[データベース](../../../_/chapters/database.md#データベース)処理において重要となる3つの性質を合わせた用語。[ビッグデータ](../../../../../artificial_intelligence/_/chapters/artificial_intelligence.md#ビッグデータ)を扱う[システム](../../../../../system/_/chapters/system.md#システム)においては、[ACID特性](../../../../../system/_/chapters/system_processing_model.md#acid特性)を保持することが困難となるため、BASE特性を満たした[NoSQL](#nosql)が採用される場合が多い。

- **Basically Available**: 基本的にいつでも利用できるという性質。[ACID特性](../../../../../system/_/chapters/system_processing_model.md#acid特性)を満たした[データベース](../../../_/chapters/database.md#データベース)では、[トランザクション](../../../_/chapters/transaction.md#トランザクション)の独立性が求められるため、他の処理が介入できないようにデータに対して[ロック](../../../_/chapters/transaction.md#ロック)がかけられる。一方で、BASE特性を満たした[データベース](../../../_/chapters/database.md#データベース)では、[ロック](../../../_/chapters/transaction.md#ロック)を獲得しない、つまりいつでもデータにアクセスできることが保証されている。
- **Soft-State**: 常に一貫性・整合性を保っている必要はないという性質。[ACID特性](../../../../../system/_/chapters/system_processing_model.md#acid特性)を満たした[データベース](../../../_/chapters/database.md#データベース)では、[強整合性](../../../../../system/_/chapters/system_processing_model.md#acid特性)が保証されており、常に最新のデータが取得できることが約束されている。一方で、BASE特性を満たした[データベース](../../../_/chapters/database.md#データベース)では、大量のデータを[分散](../../../../../system/_/chapters/system_processing_model.md#分散処理)して扱うことを想定しているため、それらのデータを同期するまでの間、一時的に整合性が保たれていない可能性がある。
- **Eventual Consistency**: 最終的には一貫性・整合性が保証されるという性質。BASE特性は[結果整合性](../../../../../system/_/chapters/system_processing_model.md#acid特性)さえ保証できていれば良いという思想のため、Soft-Stateの性質にあるように一時的に整合性が保たれていない状態があっても、最終的には整合される。

### スキーマレス

**スキーマレス**は、[RDB](../../../_/chapters/database.md#リレーショナルデータベース)とは異なり、データの構造を事前に厳密に定義する必要がない特徴を指す用語。[NoSQL](#nosql)はスキーマレスな[データベース](../../../_/chapters/database.md#データベース)が多いが、スキーマがないことにより管理が難しくなるというデメリットもあるため、スキーマ定義の機能をサポートしているものもある。


## CAP定理

**CAP定理**(Consistency, Availability, Partition-tolerance)は、[分散システム](../../../../../system/_/chapters/system_processing_model.md#分散システム)において、一貫性・可用性・分断耐性のうち2つの性質は同時に保証できるが、3つは同時に保証できない、という定理。

- **一貫性**(Consistency): 全てのノードで同時に同じデータを確認することができるという性質
- **可用性**(Availability): どのノードで障害が発生しても、処理の継続性が失われない性質（[単一障害点](../../../../../system/_/chapters/system_architecture.md#単一障害点)がない）
- **分断耐性**(Partition-tolerance): 通信障害が発生して2つ以上のノード群が疎通を取れなくなっても、正常に動作する性質

CAを満たす[データベース](../../../_/chapters/database.md#データベース)は、全てのノードで常に通信を行って、最新のデータを更新し続けることで、一貫性と可用性を保つ。しかし、ネットワーク分断が発生すると、最新のデータが更新できなくなるノードが発生するため、分断耐性を満たすことはできない。[RDB](../../../_/chapters/database.md#リレーショナルデータベース)([OLTP](../../../_/chapters/database.md#oltp))がこれに該当する。

CPを満たす[データベース](../../../_/chapters/database.md#データベース)は、各ノードが通信しながら最新のデータを更新し、ネットワーク分断が発生した際には過半数より多くのノードと通信できる集団をクラスタとして運用することで、一貫性と分断耐性を保つ。しかし、このような場合には分断された小さなクラスが利用できなくなるため、可用性が損なわれる。RedisやMongoDB、HBaseなどがこれに該当する。

APを満たす[データベース](../../../_/chapters/database.md#データベース)は、各ノードが通信しながら最新のデータを更新し、ネットワーク分断が発生した際には各ノードはその時点で最新のデータを利用して処理を続けることで、可用性と分断耐性を保つ。しかし、それぞれのノードが持つ情報が異なる状態となるため、一貫性は保たれない。Cassandraや[Amazon DynamoDB](../../../../../network/aws/_/chapters/database.md#dynamodb)などがこれに該当するほか、[Git](../../../../git/_/chapters/git.md#git)もこのような性質を持つ。


## KVS

**KVS**(Key-Value Store)は、[NoSQL](#nosql)のひとつで、キーに対して何かしらの値が保存されており、それぞれのデータが疎結合で[分散](../../../../../system/_/chapters/system_processing_model.md#分散処理)して格納することができるという特徴をもった[データベース](../../../_/chapters/database.md#データベース)。キーに対してひとつの値を格納するキーバリューモデルと、キーに対して複数の[カラム](../../../_/chapters/rdb.md#カラム)を格納するワイドカラムモデルがある。[スケールアウト](../../../../../system/_/chapters/system_performance_evaluation.md#スケールアウト)が容易なため、大量データに対する[クエリ](../../../_/chapters/sql.md#クエリ)に高速に応答することができる。KVSの製品としては、CassandraやRedis、HBaseなどがある。


## ドキュメントDB

**ドキュメントDB**は、[NoSQL](#nosql)のひとつで、[KVS](#kvs)において格納する値をJSONとして、そのJSONを操作するのに特化した機能を持った[データベース](../../../_/chapters/database.md#データベース)。JSONの中身に対して検索を書けたり、検索を高速化するための[インデックス](../../../_/chapters/index.md#インデックス)を貼ったりといったことが可能となる。JSONは様々な[プログラミング言語](../../../../../programming/_/chapters/programming.md#プログラミング言語)で扱いやすい他、[Web API](../../../../../network/_/chapters/web.md#web-api)のデータ形式として標準的であるなど、開発生産性を高めるのに役立つ。ドキュメントDBの製品としては、MongoDBやCouchbase、CouchDBなどがある。


## グラフDB

**グラフDB**は、[NoSQL](#nosql)のひとつで、[RDB](../../../_/chapters/database.md#リレーショナルデータベース)では表現が困難なデータ同士のつながりを表現し、複雑な[結合](../../../_/chapters/join.md#結合)が必要となるような[クエリ](../../../_/chapters/sql.md#クエリ)を高速に実行できる[データベース](../../../_/chapters/database.md#データベース)。[RDB](../../../_/chapters/database.md#リレーショナルデータベース)よりもデータ間の関連性が強いため、[分散処理](../../../../../system/_/chapters/system_processing_model.md#分散処理)には不向き。グラフDBの製品としては、Neo4jやOrientDBなどがある。
