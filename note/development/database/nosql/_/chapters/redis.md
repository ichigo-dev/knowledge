# 『Redis』ノート

（最終更新： 2023-10-26）


## 目次

1. [Redis](#redis)
1. [データ型](#データ型)
1. [永続化](#永続化)
	1. [Redis Database](#redis-database)
	1. [Append Only File](#append-only-file)


## Redis

**Redis**は、[NoSQL](./nosql.md#nosql)の[KVS](./nosql.md#kvs)に分類されるプロダクト。基本的にはデータは[メモリ](../../../../../computer/hardware/_/chapters/memory.md#メモリ)上で扱われるが、オプションにより[ディスク](../../../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)に永続化することも可能。[データベース](../../../_/chapters/database.md#データベース)としてだけでなく、[アプリケーション](../../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の[キャッシュ](../../../../../network/_/chapters/web.md#キャッシュ)や、Pub-Subモデルによるメッセージブローカなどにも利用される。


## データ型

Redisで扱うことができるデータ型は以下の通り。

| データ型          | 概要                                                                                       |
| ----------------- | ------------------------------------------------------------------------------------------ |
| Redis Strings     | 文字列、数値、バイナリデータなどを格納可能なデータ型（最大512MB）                          |
| Redis Lists       | 順序を持つStringsのリストで、重複を許し、左端と右端のどちらかから追加/削除が可能なデータ型 |
| Redis Sets        | 順序を持たないStringsのリストで、重複を許さないデータ型                                    |
| Redis Sorted Sets | Setsの各要素に順序付けのスコアを付与したデータ型                                           |
| Redis Hashes      | 複数のフィールドとバリューのマップを扱うデータ型                                           |
| Bitmap            | 0か1のbitの集合を扱うデータ型                                                              |
| HyperLogLog       | 大量のデータ中のユニークな要素数を高速に推定するためのデータ型                             |

Redis Listsは、[リンクリスト](../../../../../programming/_/chapters/data_type.md#リスト)による実装となっており、先頭か末尾の要素へのアクセスでは高速であるが、中間の要素へのアクセスには時間がかかる。

Redis Setsは、追加や削除の他に存在チェックの機能があり、[集合](../../../../../basics/discrete_mathematics/_/chapters/set_and_proposition.md#集合)の共通部分の抽出や併合といった集合演算を高速に実行できる。

Redis Sorted Setsは、スコアによって順序付けされることから、要素が[集合](../../../../../basics/discrete_mathematics/_/chapters/set_and_proposition.md#集合)のどこに位置していたとしても、通常のSetsに比べてアクセスを高速にすることができる。要素の取得をスコアや順序指定で行うことも可能。このような特性から、Redis Hashesなどの[インデックス](../../../_/chapters/index.md#インデックス)としても利用される。

Bitmapは、特別なデータ型ではないが、Stringsを[ビット](../../../../../basics/_/chapters/computer_and_number.md#ビット)列として扱うための[論理演算](../../../../../basics/discrete_mathematics/_/chapters/logical_operation.md)コマンドが用意されている。


## 永続化

### Redis Database

**Redis Database**(**RDB**)は、[Redis](#redis)のデータを永続化する方法のひとつで、[Redis](#redis)の状態をスナップショットとして書き出す。書き出すタイミングは、キーの追加・更新回数の閾値とそのチェック間隔により指定する。例えば、300秒ごとにキーに100回以上の変更があったことを確認する場合は、設定ファイルに `save 300 100` のように記述する。

### Append Only File

**Append Only File**(**AOF**)は、[Redis](#redis)のデータを永続化する方法のひとつで、[サーバ](../../../../../computer/_/chapters/computer.md#サーバ)に対して発行されたコマンドを記録した[ファイル](../../../../../computer/software/_/chapters/file_system.md#ファイル)。

AOFへの書き込みには[Linux](../../../../../computer/linux/_/chapters/linux.md#linux)の[システムコール](../../../../../computer/software/_/chapters/operating_system.md#システムコール)である `fsync` が用いられている。書き込みのタイミングには3つのポリシーがあり、デフォルトでは2つ目のポリシーに設定されている。

1. `fsync` を行わない: リスク回避よりも速度が求められる場合に有効
1. 毎秒 `fsync` する: 耐久性と速度のトレードオフから推奨される設定
1. クエリが発行される度に `fsync` する: クエリ数が少ない場合に有効であるが、クエリ数が多いと非常に遅くなる

AOFは、[RDB](#redis-database)と比べて柔軟な変更が可能な一方で、ファイルサイズが大きくなってしまうという欠点もある。
