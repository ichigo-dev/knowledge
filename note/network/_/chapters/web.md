# 『Web』ノート

（最終更新： 2023-09-27）


## 目次

1. [Web](#web)
	1. [Webブラウザ](#webブラウザ)
	1. [Webサーバ](#webサーバ)
	1. [ユーザエージェント](#ユーザエージェント)
	1. [Web API](#web-api)
	1. [W3C](#w3c)
	1. [Webの歴史](#webの歴史)
1. [ハイパーメディア](#ハイパーメディア)
	1. [ハイパーリンク](#ハイパーリンク)
	1. [ハイパーテキスト](#ハイパーテキスト)
	1. [Memex](#memex)
	1. [Xanadu](#xanadu)
	1. [HyperCard](#hypercard)
1. [検索エンジン](#検索エンジン)
	1. [クローリング](#クローリング)
	1. [インデックス](#インデックス)
	1. [ランキング](#ランキング)
	1. [ページランク](#ページランク)
1. [REST](#rest)
	1. [SOAP](#soap)
	1. [クライアントサーバ](#クライアントサーバ)
	1. [ステートレスサーバ](#ステートレスサーバ)
	1. [キャッシュ](#キャッシュ)
	1. [統一インタフェース](#統一インタフェース)
	1. [階層化システム](#階層化システム)
	1. [コードオンデマンド](#コードオンデマンド)
	1. [RESTful](#restful)
1. [URI](#uri)
	1. [URL](#url)
	1. [URN](#urn)
	1. [スキーム](#スキーム)
	1. [クエリパラメータ](#クエリパラメータ)
	1. [URIフラグメント](#uriフラグメント)
	1. [リソース](#リソース)
	1. [アドレス可能性](#アドレス可能性)
	1. [クールURI](#クールuri)
	1. [リダイレクト](#リダイレクト)
1. [HTML](#html)
1. [HTTPにおける認証](#httpにおける認証)
	1. [Basic認証](#basic認証)
	1. [Digest認証](#digest認証)
1. [HTTPメッセージ](#httpメッセージ)
	1. [HTTPヘッダ](#httpヘッダ)
	1. [HTTPボディ](#httpボディ)
	1. [リクエストメッセージ](#リクエストメッセージ)
	1. [リクエストライン](#リクエストライン)
	1. [レスポンスメッセージ](#レスポンスメッセージ)
	1. [ステータスライン](#ステータスライン)
1. [HTTPメソッド](#httpメソッド)
1. [ステータスコード](#ステータスコード)
	1. [頻出のステータスコード](#頻出のステータスコード)
1. [フォーム](#フォーム)
	1. [マルチパートフォーム](#マルチパートフォーム)
1. [コンテントネゴシエーション](#コンテントネゴシエーション)
1. [セッション](#セッション)
	1. [Cookie](#cookie)
	1. [ローカルストレージ](#ローカルストレージ)
	1. [オリジン](#オリジン)
1. [FastCGI](#fastcgi)


## Web

**Web**は、[TCP/IP](./communication_protocol.md#tcpip)の[アプリケーションレイヤ](./communication_protocol.md#アプリケーションレイヤ)の[システム](../../../system/_/chapters/system.md#システム)で、[インターネット](./network.md#インターネット)上で[HTTP](./application_layer.md#http)通信によりハイパーテキスト形式の情報をやり取りすることができる。世界最大の[分散システム](../../../system/_/chapters/system_processing_model.md#分散システム)であり、[ハイパーメディア](#ハイパーメディア)という情報システムの一種である。

[Webブラウザ](#webブラウザ)の画面に表示されるイメージ全体を**Webページ**、Webページの見出しとなるページを**ホームページ**と呼ぶ。

### Webブラウザ

**Webブラウザ**は、[Web](#web)の情報を画面に表示するための[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)。代表的なブラウザとしてはMicrosoftのMicrosoft EdgeやMozilla FoundationのFirefox、GoogleのGoogle Chrome、Opera SoftwareのOpera、AppleのSafariなどがある。

[HTML](#html)やCSS、JavaScriptをどのように解釈するかはブラウザの実装次第であるため、[Web](#web)コンテンツの表示や挙動が微妙に異なる場合がある。

### Webサーバ

**Webサーバ**は、[インターネット](./network.md#インターネット)上で[Webブラウザ](#webブラウザ)にコンテンツを配信する[サーバ](../../../computer/_/chapters/computer.md#サーバ)。

### ユーザエージェント

**ユーザエージェント**(**UA**: User Agent)は、[Web](#web)コンテンツにアクセスする具体的な[プログラム](../../../programming/_/chapters/programming.md#プログラム)や[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)。[OS](../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)や[ブラウザ](#webブラウザ)の名称、[バージョン](../../../computer/software/_/chapters/package.md#バージョン)などの情報はユーザエージェントとして[サーバ](../../../computer/_/chapters/computer.md#サーバ)に通知される。

### Web API

**Web API**(Web Application Programming Interface)は、[Web](#web)を介して他の[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)と機能やサービスを共有する仕組み。データを共有する際のフォーマットとしてはXMLやJSONがよく用いられる。

### W3C

**W3C**(World Wide Web Consortium)は、[IETF](./communication_protocol.md#ietf)に代わって[Web](#web)の[標準化](./communication_protocol.md#標準化の流れ)を行う団体。[HTML](#html)やXML、[HTTP](./application_layer.md#http)、[URI](#uri)、CSSなどの[標準化](./communication_protocol.md#標準化の流れ)作業を行う。

### Webの歴史

1980年代までに[ハイパーメディア](#ハイパーメディア)の構想が成熟し、[インターネット](./network.md#インターネット)の技術が登場し、[分散システム](../../../system/_/chapters/system_processing_model.md#分散システム)の基盤が構築された。1990年11月、スイスの**CERN**（European Organization for Nuclear Research: 欧州原子核研究機構）のTim Berners-Leeが、[インターネット](./network.md#インターネット)ベースの分散情報管理システムとして[Web](#web)の提案書を書き、その年の年末には最初の[バージョン](../../../computer/software/_/chapters/package.md#バージョン)の[ブラウザ](#webブラウザ)と[サーバ](#webサーバ)を完成させた。

1993年にイリノイ大学の**NCSA**（National Center for Supercomputing Application: 米国立スーパーコンピュータ応用研究所）が公開した**Mosaic**という[ブラウザ](#webブラウザ)は、それまでの[ブラウザ](#webブラウザ)がテキストのみを扱っていたのに対して、画像をインライン表示できるという機能を備えていた。

様々な高機能[ブラウザ](#webブラウザ)が登場すると[Web](#web)は急速に普及し、[インターネット](./network.md#インターネット)の[標準化](./communication_protocol.md#標準化の流れ)を行う[IETF](./communication_protocol.md#ietf)の[RFC](./communication_protocol.md#rfc)の仕様策定が追いつかなくなった。そこで、1994年にBerners-Leeが中心となって[W3C](#w3c)を設立し、[IETF](./communication_protocol.md#ietf)に代わって[Web](#web)の[標準化](./communication_protocol.md#標準化の流れ)を進めた。


## ハイパーメディア

**ハイパーメディア**は、テキストや音声、映像などの様々なメディアを[リンク](#ハイパーリンク)で結び付けて構成した[システム](../../../system/_/chapters/system.md#システム)。

### ハイパーリンク

**ハイパーリンク**（**リンク**）は、[ハイパーメディア](#ハイパーメディア)において情報同士を結びつける機能。

### ハイパーテキスト

**ハイパーテキスト**は、[ハイパーリンク](#ハイパーリンク)を用いて情報を結びつけた文字情報中心の文書。[ハイパーメディア](#ハイパーメディア)はハイパーテキストを拡張した考え方。

### Memex

**Memex**は、1945年にアメリカの研究者Vannevar Bushが発表した情報検索システムについての論文で、[ハイパーメディア](#ハイパーメディア)の起源。[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)が登場したばかりの頃の論文であるが、電気的に接続した本やフィルムなどを相互に[リンク](#ハイパーリンク)し、[リンク](#ハイパーリンク)をたどって情報を探索するという現在の[Web](#web)を予感させる[システム](../../../system/_/chapters/system.md#システム)が構想されていた。

### Xanadu

**Xanadu**は、[Memex](#memex)の構想に影響を受けたTed Nelsonによって構想された[ハイパーメディア](#ハイパーメディア)[システム](../../../system/_/chapters/system.md#システム)。現在の[Web](#web)よりもさらに高機能な理想の[ハイパーメディア](#ハイパーメディア)として開発されたが、あまりの複雑さから計画が頓挫し、[Web](#web)の圧倒的な普及速度に追いつけなかった。

### HyperCard

**HyperCard**は、1987年にAppleのBill Atkinsonが開発した[ハイパーメディア](#ハイパーメディア)で、カードと呼ばれる文書を単位に相互に[リンク](#ハイパーリンク)を張るという構成。当時Appleがよく利用していたスクリプト言語、HyperTalkにより実装されていた。


## 検索エンジン

**検索エンジン**は、広大な[インターネット](./network.md#インターネット)上の情報から、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が求める情報を探し出すための[システム](../../../system/_/chapters/system.md#システム)。ユーザが[Webブラウザ](#webブラウザ)の検索ボックスに入力したキーワードを元に、最適な情報を探索する。現在使われている検索エンジンは、ほとんどがGoogleとなっているが、YahooやBingといった検索エンジンもよく用いられている。

### クローリング

**クローリング**は、**クローラ**という[プログラム](../../../programming/_/chapters/programming.md#プログラム)によって[インターネット](./network.md#インターネット)上の[リンク](#ハイパーリンク)をたどり、[Web](#web)ページを巡回して情報を収集する処理。

### インデックス

**インデックス**は、[クローリング](#クローリング)によって収集したデータを蓄積し、検索インデックスに[Web](#web)ページの情報を格納する処理。

### ランキング

**ランキング**は、[インデックス](#インデックス)された情報を検索[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)と照合して評価し、関連性の高い回答を検索結果として表示する処理。

### ページランク

**ページランク**は、Googleが採用している[検索エンジン](#検索エンジン)の[ランキング](#ランキング)[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)。


## REST

**REST**は、[ネットワーク](./network.md#ネットワーク)[システム](../../../system/_/chapters/system.md#システム)のアーキテクチャスタイルのひとつであり、[クライアントサーバシステム](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)に様々な制約を加えたものとなっている。RESTは[Web](#web)をソフトウェアアーキテクチャの観点から分析してまとめられている。

RESTは、**ULCODC$SS**(Uniform Layered Code on Demand Client Cache Stateless Server)というアーキテクチャスタイルの通称。[Web](#web)サービスを実装する上ではRESTを構成するいくつかのアーキテクチャスタイルは除外してもよい。

### SOAP

**SOAP**は、[ネットワーク](./network.md#ネットワーク)[システム](../../../system/_/chapters/system.md#システム)のアーキテクチャスタイルのひとつであり、メッセージの転送方法を定めた仕様。1990年代後半から2000年代にかけて、[Web API](#web-api)の[標準化](./communication_protocol.md#標準化の流れ)をめぐって[REST](#rest)と争っていたが、多くのベンダがドラフトを持ち寄って実装を進めた結果、相互運用性に欠ける状態となったために[REST](#rest)に敗北した。

### クライアントサーバ

**クライアントサーバ**(Client/Server)は、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)に[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)を送り、それに対して[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が[レスポンス](../../../system/_/chapters/system_processing_model.md#レスポンス)を返すというアーキテクチャスタイル。

### ステートレスサーバ

**ステートレスサーバ**(Stateless Server)は、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)の**アプリケーション状態**（**セッション状態**）を[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が管理しないアーキテクチャスタイル。しかし現実にはステートレスではない[Web](#web)サービスや[Web API](#web-api)がほとんどであり、[Cookie](#cookie)などのセッションを使ったステートフルな状態管理が多用されている。

ステートフルな[アプリケーション](../../../computer/software/_/chapters/software.md#応用ソフトウェア)では、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)の数が増え、保持しなければいけないアプリケーション状態が増加した際に[スケールアウト](../../../system/_/chapters/system_performance_evaluation.md#スケールアウト)することが難しい。ステートフルな構成はステートレスサーバの利点をあえて捨てているということを理解した上で利用する必要がある。

### キャッシュ

**キャッシュ**(Cache)は、[リソース](#リソース)の鮮度に基づいて、一度取得した[リソース](#リソース)の表現を[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)側で使いまわすアーキテクチャスタイル。[CDN](../../../system/_/chapters/system_architecture.md#cdn)を用いるなどして[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)側でキャッシュを扱う場合もある。

キャッシュを用いることにより、[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)と[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)の間の通信を減らして[ネットワーク](./network.md#ネットワーク)[帯域](./network.md#帯域)の利用や処理時間を縮小し、より効率的な処理ができるようになる。ただし、古いキャッシュを利用してしまい、情報の信頼性が下がる可能性もある。

### 統一インタフェース

**統一インタフェース**(Uniform Interface)は、[URI](#uri)で指示した[リソース](#リソース)に対する操作を、統一した限定的なインタフェースにより行うアーキテクチャスタイル。実際に[HTTP](./application_layer.md#http)には限られたいくつかの[メソッド](#httpメソッド)しか用意されておらず、[Web](#web)のアーキテクチャ全体をシンプルに保っている。

### 階層化システム

**階層化システム**(Layered System)は、[システム](../../../system/_/chapters/system.md#システム)をいくつかの階層に分類するアーキテクチャスタイル。[統一インタフェース](#統一インタフェース)の恩恵もあり、[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)と[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)の間にロードバランサを設置して負荷分散をしたり、プロキシサーバを設置してアクセスを制限したりすることが容易となっている。

### コードオンデマンド

**コードオンデマンド**は、[プログラム](../../../programming/_/chapters/programming.md#プログラム)[コード](../../../programming/_/chapters/programming.md#ソースコード)を[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)からダウンロードして、それを[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)側で実行するアーキテクチャスタイル。[JavaScript](../../../programming/_/chapters/programming_language.md#javascript)や**Flash**、**Javaアンプレット**がこれに該当する技術で、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)側の拡張性が高くなるという利点がある。

### RESTful

**RESTful**は、[REST](#rest)の制約に従う[REST](#rest)らしいサービスのことを指す用語。個別の[Web](#web)サービスや[Web API](#web-api)がRESTfulになると、[Web](#web)は全体としてより良くなるため、開発者は[REST](#rest)を意識して個別の[システム](../../../system/_/chapters/system.md#システム)を設計する必要がある。


## URI

**URI**（Uniform Resource Identifier: **統一リソース識別子**）は、[インターネット](./network.md#インターネット)上の資源（[リソース](#リソース)）を表す識別子。

URIで表されるパスには[ASCII](../../../basics/information_theory/_/chapters/character_representation.md#asciiコード)文字のみ利用することができ、日本語などの[ASCII](../../../basics/information_theory/_/chapters/character_representation.md#asciiコード)以外の文字を使いたい場合は**%エンコーディング**（**パーセントエンコーディング**）という方式を用いて文字をエンコードする。

### URL

**URL**(Uniform Resource Locator)は、[URI](#uri)よりも狭義の概念で、[インターネット](./network.md#インターネット)上の[Web](#web)[リソース](#リソース)を表す識別子のことを指す。

### URN

**URN**(Uniform Resource Name)は、[URI](#uri)よりも狭義の概念で、[リソース](#リソース)に恒久的なIDを振るための識別子。例えば、書籍には**ISBN**という世界的に統一されたURNが割り振られている。

### スキーム

**スキーム**(Scheme)は、[URI](#uri)中でその[URI](#uri)が利用する[プロトコル](./network_architecture.md#プロトコル)を指す部分。スキームの公式な一覧は**IANA**(Internet Assigned Numbers Authority)にある。

### クエリパラメータ

**クエリパラメータ**（**クエリ文字列**）は、[URI](#uri)中に指定することができるパラメータで、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)に対して情報を送信したい場合に用いられる。[URI](#uri)中で `?` 以降に記載され、キーと値が `=` 区切りでつなげられる。複数のパラメータを含めたい場合は、 `&` によってパラメータ同士が結合された文字列となる。

### URIフラグメント

**URIフラグメント**は、[URI](#uri)中で[リソース](#リソース)の内部のさらに細かい部分を指定するときに使用される部分。[URI](#uri)中で `#` 以降に記載され、[HTML](#html)のid[属性](../../../web_development/html/_/chapters/html.md#属性)に紐付けられる。

### リソース

**リソース**は、[Web](#web)上に存在する[URI](#uri)を持ったありとあらゆる情報を指す用語。ひとつのリソースが複数の名前を持つこともできる。

実際に[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)と[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)がやり取りするデータのことを**リソースの表現**といい、リソースの表現は時間によって**状態**が変化する可能性がある。

### アドレス可能性

**アドレス可能性**は、[URI](#uri)が持つ、[リソース](#リソース)を一意的に指し示すことができる性質。

### クールURI

**クールURI**は、良い[URI](#uri)や綺麗な[URI](#uri)を指す言葉で、Berners-Leeが1998年に発表した「Cool URIs don't change」という[Web](#web)ページが発祥となっている。Berners-Leeは、「[URI](#uri)は変わらないべきである。変わらない[URI](#uri)こそが最上のURIである」と主張した。

### リダイレクト

**リダイレクト**(Redirect)は、古い[URI](#uri)を新しい[URI](#uri)に転送する[HTTP](./application_layer.md#http)の仕組み。[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)にリダイレクトを要求する際には、300番台の[ステータスコード](#ステータスコード)とLocation[ヘッダ](#httpヘッダ)が含まれる[レスポンスメッセージ](#レスポンスメッセージ)を送信し、[ブラウザ](#webブラウザ)はLocationに指定された新しい[URL](#url)に再度[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)を行う。

[URI](#uri)を変更したい場合はリダイレクトにより古い[URI](#uri)が完全に使えなくなることを防ぐとよい。


## HTML

**HTML**(HyperText Markup Language)は、[Web](#web)ページを記述するためのマークアップ言語あるいはデータ形式。HTMLは[ハイパーテキスト](#ハイパーテキスト)であり、画面に表示する文字や画像に[リンク](#ハイパーリンク)を張って別の情報と紐づけることができる。

HTMLは[WWW](./application_layer.md#www)の[プレゼンテーション層](./network_architecture.md#プレゼンテーション層)の機能であるが、この[プレゼンテーション層](./network_architecture.md#プレゼンテーション層)は完全には整備されておらず、[OS](../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)や[ブラウザ](#webブラウザ)が異なると表示の細かい部分に違いを生じる場合がある。

- [HTML](../../../web_development/html/_/chapters/html.md)


## HTTPにおける認証

### Basic認証

**Basic認証**は、base64でエンコードされたユーザIDとパスワードによる認証。平文で[ネットワーク](./network.md#ネットワーク)を流れるので、安全性は低く、HTTPSの暗号化通信と組み合わせて使用することが推奨されている。

### Digest認証

**Digest認証**は、[Basic認証](#basic認証)の欠点であった平文で認証情報が流れてしまうことを改善した認証方式。ユーザIDとパスワードをMD5でハッシュ化して送信する。ただし、Digest認証の暗号化方式も解析が可能となっているため、HTTPSと合わせて使用するなど安全性を高める対策は必要となる。


## HTTPメッセージ

**HTTPメッセージ**は、[HTTP](./application_layer.md#http)通信において[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)や[レスポンス](../../../system/_/chapters/system_processing_model.md#レスポンス)の際にやり取りされるメッセージ。

### HTTPヘッダ

**HTTPヘッダ**は、[HTTPメッセージ](#httpメッセージ)において、[HTTP](./application_layer.md#http)通信のメタデータが格納された部分。[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)と[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が相互に情報をやり取りするために使用される。HTTPヘッダはキーと値のペアから構成され、 `User-Agent` や `Accept` 、 `Content-Type` 、 `Content-Length` といったものが頻繁に利用される。

### HTTPボディ

**HTTPボディ**は、[HTTPメッセージ](#httpメッセージ)において様々な情報が格納される部分で、その扱いは[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)次第。HTTPボディは省略することも可能。通常の[Web](#web)ページの[レスポンス](../../../system/_/chapters/system_processing_model.md#レスポンス)ではHTTPボディに[HTML](#html)が格納され、[Web API](#web-api)では[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)のパラメータや[レスポンス](../../../system/_/chapters/system_processing_model.md#レスポンス)データとしてHTTPボディにXMLやJSONなどが格納されることが多い。

### リクエストメッセージ

**リクエストメッセージ**は、[HTTP](./application_layer.md#http)通信において[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)の際にやり取りされる[HTTPメッセージ](#httpメッセージ)。

```
[リクエストメッセージの例]
GET /test?q=test HTTP/1.1
Host: example.jp
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)
Accept: text/html,application/xhtml+xml,application/xml;q=0.9
```

### リクエストライン

**リクエストライン**は、[リクエストメッセージ](#リクエストメッセージ)の1行目にあたる部分で、[HTTPメソッド](#httpメソッド)、[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)[URI](#uri)、[プロトコル](./network_architecture.md#プロトコル)[バージョン](../../../computer/software/_/chapters/package.md#バージョン)からなる。

### レスポンスメッセージ

**レスポンスメッセージ**は、[HTTP](./application_layer.md#http)通信において[レスポンス](../../../system/_/chapters/system_processing_model.md#レスポンス)の際にやり取りされる[HTTPメッセージ](#httpメッセージ)。

```
[レスポンスメッセージの例]
HTTP/1.1 200 OK
Content-Type: application/xhtml+xml; charset=utf-8

<html>
 ...
</html>
```

### ステータスライン

**ステータスライン**は、[レスポンスメッセージ](#レスポンスメッセージ)の1行目にあたる部分で、[プロトコル](./network_architecture.md#プロトコル)[バージョン](../../../computer/software/_/chapters/package.md#バージョン)、[ステータスコード](#ステータスコード)、テキストフレーズからなる。


## HTTPメソッド

**HTTPメソッド**は、[HTTP](./application_layer.md#http)[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)の際に、対象となる[リソース](#リソース)に対してどのような操作を行うかを表す命令。中でもGET、POST、PUT、DELETEは、これら4つでCRUDの性質を満たす。

| メソッド  | 意味                                                     |
| --------- | -------------------------------------------------------- |
| `GET`     | リソースの取得                                           |
| `POST`    | 子リソースの作成、リソースへのデータの追加、その他の処理 |
| `PATCH`   | リソースの部分的な更新                                   |
| `PUT`     | リソースの更新、リソースの作成                           |
| `DELETE`  | リソースの削除                                           |
| `HEAD`    | リソースのヘッダの取得                                   |
| `OPTIONS` | リソースがサポートしているメソッドの取得                 |
| `TRACE`   | 自分宛てにリクエストメッセージを返す（ループバック）試験 |
| `CONNECT` | プロキシ動作のトンネル接続への変更                       |


## ステータスコード

**ステータスコード**は、[HTTP](./application_layer.md#http)[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)の処理結果を表す3桁の数字。[レスポンスメッセージ](#レスポンスメッセージ)の[ステータスライン](#ステータスライン)に記載され、先頭の数字によって5つに分類することができる。

| ステータスコードの分類    | 概要                                                                                                                                     |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| 1xx（処理中）             | 処理が継続していることを示す。クライアントはそのままリクエストを継続するか、サーバの指示に従ってプロトコルをアップデートして再送信する。 |
| 2xx（成功）               | リクエストが成功したことを示す。                                                                                                         |
| 3xx（リダイレクト）       | ほかのリソースへのリダイレクトを示す。クライアントはレスポンスメッセージのLocationヘッダを見て新しいリソースへ接続する。                 |
| 4xx（クライアントエラー） | クライアントエラーを示す。原因はクライアント側にあり、エラーを解消しない限りは正常な結果が得られない。                                   |
| 5xx（サーバエラー）       | サーバエラーを示す。サーバ側の原因を解決すれば同一のリクエストを再送信して正常な結果が得られる可能性がある。                             |

### 頻出のステータスコード

| ステータスコード          | 概要                                                                                                                   |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| 200 OK                    | リクエストが成功したことを示す。                                                                                       |
| 201 Created               | リソースを新たに作成したことを示す。                                                                                   |
| 301 Moved Permanently     | リクエストで指定したリソースが新しいURIに移動したことを示す。古いURIを保ちつつ新しいURIに移行する際に用いられる。      |
| 303 See Other             | リクエストに対する処理結果が別のURIで取得できることを示す。POSTでリソースを操作した結果をGETで返す場合など。           |
| 400 Bad Request           | リクエストの構文やパラメータが間違っていることを示す。適切なクライアントエラーがない場合にも用いられる。               |
| 401 Unauthorized          | リクエストの認証情報が不適切であることを示す。                                                                         |
| 404 Not Found             | 指定したリソースが見つからなかったことを示す。                                                                         |
| 500 Internal Server Error | サーバ側に何らかの異常が生じていて、正しいレスポンスが返せないことを示す。適切なサーバエラーがない場合にも用いられる。 |
| 503 Service Unavailable   | サーバがメンテナンスなどで一時的にアクセスできないことを示す。                                                         |


## フォーム

**フォーム**は、[HTTP](./application_layer.md#http)において、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)から[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)に対してデータを送信するための技術。

[HTML](#html)では `form` [タグ](../../../web_development/html/_/chapters/html.md#タグ)を用いることでフォームを作成でき、 `action` [属性](../../../web_development/html/_/chapters/html.md#属性)に指定された[URL](#url)にデータが送信される。また、使用する[HTTPメソッド](#httpメソッド)を `method` [属性](../../../web_development/html/_/chapters/html.md#属性)で指定することもできる。

### マルチパートフォーム

**マルチパートフォーム**は、[HTTP](./application_layer.md#http)の[フォーム](#フォーム)におけるエンコードタイプの形式で、通常は単一のコンテンツしか持つことのできない[HTTPメッセージ](#httpメッセージ)に複数のコンテンツ（[ファイル](../../../computer/software/_/chapters/file_system.md#ファイル)）を持たせるためのオプション。マルチパートフォームの[HTTPボディ](#httpボディ)は、[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)ごとに異なる境界文字列で複数のコンテンツを区切った形式となっており、区切られた各コンテンツはそれぞれが[ヘッダ](#httpヘッダ)を持つことができる。境界文字列は `Content-Type` [ヘッダ](#httpヘッダ)に付与された `boundary` に記載される。

[HTML](#html)の `form` では、 `enctype="multipart/form-data"` を指定することでマルチパートフォームを作成することができる。


## コンテントネゴシエーション

**コンテントネゴシエーション**は、[HTTP](./application_layer.md#http)において、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が期待している形式や設定を[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)に伝えることで、最適なコンテンツを[レスポンス](../../../system/_/chapters/system_processing_model.md#レスポンス)するための仕組み。以下のネゴシエーション用の[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)の[ヘッダ](#httpヘッダ)と、それに対応する[レスポンス](../../../system/_/chapters/system_processing_model.md#レスポンス)の[ヘッダ](#httpヘッダ)に示したように、[MIME](./application_layer.md#mime)タイプや言語、文字セットなどをネゴシエーションできる。

| リクエストヘッダ  | レスポンスヘッダ              | ネゴシエーション対象 |
| ----------------- | ----------------------------- | -------------------- |
| `Accept`          | `Content-Type`                | MIMEタイプ           |
| `Accept-Language` | `Content-Language` / HTMLタグ | 表示言語             |
| `Accept-Charset`  | `Content-Type`                | 文字セット           |
| `Accept-Encoding` | `Content-Encoding`            | ボディの圧縮形式     |


## セッション

**セッション**は、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)がある[Web](#web)サイトに訪問してから離脱するまでの通信。各[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)ごとのセッション情報は[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が利用する[ストレージ](../../../computer/hardware/_/chapters/hardware.md#記憶装置)に格納されており、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)は自分のセッション情報を参照するためのキーを[Cookie](#cookie)などで保持する。

[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)を[冗長化](../../../system/_/chapters/system_architecture.md#冗長化)する場合は、全ての[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が共有できる[ストレージ](../../../computer/hardware/_/chapters/hardware.md#記憶装置)（[DB](../../../development/database/_/chapters/database.md#データベース)や[キャッシュ](#キャッシュ)[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)など）でセッション情報を管理する必要がある。

### Cookie

**Cookie**は、[ブラウザ](#webブラウザ)に情報を保存するための機能で、[JavaScript](../../../web_development/javascript/_/chapters/javascript.md#javascript)や[HTTPヘッダ](#httpヘッダ)によって制御される。Cookieの情報をどれだけの期間保持しておくかは、[ブラウザ](#webブラウザ)の設定や[HTTPメッセージ](#httpメッセージ)内のメタ情報により制御する。

[セッション](#セッション)情報を参照するためのキーや、一時的なデータの保存などに用いられる。

### ローカルストレージ

**ローカルストレージ**(Local Storage)は、[ブラウザ](#webブラウザ)に情報を保存するための機能で、[Cookie](#cookie)よりも情報の保存期間が長く、格納できるデータ量も大きい。[Cookie](#cookie)の情報は[HTTPメッセージ](#httpメッセージ)を介して[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)にも送信されるが、ローカルストレージの情報はブラウザのみで使用されるため、[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)側の[セッション](#セッション)の管理には向いていない。

### オリジン

**オリジン**は、[HTTP](./application_layer.md#http)における、[スキーム](#スキーム)、[ホスト名](./internet_layer.md#ホスト名)、[ポート](./address_on_network.md#ポート)の組み合わせ。[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)はこれら全てが一致した場合のみ同一オリジンであると判定する。[Cookie](#cookie)や[ローカルストレージ](#ローカルストレージ)は、オリジンごとにアクセス権限を分離している。


## FastCGI

**FastCGI**は、[Webサーバ](#webサーバ)上で[プログラム](../../../programming/_/chapters/programming.md#プログラム)を高速に動作させるためのインタフェース。[プロセス](../../../computer/software/_/chapters/operating_system.md#プロセス)の起動には時間がかかるため、あらかじめ[プロセス](../../../computer/software/_/chapters/operating_system.md#プロセス)をいくつか[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)上に常駐させておき、[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)があった際にはその[プロセス](../../../computer/software/_/chapters/operating_system.md#プロセス)を利用して[プログラム](../../../programming/_/chapters/programming.md#プログラム)を実行する。
