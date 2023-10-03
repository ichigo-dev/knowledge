# 『GraphQL』ノート

（最終更新： 2023-10-03）


## 目次

1. [GraphQL](#graphql)
1. [スキーマ言語](#スキーマ言語)
	1. [Schema](#schema)
	1. [Type](#type)
	1. [スカラー型](#スカラー型)
	1. [オブジェクト型](#オブジェクト型)
	1. [Field](#field)
	1. [Operation Types](#operation-types)
	1. [Directives](#directives)
	1. [Resolvers](#resolvers)
1. [クエリ言語](#クエリ言語)
	1. [オペレーション型](#オペレーション型)
	1. [Query](#query)
	1. [Mutation](#mutation)
	1. [Subscription](#subscription)
	1. [Fragment](#fragment)
	1. [Argument](#argument)


## GraphQL

**GraphQL**は、[API](../../../../network/_/chapters/web.md#web-api)向けのデータ取得・操作のための[クエリ](../../../database/_/chapters/sql.md#クエリ)言語および実行環境。Facebookによって開発され、2015年に[オープンソース](../../../../computer/software/_/chapters/open_source_software.md#オープンソースソフトウェア)として公開された。[クライアント](../../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が明示的に必要なデータを要求し、それに対して[サーバ](../../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が適切なデータを返すことができるようにするための柔軟な構成となっている。

モバイル[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)や[SPA](../../../../programming/_/chapters/programming_language.md#javascript)との相性がよく、活躍の場を広げている。


## スキーマ定義言語

**スキーマ定義言語**(**SDL**: Schema Definition Language)は、[GraphQL](#graphql)において、[Schema](#schema)を記述するための言語。[GraphQL](#graphql)を介した通信は、このスキーマ言語による型定義を元にしている。

### Schema

**Schema**は、[GraphQL](#graphql)において、データの構造と[クエリ](#クエリ言語)のエンドポイントを定義するもの。クエリの構造や利用可能なデータ型、データの取得・変更方法を定義し、[GraphQL](#graphql)の通信のルールを確立する。

### Type

**Type**は、[GraphQL](#graphql)において、[Schema](#schema)で定義されるデータ型。Typeには、[スカラー型](#スカラー型)と[オブジェクト型](#オブジェクト型)がある。

以下は、Typeを定義する[Schema](#schema)の例。

```graphql
type Person
{
    name: String!
    age: Int
    favoirtes: [String]
}
```

### スカラー型

**スカラー型**は、[GraphQL](#graphql)の[Type](#type)のひとつで、文字列や数値、ブール値などの[プリミティブ](../../../../programming/_/chapters/data_type.md#プリミティブ型)なデータを表す型。以下の型が定義されている。

| 型名      | 概要                                                             |
| --------- | ---------------------------------------------------------------- |
| `ID`      | オブジェクトの一意な識別子で、実際には `String` が格納されている |
| `String`  | 文字列型                                                         |
| `Int`     | 整数型                                                           |
| `Float`   | 浮動小数点数型                                                   |
| `Boolean` | 真偽値型                                                         |

### オブジェクト型

**オブジェクト型**は、[GraphQL](#graphql)の[Type](#type)のひとつで、1つ以上の[Field](#field)を持つ複合的なデータを表す型。

### Field

**Field**は、[GraphQL](#graphql)の[オブジェクト型](#オブジェクト型)において、名前と型によって表現されるもので、プロパティやリレーションを表す。

### Operation Types

**Operation Types**は、[GraphQL](#graphql)において、データの取得や更新、削除等の操作を定義するもの。[Schema](#schema)に記述され、クエリの[Query](#query)や[Mutation](#mutation)と対応している。
