# REST


## 目次

1. [アーキテクチャスタイル](#アーキテクチャスタイル)
	1. [アーキテクチャスタイルとデザインパターン](#アーキテクチャスタイルとデザインパターン)
	1. [RESTアーキテクチャスタイル](#restアーキテクチャスタイル)
1. [リソース](#リソース)
	1. [アドレス可能性](#アドレス可能性)
	1. [リソースの表現と状態](#リソースの表現と状態)
1. [RESTアーキテクチャスタイル](#restアーキテクチャスタイル)
	1. [クライアント/サーバ](#クライアントサーバ)
	1. [ステートレスサーバ](#ステートレスサーバ)
	1. [キャッシュ](#キャッシュ)
	1. [統一インタフェース](#統一インタフェース)
	1. [階層化システム](#階層化システム)
	1. [コードオンデマンド](#コードオンデマンド)
	1. [ULCODC$SS](#ulcodcss)
1. [RESTful](#restful)
	1. [RESTfulサービス](#restfulサービス)


## アーキテクチャスタイル

### アーキテクチャスタイルとデザインパターン

**アーキテクチャスタイル**は（マクロ）アーキテクチャパターンとも呼ばれ、複数のアーキテクチャに共通する性質、様式、作法あるいは流儀を指す言葉。アーキテクチャスタイルには、**MVC**（Model-View-Controller）や**パイプ&フィルタ**（Pipe and Filter）、**イベントシステム**（Event System）などがある。

**デザインパターン**はマイクロアーキテクチャパターンとも呼ばれ、アーキテクチャスタイルよりも**粒度**の小さいクラスなどの設計仕様を指す。**GoF**（Gang of Four）の23種類のデザインパターンが有名。

### RESTアーキテクチャスタイル

RESTはネットワークシステムのアーキテクチャスタイルであり、**クライアント/サーバ**に制約を加えたものとなっている。RESTはWeb全体のアーキテクチャスタイルでもあり、個別のサービスのアーキテクチャスタイルでもある。

| 抽象化レベル           | Webでの例                                   |
| ---------------------- | ------------------------------------------- |
| アーキテクチャスタイル | REST                                        |
| アーキテクチャ         | ブラウザ、サーバ、プロキシ、HTTP、URI、HTML |
| 実装                   | Apache、Firefox、Chrome                     |


## リソース

[REST](#restアーキテクチャスタイル)において重要な概念の1つである**リソース**とは、Web上に存在する、名前（[URI](./04_uri.ja.md#uriの役割)）を持ったありとあらゆる情報のことである。

また、1つのリソースが複数の名前を持つことも可能である。

### アドレス可能性

[URI](./04_uri.ja.md#uriの役割)が備える、[リソース](#リソース)を一意的に指し示すことができる性質のことを、**アドレス可能性**と呼ぶ。

### リソースの表現と状態

[リソース](#リソース)はWeb上に存在する情報という抽象的な概念であるが、実際に[クライアントとサーバ](./01_basic_knowledge_of_web.ja.md#クライアントとサーバ)の間でやり取りされる具体的なデータのことを、**リソースの表現**という。

1つの[リソース](#リソース)は、HTML形式やPDF形式といった[メディアタイプ](https://ja.wikipedia.org/wiki/%E3%83%A1%E3%83%87%E3%82%A3%E3%82%A2%E3%82%BF%E3%82%A4%E3%83%97)の違いや使用する言語の違いなどがある、複数の表現を持つことができる。それぞれの表現に別の[URI](./04_uri.ja.md#uriの役割)を割り当てるなどの方法によって、提供するリソース表現を判断する。

[リソース](#リソース)には**状態**があり、提供する情報は時間によって変化する可能性がある（天気予報など）。同じタイミングでアクセスした複数のクライアントには同じ状態の[リソース](#リソース)が提供される。


## RESTアーキテクチャスタイル

### クライアント/サーバ

Webは[HTTP](./05_http.ja.md#httpの基本)という[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)で**クライアント**と**サーバ**が通信する、**クライアント/サーバ**（Client Server）の[アーキテクチャスタイル](#アーキテクチャスタイル)を採用している。クライアントがサーバに**リクエスト**を送り、サーバはそれに対して**レスポンス**を返す。

この[アーキテクチャスタイル](#アーキテクチャスタイル)の利点は、処理をクライアントとサーバに分離できることである。これにより、クライアントを**マルチプラットフォーム**にすることができる。

また、ユーザインタフェースをクライアントに任せ、サーバはデータストレージとしての機能のみを提供すればよい。複数のサーバを組み合わせて**冗長化**することで、可用性を上げることもできる。

### ステートレスサーバ

**ステートレスサーバ**（Stateless Server）とは、クライアントの[アプリケーション状態](./05_http.ja.md#httpのステートレス性)をサーバで管理しないことを意味する。しかし、現実には[ステートレス](./05_http.ja.md#httpのステートレス性)ではないWebサービスやWeb APIは多々ある。[HTTP](./05_http.ja.md#httpの基本)をステートフルにする例は、Cookieを使ったセッション管理である。

Cookieはステートレスサーバの利点をあえて捨てることを理解したうえで、必要最低限に利用する。

### キャッシュ

**キャッシュ**（Cache）は、[リソース](#リソース)の鮮度に基づいて、一度取得した[リソース](#リソース)をクライアント側で使いまわす方式のこと。

サーバとクライアントの間の通信を減らすことでネットワーク帯域の利用や処理時間を縮小し、より効率的に処理できるようになる。ただし、古いキャッシュを利用してしまい、情報の信頼性が下がる可能性もある。

### 統一インタフェース

**統一インタフェース**（Uniform Interface）とは、[URI](./04_uri.ja.md#uriの仕様)で指示した[リソース](#リソース)に対する操作を統一した限定的なインタフェースで行うこと。

HTTP1.1では、GETやPOSTなど8個のメソッドだけに限定されており、拡張できないという制約がある。柔軟性を犠牲にしたこのような制約により、アーキテクチャ全体をシンプルに保っている。

### 階層化システム

**階層化システム**（Layered System）は、システムをいくつかの階層に分離する[アーキテクチャスタイル](#アーキテクチャスタイル)のことである。

[統一インタフェース](#統一インタフェース)の恩恵により、Webは非常にシステム全体を階層化しやすくなっている。サーバとクライアントの間に**ロードバランサ**（Load Balancer）を設置して負荷分散をしたり、**プロキシサーバ**（Proxy Server）を設置してアクセスを制限したりすることができる。

これらのサーバやプロキシなどの各コンポーネント間のインタフェースを[HTTP](./05_http.ja.md#httpの基本)で統一しているため、クライアントは接続先の構成が変更されてもそれを意識する必要がない。

### コードオンデマンド

**コードオンデマンド**は、プログラムコードをサーバからダウンロードし、クライアント側でそれを実行する[アーキテクチャスタイル](#アーキテクチャスタイル)。**JavaScript**や**Flash**、**Javaアンプレット**などがこれに該当する。

コードオンデマンドの利点は、クライアントを後から拡張できる点である。一方でコードオンデマンドの欠点は、プログラムをクライアント側で実行することによるアプリケーション[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)の可視性の低下である。

### ULCODC$SS

クライアント/サーバにこれらの[アーキテクチャスタイル](#アーキテクチャスタイル)を追加した複合[アーキテクチャスタイル](#アーキテクチャスタイル)は、「**ULCODC$SS**（Uniform Layered Code on Demand Client Cache Stateless Server）」という。この[アーキテクチャスタイル](#アーキテクチャスタイル)のことを、Fieldingは[REST](#restアーキテクチャスタイル)と名付けた。

Webサービスを実装するうえで[REST](#restアーキテクチャスタイル)を意識した設計は重要となるが、[REST](#restアーキテクチャスタイル)を構成するスタイルのうちいくつかは除外しても構わない。

## RESTful

### RESTfulサービス

[REST](#restアーキテクチャスタイル)の制約に従っていて[REST](#restアーキテクチャスタイル)らしいサービスのことを**RESTful**と呼ぶ。個別のWebサービスやWeb APIがRESTfulになると、Webは全体としてより良くなる。

Web上の[リソース](#リソース)同士が持つリンクをたどる作業を経ることでアプリケーションを実現すること（ソーシャルブックマークサービスなど）を、[REST](#restアーキテクチャスタイル)では**アプリケーション状態エンジンとしてのハイパーメディア**と呼ぶ。

[リソース](#リソース)をリンクで接続することで1つのアプリケーションを構成するという考え方は、**接続性**という。
