# 『Redis』ノート

（最終更新： 2023-10-31）


## 目次

1. [Redis](#redis)
1. [データ型](#データ型)
	1. [Strings](#strings)
	1. [Lists](#lists)
	1. [Sets](#sets)
	1. [Sorted Sets](#sorted-sets)
	1. [Hashes](#hashes)
	1. [Bitmaps](#bitmaps)
	1. [HyperLogLog](#hyperloglog)
1. [永続化](#永続化)
	1. [Redis Database](#redis-database)
	1. [Append Only File](#append-only-file)
1. [クエリ](#クエリ)
	1. [SET](#set)
	1. [GET](#get)
	1. [DEL](#del)
	1. [INCR](#incr)
	1. [MSET](#mset)
	1. [MGET](#mget)
	1. [LPUSH](#lpush)
	1. [RPUSH](#rpush)
	1. [LLEN](#llen)
	1. [LRANGE](#lrange)
	1. [LPOP](#lpop)
	1. [RPOP](#rpop)
	1. [SADD](#sadd)
	1. [SREM](#srem)
	1. [SISMEMBER](#sismember)
	1. [SMEMBERS](#smembers)
	1. [SUNION](#sunion)
	1. [ZADD](#zadd)
	1. [ZRANGE](#zrange)
	1. [HSET](#hset)
	1. [HGETALL](#hgetall)
	1. [HMSET](#hmset)
	1. [HGET](#hget)
	1. [HINCRBY](#hincrby)
	1. [HDEL](#hdel)
	1. [EXPIRE](#expire)
	1. [TTL](#ttl)
	1. [INFO](#info)
1. [トランザクション](#トランザクション)


## Redis

**Redis**は、[NoSQL](./nosql.md#nosql)の[KVS](./nosql.md#kvs)に分類されるプロダクト。基本的にはデータは[メモリ](../../../../../computer/hardware/_/chapters/memory.md#メモリ)上で扱われるが、オプションにより[ディスク](../../../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)に永続化することも可能。[データベース](../../../_/chapters/database.md#データベース)としてだけでなく、[アプリケーション](../../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の[キャッシュ](../../../../../network/_/chapters/web.md#キャッシュ)や、Pub-Subモデルによるメッセージブローカなどにも利用される。


## データ型

Redisで扱うことができるデータ型は以下の通り。

### Strings

**Strings**は、文字列、数値、バイナリデータなどを格納可能な[Redis](#redis)のデータ型。

### Lists

**Lists**は、順序を持つ[Strings](#strings)のリストで、重複を許し、左端と右端のどちらかから追加/削除が可能な[Redis](#redis)のデータ型。[リンクリスト](../../../../../programming/_/chapters/data_type.md#リスト)による実装となっており、先頭か末尾の要素へのアクセスでは高速であるが、中間の要素へのアクセスには時間がかかる。

### Sets

**Sets**は、順序を持たない[Strings](#strings)のリストで、重複を許さない[Redis](#redis)のデータ型。追加や削除の他に存在チェックの機能があり、[集合](../../../../../basics/discrete_mathematics/_/chapters/set_and_proposition.md#集合)の共通部分の抽出や併合といった集合演算を高速に実行できる。

### Sorted Sets

**Sorted Sets**は、[Sets](#sets)の各要素に順序付けのスコアを付与し、スコアによってソートされた[Redis](#redis)のデータ型。スコアによって順序付けされることから、要素が[集合](../../../../../basics/discrete_mathematics/_/chapters/set_and_proposition.md#集合)のどこに位置していたとしても、通常の[Sets](#sets)に比べてアクセスを高速にすることができる。要素の取得をスコアや順序指定で行うことも可能。このような特性から、[Hashes](#hashes)などの[インデックス](../../../_/chapters/index.md#インデックス)としても利用される。

### Hashes

**Hashes**は、複数のフィールドとバリューのマップを扱う[Redis](#redis)のデータ型。

### Bitmaps

**Bitmaps**は、0か1の[ビット](../../../../../basics/_/chapters/computer_and_number.md#ビット)の集合を扱う[Redis](#redis)のデータ型。特別なデータ型ではないが、[Strings](#strings)を[ビット](../../../../../basics/_/chapters/computer_and_number.md#ビット)列として扱うための[論理演算](../../../../../basics/discrete_mathematics/_/chapters/logical_operation.md)コマンドが用意されている。

### HyperLogLog

**HyperLogLog**は、大量のデータ中のユニークな要素数を高速に推定するための[Redis](#redis)のデータ型。


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


## クエリ

ここで紹介するサンプルは各章ごとに独立しており、データは共有されていないものとする。

### SET

**SET**は、キーとそれに対するバリューを指定して、それを[Strings](#strings)として新規に格納もしくは更新する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> SET key value
OK
```

### GET

**GET**は、キーを指定して、[Strings](#strings)のバリューを取得する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> SET key value
OK
127.0.0.1:6379> GET key
"value"
127.0.0.1:6379> GET nil_key
(nil)
```

### DEL

**DEL**は、キーを指定して、そのバリューを削除する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> SET key value
OK
127.0.0.1:6379> GET key
"value"
127.0.0.1:6379> DEL key
(integer) 1
127.0.0.1:6379> GET key
(nil)
```

### INCR

**INCR**は、キーを指定して、その[Strings](#strings)のバリューに1を加算する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> SET name Smith
OK
127.0.0.1:6379> SET age 20
OK
127.0.0.1:6379> INCR age
(integer) 21
127.0.0.1:6379> GET age
"21"
127.0.0.1:6379> INCR name
(integer) 1
127.0.0.1:6379> GET name
"1"
```

### MSET

**MSET**は、複数のキーとバリューのペアを指定して、一括で[Strings](#strings)を新規に格納もしくは更新する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> MSET name Smith age 20
OK
127.0.0.1:6379> GET name
"Smith"
```

### MGET

**MGET**は、複数のキーを指定して、一括で[Strings](#strings)のバリューを取得する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> MSET name Smith age 20
OK
127.0.0.1:6379> MGET name age
1) "Smith"
2) "20"
```

### LPUSH

**LPUSH**は、キーを指定して、[Lists](#lists)の先頭にバリューを追加する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> LPUSH fruits apple orange banana
(integer) 3
127.0.0.1:6379> LRANGE 0 3
1) "banana"
2) "orange"
3) "apple"
```

### RPUSH

**RPUSH**は、キーを指定して、[Lists](#lists)の末尾にバリューを追加する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> RPUSH fruits apple orange banana
(integer) 3
127.0.0.1:6379> LRANGE 0 3
1) "apple"
2) "orange"
3) "banana"
```

### LLEN

**LLEN**は、キーを指定して、[Lists](#lists)の長さを取得する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> LPUSH fruits apple orange banana
(integer) 3
127.0.0.1:6379> LLEN fruits
(integer) 3
```

### LRANGE

**LRANGE**は、キーと範囲（開始インデックスと終了インデックス）を指定して、[Lists](#lists)の要素を取得する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> LPUSH fruits apple orange banana
(integer) 3
127.0.0.1:6379> LRANGE 0 3
1) "banana"
2) "orange"
3) "apple"
127.0.0.1:6379> LRANGE 1 1
1) "orange"
```

### LPOP

**LPOP**は、キーを指定して、[Lists](#lists)の先頭要素を抜き出す（抜き出された要素はリストから削除される）[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> LPUSH fruits apple orange banana
(integer) 3
127.0.0.1:6379> LPOP fruits
"banana"
127.0.0.1:6379> LRANGE 0 3
1) "orange"
2) "apple"
```

### RPOP

**RPOP**は、キーを指定して、[Lists](#lists)の末尾要素を抜き出す（抜き出された要素はリストから削除される）[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> LPUSH fruits apple orange banana
(integer) 3
127.0.0.1:6379> RPOP fruits
"apple"
127.0.0.1:6379> LRANGE 0 3
1) "banana"
2) "orange"
```

### SADD

**SADD**は、キーと要素を指定して、指定した[Sets](#sets)に対して要素を追加する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> SADD fruits apple orange banana
(integer) 3
127.0.0.1:6379> SMEMBERS fruits
1) "apple"
2) "orange"
3) "banana"
127.0.0.1:6379> SADD fruits apple grape
(integer) 1
127.0.0.1:6379> SMEMBERS fruits
1) "apple"
2) "orange"
3) "banana"
4) "grape"
```

### SREM

**SREM**は、キーと要素を指定して、指定した[Sets](#sets)から要素を削除する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> SADD fruits apple orange banana
(integer) 3
127.0.0.1:6379> SMEMBERS fruits
1) "apple"
2) "orange"
3) "banana"
127.0.0.1:6379> SREM fruits apple grape
(integer) 1
127.0.0.1:6379> SMEMBERS fruits
1) "orange"
2) "banana"
```

### SISMEMBER

**SISMEMBER**は、キーと要素を指定して、指定した要素が[Sets](#sets)に存在するか確認する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> SADD fruits apple orange banana
(integer) 3
127.0.0.1:6379> SISMEMBER fruits apple
(integer) 1
127.0.0.1:6379> SISMEMBER fruits grape
(integer) 0
```

### SMEMBERS

**SMEMBERS**は、キーを指定して、指定した[Sets](#sets)の要素をすべて取得する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> SADD fruits apple orange banana
(integer) 3
127.0.0.1:6379> SMEMBERS fruits
1) "apple"
2) "orange"
3) "banana"
```

### SUNION

**SUNION**は、キーで指定した2つ以上の[Sets](#sets)を結合した結果を返す[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> SADD fruits1 apple orange banana
(integer) 3
127.0.0.1:6379> SADD fruits2 apple grape melon
(integer) 3
127.0.0.1:6379> SUNION fruits1 fruits2
1) "apple"
2) "orange"
3) "banana"
4) "grape"
5) "melon"
```

### ZADD

**SUNION**は、キーとスコア、バリューを指定して、キーに対応する[Sorted Sets](#sorted-sets)に指定したスコアとバリューのエントリを追加する[Redis](#redis)のクエリ。

```sh
$ redis-cli
127.0.0.1:6379> ZADD scores 80 Smith
(integer) 1
127.0.0.1:6379> ZADD scores 60 Johnson
(integer) 1
127.0.0.1:6379> ZADD scores 95 Williams
(integer) 1
127.0.0.1:6379> ZRANGE 0 3
1) "Johnson"
2) "Smith"
3) "Williams"
```


## トランザクション

Redisにおけるトランザクションは、複数の[クエリ](#クエリ)をまとめて実行する機能。トランザクションを使用すると、複数の[クエリ](#クエリ)をひとつの単位として実行することで、データの整合性を保つことができる。

`MULTI` によりトランザクションを開始し、 `EXEC` によりトランザクション中にキューに蓄積された[クエリ](#クエリ)をまとめて実行する。 `EXEC` 時には、誤った[クエリ](#クエリ)の呼び出しがあった場合にはトランザクションの実行を拒否し、すべての[クエリ](#クエリ)が正常であった場合にはコミットされる。ただし、トランザクション中の一部の[クエリ](#クエリ)が失敗した場合でも、他の[クエリ](#クエリ)はすべて実行される点に注意。
