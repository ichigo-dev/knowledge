# 『GraphQL』ノート

（最終更新： 2023-10-04）


## 目次

1. [GraphQL](#graphql)
1. [スキーマ定義言語](#スキーマ定義言語)
	1. [Schema](#schema)
	1. [Type](#type)
	1. [ルート型](#ルート型)
	1. [スカラー型](#スカラー型)
	1. [オブジェクト型](#オブジェクト型)
	1. [Field](#field)
	1. [Interface](#interface)
	1. [Union](#union)
	1. [Enum](#enum)
	1. [Directive](#directive)
	1. [リゾルバ](#リゾルバ)
1. [クエリ言語](#クエリ言語)
	1. [Query](#query)
	1. [Mutation](#mutation)
	1. [Subscription](#subscription)
	1. [Fragment](#fragment)


## GraphQL

**GraphQL**は、[API](../../../../network/_/chapters/web.md#web-api)向けのデータ取得・操作のための言語仕様および実行環境。Facebookによって開発され、2015年に[オープンソース](../../../../computer/software/_/chapters/open_source_software.md#オープンソースソフトウェア)として公開された。[サーバ](../../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)側はデータの構成や受け付ける[リクエスト](../../../../system/_/chapters/system_processing_model.md#リクエスト)を[スキーマ定義言語](#スキーマ定義言語)で定義し、[クライアント](../../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が[クエリ言語](#クエリ言語)によって必要なデータのみを明示的に要求する、という構成になっており、[Schema](#schema)次第で柔軟にデータをフェッチすることが可能となる。

モバイル[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)や[SPA](../../../../programming/_/chapters/programming_language.md#javascript)との相性がよく、活躍の場を広げている。


## スキーマ定義言語

**スキーマ定義言語**(**SDL**: Schema Definition Language)は、[GraphQL](#graphql)において、[Schema](#schema)を記述して[API](../../../../network/_/chapters/web.md#web-api)仕様を定めるための言語。[クエリ言語](#クエリ言語)の[リクエスト](../../../../system/_/chapters/system_processing_model.md#リクエスト)は、[Schema](#schema)に従って処理され、[レスポンス](../../../../system/_/chapters/system_processing_model.md#レスポンス)が生成される。

[GraphQL](#graphql)のスキーマ定義言語は型システムを内包しており、[Query](#query)に対する[レスポンス](../../../../system/_/chapters/system_processing_model.md#レスポンス)のバリデーションや、[リゾルバ](#リゾルバ)の適用のために使用される。

### Schema

**Schema**は、[GraphQL](#graphql)において、データの構造と[クエリ](#クエリ言語)のエンドポイントを定義するもの。クエリの構造や利用可能なデータ型、データの取得・変更方法を定義し、[GraphQL](#graphql)の通信のルールを確立する。

### Type

**Type**は、[GraphQL](#graphql)において、[Schema](#schema)で定義されるデータ型。データ型を表すTypeには、[スカラー型](#スカラー型)と[オブジェクト型](#オブジェクト型)がある。

以下は、独自の[オブジェクト型](#オブジェクト型)のTypeを定義する[Schema](#schema)の例。型名に `!` を付けた場合は必須となる。また、 `[Type]` のように記述することで、配列として扱うことができる。

```graphql
type Person
{
    name: String!
    age: Int
    favoirtes: [String]
}
```

### ルート型

**ルート型**は、[GraphQL](#graphql)において、データソースに対する操作を表現する型。あらかじめ決められた `Query` 、 `Mutation` 、 `Subscription` という[Type](#type)名がルート型として扱われる。

ルート型の[Type](#type)名は、以下のようにして任意に変更することができる。

```graphql
schema
{
    query: Root
}

type Root
{
    person( id: ID ): Person
}

type Person
{
    id: ID!
    name: String!
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

また、独自のスカラー型を定義するには、以下のように記述する。こうして定義された型に対しては、[GraphQL](#graphql)を利用する環境で必要に応じてシリアライズ・デシリアライズの処理を記述する。

```graphql
scalar Date
```

### オブジェクト型

**オブジェクト型**は、[GraphQL](#graphql)の[Type](#type)のひとつで、1つ以上の[Field](#field)を持つ複合的なデータを表す型。[Type](#type)によって独自のオブジェクト型を定義することができる。

```graphql
type Person
{
    name: String!
    age: Int
    favoirtes: [String]
}
```

### Field

**Field**は、[GraphQL](#graphql)の[オブジェクト型](#オブジェクト型)において、名前と型によって表現されるもので、プロパティやリレーションを表す。Fieldには引数を設定することができ、引数に応じて[Query](#query)や[Mutation](#mutation)の動作を制御することができる。

```graphql
type Query
{
    person( id: ID ): Person
}

type Person
{
    id: ID!
    name: String!
}
```

### Interface

**Interface**は、[GraphQL](#graphql)において、具体的なリソースを持たない抽象的なデータ型。[Type](#type)はInterfaceを実装することで、共通の構造を表現することができる。

```graphql
interface Person
{
    id: ID!
    name: String!
}

type Employee implements Person
{
    id: ID!
    name: String!
    company: String!
}
```

### Union

**Union**は、[GraphQL](#graphql)において、指定された複数の型のうちいずれかの型を示す抽象的なデータ型。

```graphql
type Post
{
    id: ID!
    content: String!
    author: String
}

type Comment
{
    id: ID!
    content: String!
}

union Searchable = Post | Comment;

type Query
{
    search( q: String! ) [Searchable!]!
}
```

### Enum

**Enum**は、[GraphQL](#graphql)において、[スカラー型](#スカラー型)と同様、特定の値のみを持つ型。とり得る値の範囲をあらかじめ限定することができる。

```graphql
type Comment
{
    id: ID!
    content: String!
}

type Reply
{
    id: ID!
    content: String!
    comment_id: ID!
}

enum CommentNode
{
    Comment
    Reply
}
```

### Directive

**Directive**は、[GraphQL](#graphql)において、[Schema](#schema)や[Query](#query)に対してメタデータを与えるための宣言。[GraphQL](#graphql)の処理系やツールによって解釈され、様々な効果を持つ。サポート状況は[ライブラリ](../../../../computer/software/_/chapters/package.md#ライブラリ)によって異なる。

### リゾルバ

**リゾルバ**は、[GraphQL](#graphql)において、[Query](#query)を解析して処理を行う関数。[GraphQL](#graphql)を扱う実行環境で、外部の[プログラム](../../../../programming/_/chapters/programming.md#プログラム)がリゾルバとして扱われる場合が多い。

以下は、[TypeScript](../../../../programming/_/chapters/programming_language.md#typescript)（Apolloを使用した場合）にてリゾルバを記述する例。

```ts
import { ApolloServer } from '@apollo/server';

// GraphQL
const gql = `#graphql
    type Person
    {
        name: String!
        age: Int
    }

    type Query
    {
        persons: [Person]
    }
`;

// Data source
const persons =
[
    {
        "name": "Smith",
        "age": 20,
    },
    {
        "name": "Johnson",
        "age": 40,
    },
];

// Resolver
const resolvers =
{
    Query:
    {
        persons: () => persons,
    },
};

// GraphQL server
const server = new ApolloServer(
{
    gql,
    resolvers,
});
```


## クエリ言語

**クエリ言語**は、[GraphQL](#graphql)において、[クライアント](../../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)からの[リクエスト](../../../../system/_/chapters/system_processing_model.md#リクエスト)を記述するための言語。[GraphQL](#graphql)には、取得系の[Query](#query)、更新系の[Mutation](#mutation)、イベント通知を受け取るための[Subscription](#subscription)の3種類の[リクエスト](../../../../system/_/chapters/system_processing_model.md#リクエスト)形式がある。

[REST](../../../../network/_/chapters/web.md#rest) [API](../../../../network/_/chapters/web.md#web-api)とは異なり、[リクエスト](../../../../system/_/chapters/system_processing_model.md#リクエスト)の形式は複雑になる。

### Query

**Query**は、[GraphQL](#graphql)におけるデータ取得系の[クエリ](#クエリ言語)で、対応する[ルート型](#ルート型)は `Query` となる。[クエリ](#クエリ言語)を受けてどのようなデータを取得するかは、[リゾルバ](#リゾルバ)の実装次第となる。

例えば、[スキーマ定義言語](#スキーマ定義言語)側で以下のようなQueryが定義されている場合を考える。

```graphql
type Person
{
    name: String!
    age: Int
}

type Company
{
    name: String!
    office: String
}

type Query
{
    persons: [Person]
    company( name: String! ): [Company]
}
```

この時、 `persons` を実行するQueryは以下のようになる。Queryに取得したい対象のデータだけでなく、その中の[Field](#field)を指定する必要があり、これは[スカラー型](#スカラー型)か[Enum](#enum)に解決されなければならない。

```graphql
query
{
    persons
    {
        name
    }
}
```

また、次の例のようにして、複数のQueryを1つの[リクエスト](../../../../system/_/chapters/system_processing_model.md#リクエスト)で同時に実行することもできる。

```graphql
query
{
    persons
    {
        name
    }

    company( name: "Example Company" )
    {
        office
    }
}
```

Queryにはオペレーション名を付けることで、解析や再利用を容易にすることができる。

```graphql
query getPersonList
{
    persons
    {
        name
        age
    }
}
```

### Mutation

**Mutation**は、[GraphQL](#graphql)におけるデータ更新系の[クエリ](#クエリ言語)で、対応する[ルート型](#ルート型)は `Mutation` となる。[クエリ](#クエリ言語)を受けてどのようにデータを更新するかは、[リゾルバ](#リゾルバ)の実装次第となる。

例えば、[スキーマ定義言語](#スキーマ定義言語)側で以下のようなMutationが定義されている場合を考える。

```graphql
type Person
{
    name: String!
    age: Int
}

type Mutation
{
    increaseAge( name: String!, age: Int ): Person
}
```

この時、 `increaseAge` を実行するMutationは以下のようになる。

```graphql
mutation
{
    increaseAge( name: "Smith" )
    {
        age
    }
}
```

その他、主な記法については[Query](#query)と同様となる。

### Subscription

**Subscription**は、[GraphQL](#graphql)におけるイベント監視のための[クエリ](#クエリ言語)で、対応する[ルート型](#ルート型)は `Subscription` となる。Subscription機能を利用する場合、[WebSocket](../../../../network/_/chapters/application_layer.md#websocket)などの[クライアント](../../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)と[サーバ](../../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)の間での相互通信が可能な環境が必要となる。

例えば、[スキーマ定義言語](#スキーマ定義言語)側で以下のようなSubscriptionが定義されている場合を考える。

```graphql
type Person
{
    name: String!
    age: Int
}

type Subscription
{
    personChanged( name: String! ): Person
}
```

この時、 `personChanged` を実行するSubscriptionは以下のようになる。

```graphql
subscription
{
    personChanged( name: "Smith" )
    {
        name
        age
    }
}
```

その他、主な記法については[Query](#query)と同様となる。

### Fragment

**Fragment**は、複数の[Query](#query)や[Mutation](#mutation)などで利用される共通部分をまとめることができる機能。

例えば、[スキーマ定義言語](#スキーマ定義言語)側が以下のように定義されている場合を考える。

```graphql
type Person
{
    name: String!
    age: Int
    weight: Int
    height: Int
    country: String
}

type Query
{
    persons: [Person]
}
```

このとき、複数の[Query](#query)で利用する[Field](#field)が共通している場合、以下のようなFragmentを用意するとよい。

```graphql
fragment PersonFragment on Person
{
    name
    country
}
```

[Query](#query)でこのFragmentを利用する場合、次のように記述する。

```graphql
query
{
    persons
    {
        ...PersonFragment
    }
}
```
