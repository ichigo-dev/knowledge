# 『NoSQL』ノート

（最終更新： 2023-10-23）


## 目次

1. [NoSQL](#nosql)
	1. [スキーマレス](#スキーマレス)
	1. [KVS](#kvs)
	1. [ドキュメントDB](#ドキュメントdb)
	1. [グラフDB](#グラフdb)


## NoSQL

**NoSQL**(Not only SQL)は、[RDB](../../../_/chapters/database.md#リレーショナルデータベース)のように汎用的ではないものの、特定のデータモデルに対して高いパフォーマンスを発揮する[データベース](../../../_/chapters/database.md#データベース)。

[RDB](../../../_/chapters/database.md#リレーショナルデータベース)はその性質上、[スケールアウト](../../../../../system/_/chapters/system_performance_evaluation.md#スケールアウト)が難しいため、高速で大量データを扱うことには向いていない。そのため、「高速(Velocity)」で「大量(Volume)」の「[リレーショナルモデル](../../../_/chapters/rdb.md#リレーショナルモデル)でない(Variety)」データを扱う必要がある場合にはNoSQLの活用が検討される。ただし、NoSQLは定義の幅が広く、ものによっては大量データが扱えなかったり、半構造データを扱えないものもあるので、必ずしも全てが要件を満たしているとは限らない。

NoSQLには、大きく分けると[KVS](#kvs)、[ドキュメントDB](#ドキュメントdb)、[グラフDB](#グラフdb)がある。

### スキーマレス

**スキーマレス**は、[RDB](../../../_/chapters/database.md#リレーショナルデータベース)とは異なり、データの構造を事前に厳密に定義する必要がない特徴を指す用語。[NoSQL](#nosql)はスキーマレスな[データベース](../../../_/chapters/database.md#データベース)が多いが、スキーマがないことにより管理が難しくなるというデメリットもあるため、スキーマ定義の機能をサポートしているものもある。

### KVS

**KVS**(Key-Value Store)は、[NoSQL](#nosql)のひとつで、キーに対して何かしらの値が保存されており、それぞれのデータが疎結合で[分散](../../../../../system/_/chapters/system_processing_model.md#分散処理)して格納することができるという特徴をもった[データベース](../../../_/chapters/database.md#データベース)。キーに対してひとつの値を格納するキーバリューモデルと、キーに対して複数の[カラム](../../../_/chapters/rdb.md#カラム)を格納するワイドカラムモデルがある。[スケールアウト](../../../../../system/_/chapters/system_performance_evaluation.md#スケールアウト)が容易なため、大量データに対する[クエリ](../../../_/chapters/sql.md#クエリ)に高速に応答することができる。KVSの製品としては、CassandraやRedis、HBaseなどがある。

### ドキュメントDB

**ドキュメントDB**は、[NoSQL](#nosql)のひとつで、[KVS](#kvs)において格納する値をJSONとして、そのJSONを操作するのに特化した機能を持った[データベース](../../../_/chapters/database.md#データベース)。JSONの中身に対して検索を書けたり、検索を高速化するための[インデックス](../../../_/chapters/index.md#インデックス)を貼ったりといったことが可能となる。JSONは様々な[プログラミング言語](../../../../../programming/_/chapters/programming.md#プログラミング言語)で扱いやすい他、[Web API](../../../../../network/_/chapters/web.md#web-api)のデータ形式として標準的であるなど、開発生産性を高めるのに役立つ。ドキュメントDBの製品としては、MongoDBやCouchbase、CouchDBなどがある。

### グラフDB

**グラフDB**は、[NoSQL](#nosql)のひとつで、[RDB](../../../_/chapters/database.md#リレーショナルデータベース)では表現が困難なデータ同士のつながりを表現し、複雑な[結合](../../../_/chapters/join.md#結合)が必要となるような[クエリ](../../../_/chapters/sql.md#クエリ)を高速に実行できる[データベース](../../../_/chapters/database.md#データベース)。[RDB](../../../_/chapters/database.md#リレーショナルデータベース)よりもデータ間の関連性が強いため、[分散処理](../../../../../system/_/chapters/system_processing_model.md#分散処理)には不向き。グラフDBの製品としては、Neo4jやOrientDBなどがある。
