# Webの歴史


## 目次

1. [インターネットの歴史](#インターネットの歴史)
1. [ハイパーメディアの歴史](#ハイパーメディアの歴史)
	1. [Memex](#memex)
	1. [Xanadu](#xanadu)
	1. [HyperCard](#hypercard)
	1. [Web以前のハイパーメディアの問題点](#web以前のハイパーメディアの問題点)
1. [分散システムの歴史](#分散システムの歴史)
	1. [集中システムと分散システム](#集中システムと分散システム)
	1. [RPC](#rpc)
	1. [分散オブジェクト](#分散オブジェクト)
	1. [Web以前の分散システムの問題点](#web以前の分散システムの問題点)
1. [Webの歴史](#webの歴史-1)
	1. [Webの誕生](#webの誕生)
	1. [Webの普及](#webの普及)
	1. [Webの標準化](#webの標準化)
	1. [RESTの誕生](#restの誕生)
	1. [様々なハイパーメディアフォーマットの誕生](#様々なハイパーメディアフォーマットの誕生)
	1. [SOAP対REST](#soap対rest)


## インターネットの歴史

インターネットの起源は1969年の**ARPANET**で、そこから研究が進み**TCP/IP**が登場した。

[インターネットの詳しい歴史について](/note/internet/chapters/02_history_of_the_internet.ja.md)


## ハイパーメディアの歴史

### Memex

[ハイパーメディア](./01_basic_knowledge_of_web.ja.md#ハイパーメディア)の起源は、1945年にアメリカの研究者Vannevar Bushが発表した**Memex**という情報検索システムについての論文である。

Memexは実在するシステムではなく構想であるが、電気的に接続した本やフィルムなどを相互にリンクし、リンクをたどって情報を探索するという、現在のWebを予感させるシステムであった。

### Xanadu

Memexの構想に影響を受けたTed Nelsonは、1965年にハイパーテキストと[ハイパーメディア](./01_basic_knowledge_of_web.ja.md#ハイパーメディア)という言葉を考案した。**ハイパーテキスト**は文字情報中心の文書を相互にリンクさせた概念で、**ハイパーメディア**はその考え方を拡張し、音声や動画など多様なメディアを相互にリンクさせた概念である。

Nelsonは現在のWebよりもさらに高機能な理想のハイパーメディアである**Xanadu**を構想し、開発を始めた。しかしXanaduは、あまりの複雑さから計画が頓挫し、失敗に終わった（Webに敗北した）。

- [Xanadu計画](https://ja.wikipedia.org/wiki/%E3%82%B6%E3%83%8A%E3%83%89%E3%82%A5%E8%A8%88%E7%94%BB)

### HyperCard

Web以前に成功を収めた[ハイパーメディア](./01_basic_knowledge_of_web.ja.md#ハイパーメディア)としては、Bill Atkinsonが1987年にAppleで開発した**HyperCard**がある。**カード**と呼ばれる文書を単位に相互リンクを張り、スクリプト言語である**HyperTalk**によるプログラムを実行できる。

HyperCardにはネットワークを通じてデータをやり取りする機能はなく、スタンドアロンのWebのようなものであった。

### Web以前のハイパーメディアの問題点

Webは単方向リンクしかサポートしておらず、リンクが切れる可能性があり、バージョン管理やトランスクルージョン（Transclusion、参照によってある文書を別の文書に掲載すること）の機能がないなど、不完全なハイパーメディアに見える。

しかし、Web以前のハイパーメディアの最大の問題点はその複雑さにあり、必要最低限のリンク機構だけを備えているというシンプルさこそWebの成功の一因であるといえる。


## 分散システムの歴史

### 集中システムと分散システム

最初期のコンピュータは科学技術計算などの専用目的で作られていた。

1960年代には[タイムシェアリングシステム](/note/internet/chapters/02_history_of_the_internet.ja.md#タイムシェアリングシステム)の普及などにより、1つのコンピュータが複数の目的に利用できるようになった。

1970年代以降は、コンピュータの**ダウンサイジング化**や**マルチベンダ接続**が可能になり、複数のコンピュータを組み合わせて処理を分散させ、システム全体としての性能を向上させる手法が登場した。

### RPC

**RPC**（Remote Procedure Call）は[分散システム](./01_basic_knowledge_of_web.ja.md#分散システム)を実現するための技術のひとつで、リモートのサーバで実行しているプログラムをクライアント側から呼び出すことができる。

有名なRPCシステムには、Sun Microsystemsの**SunRPC**（ONC RPC）や**アポロ**、IBMとDECが共同開発した**DCE**（Distributed Computing Environment）がある。これらのシステムが開発されていた1980年代後半は**UNIX戦争**とも呼ばれる、UNIXベンダーによる標準化競争が激化していた時代で、各社が自社の分散システム技術を標準にしようとしていた。

### 分散オブジェクト

**分散オブジェクト**（Distributed Object）は、[RPC](#rpc)のような単なる関数の呼び出しではなく、オブジェクト自体をリモート側に配置する技術。

分散オブジェクトの代表例は、**CORBA**（Common Object Request Broker Architecture）やMicrosoftが開発した**DCOM**（Distributed Component Object Model）である。

CORBAやDCOMは、**IDL**（Interface Definition Language）でオブジェクトメソッドを定義し、ネットワーク越しにシリアライズしたメッセージを交換する。

### Web以前の分散システムの問題点

RPCは現在でも**NFS**（Network File System）などの実装に使われている。しかし、以下の問題点から現実的に動作するのは[イントラネット](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#インターネットの意味)環境までとされている。

- 性能劣化の問題<br>
    ネットワーク越しの関数呼び出しには、時間がかかる、ネットワークのオーバヘッドが生じる、といった問題がある。
- データ型変換の問題<br>
    プログラミング言語ごとにサポートするデータ型が異なるため、複数の言語が混在する環境ではデータ型の変換時に問題が発生する。
- インタフェースバージョンアップ時の互換性の問題<br>
    機能追加に伴ってサーバのインタフェースを更新した場合に、下位互換性を保つことが難しい。
- 負荷分散の問題<br>
    [RPC](#rpc)ベースのシステムでは一般的にサーバ側でクライアントの[アプリケーション状態](./05_http.md#httpのステートレス性)を保持する。そのため、サーバー間でその状態を共有する必要があり、スケールアップが難しい。


## Webの歴史

### Webの誕生

1980年代までにハイパーメディア構想が生まれ、インターネットが登場し、分散システムが構築された。1990年11月12日、スイスの**CERN**（European Organization for Nuclear Research、欧州原子核研究機構）のTim Berners-Leeが、[インターネット](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#インターネット)ベースの分散情報管理システムとして、**Web**の提案書を書いた。Berners-Leeは翌日から実装を開始し、その年の年末には最初のバージョンの[ブラウザ](./01_basic_knowledge_of_web.ja.md#ブラウザ)と[サーバ](./01_basic_knowledge_of_web.ja.md#クライアントとサーバ)を完成させた。

### Webの普及

Webの普及を推し進めたのは、1993年にイリノイ大学の**NCSA**（National Center for Supercomputing Application、米国立スーパーコンピュータ応用研究所）が公開した[ブラウザ](./01_basic_knowledge_of_web.ja.md#ブラウザ)**Mosaic**である。それまでの[ブラウザ](./01_basic_knowledge_of_web.ja.md#ブラウザ)ではテキストのみを扱っていたが、Mosaicは本文中にインラインで画像を混在させることができた。

### Webの標準化

Web以前のインターネット標準はすべて[IETF](/note/internet/chapters/03_standarization_of_tcpip.ja.md#isoとietf)（Internet Engineering Task Force）の[RFC](/note/internet/chapters/03_standarization_of_tcpip.ja.md#rfc)（Request for Comments）として定められてきた。

しかし、Webがあまりにも急速に普及してしまったため、[IETF](/note/internet/chapters/03_standarization_of_tcpip.ja.md#isoとietf)での仕様策定が追いつかず、各社の実装がバラバラで相互運用性に欠ける状態が発生した。**Netscape Navigator**と**Internet Explorer**が独自拡張を繰り返した結果、HTMLとCSSのレンダリング結果が大きく異なった。当時の状況を**ブラウザ戦争**と呼ぶこともある。

そこで、Webの標準化を行う団体として、1994年にBerners-Leeが中心となって**W3C**（World Wide Web Consortium）が設立された。W3Cでは、HTML、XML、[HTTP](./05_http.ja.md#httpの基本)、[URI](./04_uri.ja.md#uriの仕様)、CSSなどの標準化作業が行われた。

### RESTの誕生

カリフォルニア大学アーバイン校の大学院生だったRoy Fieldingは、Web創成期から各種ソフトウェア（Apache httpd、libwww-perl、www-statなど）の実装に携わってきた。さらにFieldingは、Berners-LeeらとともにHTTP1.0とHTTP1.1の仕様策定にも関わった。

彼はWebをソフトウェアアーキテクチャの観点から分析し、1つの[アーキテクチャスタイル](./03_rest.ja.md#アーキテクチャスタイル)としてまとめた。これを[REST](./03_rest.ja.md#restアーキテクチャスタイル)（Representational State Transfer）という。

[HTTP](./05_http.ja.md#httpの基本)はもともと、ハイパーテキストを転送するための[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)であったが、Fieldingの主張では[HTTP](./05_http.ja.md#httpの基本)は**リソースの状態**（Resource State）の**表現**（Representation）を運んでいる。

### 様々なハイパーメディアフォーマットの誕生

HTMLの構造はそのままに、HTMLに様々な意味を持たせることのできる技術として、**microformats**が登場した。

また、Webページの新着情報をサーバで配信して、専用のプログラムでそれをチェックするための用途として、**RSS**（RDF Site Summary、Rich Site Summary、Really Simple Syndication）が提案された。しかしRSSは複数のバージョンが乱立して混乱したため、最終的には[IETF](/note/internet/chapters/03_standarization_of_tcpip.ja.md#isoとietf)で[Atom](./06_hypermedia_format.ja.md#atom)が標準化された。

さらに、データを記述する目的としては冗長であったHTMLやAtomに代わり、**JSON**がデファクトスタンダードとなった。

### SOAP対REST

Web APIの標準化にあたって大きな勢力を持っていたのが[RPC/分散オブジェクト](#rpc)のグループであった。[RPC/分散オブジェクト](#rpc)グループの提唱した[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)は、[HTTP](./05_http.ja.md#httpの基本)をアプリケーション[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)ではなくトランスポートプロトコルとして扱い、[HTTP](./05_http.ja.md#httpの基本)上で独自のメッセージを転送することができる**SOAP**である。

SOAPはメッセージ転送の方法だけを定めた仕様であり、実際のサービスとしては**WS-\*** と呼ばれる周辺仕様群が、[W3C](#webの標準化)や**OASIS**（Organization for the Advancement of Structured Information Standards）によって次々と提案された。WS-\*には**WS-Security**や、**WS-Transaction**、**WS-ReliableMessaging**などがある。

これに対してFieldingは、SOAPベースの技術を否定し、WebがWebらしくあるためのアーキテクチャスタイルとして[REST](./03_rest.ja.md#restアーキテクチャスタイル)を推奨した。

Web以前から[分散オブジェクト](#分散オブジェクト)技術に関わってきた[分散オブジェクト](#分散オブジェクト)の技術者であるMark Baker（ハンドルネームdistobj）や、**SGML**（Standard Generalized Markup Language）の流れをくむXML/構造化文書技術者であるPaul Prescodは、Fieldingの強力な賛同者であった。

SOAPと[REST](./03_rest.ja.md#restアーキテクチャスタイル)に関する論争は2000年前後から始まり、2004年からスタートしたWeb2.0の流れの中でGoogleやAmazonといった企業が[REST](./03_rest.ja.md#restアーキテクチャスタイル)形式のWeb APIを提供し始めたことで決着がついた。Web2.0では、**マッシュアップ**（いろいろなWeb APIが提供する情報を組み合わせて1つのアプリケーションを実現する手法）が重要な要素であったため、手軽に操作できる[REST](./03_rest.ja.md#アーキテクチャスタイル)のスタイルが受け入れられた結果となった。

SOAPは多くのベンダーがドラフトを持ち寄って標準化を進めていたため、各社の実装が食い違い、相互運用性に欠けてしまった点も敗因といえる。
