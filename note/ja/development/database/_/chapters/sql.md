# 『SQL』

（最終更新： 2023-02-11）


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

**SQL**は、リレーショナルデータベースを操作するための言語。各RDBMSごとに多少違いはあるものの、基本的には標準SQLの仕様に準拠している場合が多い。

### DDL

**DDL**(Data Definition Language)は、データベースに格納されるデータ構造（スキーマ）を定義するためのSQL命令。テーブルの生成や削除、カラムの変更などを行うことができる。

### DML

**DML**(Data Manipulation Language)は、DDLによってあらかじめ定義したデータ構造を操作するためのSQL命令。データの抽出や挿入、変更、削除を行うことができる。

### DCL

**DCL**(Data Control Language)は、複数のユーザにより同時にデータ要求を行っても矛盾が生じないようにするための、保全機能やデータ機密保護機能を提供するためのSQL命令。

### クエリ

**クエリ**（問い合わせ）は、データベースに対するデータの抽出や更新といった処理要求の文字列のこと。

### サブクエリ

**サブクエリ**は、SQLの命令文中で呼び出す別の命令文のこと。


## データベース操作

### データベースの一覧表示

```sql
SHOW DATABASES;
```

### データベースの作成

```sql
CREATE DATABASE [name];
```

### データベースの削除

```sql
DROP DATABASE [name];
```

### データベースの選択

```sql
USE [name];
```


## テーブル操作

### テーブルの一覧表示

```sql
SHOW TABLES;
```

### テーブルの作成

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

```sql
DROP TABLE [name];
```


## データ操作

### データの取得

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

```sql
UPDATE
    [table]
SET
    [column1] = [value1],
    [column2] = [value2],
    ...;
```

### データの削除

```sql
DELETE FROM [table];
```

### データの抽出

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

```sql
SELECT
    [column] AS [alias]
FROM
    [table]
```

### データのソート

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

**内部結合**は、結合するテーブルのデータのうち、条件に指定したカラムの値が一致するデータのみを結合する。

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

**外部結合**は、結合するテーブルのデータのうち、片方のテーブルのデータをすべて取り出し、条件に指定したカラムの値が一致するデータに関しては結合される。

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

**交差結合**（**直積結合**）は、結合するテーブルのデータのすべての組み合わせを取り出す。

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

ビューはテーブルと同じ扱いのため、テーブルの一覧表示と同様。

```sql
SHOW TABLES;
```

### ビューの作成

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

```sql
DROP VIEW [name];
```


## トランザクション

### トランザクションの開始

```sql
START TRANSACTION;
```

### コミット

```sql
COMMIT;
```

### ロールバック

```sql
ROLLBACK;
```

### セーブポイント

```sql
SAVEPOINT [name];

-- ロールバック時にセーブポイントを復元
ROLLBACK TO SAVEPOINT [name];
```


## ユーザ操作

### ユーザの作成

```sql
CREATE USER [name]@[host] IDENTIFIED BY [password];
```

### 権限の付与

```sql
GRANT [type] ON [database].[table] TO [user]@[host];

-- exampleデータベースに全権限を与える例
GRANT ALL ON example.* TO my_user@localhost;
```
