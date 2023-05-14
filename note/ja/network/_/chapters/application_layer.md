# 『アプリケーション層』ノート

（最終更新： 2023-05-01）


## 目次

1. [遠隔ログイン](#遠隔ログイン)
	1. [TELNET](#telnet)
	1. [SSH](#ssh)
1. [ファイル転送](#ファイル転送)
	1. [FTP](#ftp)
1. [電子メール](#電子メール)
	1. [MIME](#mime)
	1. [SMTP](#smpt)
	1. [POP before SMTP](#pop-before-smtp)
	1. [SMTP認証](#smtp認証)
	1. [SPF](#spf)
	1. [DKIM](#dkim)
	1. [DMARC](#dmarc)
	1. [POP](#pop)
	1. [IMAP](#imap)
1. [WWW](#www)
	1. [HTTP](#http)
	1. [WebSocket](#websocket)
1. [ネットワーク管理](#ネットワーク管理)
	1. [SNMP](#snmp)
	1. [MIB](#mib)
	1. [RMON](#rmon)
1. [H.323](#h323)
1. [SIP](#sip)
1. [RTP](#rtp)
	1. [RTCP](#rtcp)
1. [P2P](#p2p)
1. [LDAP](#ldap)
1. [NTP](#ntp)
1. [制御システムのプロトコル](#制御システムのプロトコル)


## 遠隔ログイン

### TELNET

**TELNET**は、[TCP](./transport_layer.md#tcp)の[コネクション](./network.md#コネクション)を1つ利用して、通信相手の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)に[コマンド](../../../computer/linux/_/chapters/basic_command.md#コマンド)を文字列として送信し実行する[プロトコル](./network_architecture.md#プロトコル)。相手の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)内部で動作している[シェル](../../../computer/linux/_/chapters/shell_and_terminal.md#シェル)に接続しているような状態となる。

TELNETでは、ユーザが入力した文字以外にもオプションをやり取りすることができる。**NVT**(Network Virtual Terminal)を実現するための画面制御情報はこのオプション機能を利用して送信される。

またTELNETには、改行キーが入力されるごとに1行分のデータをまとめて送る**行モード**と、入力された文字を1文字ごとに送る**透過モード**がある。

TELNET[プロトコル](./network_architecture.md#プロトコル)を利用する[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)[プログラム](../../../programming/_/chapters/programming.md#プログラム)を**TELNETクライアント**といい、TELNETクライアントは基本的に23番[ポート](./address_on_network.md#ポート番号)に接続することでtelnetdとやり取りをする。しかし、それ以外の[ポート](./address_on_network.md#ポート番号)に接続することで、キーボードから各[プロトコル](./network_architecture.md#プロトコル)の[コマンド](../../../computer/linux/_/chapters/basic_command.md#コマンド)を直接入力することもできる。これを利用して、[TCP/IP](./communication_protocol.md#tcpip)[アプリケーション](../../../computer/software/_/chapters/software.md#応用ソフトウェア)開発時のデバッグに利用されることもある。

### SSH

**SSH**(Secure SHell)は、暗号化された遠隔ログインシステムで、一般的に22番[ポート](./address_on_network.md#ポート番号)が利用される。SSHの基本的な機能は以下の通り。

- 通信内容の暗号化
- ファイルの転送
- ポートフォワード機能

ポートフォワード機能は、特定の[ポート](./address_on_network.md#ポート番号)に届けられたメッセージを、特定の[IPアドレス](./address_on_network.md#ipアドレス)、[ポート番号](./address_on_network.md#ポート番号)に転送する仕組み。

SSHの認証には、パスワード認証の他にも**公開鍵認証**や**ワンタイムパスワード認証**が利用できる。


## ファイル転送

### FTP

**FTP**(File Transfer Protcol)は、異なる[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)間で[ファイル](../../../computer/software/_/chapters/file_system.md#ファイル)を転送するときに使われる[プロトコル](./network_architecture.md#プロトコル)。[インターネット](./network.md#インターネット)上には、誰でもログインできる**anonymous FTPサーバ**があり、これらはanonymousかftpというユーザ名でログインできる。anonymous FTPサーバは不特定多数への[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)の公開などのために用いられる。

FTPでは、2つの[TCP](./transport_layer.md#tcp)[コネクション](./network.md#コネクション)が利用され、1つは制御用（21番[ポート](./address_on_network.md#ポート番号)）で、もう1つはファイル転送用（20番[ポート](./address_on_network.md#ポート番号)）である。ファイル転送には、一般的に20番[ポート](./address_on_network.md#ポート番号)を用いるが、セキュリティ向上のために[ポート番号](./address_on_network.md#ポート番号)を乱数的に割り当てるのが一般的。


## 電子メール

電子メールサービスを提供するための[プロトコル](./network_architecture.md#プロトコル)は[SMTP](#smtp)で、[TCP](./transport_layer.md#tcp)を利用している。通常のユーザが利用する[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)は常に電源が入っているとは限らないので、常に起動している**メールサーバ**を経由してメッセージの送受信を行う。受信者がメールサーバから電子メールを受け取るための[プロトコル](./network_architecture.md#プロトコル)としては[POP](#pop)がある。

### MIME

**MEME**(Multipurpose Internet Mail Extensions)は、電子メールでテキスト以外のデータ形式を送信できるようにした拡張形式。MIMEを利用することで、画像や音声、動画などの[マルチメディア](../../../computer/software/_/chapters/multimedia.md#マルチメディア)データが添付できる。

### SMTP

**SMTP**(Simple Mail Transfer Protocol)は、電子メールを配送するアプリケーションプロトコルで、[TCP](./transport_layer.md#tcp)の25番[ポート](./address_on_network.md#ポート番号)を用いる。SMTPには認証の仕組みがないため、迷惑メール（スパムメール）を送り付けるような悪用が簡単にできてしまう。

### POP before SMTP

**POP before SMTP**は、[POP](#pop)によるユーザ認証を行い、認証が正しければ一定期間[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)[IPアドレス](./address_on_network.md#ipアドレス)からの[SMTP](#smtp)通信を受け入れる仕組み。

### SMTP認証

**SMTP認証**(SMTP Authentication)は、メール送信時に[SMTP](#smtp)[サーバ](../../../computer/_/chapters/computer.md#サーバ)でユーザ認証を行うようにした仕組み。

### SPF

**SPF**(Sender Policy Framework)は、送信元メールサーバの[IPアドレス](./address_on_network.md#ipアドレス)を[DNS](./internet_layer.md#dns)[サーバ](../../../computer/_/chapters/computer.md#サーバ)に登録しておき、受信側で受信したメールの[IPアドレス](./address_on_network.md#ipアドレス)と送信元メールサーバの[IPアドレス](./address_on_network.md#ipアドレス)を確認してドメイン認証をすることで、なりすましを防止する仕組み。

### DKIM

**DKIM**(DomainKeys Identified Mail)は、送信元メールサーバで電子署名を付与し、受信側では電子署名を認証することで、なりすましを防止する仕組み。送信元は公開鍵を[DNS](./internet_layer.md#dns)[サーバ](../../../computer/_/chapters/computer.md#サーバ)に登録しておく。

### DMARC

**DMARC**(Domain-based Message Authentication, Reporting and Conformance)は、[SPF](#spf)や[DKIM](#dkim)など送信元ドメインを認証する仕組みにおいて、認証が失敗した場合のメールの取り扱いポリシーを送信者が[DNS](./internet_layer.md#dns)に登録しておく仕組み。

### POP

**POP**(Post Office Protocol)は、電子メールの受信[ホスト](./network.md#ホスト)がメールサーバからメールを受け取るための[プロトコル](./network_architecture.md#プロトコル)。現在は主に**POP3**(POP version 3.0)が使われている。

### IMAP

**IMAP**(Internet Message Access Protocol)は、[POP](#pop)と同様に電子メールなどのメッセージを受信するための[プロトコル](./network_architecture.md#プロトコル)。IMAPでは、サーバ上の電子メールを全てダウンロードすることなく電子メールを読むことができる。

IMAPを使用することにより、[サーバ](../../../computer/_/chapters/computer.md#サーバ)上に補完されているメールを、あたかも自分の使う[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)の記憶媒体のように使うことができる。そのため、ある端末から一度開いたメールは、他の端末から見ても既読したことになっている。


## WWW

**WWW**(World Wide Web)は、[インターネット](./network.md#インターネット)上の情報をハイパーテキスト形式で参照できる情報提供[システム](../../../system/_/chapters/system.md#システム)。単に[Web](./web.md#web)と呼ばれることも多い。

### HTTP

**HTTP**(HyperText Transfer Protocol)は、HTML文書や画像、音声、動画などのコンテンツ送受信に用いられる[プロトコル](./network_architecture.md#プロトコル)で、TCPの80番[ポート](./address_on_network.md#ポート番号)を用いる。

HTTPでは、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)がHTTP[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)（[Webサーバ](./web.md#webサーバ)）に情報を要求（[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)）し、この要求に対してHTTP[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)が[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)に情報を返却（[レスポンス](../../../system/_/chapters/system_processing_model.md#レスポンス)）する。

HTTP1.0は、[IETF](./communication_protocol.md#ietf)で[標準化](./communication_protocol.md#標準化の流れ)が行われた最初の[バージョン](../../../computer/software/_/chapters/package.md#バージョン)である。HTTP1.0の最初の[ドラフト](./communication_protocol.md#標準化の流れ)は1993年に公開され、3年後の1996年に最終[バージョン](../../../computer/software/_/chapters/package.md#バージョン)が公開された。この時期はNetscape NavigatorやInternet Explorerの[ブラウザ](./web.md#webブラウザ)戦争が最も激化していたため、仕様と実装の乖離が生じてしまった。HTTP1.0では、ヘッダの導入、[GET](./web.md#httpメソッド)以外のメソッドの追加などが行われた。

HTTP1.1は1997年に策定されて、1999年から2015年まで利用されていた。HTTP1.1では、チャンク転送、Acceptヘッダによるコンテントネゴシエーション、複雑な[キャッシュ](./web.md#キャッシュ)コントロール、持続的接続などの機能を追加している。また、パイプラインという前の[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)の転送が完了する前に次の[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)を転送できる機能や、バーチャルホストという1つの[Webサーバ](./web.md#webサーバ)で別々の異なる[ドメイン](./internet_layer.md#ドメイン名)の[ホームページ](./web.md#web)が公開できる仕組みが搭載された。

HTTP2.0は2015年に公開された。複数の[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)を同時に処理可能になり、ヘッダの圧縮やサーバプッシュ、転送するコンテンツの優先度設定などの複数の機能追加が行われた。

HTTP3.0は2018年に公開された。[TCP](./transport_layer.md#tcp)ではなく、[UDP](./transport_layer.md#udp)と[QUIC](./transport_layer.md#quic)という[プロトコル](./network_architecture.md#プロトコル)上で動作するアプリケーションプロトコルである。HTTP3.0では、暗号化通信が[プロトコル](./network_architecture.md#プロトコル)自体に組み込まれ、[スリーウェイハンドシェイク](./transport_layer.md#スリーウェイハンドシェイク)の必要がないため接続が高速であるなどの特徴がある。

### WebSocket

**WebSocket**は、[クライアント](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)と[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)の間で双方向通信を実現するための[プロトコル](./network_architecture.md#プロトコル)。最初に[HTTP](#http)通信を行い、upgrade[リクエスト](../../../system/_/chapters/system_processing_model.md#リクエスト)によってWebSocket用の通信路を確立する。


## ネットワーク管理

### SNMP

**SNMP**(Simple Network Management Protocol)は、[ネットワーク](./network.md#ネットワーク)管理に用いられる[プロトコル](./network_architecture.md#プロトコル)で、[UDP](./transport_layer.md#udp)/[IP](./internet_layer.md#ip)上で動作する。

SNMPでは、管理する側を**マネージャ**（ネットワーク監視端末）、管理される側を**エージェント**（[ルータ](./network_architecture.md#ルータ)、[スイッチ](./network_architecture.md#スイッチ)など）と呼ぶ。

SNMPでの処理は機器へのデータの書き込みと読み込みに集約される。この方法を、**フェッチ/ストアパラダイム**と呼ぶ。

### MIB

**MIB**(Management Information Base)は、[SNMP](#snmp)でやり取りされる情報で、ツリー型の構造をもった管理情報[データベース](../../../development/database/_/chapters/database.md#データベース)となっている。 MIBには、**標準MIB**と各メーカーが独自に作成した**拡張MIB**がある。

### RMON

**RMON**(Remote Monitoring MIB)は、通常の[MIB](#mib)が[ネットワーク](./network.md#ネットワーク)機器のインタフェース（点）を監視するのに対し、接続される[ネットワーク](./network.md#ネットワーク)の回線（線）を監視する。

ある特定の[ホスト](./network.md#ホスト)がどこの誰と、どのような[プロトコル](./network_architecture.md#プロトコル)で通信しているかという統計情報を知ることができる。


## H.323

**H.323**は、[IP](./internet_layer.md#ip)[ネットワーク](./network.md#ネットワーク)上で音声や映像をやり取りするための[プロトコル](./network_architecture.md#プロトコル)体系。


## SIP

**SIP**は、[H.323](#h323)より後に開発された、[インターネット](./network.md#インターネット)での利用によりマッチした[プロトコル](./network_architecture.md#プロトコル)。


## RTP

**RTP**(Real-Time Transport Protocol)は、[UDP](./transport_layer.md#udp)でリアルタイムな[マルチメディア](../../../computer/software/_/chapters/multimedia.md#マルチメディア)通信を実現するために、[アプリケーション層](./communication_protocol.md#アプリケーション層)で[シーケンス番号](./transport_layer.md#シーケンス番号)や[パケット](./network.md#パケット)送信時刻の管理をする[プロトコル](./network_architecture.md#プロトコル)。

### RTCP

**RTCP**(RTP Control Protocol)は、[パケット](./network.md#パケット)喪失率など通信回線の品質を管理することで、[RTP](#rtp)による通信を補助する[プロトコル](./network_architecture.md#プロトコル)。


## P2P

**P2P**(Peer To Peer)は、[ネットワーク](./network.md#ネットワーク)上に展開する各端末や[ホスト](./network.md#ホスト)が[サーバ](../../../system/_/chapters/system_processing_model.md#クライアントサーバシステム)などを介さずに1対1で直接接続して通信する形態。


## LDAP

**LDAP**(Lightweight Directory Access Protocol)は、ユーザ名やパスワードなどの情報を一元管理する仕組みである、ディレクトリサービスへのアクセスに使われる[プロトコル](./network_architecture.md#プロトコル)。**ディレクトリサービス**は、ネットワーク上の資源に関してデータベース的な情報提供を行うサービスである。


## NTP

**NTP**(Network Time Protocol)は、[ネットワーク](./network.md#ネットワーク)に接続される機器の時刻を同期するためのアプリケーションプロトコル。


## 制御システムのプロトコル

制御システムは、**OT**(Operational Technology)や**ICS**(Industrial Control Systems)とも呼ばれ、機器や装置の監視や制御、およびそれを自動的に行うPID制御などのプロセス制御で使用される。
