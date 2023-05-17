# 『SQL』ノート

（最終更新： 2023-05-17）


## 目次

1. [SQL](#sql)
	1. [DDL](#ddl)
	1. [DML](#dml)
	1. [DCL](#dcl)
	1. [クエリ](#クエリ)
	1. [サブクエリ](#サブクエリ)
1. [データベース操作](#データベース操作)
	1. [データベースの一覧表示](#データベースの一覧表示)
	1. [データベースの作成](#データベースの作成)
	1. [データベースの削除](#データベースの削除)
	1. [データベースの選択](#データベースの選択)
1. [テーブル操作](#テーブル操作)
	1. [テーブルの一覧表示](#テーブルの一覧表示)
	1. [テーブルの作成](#テーブルの作成)
	1. [テーブルの変更](#テーブルの変更)
	1. [テーブルの削除](#テーブルの削除)
1. [データ操作](#データ操作)
	1. [データの取得](#データの取得)
	1. [データの挿入](#データの挿入)
	1. [データの更新](#データの更新)
	1. [データの削除](#データの削除)
	1. [データの抽出](#データの抽出)
	1. [エイリアスの設定](#エイリアスの設定)
	1. [データのソート](#データのソート)
	1. [データの制限](#データの制限)
	1. [データの集約](#データの集約)
	1. [データの加工](#データの加工)
	1. [データのグループ化](#データのグループ化)
1. [テーブルの結合](#テーブルの結合)
	1. [内部結合](#内部結合)
	1. [外部結合](#外部結合)
	1. [交差結合](#交差結合)
1. [条件分岐](#条件分岐)
1. [ビュー操作](#ビュー操作)
	1. [ビューの一覧表示](#ビューの一覧表示)
	1. [ビューの作成](#ビューの作成)
	1. [ビューの変更](#ビューの変更)
	1. [ビューの削除](#ビューの削除)
1. [トランザクション](#トランザクション)
	1. [トランザクションの開始](#トランザクションの開始)
	1. [コミット](#コミット)
	1. [ロールバック](#ロールバック)
	1. [セーブポイント](#セーブポイント)
1. [ユーザ操作](#ユーザ操作)
	1. [ユーザの作成](#ユーザの作成)
	1. [権限の付与](#権限の付与)


## SQL

**SQL**は、[リレーショナルデータベース](./database.md#リレーショナルデータベース)を操作するための言語。各[RDBMS](./database.md#rdbms)ごとに多少違いはあるものの、基本的には標準SQLの仕様に準拠している場合が多い。

### DDL

**DDL**(Data Definition Language)は、[データベース](./database.md#データベース)に格納されるデータ構造（[スキーマ](./database.md#スキーマ)）を定義するための[SQL](#sql)命令。[テーブル](./rdb.md#テーブル)の生成や削除、[カラム](./rdb.md#カラム)の変更などを行うことができる。

### DML

**DML**(Data Manipulation Language)は、[DDL](#ddl)によってあらかじめ定義したデータ構造を操作するための[SQL](#sql)命令。データの抽出や挿入、変更、削除を行うことができる。

### DCL

**DCL**(Data Control Language)は、複数のユーザにより同時にデータ要求を行っても矛盾が生じないようにするための、保全機能やデータ機密保護機能を提供するための[SQL](#sql)命令。

### クエリ

**クエリ**（問合せ）は、[データベース](./database.md#データベース)に対するデータの抽出や更新といった処理要求の文字列。

### サブクエリ

**サブクエリ**（副問合せ）は、[SQL](#sql)の命令文中で呼び出す別の命令文。


## データベース操作

### データベースの一覧表示

[インスタンス](./rdb.md#インスタンス)に存在する[データベース](./database.md#データベース)の一覧を表示するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
SHOW DATABASES;
```

### データベースの作成

[インスタンス](./rdb.md#インスタンス)に新しい[データベース](./database.md#データベース)を作成するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
CREATE DATABASE [name];
```

### データベースの削除

[インスタンス](./rdb.md#インスタンス)に存在する[データベース](./database.md#データベース)を削除するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
DROP DATABASE [name];
```

### データベースの選択

操作したい[データベース](./database.md#データベース)を選択するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
USE [name];
```


## テーブル操作

### テーブルの一覧表示

[データベース](./database.md#データベース)中の[テーブル](./rdb.md#テーブル)の一覧を表示するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
SHOW TABLES;
```

### テーブルの作成

[データベース](./database.md#データベース)中に新しい[テーブル](./rdb.md#テーブル)を作成するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
CREATE TABLE [name]
(
    [column1] [type] [restriction],
    [column2] [type] [restriction],
    ...
    PRIMARY KEY ([column])
);
```

```sql
-- テーブル作成の例
CREATE TABLE `user`
(
    `user_id` INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(64) NOT NULL,
    `address` VARCHAR(128) NOT NULL,
    PRIMARY KEY (`user_id`)
);
```

### テーブルの変更

[データベース](./database.md#データベース)中の[テーブル](./rdb.md#テーブル)の定義を変更するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
-- カラムの追加
ALTER TABLE [name] ADD COLUMN [column] [type] [restriction];
ALTER TABLE [name] ADD
(
    [column1] [type] [restriction],
    [column2] [type] [restriction],
    ...
);

-- カラムの変更
ALTER TABLE [name] MODIFY COLUMN [column] [type] [restriction];
ALTER TABLE [name] MODIFY
(
    [column1] [type] [restriction],
    [column2] [type] [restriction],
    ...
);

-- カラムの削除
ALTER TABLE [name] DROP COLUMN [column];
ALTER TABLE [name] DROP ([column1], [column2], ...);
```

### テーブルの削除

[データベース](./database.md#データベース)中の[テーブル](./rdb.md#テーブル)を削除するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
DROP TABLE [name];
```


## データ操作

### データの取得

[テーブル](./rdb.md#テーブル)からデータを取得するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
SELECT
    [column1],
    [column2],
    ...
FROM
    [table];

-- 重複を削除した検索
SELECT DISTINCT
    [column1],
    [column2],
    ...
FROM
    [table];
```

### データの挿入

[テーブル](./rdb.md#テーブル)に新しいデータを挿入するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
INSERT INTO [table]
(
    [column1],
    [column2],
    ...
)
VALUES
(
    [value1],
    [value2],
    ...
);
```

### データの更新

[テーブル](./rdb.md#テーブル)のデータを更新するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
UPDATE
    [table]
SET
    [column1] = [value1],
    [column2] = [value2],
    ...;
```

### データの削除

[テーブル](./rdb.md#テーブル)のデータを削除するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
DELETE FROM [table];
```

### データの抽出

[テーブル](./rdb.md#テーブル)のデータを抽出して取得するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
WHERE
    [condition];

-- OR条件
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
WHERE
    [condition1] OR
    [condition2] OR
    ...;

-- AND条件
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
WHERE
    [condition1] AND
    [condition2] AND
    ...;

-- NOT条件
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
WHERE
    NOT [condition];

-- 範囲条件
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
WHERE
    [column] BETWEEN [value1] AND [value2];

-- 含有条件
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
WHERE
    [column] IN ([value1], [value2], ...);

-- ワイルドカード
-- '%': 0文字以上の任意の文字列
-- '_': 任意の1文字
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
WHERE
    [column] LIKE '%[value]%';
```

### エイリアスの設定

操作するデータに別名をつけるには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。複数のテーブルを組み合わせたクエリを記述する場合などに、長いテーブル名を繰り返し記述して冗長になることを避けるために用いられる。

```sql
SELECT
    [column] AS [alias]
FROM
    [table]
```

### データのソート

[テーブル](./rdb.md#テーブル)のデータをソートして取得するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
-- 昇順
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
ORDER BY
    [column];

-- 降順
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
ORDER BY
    [column] DESC;
```

### データの制限

[テーブル](./rdb.md#テーブル)のデータの取得数を制限するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
LIMIT
    [offset], [num];
```

### データの集約

[テーブル](./rdb.md#テーブル)のデータを集約して様々な統計データを取得するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
-- 合計値
SELECT
    SUM([column])
FROM
    [table];

-- 平均値
SELECT
    AVG([column])
FROM
    [table];

-- 最大値
SELECT
    MAX([column])
FROM
    [table];

-- 最小値
SELECT
    MIN([column])
FROM
    [table];

-- データ数
SELECT
    COUNT([column])
FROM
    [table];
```

### データの加工

[テーブル](./rdb.md#テーブル)のデータを加工するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
-- 四捨五入
SELECT
    ROUND([column], [valid digit])
FROM
    [table];

-- 切り捨て
SELECT
    FLOOR([column])
FROM
    [table];

-- 切り上げ
SELECT
    CEILING([column])
FROM
    [table];
```

### データのグループ化

[テーブル](./rdb.md#テーブル)のデータをグループ化するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
SELECT
    [column1],
    [column2],
    ...
FROM
    [table]
GROUP BY
    [column]

-- グループ化したカラムを元に集計
SELECT
    [column1],
    SUM([column2]),
    AVG([column3]),
    ...
FROM
    [table]
GROUP BY
    [column1]
```


## テーブルの結合

### 内部結合

**内部結合**は、結合する[テーブル](./rdb.md#テーブル)のデータのうち、条件に指定した[カラム](./rdb.md#カラム)の値が一致するデータのみを結合する。

```sql
SELECT
    [column1],
    [column2],
    ...
FROM
    [table1]
INNER JOIN
    [table2]
ON
    [condition];

SELECT
    [column1],
    [column2],
    ...
FROM
    [table1],
    [table2],
    ...
WHERE
    [condition]
```

### 外部結合

**外部結合**は、結合する[テーブル](./rdb.md#テーブル)のデータのうち、片方の[テーブル](./rdb.md#テーブル)のデータをすべて取り出し、条件に指定した[カラム](./rdb.md#カラム)の値が一致するデータに関してのみ結合される。

```sql
-- 左外部結合
SELECT
    [column1],
    [column2],
    ...
FROM
    [table1]
LEFT OUTER JOIN
    [table2]
ON
    [condition];

-- 右外部結合
SELECT
    [column1],
    [column2],
    ...
FROM
    [table1]
RIGHT OUTER JOIN
    [table2]
ON
    [condition];

-- 完全外部結合
SELECT
    [column1],
    [column2],
    ...
FROM
    [table1]
FULL OUTER JOIN
    [table2]
ON
    [condition];
```

### 交差結合

**交差結合**（**直積結合**）は、結合する[テーブル](./rdb.md#テーブル)のデータのすべての組み合わせを取り出す。

```sql
SELECT
    [column1],
    [column2],
    ...
FROM
    [table1]
CROSS JOIN
    [table2]
```


## 条件分岐

[クエリ](#クエリ)中で条件分岐を行いたい場合は、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
CASE
    WHEN [condition1] THEN [value1]
    WHEN [condition2] THEN [value2]
    ...
    ELSE [value]
END
```


## ビュー操作

### ビューの一覧表示

ビューは[テーブル](./rdb.md#テーブル)と同じ扱いのため、[テーブル](./rdb.md#テーブル)の一覧表示と同様。

```sql
SHOW TABLES;
```

### ビューの作成

[データベース](./database.md#データベース)中に新しいビューを作成するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
CREATE VIEW [name]
(
    [column1],
    [column2],
    ...
) AS
SELECT
    [column1],
    [column2],
    ...
FROM
    [table];
```

### ビューの変更

[データベース](./database.md#データベース)中のビューを変更するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
-- ビュー名の変更
ALTER VIEW [old name] RENAME TO [new name];

-- ビュー定義の変更
ALTER VIEW [name]
(
    [column1],
    [column2],
    ...
) AS
SELECT
    [column1],
    [column2],
    ...
FROM
    [table];
```

### ビューの削除

[データベース](./database.md#データベース)中のビューを削除するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
DROP VIEW [name];
```


## トランザクション

### トランザクションの開始

[トランザクション](./transaction.md#トランザクション)を開始するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
START TRANSACTION;
```

### コミット

[トランザクション](./transaction.md#トランザクション)中の実行結果を[コミット](./transaction.md#コミット)するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
COMMIT;
```

### ロールバック

[トランザクション](./transaction.md#トランザクション)中の実行結果を[ロールバック](./transaction.md#ロールバック)するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
ROLLBACK;
```

### セーブポイント

[トランザクション](./transaction.md#トランザクション)中に[セーブポイント](./transaction.md#セーブポイント)を設けるには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
SAVEPOINT [name];

-- ロールバック時にセーブポイントを復元
ROLLBACK TO SAVEPOINT [name];
```


## ユーザ操作

### ユーザの作成

[インスタンス](./rdb.md#インスタンス)中に新しいユーザを作成するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
CREATE USER [name]@[host] IDENTIFIED BY [password];
```

### 権限の付与

ユーザに権限を付与するには、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sql
GRANT [type] ON [database].[table] TO [user]@[host];

-- exampleデータベースに全権限を与える例
GRANT ALL ON example.* TO my_user@localhost;
```
