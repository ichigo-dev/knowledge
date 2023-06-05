# 『サイバー攻撃』ノート

（最終更新： 2023-06-05）


## 目次

1. [サイバー攻撃](#サイバー攻撃)
1. [攻撃者の種類](#攻撃者の種類)
	1. [ハッカー](#ハッカー)
	1. [スクリプトキディ](#スクリプトキディ)
	1. [ボットハーダー](#ボットハーダー)
	1. [ハクティビスト](#ハクティビスト)
1. [マルウェア](#マルウェア)
	1. [コンピュータウイルス](#コンピュータウイルス)
	1. [ワーム](#ワーム)
	1. [トロイの木馬](#トロイの木馬)
	1. [バンキングトロジャン](#バンキングトロジャン)
	1. [ランサムウェア](#ランサムウェア)
	1. [ダウンローダ](#ダウンローダ)
	1. [ドロッパ](#ドロッパ)
	1. [ルートキット](#ルートキット)
	1. [バックドア](#バックドア)
	1. [キーロガー](#キーロガー)
1. [C2サーバ](#c2サーバ)
1. [ボットネット](#ボットネット)
1. [スパイウェア](#スパイウェア)
	1. [アドウェア](#アドウェア)
	1. [マルバタイジング](#マルバタイジング)
	1. [トラッキングCookie](#トラッキングcookie)
	1. [パスワードスティーラ](#パスワードスティーラ)
1. [偽セキュリティソフトウェア](#偽セキュリティソフトウェア)
1. [不正ドキュメント](#不正ドキュメント)
1. [フィッシング](#フィッシング)
1. [ドライブバイダウンロード](#ドライブバイダウンロード)
1. [水飲み場攻撃](#水飲み場攻撃)
1. [マンインザミドル](#マンインザミドル)
1. [マンインザブラウザ](#マンインザブラウザ)
1. [ブルートフォース攻撃](#ブルートフォース攻撃)
1. [XSS](#xss)
1. [CSRF](#csrf)
1. [SSRF](#ssrf)
1. [SQLインジェクション](#sqlインジェクション)
1. [バッファオーバフロー](#バッファオーバフロー)
1. [DDoS攻撃](#ddos攻撃)
1. [ゼロデイ攻撃](#ゼロデイ攻撃)
1. [ソーシャルエンジニアリング](#ソーシャルエンジニアリング)
1. [高度標的型攻撃](#高度標的型攻撃)
1. [パスザハッシュ](#パスザハッシュ)


## サイバー攻撃

**サイバー攻撃**は、[インターネット](../../../../network/_/chapters/network.md#インターネット)やデジタル機器を絡めた手口で、[システム](../../../_/chapters/system.md#システム)の破壊や情報の改ざん、窃取などをする行為。サイバー攻撃は、[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)や[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)、[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)、データなどの様々なレベルで実行される。


## 攻撃者の種類

### ハッカー

**ハッカー**（**クラッカー**）は、[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)[システム](../../../_/chapters/system.md#システム)や[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)に対して、技術的な知識やスキルを駆使して不正な行為を行う攻撃者。ハッカーは[セキュリティ](./security.md#セキュリティ)の脆弱性を研究し、これを悪用して攻撃を行う。一方で、[セキュリティ](./security.md#セキュリティ)向上のために善意で活動するハッカー（**ホワイトハッカー**）も存在する。

### スクリプトキディ

**スクリプトキディ**は、既成のクラッキングツールを利用して不正アクセスを試みる攻撃者。[ハッカー](#ハッカー)と比べるとスキルや知識は限定的で、自身で新しい攻撃技術を開発したり、複雑な[セキュリティ](./security.md#セキュリティ)の脆弱性を解析することはできない場合が多い。悪戯目的で攻撃を行うことが多いため、[セキュリティ](./security.md#セキュリティ)対策は比較的容易。

### ボットハーダー

**ボットハーダー**は、[ボットネット](#ボットネット)と呼ばれるボットコンピュータを利用することで[サイバー攻撃](#サイバー攻撃)を実行する攻撃者。不正に侵入した[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)[システム](../../../../chapters/system.md#システム)に[マルウェア](#マルウェア)やボットコードを送り込み、その[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)を制御して[ボットネット](#ボットネット)を形成する。こうして築いた[ボットネット](#ボットネット)を、[マルウェア](#マルウェア)の配信やスパム[メール](../../../../network/_/chapters/application_layer.md#電子メール)の送信、[DDoS攻撃](#ddos攻撃)などの悪意のある活動に利用する。

### ハクティビスト

**ハクティビスト**は、特定の政治的、社会的な目的のために[サイバー攻撃](#サイバー攻撃)を行う攻撃者。[Web](../../../../network/_/chapters/web.md#web)サイトの改ざんや、機密情報の公開、オンライン上での抗議活動などを通じてメッセージの発信を試みる。ハクティビストは[ハッカー](#ハッカー)とアクティビスト（活動家）を合わせた言葉。


## マルウェア

**マルウェア**は、[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)や[プログラム](../../../../programming/_/chapters/programming.md#プログラム)に対して被害を加えることを目的とした、悪意のある[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)の総称。[コンピュータウイルス](#コンピュータウイルス)や[ワーム](#ワーム)、[トロイの木馬](#トロイの木馬)、[ランサムウェア](#ランサムウェア)などはマルウェアの一種である。

### コンピュータウイルス

**コンピュータウイルス**は、自己複製機能を有する、[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)や[プログラム](../../../../programming/_/chapters/programming.md#プログラム)を害することを目的として作成された[マルウェア](#マルウェア)。コンピュータウイルスは多くの場合、[Web](../../../../network/_/chapters/web.md#web)サイトの閲覧や[メール](../../../../network/_/chapters/application_layer.md#電子メール)の添付[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)、[USB](../../../../computer/hardware/_/chapters/bus.md#usb)メモリなどの外部記憶媒体を通じて感染する。コンピュータウイルスは、以下のような特徴を持つ。

- **自己増殖機能**: 感染した[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)内の[プログラム](../../../../programming/_/chapters/programming.md#プログラム)を書き換えたり、[システム](../../../_/chapters/system.md#システム)上の機能を利用することで、他の[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)にウイルス自身をインストールする機能
- **潜伏機能**（**休眠機能**）: 発病機能が実行されるまで、不正な挙動を示さないようにするための機能
- **発病機能**: ユーザの意図しない動作を行う悪性機能

### ワーム

**ワーム**は、[自己増殖](#コンピュータウイルス)するために感染活動を自ら行う[マルウェア](#マルウェア)。[コンピュータウイルス](#コンピュータウイルス)は宿主となる[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)が必要となるが、ワームはユーザの関与を必要とせず自動的に[増殖活動](#コンピュータウイルス)を行う。また、[ウイルス](#コンピュータウイルス)は感染した[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)が存在する範囲内で影響を及ぼすが、ワームは[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)全体に迅速に広がり広範な影響を与える可能性がある。

### トロイの木馬

**トロイの木馬**(**RAT**: Remote Administrator Tools/Remote Access Trojan)は、標的[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)に潜伏し、攻撃者からの遠隔操作による破壊活動や情報窃取を行う[マルウェア](#マルウェア)。[コンピュータウイルス](#コンピュータウイルス)とは異なり[自己増殖機能](#コンピュータウイルス)はない。トロイの木馬は[クライアントサーバシステム](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)によって構成されており、攻撃対象の[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)を[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)として、攻撃者は[クライアント](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)を経由して攻撃を行う。トロイの木馬に感染した[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)のことを**ゾンビPC**という。

### バンキングトロジャン

**バンキング・トロジャン**(Banking Trojan)は、特定の金融機関やオンラインバンキングの利用者を標的にする[トロイの木馬](#トロイの木馬)。標的の銀行口座や金融情報を窃取し、攻撃者が不正な利益を得るために使用される。

### ランサムウェア

**ランサムウェア**は、標的[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)上の[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)や[システム](../../../_/chapters/system.md#システム)を強制的に暗号化するなどして、ユーザに身代金を要求することを目的とした不正[プログラム](../../../../programming/_/chapters/programming.md#プログラム)。標的[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)の表示の変更や、[システム](../../../_/chapters/system.md#システム)のアクセス権の変更、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の暗号化などを行い、身代金を支払わない場合はデータを永久に失うと脅迫する。

### ダウンローダ

**ダウンローダ**は、[インターネット](../../../../network/_/chapters/network.md#インターネット)上で他の[プログラム](../../../../programming/_/chapters/programming.md#プログラム)や[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)をダウンロードするための[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)やスクリプト。[マルウェア](#マルウェア)の本体[プログラム](../../../../programming/_/chapters/programming.md#プログラム)はファイルサイズが大きい場合が多く、標的ユーザに発見されてしまう可能性があるため、ウイルス対策ソフトでの検出が難しいダウンローダを通じて外部から[マルウェア](#マルウェア)を取得する。また、ダウンロードしてくる[マルウェア](#マルウェア)を都度変更したり、[マルウェア](#マルウェア)の機能追加や更新といった拡張を支援することができる。

### ドロッパ

**ドロッパ**は、不正[プログラム](../../../../programming/_/chapters/programming.md#プログラム)の本体を標的[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)内に生成するための[プログラム](../../../../programming/_/chapters/programming.md#プログラム)で、ステルス機能を備えているものが多いため、ウイルス対策ソフトでの検出が難しい。また、[ダウンローダ](#ダウンローダ)との組み合わせによって攻撃が行われる場合もある。

### ルートキット

**ルートキット**は、標的[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)に侵入して攻撃の手助けをするツールを[パッケージ](../../../../computer/software/_/chapters/package.md#パッケージ)化したものの総称。直接的な被害を加えるのではなく、他の[マルウェア](#マルウェア)がウイルス対策ソフトに検知されることを防止するといった機能がある。

ルートキットは最初に、[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)と[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の間に介入し、[マルウェア](#マルウェア)に関する情報を削除することで存在を隠蔽する。そして、**DKOM**(Direct kernel Object Modification)と呼ばれる手法で、[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)が管理する内部情報そのものを削除するなどして、[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)に情報を渡さないように細工する。

また、[マルウェア](#マルウェア)の権限を不正に昇格する、[プロセス](../../../../computer/linux/_/chapters/process_and_job.md#プロセス)を隠蔽する、追加の[マルウェア](#マルウェア)をインストールする、といった機能を備えているものもある。

### バックドア

**バックドア**は、標的[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)の[システム](../../../_/chapters/system.md#システム)や[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)などに不正侵入するためのアクセスポイント。バックドアを仕掛けると、攻撃者は[認証](./security_technology.md#認証)を回避して[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)に侵入することができるようになる。バックドアは、一度攻撃のために侵入した[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)に再度侵入しやすくするために設置される場合が多い。

### キーロガー

**キーロガー**は、標的[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)の情報を窃取するために、[キーボード](../../../../computer/hardware/_/chapters/io_unit.md#キーボード)の操作内容を記録して攻撃者に送信する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)。また、[マウス](../../../../computer/hardware/_/chapters/io_unit.md#マウス)の動きを追跡するための**マウスロガー**もある。


## C2サーバ

**C2サーバ**（C&Cサーバ: Command and Control Server）は、[サイバー攻撃](#サイバー攻撃)において攻撃者が操作や私事を送るために使用される[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)。攻撃者が[マルウェア](#マルウェア)や[ボットネット](#ボットネット)に対して命令を行うための拠点で、攻撃活動の重要な要素となる。


## ボットネット

**ボットネット**は、[マルウェア](#マルウェア)に感染した[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)（ボット）などで構成される[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)で、攻撃者（[ボットハーダ](#ボットハーダ)、マスタ）の指示で作動する。[トロイの木馬](#トロイの木馬)に感染した[ゾンビPC](#トロイの木馬)で構成された[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)のことをボットネットということもある。[DDoS攻撃](#ddos攻撃)を用いたサイバー犯罪やスパム[メール](../../../../network/_/chapters/application_layer.md#電子メール)、[スパイウェア](#スパイウェア)などに利用される。


## スパイウェア

**スパイウェア**は、[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)やモバイルデバイスに侵入し、ユーザのプライバシーや[セキュリティ](./security.md#セキュリティ)を侵害する悪意のある[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)。ユーザ情報の収集や行動の監視、不正なデータ送信などによって悪影響を及ぼす。

### アドウェア

**アドウェア**は、不正な広告を表示することで、ユーザのプライバシや[セキュリティ](./security.md#セキュリティ)を侵害する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)。広告を表示して有料の[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)のインストールを促したり、[ブラウザ](../../../../network/_/chapters/web.md#webブラウザ)に保存された情報を窃取したりする。

### マルバタイジング

**マルバタイジング**（**不正広告**）は、オンライン広告を介して[マルウェア](#マルウェア)配布や不正[Web](../../../../network/_/chapters/web.md#web)サイトへの誘導などを行う攻撃手口。[Web](../../../../network/_/chapters/web.md#web)広告に悪性コード（[JavaScript](../../../../programming/_/chapters/programming_language.md#javascript)など）を混入し、ユーザが広告をクリックしたときに悪意のあるサイトに[リダイレクト](../../../../network/_/chapters/web.md#リダイレクト)させたり、[マルウェア](#マルウェア)がインストールさせたりする。

### トラッキングCookie

**トラッキングCookie**は、ユーザの閲覧履歴などの窃取を目的として仕掛けられる[Cookie](../../../../network/_/chapters/web.md#cookie)。ユーザの[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)に[Cookie](../../../../network/_/chapters/web.md#cookie)を送信することで、[Web](../../../../network/_/chapters/web.md#web)サイトの閲覧履歴や傾向を取得し、統計的にマーケティングなどに利用する。必ずしも悪質なものとは言い切れないので、削除するか否かはユーザの判断に委ねられる。

### パスワードスティーラ

**パスワードスティーラ**は、パスワード情報を窃取することを目的とした[スパイウェア](#スパイウェア)。[キーロギング](#キーロガー)や[フィッシング](#フィッシング)、[ソーシャルエンジニアリング](#ソーシャルエンジニアリング)などの手口によってユーザアカウントの乗っ取りなどを行う。


## 偽セキュリティソフトウェア

**偽セキュリティソフトウェア**は、実際のセキュリティソフトウェアのように見せかけて、ユーザを騙して悪意のある[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)をインストールさせる詐欺的な[プログラム](../../../../programming/_/chapters/programming.md#プログラム)。存在しない脅威や[ウイルス](#コンピュータウイルス)感染などの虚偽の報告を行い、ユーザに有料[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)の購入を強要するという手口が一般的。さらに、このようにして購入させる[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)が[マルウェア](#マルウェア)である可能性もあるため、追加の被害にも注意が必要。


## 不正ドキュメント

**不正ドキュメント**は、不正[プログラム](../../../../programming/_/chapters/programming.md#プログラム)や悪性コードを含むドキュメント[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の総称。Microsoft OfficeやPDF[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)などに不正[プログラム](../../../../programming/_/chapters/programming.md#プログラム)を忍び込ませるのがその典型で、[ダウンローダ](#ダウンローダ)や[ドロッパ](#ドロッパ)といった[マルウェア](#マルウェア)本体をインストールする前段階の機能を持つものや、エクスプロイトコード（[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)の脆弱性を悪用して、意図しない動作をさせるための悪性コード）などがある。


## フィッシング

**フィッシング**は、信頼された組織やサービスを詐称して、ユーザの個人情報や機密データを窃取しようとする手口。金融機関などを騙った[メール](../../../../network/_/chapters/application_layer.md#電子メール)などから、偽の[Web](../../../../network/_/chapters/web.md#web)サイトに誘導し、銀行口座のパスワードを入力させて情報を盗み出す、といった手口が用いられる。また、[メール](../../../../network/_/chapters/application_layer.md#電子メール)ではなくSMSを利用した**スミッシング**もある。


## ドライブバイダウンロード

**ドライブバイダウンロード**は、[インターネット](../../../../network/_/chapters/network.md#インターネット)経由で標的ユーザの意図しないダウンロードを行わせる攻撃方法。[Web](../../../../network/_/chapters/web.md#web)サイトの脆弱性や[セキュリティ](./security.md#セキュリティ)の欠陥を悪用して行われ、ユーザは意図せず[マルウェア](#マルウェア)を[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)にダウンロードしてしまう。


## 水飲み場攻撃

**水飲み場攻撃**は、[Web](../../../../network/_/chapters/web.md#web)サイトに[マルウェア](#マルウェア)などを仕込み、標的ユーザのアクセスを待ち伏せる手法。標的ユーザがアクセスする可能性の高い[Web](../../../../network/_/chapters/web.md#web)サイトを調査し、[セキュリティ](./security.md#セキュリティ)に欠陥のある[Web](../../../../network/_/chapters/web.md#web)サイトに侵入して悪意のあるコードを埋め込むなどして、再度標的ユーザがその[Web](../../../../network/_/chapters/web.md#web)サイトにアクセスすることを待ち伏せする。


## マンインザミドル

**マンインザミドル**(**MitM**: Man in the Middle)は、通信の送信者と受信者の間に攻撃者が割り込み、通信内容を盗聴したり改ざんしたりする攻撃。


## マンインザブラウザ

**マンインザブラウザ**(**MitB**: Man in the Browser)は、攻撃者が[Webブラウザ](../../../../network/_/chapters/web.md#webブラウザ)内に悪意のある[プログラム](../../../../programming/_/chapters/programming.md#プログラム)を埋め込むことで、ユーザの[セッション](../../../../network/_/chapters/web.md#セッション)情報や機密情報を窃取する攻撃。


## ブルートフォース攻撃

**ブルートフォース攻撃**（**総当たり攻撃**）は、すべての可能な組み合わせを試行することによって、パスワードや暗号鍵を推測する攻撃手法。攻撃者は自動化されたツールやスクリプトを使用して、連続的にパスワードや鍵を試し続ける。正しい組み合わせを推測するのには非常に多くの時間がかかる可能性があるため、よく用いられるパスワードや単語を使用する**辞書攻撃**や、予め計算された[ハッシュ値](./encryption_technology.md#ハッシュ)を用いて元のパスワードを逆引きする**レインボーテーブル**などで、攻撃を効率化する。

強力なパスワードを設定する、ログイン試行回数に制限を設ける、[二要素認証](./security_technology.md#多要素認証)を使用する、といった対策方法がある。


## XSS

**XSS**（**クロスサイトスクリプティング**）は、[Web](../../../../network/_/chapters/web.md#web)[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の脆弱性を悪用して攻撃者がスクリプトを挿入し、ユーザの[ブラウザ](../../../../network/_/chapters/web.md#webブラウザ)上で実行される攻撃手法。攻撃者は入力フィールドや[URL](../../../../network/_/chapters/web.md#url)パラメータなどにスクリプトを埋め込み、ユーザがそのページにアクセスした際にスクリプトが実行されるようにする。


## CSRF

**CSRF**(Cross-Site Request Forgery)は、ユーザが意図しない[リクエスト](../../../_/chapters/system_processing_model.md#リクエスト)を[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)に送信させる攻撃。ログイン機能を備えた[Web](../../../../network/_/chapters/web.md#web)[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)において、ユーザがログイン[セッション](../../../../network/_/chapters/web.md#セッション)を持っている状態で悪意のある[Web](../../../../network/_/chapters/web.md#web)サイトに誘導し、元の[Web](../../../../network/_/chapters/web.md#web)[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)に対して不正な[リクエスト](../../../_/chapters/system_processing_model.md#リクエスト)を行うような[リンク](../../../../network/_/chapters/web.md#ハイパーリンク)やフォームを設置しておくことで攻撃を行う。この攻撃が行われると、ユーザアカウントの乗っ取りや機密情報の漏洩が発生する可能性がある。

[Web](../../../../network/_/chapters/web.md#web)[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)側で[リクエスト](../../../_/chapters/system_processing_model.md#リクエスト)に対してCSRFトークン（一意なトークン）を付与し、不正なユーザからの[リクエスト](../../../_/chapters/system_processing_model.md#リクエスト)を判別できるようにするといった対策が一般的。


## SSRF

**SSRF**(Server-Side Request Forgery)は、攻撃者が[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)上で[リクエスト](../../../_/chapters/system_processing_model.md#リクエスト)を送信するように[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)を欺く攻撃手法。攻撃者は[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)内部のリソースにアクセスし、機密情報の取得や他[システム](../../../_/chapters/system.md#システム)への攻撃を行う。本来は外部からアクセスすることのできない[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)に対して、[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)の特権を活かして[リクエスト](../../../_/chapters/system_processing_model.md#リクエスト)を行うことができる。


## SQLインジェクション

**SQLインジェクション**は、[システム](../../../_/chapters/system.md#システム)で利用される[SQL](../../../../development/database/_/chapters/sql.md#sql)[クエリ](../../../../development/database/_/chapters/sql.md#クエリ)に悪意のあるコードを挿入する攻撃手法。攻撃者は[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の入力フィールドやパラメータに対して[SQL](../../../../development/database/_/chapters/sql.md#sql)分の一部として解釈されるコードを挿入することで、[データベース](../../../../development/database/_/chapters/database.md#データベース)に対する不正なアクセスや操作を行うことができる。

SQLインジェクションの一般的な攻撃例として、入力フィールドに以下のような文字列を入力して[リクエスト](../../../_/chapters/system_processing_model.md#リクエスト)を行うことで、その[テーブル](../../../../development/database/_/chapters/rdb.md#テーブル)で取得可能なすべてのデータを取得できる。

```sql
' OR '1' = 1
```

また、以下のように[SQL](../../../../development/database/_/chapters/sql.md#sql)の終わりを表す `;` に続けて[SQL](../../../../development/database/_/chapters/sql.md#sql)を記述することで、不正な[クエリ](../../../../development/database/_/chapters/sql.md#クエリ)を実行することができる可能性もある。

```sql
'; DROP TABLE users --
```


## バッファオーバフロー

**バッファオーバフロー**は、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)が確保された[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)領域を超えてデータを書き込むことにより、攻撃者が不正な[コード](../../../../programming/_/chapters/programming.md#ソースコード)を実行させる攻撃手法。主に[C](../../../../programming/_/chapters/programming_language.md#c言語)や[C++](../../../../programming/_/chapters/programming_language.md#c)などの[低水準言語](../../../../programming/_/chapters/programming.md#低水準言語)で記述された[プログラム](../../../../programming/_/chapters/programming.md#プログラム)で発生しやすい。攻撃者はバッファの範囲を越えるデータを書き込むことで、[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)の隣接領域や[スタック](../../../../computer/hardware/_/chapters/memory.md#スタック領域)上の重要なデータを上書きしようとする。


## DDoS攻撃

**DDoS攻撃**(Distributed Denial-of-Service)は、複数の[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)を用いて（[ボットネット](#ボットネット)など）、標的[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)に対して一斉に[パケット](../../../../network/_/chapters/network.md#パケット)を送付する攻撃。大量の[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)トラフィックを発生させることで、サーバの可用性を妨害することを目的としている。

**Dos攻撃**は、単一の攻撃者が標的[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)に対して[リソース](../../../../network/_/chapters/web.md#リソース)を消耗させるために行う攻撃であるが、攻撃に成功するためには十分な環境が必要となる。そのため、DDoS攻撃では、複数の[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)を[マルウェア](#マルウェア)に感染させて攻撃のツールとして用いることで、[リソース](../../../../network/_/chapters/web.md#リソース)を補填している。


## ゼロデイ攻撃

**ゼロデイ攻撃**は、[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)の脆弱性が公開されてから修正パッチが提供されていない状態で行われる攻撃手法。攻撃者は[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)の脆弱性を見つけ、セキュリティベンダーや[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)の開発者が認識しておらず、防御策が存在してない状態で攻撃が行われる。


## ソーシャルエンジニアリング

**ソーシャルエンジニアリング**は、人々の心理的な弱点や信頼を悪用して情報やアクセス権限を入手するための攻撃方法。技術的な脆弱性を突くのではなく、人間の誤解や信頼に漬け込むことで攻撃を行う。


## 高度標的型攻撃

**高度標的型攻撃**(**APT**: Advanced Persistent Threat)は、攻撃者が特定の組織や個人を狙い、継続的に行われる[サイバー攻撃](#サイバー攻撃)の総称。攻撃者は高度な技術や知識を持っており、ターゲットを選定して、継続的な攻撃や多段的な攻撃を行う。


## パスザハッシュ

**パスザハッシュ**は、主に[Windows](../../../../computer/software/_/chapters/operating_system.md#windows)の[認証](./security_technology.md#認証)[システム](../../../_/chapters/system.md#システム)に対して行われる攻撃で、攻撃者はパスワードハッシュを窃取することで標的ユーザのパスワードを推測することなく認証をバイパスする。[Windows](../../../../computer/software/_/chapters/operating_system.md#windows)はNTLM認証を用いており、入力されたパスワードを[ハッシュ](./encryption_technology.md#ハッシュ)化して[システム](../../../_/chapters/system.md#システム)の特定領域に保存している。攻撃者はこの[ハッシュ値](./encryption_technology.md#ハッシュ)を摂取することで、[システム](../../../_/chapters/system.md#システム)の認証を突破する。

また、[Windows](../../../../computer/software/_/chapters/operating_system.md#windows)のよりセキュアな認証方式であるKerberos認証を採用しているが、パスザハッシュで得た権限をもとにKerberos認証を破る手法として**パスザチケット**攻撃も生み出されている。
