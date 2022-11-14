# アプリケーション層


## 目次

1. [通信の処理](#通信の処理)
	1. [デーモン](#デーモン)
1. [遠隔ログイン](#遠隔ログイン)
	1. [TELNET](#telnet)
	1. [SSH](#ssh)
1. [ファイル転送](#ファイル転送)
	1. [FTP](#ftp)
1. [電子メール](#電子メール)
	1. [電子メールの仕組み](#電子メールの仕組み)
	1. [メールアドレス](#メールアドレス)
	1. [MIME](#mime)
	1. [SMTP](#smtp)
	1. [POP](#pop)
	1. [IMAP](#imap)
1. [WWW](#www)
	1. [URI](#uri)
	1. [HTML](#html)
	1. [HTTP](#http)
	1. [HTTPにおける認証](#httpにおける認証)
	1. [次世代のHTTP](#次世代のhttp)
	1. [JavaScript](#javascript)
	1. [CGI](#cgi)
	1. [クッキー](#クッキー)
	1. [WebSocket](#websocket)
1. [ネットワーク管理](#ネットワーク管理)
	1. [SNMP](#snmp)
	1. [MIB](#mib)
	1. [RMON](#rmon)
1. [その他のアプリケーションプロトコル](#その他のアプリケーションプロトコル)
	1. [マルチメディア通信を実現する技術](#マルチメディア通信を実現する技術)
	1. [RTP](#rtp)
	1. [デジタル圧縮技術](#デジタル圧縮技術)
	1. [P3P](#p2p)
	1. [LDAP](#ldap)
	1. [NTP](#ntp)
	1. [制御システムのプロトコル](#制御システムのプロトコル)


## 通信の処理

### デーモン

クライアントサーバシステムにおいて、サービスを提供するコンピュータではあらかじめサーバープログラムを起動しておいて、クライアントプログラムからの要求を待つ必要がある。このようなプログラムを、UNIXでは**デーモン**と呼ぶ。

[HTTP](#http)サーバーのプログラムは**httpd**（[HTTP](#http)デーモン）、[SSH](#ssh)サーバーのプログラムは**sshd**（[SSH](#ssh)デーモン）というような呼び方をする。個々のデーモンを別々に動かすのではなく、代表としてクライアントの要求を待つ**inetd**（インターネットデーモン）という**スーパーデーモン**を使うこともある。スーパーデーモンはサービスの要求を受け付けると**分身**（**fork**）して、サービスのデーモンに**変身**（**exec**）する。


## 遠隔ログイン

### TELNET

**TELNET**は、[TCP](/note/internet/chapters/08_transport_layer.ja.md#tcp)の[コネクション](/note/internet/chapters/08_transport_layer.ja.md#コネクション)を1つ利用して、この通信路を通して相手のコンピュータにコマンドを文字列として送信し実行する[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)。相手のコンピュータ内部で動作している**シェル**に接続しているようなイメージ。シェルとは、OSが提供する機能をユーザーが利用しやすいように包み込んだユーザインタフェースのこと。

TELNETではユーザが入力した文字以外にもオプションをやり取りすることができる。**NVT**（Network Virtual Terminal）を実現するための画面制御情報はこのオプションの機能を利用して送信される。

またTELNETには、改行キーが入力されるごとに1行分のデータをまとめて送る**行モード**と、入力された文字を1文字ごとに送る**透過モード**がある。

TELNET[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)を利用するクライアントプログラムを**TELNETクライアント**といい、TELNETクライアントは基本的に**23番**ポートに接続することでtelnetdとやり取りをする。しかし、それ以外のポートに接続することで、キーボードから各[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)のコマンドを直接入力することもできる。これを利用して、TCP/IPアプリケーション開発時のデバッグに利用されることもある。

### SSH

**SSH**（Secure SHell）は、暗号化された遠隔ログインシステムで、**22番**ポートが利用される。SSHの基本的な機能は以下の通り。

- 通信内容の暗号化
- ファイルの転送
- ポートフォワード機能

ポートフォワード機能とは、特定のポートに届けられたメッセージ、特定の[IPアドレス](/note/internet/chapters/07_internet_layer.ja.md#ipアドレス)、[ポート番号](/note/internet/chapters/08_transport_layer.ja.md#ポート番号)に転送する仕組み。

SSHの認証には、パスワード認証のほかにも**公開鍵認証**や**ワンタイムパスワード認証**が利用できる。


## ファイル転送

### FTP

**FTP**（File Transfer Protocol）は、異なるコンピュータ間でファイルを転送するときに使われる[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)。インターネット上には、誰でもログインできる**anonymous FTPサーバー**があり、これらはanonymousかftpというログイン名でログインできる。anonymous FTPサーバは不特定多数へのソフトウェアの公開などのために用いられる。

FTPでは2つの[TCPコネクション](/note/internet/chapters/08_transport_layer.ja.md#コネクション)が利用され、1つは制御用（**21番**ポート）で、もう1つはデータ（ファイル）の転送用（**20番**ポート）である。データ転送には一般的に20番ポートを用いるが、セキュリティ向上のためにポート番号を乱数的に割り当てるのが一般的。


## 電子メール

### 電子メールの仕組み

電子メールサービスを提供するための[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)は**SMTP**（Simple Mail Transfer Protocol）で、[TCP](/note/internet/chapters/08_transport_layer.ja.md#tcp)を利用している。通常のユーザーが利用するコンピュータは電源が常に入っているとは限らないので、電源を切らない**メールサーバー**を経由してメッセージの送受信を行う。受信者がメールサーバーから電子メールを受け取るための[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)は**POP**（Post Office Protocol）である。

### メールアドレス

メールアドレスは、電子メールのやり取りを行う際に必要となる識別子。現在この電子メールの配送先の管理は[DNS](/note/internet/chapters/07_internet_layer.ja.md#dns)によって行われる。[DNS](/note/internet/chapters/07_internet_layer.ja.md#dns)には、メールアドレスと、そのメールアドレス宛てのメールを送信すべきメールサーバーの[ドメイン名](/note/internet/chapters/07_internet_layer.ja.md#dnsの役割)を登録しておく。 これを[MXレコード](/note/internet/chapters/07_internet_layer.ja.md#ネームサーバー)という。

### MIME

もともと電子メールにはテキスト形式しか使えなかったが、現在は電子メールで転送できるデータ形式を拡張する**MIME**（Multipurpose Internet Mail Extensinos）が一般的となり、画像や動画、音声などを添付できるようになった。

### SMTP

**SMTP**（Simple Mail Trnasfer Protocol）は電子メールを配送するアプリケーション[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)で、[TCP](/note/internet/chapters/08_transport_layer.ja.md#tcp)の**25番**ポートを用いる。SMTPには認証の仕組みがないため、**迷惑メール**（スパムメール）を送り付けるような悪用が簡単にできてしまう。この迷惑メールに対しては、様々な対策が行われている。

**POP before SMTP**は、[POP](#pop)によるユーザー認証を行い、認証が正しければ一定期間クライアント[IPアドレス](/note/internet/chapters/07_internet_layer.ja.md#ipアドレス)からのSMTP通信を受け入れる仕組み。

**SMTP認証**（SMTP Authentication）は、メール送信時にSMTPサーバーでユーザー認証を行うようにした仕組み。

**SPF**（Sender Policy Framework）は、送信元メールサーバーの[IPアドレス](/note/internet/chapters/07_internet_layer.ja.md#ipアドレス)を[DNS](/note/internet/chapters/07_internet_layer.ja.md#dns)サーバーに登録しておき、受信側で受信したメールの[IPアドレス](/note/internet/chapters/07_internet_layer.ja.md#ipアドレス)と送信元メールサーバーの[IPアドレス](/note/internet/chapters/07_internet_layer.ja.md#ipアドレス)を確認してドメイン認証し、なりすましを防止する仕組み。

**DKIM**（DomainKeys Identified Mail）は、送信元メールサーバーで電子署名を付与し、受信側では電子署名を認証することで、なりすましを防止する仕組み。 送信元は公開鍵を[DNS](/note/internet/chapters/07_internet_layer.ja.md#dns)サーバーに登録しておく。

**DMARC**（Domain-based Message Authentication, Reporting and Conformance）は、SPFやDKIMなど送信元ドメインを認証する仕組みにおいて、認証が失敗した場合のメールの取り扱いポリシーを送信者が[DNS](/note/internet/chapters/07_internet_layer.ja.md#dns)に登録しておく仕組み。

**OP25B**（Outbound Port 25 Blocking）は、[ISP](/note/internet/chapters/02_history_of_the_internet.ja.md#商用インターネットサービスの開始)などで迷惑メールやウィルスメールを直接送信できないように25番のSMTP通信をブロックする対策。

### POP

**POP**（Post Office Protocol）は、電子メールの受信ホストがメールサーバーからメールを受け取るための[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)。現在は主に**POP3**（POP version 3.0）が使われている。

### IMAP

**IMAP**（Internet Message Access Protocol）は、[POP](#pop)と同様に電子メールなどのメッセージを受信するための[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)。IMAPでは、サーバー上の電子メールをすべてダウンロードすることなく電子メールを読むことができる。

IMAPを使用することにより、サーバー上に保管されているメールを、あたかも自分の使うクライアントの記憶媒体のように使うことができる。


## WWW

**WWW**（World Wide Web）は、[インターネット](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#インターネット)上の情報をハイパーテキスト形式で参照できる情報提供システム。単に**Web**と呼ばれることも多い。

Webの情報を画面に表示するクライアントソフトウェアを、**Webブラウザ**と呼ぶ。代表的なWebブラウザとしては、Microsoft社のMicrosoft EdgeやMozila FoundationのFirefox、Google社のGoogle Chrome、Opera Software社のOpera、Apple社のSafariなどがある。

また、**検索エンジン**を使うことで広大な[インターネット](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#インターネット)上の情報に容易にアクセスすることができる。

Webブラウザの画面に表示されるイメージ全体は、**Webページ**と呼ばれる。会社や学校などの組織や、個人のWebページの見出しとなるページを**ホームページ**と呼ぶこともある。

### URI

**URI**（Uniform Resource Identifier）は[インターネット](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#インターネット)上の資源（リソース）を表す識別子として利用される。

一般的にWebではこの識別子のことを、ホームページのアドレスや**URL**（Uniform Resource Locator）と呼ぶ。URIはURLよりも広義の概念として用いられる。

URIが表す枠組みを**スキーム**といい、Webではhttpやhttpsといったスキームが使われる。**ホスト名**は、[ドメイン名](/note/internet/chapters/07_internet_layer.ja.md#dnsの役割)や[IPアドレス](/note/internet/chapters/07_internet_layer.ja.md#ipアドレス)を表す。

### HTML

**HTML**（HyperText Markup Language）は、Webページを記述するための言語あるいはデータ形式のこと。HTMLには画面に表示する文字や画像に**リンク**をはって別の情報と紐づける、**ハイパーテキスト**という機能がある。

HTMLは[WWW](#www)の[プレゼンテーション層](/note/internet/chapters/04_osi_reference_model.ja.md#プレゼンテーション層)の機能であるといえる。しかし、この[プレゼンテーション層](/note/internet/chapters/04_osi_reference_model.ja.md#プレゼンテーション層)は完全には整備されていないため、OSや[ブラウザ](#www)が違うと表示の細かい部分が異なる場合がある。

[WWW](#www)においてデータをファイルに保存したり、アプリケーション間でやり取りしたりする形式としては**XML**（Extensible Markup Language）が多く利用されている。XMLは**SGML**（Standard Generalized Markup Language）から派生した言語で、HTMLに似た記法を用いる。Java（Oracle社（旧 Sun Microsystems）が開発している、プラットフォームに依存しないプログラミング言語・実行環境）とXMLを組み合わせたアプリケーションも多い。

**HTML5**と呼ばれる新しい規格では、標準で音声や動画を再生できるようになり、様々なAPIを組み込んだWebアプリケーションを作りやすくなった。

**CSS**（Cascading Style Sheet）は、HTMLの要素をどのように表示するかを指定できるスタイルシート言語。**CSS3**では、より柔軟にWebページをデザインできるようになった。

HTML5とCSS3の組み合わせにより、パソコンやスマートフォンなど画面の大きさの違う端末に合わせたデザイン（**レスポンシブWebデザイン**）が行いやすくなった。

### HTTP

**HTTP**（HyperText Transfer Protocol）は、[HTML](#html)文書や画像、音声、動画などのコンテンツの送受信に用いられる[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)で、[TCP](/note/internet/chapters/08_transport_layer.ja.md#tcp)の**80番**ポートを用いる。

HTTPでは、クライアントがHTTPサーバー（Webサーバー）に情報を要求（**リクエスト**）し、この要求に対してHTTPサーバーがクライアントに情報を送信（**レスポンス**）する。HTTPサーバーはクライアントの情報を保持しない、**ステートレスサーバー**である。

HTTP1.0では1つのリクエスト・レスポンスをやり取りするたびに[TCPコネクション](/note/internet/chapters/08_transport_layer.ja.md#コネクション)を確立・切断していた。HTTP1.1では、1つの[TCPコネクション](/note/internet/chapters/08_transport_layer.ja.md#コネクション)で複数のリクエスト・レスポンスができるようになる、**キープアライブ**（keep-alive）が実装された。

### HTTPにおける認証

**Basic認証**は、base64でエンコードされたユーザーIDとパスワードを用いて認証を行う。基本的には平文でネットワークを流れるので安全ではなく、HTTPSの暗号化通信と組み合わせて使うことが推奨される。

**Digest認証**は、Basic認証の欠点であった平文で認証情報が流れてしまうことを改善した認証方式。ユーザーIDとパスワードを**MD5**でハッシュ化して送信する。 この暗号化も近年解析が可能になり、安全性に懸念があることが明らかになっているため、HTTPSと合わせて使用するのが一般的である。

### 次世代のHTTP

**HTTP/2**では1つの接続での並列処理や、バイナリデータの使用による送受信のデータ量の削減、ヘッダ圧縮、サーバープッシュなどの技術が導入され、ネットワークリソースの効率化を実現している。

**HTTP/3**は、[TCP](/note/internet/chapters/08_transport_layer.ja.md#tcp)の[スリーウェイハンドシェイク](/note/internet/chapters/08_transport_layer.ja.md#コネクション管理)のない[UDP](/note/internet/chapters/08_transport_layer.ja.md#udp)を使う、[HTTP over QUIC](/note/internet/chapters/08_transport_layer.ja.md#quic)が用いられる。

### JavaScript

[URI](#uri)、[HTML](#html)、[HTTP](#http)だけでは、条件に応じて動的にユーザーに表示する内容を変更することはできない。そこで、[HTML](#html)に埋め込み[Webブラウザ](#www)上で動作するプログラミング言語である**JavaScript**がある。JavaScriptのように[Webブラウザ](#www)上で動くプログラムを**クライアントサイドアプリケーション**という。

[HTML](#html)や[XML](#html)ドキュメントの論理的な構造（**DOM**: Document Object Model）をJavaScriptで操作することで、動的なWebサイトを作成することが可能となっている。また、サーバーからWebページ全体を読み込むことなくデータのやり取りを行う、**Ajax**（Asynchronous JavaScript and XML）という技術もある。

### CGI

**CGI**（Common Gateway Interface）は、Webサーバーが外部プログラムを呼び出す**サーバーサイドアプリケーション**の仕組み。CGIを使うとユーザーの操作に応じてさまざまに変化する情報を転送することができる。掲示板やネットショッピングなどのWebサービスでは、CGIを使用して外部プログラムを呼び出したり、データベースにアクセスしたりしているものがある。

### クッキー

Webアプリケーションでユーザーの情報を識別する**クッキー**（Cookie）という仕組みがある。これは、Webサーバーがクライアント側に情報を格納する技術である。

### WebSocket

チャットアプリやゲームアプリなど、クライアントとサーバー間の双方向通信を[HTTP](#http)上で実現する[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)として**WebSocket**が開発された。

WebSocketでは、まずクライアントとサーバー間で[HTTP](#http)通信を行い、[HTTP](#http)のupgradeリクエスト/レスポンスでWebSocket用の通信路を確立する。


## ネットワーク管理

### SNMP

**SNMP**（Simple Network Management Protocol）はネットワーク管理に用いられる[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)で、UDP/IP上で動作する。

SNMPでは、管理する側を**マネージャ**（ネットワーク監視端末）、管理される側を**エージェント**（ルーター、スイッチなど）と呼ぶ。

SNMPでの処理は機器へのデータの書き込みと読み込みに集約される。この方法を、**フェッチ/ストアパラダイム**と呼ぶ。

### MIB

[SNMP](#snmp)でやり取りされる情報を**MIB**（Management Information Base）と呼ぶ。MIBは、ツリー型の構造をもった管理情報データベース。 MIBには、標準MIBと各メーカーが独自に作成した拡張MIBがある。

### RMON

**RMON**（Remote Monitoring MIB）は、通常のMIBがネットワーク機器のインタフェース（点）を監視するのに対し、接続されるネットワークの回線（線）を監視する。

ある特定の[ホスト](/note/internet/chapters/07_internet_layer.ja.md#ホストとルーターとノード)がどこの誰と、どのような[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)で通信しているかという統計情報を知ることができる。


## その他のアプリケーションプロトコル

### マルチメディア通信を実現する技術

**H.323**は[IP](/note/internet/chapters/07_internet_layer.ja.md#ip)ネットワーク上で音声や映像をやり取りするための[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)体系。

**SIP**はH.323より後に開発されて、[インターネット](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#インターネット)での利用によりマッチしている[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)。

### RTP

[UDP](/note/internet/chapters/08_transport_layer.ja.md#udp)でリアルタイムなマルチメディア通信を実現するには、アプリケーション層でシーケンス番号やパケット送信時刻の管理をする必要がある。これを行うのが**RTP**（Real-Time Transport Protocol）である。

また、**RTCP**（RTP Control Protocol）はパケット喪失率など通信回線の品質を管理することで、RTPによる通信を補助する。

### デジタル圧縮技術

[インターネット](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#インターネット)上では、限られたネットワーク資源でマルチメディアデータを送受信するために圧縮技術が必須となっている。**MPEG**（Moving Picture Experts Group）はデジタル圧縮の規格を決めるグループで、音楽圧縮で利用される**MP3**もMPEG規格のひとつ。

### P2P

ネットワーク上に展開する各端末や[ホスト](/note/internet/chapters/07_internet_layer.ja.md#ホストとルーターとノード)がサーバーなどを介さずに1対1で直接接続して通信する形態を**P2P**（Peer To Peer）という。

### LDAP

**LDAP**（Lightweight Directory Access Protocol）は、ユーザー名やパスワードなどの情報を一元管理する仕組みである**ディレクトリサービス**へのアクセスに使われる。ディレクトリサービスは、ネットワーク上の資源に関してデータベース的な情報提供を行うサービスである。

### NTP

**NTP**（Network Time Protocol）は、ネットワークに接続される機器の時刻を同期するためのアプリケーション[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)。

### 制御システムのプロトコル

制御システムは、**OT**（Operational Technology）や**ICS**（Industrial Control Systems）とも呼ばれ、機器や装置の監視や制御、およびそれを自動的に行うPID制御などのプロセス制御で使用される。
