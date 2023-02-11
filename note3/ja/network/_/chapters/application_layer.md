# 『アプリケーション層』

（最終更新： 2023-02-10）


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
1. [H.323](#h-323)
1. [SIP](#sip)
1. [RTP](#rtp)
	1. [RTCP](#rtcp)
1. [P2P](#p2p)
1. [LDAP](#ldap)
1. [NTP](#ntp)
1. [制御システムのプロトコル](#制御システムのプロトコル)


## 遠隔ログイン

### TELNET

**TELNET**は、TCPのコネクションを1つ利用して、通信相手のコンピュータにコマンドを文字列として送信し実行するプロトコル。相手のコンピュータ内部で動作しているシェルに接続しているような状態となる。

TELNETでは、ユーザが入力した文字以外にもオプションをやり取りすることができる。**NVT**(Network Virtual Terminal)を実現するための画面制御情報はこのオプション機能を利用して送信される。

またTELNETには、改行キーが入力されるごとに1行分のデータをまとめて送る**行モード**と、入力された文字を1文字ごとに送る**透過モード**がある。

TELNETプロトコルを利用するクライアントプログラムを**TELNETクライアント**といい、TELNETクライアントは基本的に23番ポートに接続することでtelnetdとやり取りをする。しかし、それ以外のポートに接続することで、キーボードから各プロトコルのコマンドを直接入力することもできる。これを利用して、TCP/IPアプリケーション開発時のデバッグに利用されることもある。

### SSH

**SSH**(Secure SHell)は、暗号化された遠隔ログインシステムで、一般的に22番ポートが利用される。SSHの基本的な機能は以下の通り。

- 通信内容の暗号化
- ファイルの転送
- ポートフォワード機能

ポートフォワード機能は、特定のポートに届けられたメッセージを、特定のIPアドレス、ポート番号に転送する仕組み。

SSHの認証には、パスワード認証の他にも**公開鍵認証**や**ワンタイムパスワード認証**が利用できる。


## ファイル転送

### FTP

**FTP**(File Transfer Protcol)は、異なるコンピュータ間でファイルを転送するときに使われるプロトコル。インターネット上には、誰でもログインできる**anonymous FTPサーバ**があり、これらはanonymousかftpというユーザ名でログインできる。anonymous FTPサーバは不特定多数へのソフトウェアの公開などのために用いられる。

FTPでは、2つのTCPコネクションが利用され、1つは制御用（21番ポート）で、もう1つはファイル転送用（20番ポート）である。ファイル転送には、一般的に20番ポートを用いるが、セキュリティ向上のためにポート番号を乱数的に割り当てるのが一般的。


## 電子メール

電子メールサービスを提供するためのプロトコルはSMTPで、TCPを利用している。通常のユーザが利用するコンピュータは常に電源が入っているとは限らないので、常に起動している**メールサーバ**を経由してメッセージの送受信を行う。受信者がメールサーバから電子メールを受け取るためのプロトコルとしてはPOPがある。

### MIME

**MEME**(Multipurpose Internet Mail Extensions)は、電子メールでテキスト以外のデータ形式を送信できるようにした、拡張形式。MMIMEを利用することで、画像や音声、動画などが添付できる。

### SMTP

**SMTP**(Simple Mail Transfer Protocol)は、電子メールを配送するアプリケーションプロトコルで、TCPの25番ポートを用いる。SMTPには認証の仕組みがないため、迷惑メール（スパムメール）を送り付けるような悪用が簡単にできてしまう。

### POP before SMTP

**POP before SMTP**は、POPによるユーザ認証を行い、認証が正しければ一定期間クライアントIPアドレスからのSMTP通信を受け入れる仕組み。

### SMTP認証

**SMTP認証**(SMTP Authentication)は、メール送信時にSMTPサーバでユーザ認証を行うようにした仕組み。

### SPF

**SPF**(Sender Policy Framework)は、送信元メールサーバのIPアドレスをDNSサーバに登録しておき、受信側で受信したメールのIPアドレスと送信元メールサーバのIPアドレスを確認してドメイン認証をすることで、なりすましを防止する仕組み。

### DKIM

**DKIM**(DomainKeys Identified Mail)は、送信元メールサーバで電子署名を付与し、受信側では電子署名を認証することで、なりすましを防止する仕組み。送信元は公開鍵をDNSサーバに登録しておく。

### DMARC

**DMARC**(Domain-based Message Authentication, Reporting and Conformance)は、SPFやDKIMなど送信元ドメインを認証する仕組みにおいて、認証が失敗した場合のメールの取り扱いポリシーを送信者がDNSに登録しておく仕組み。

### POP

**POP**(Post Office Protocol)は、電子メールの受信ホストがメールサーバからメールを受け取るためのプロトコル。現在は主に**POP3**(POP version 3.0)が使われている。

### IMAP

**IMAP**(Internet Message Access Protocol)は、POPと同様に電子メールなどのメッセージを受信するためのプロトコル。IMAPでは、サーバ上の電子メールを全てダウンロードすることなく電子メールを読むことができる。

IMAPを使用することにより、サーバ上に補完されているメールを、あたかも自分の使うクライアントの記憶媒体のように使うことができる。そのため、ある端末から一度開いたメールは、他の端末から見ても既読したことになっている。


## WWW

**WWW**(World Wide Web)は、インターネット上の情報をハイパーテキスト形式で参照できる情報提供システム。単に**Web**と呼ばれることも多い。

### HTTP

**HTTP**(HyperText Transfer Protocol)は、HTML文書や画像、音声、動画などのコンテンツ送受信に用いられるプロトコルで、TCPの80番ポートを用いる。

HTTPでは、クライアントがHTTPサーバ（Webサーバ）に情報を要求（リクエスト）し、この要求に対してHTTPサーバがクライアントに情報を返却（レスポンス）する。HTTPサーバはクライアントの情報を保持しないステートレスサーバである。

### WebSocket

**WebSocket**は、クライアントとサーバの間で双方向通信を実現するためのプロトコル。最初にHTTP通信を行い、upgradeリクエストによってWebSocket用の通信路を確立する。


## ネットワーク管理

### SNMP

**SNMP**(Simple Network Management Protocol)は、ネットワーク管理に用いられるプロトコルで、UDP/IP上で動作する。

SNMPでは、管理する側を**マネージャ**（ネットワーク監視端末）、管理される側を**エージェント**（ルーター、スイッチなど）と呼ぶ。

SNMPでの処理は機器へのデータの書き込みと読み込みに集約される。この方法を、**フェッチ/ストアパラダイム**と呼ぶ。

### MIB

**MIB**(Management Information Base)は、SNMPでやり取りされる情報で、ツリー型の構造をもった管理情報データベースとなっている。 MIBには、**標準MIB**と各メーカーが独自に作成した**拡張MIB**がある。

### RMON

**RMON**(Remote Monitoring MIB)は、通常のMIBがネットワーク機器のインタフェース（点）を監視するのに対し、接続されるネットワークの回線（線）を監視する。

ある特定のホストがどこの誰と、どのようなプロトコルで通信しているかという統計情報を知ることができる。


## H.323

**H.323**は、IPネットワーク上で音声や映像をやり取りするためのプロトコル体系。


## SIP

**SIP**はH.323より後に開発されて、インターネットでの利用によりマッチしているプロトコル。


## RTP

**RTP**(Real-Time Transport Protocol)は、UDPでリアルタイムなマルチメディア通信を実現するために、アプリケーション層でシーケンス番号やパケット送信時刻の管理をするプロトコル。

### RTCP

**RTCP**(RTP Control Protocol)は、パケット喪失率など通信回線の品質を管理することで、RTPによる通信を補助するプロトコル。


## P2P

**P2**(Peer To Peer)は、ネットワーク上に展開する各端末やホストがサーバーなどを介さずに1対1で直接接続して通信する形態。


## LDAP

**LDAP**(Lightweight Directory Access Protocol)は、ユーザー名やパスワードなどの情報を一元管理する仕組みである、ディレクトリサービスへのアクセスに使われる。**ディレクトリサービス**は、ネットワーク上の資源に関してデータベース的な情報提供を行うサービスである。


## NTP

**NTP**(Network Time Protocol)は、ネットワークに接続される機器の時刻を同期するためのアプリケーションプロトコル。


## 制御システムのプロトコル

制御システムは、**OT**(Operational Technology)や**ICS**(Industrial Control Systems)とも呼ばれ、機器や装置の監視や制御、およびそれを自動的に行うPID制御などのプロセス制御で使用される。