# 『セキュリティ技術』ノート

（最終更新： 2023-06-05）


## 目次

1. [ファイアウォール](#ファイアウォール)
	1. [WAF](#waf)
	1. [IDS](#ids)
	1. [IPS](#ips)
1. [プロキシ](#プロキシ)
	1. [リバースプロキシ](#リバースプロキシ)
1. [ブロックチェーン](#ブロックチェーン)
1. [CSP](#csp)
1. [CORS](#cors)
1. [認証](#認証)
	1. [多要素認証](#多要素認証)
	1. [生体認証](#生体認証)
1. [デジタルフォレンジック](#デジタルフォレンジック)
	1. [ハニーポット](#ハニーポット)


## ファイアウォール

**ファイアウォール**は、[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)トラフィックの監視と制御を行い、不正なアクセスや悪意のあるトラフィックを遮断することで、[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)の[セキュリティ](./security.md#セキュリティ)を強化するデバイスや[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)。[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)上を流れる[パケット](../../../../network/_/chapters/network.md#パケット)を監視し、特定の条件に基づいてフィルタリングを行う（**パケットフィルタリング**）。

### WAF

**WAF**(Web Application Firewall)は、[Web](../../../../network/_/chapters/web.md#web)[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の[セキュリティ](./security.md#セキュリティ)を向上させるために使用される技術のひとつで、[Web](../../../../network/_/chapters/web.md#web)トラフィックを監視し、悪意のある攻撃や不正なアクセスを検出および防御する。[XSS](./cyber_attack.md#xss)や[SQLインジェクション](./cyber_attack.md#sqlインジェクション)、[CSRF](./cyber_attack.md#csrf)などの攻撃を検知して防御したり、トラフィックの異常な挙動を検知してブロックしたり、ログ記録を行うといった役割を持つ。

### IDS

**IDS**（**侵入検知システム**: Intrusion Detectino System）は、[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)や[システム](../../../_/chapters/system.md#システム)における侵入や攻撃を検知するための[セキュリティ](./security.md#セキュリティ)[システム](../../../_/chapters/system.md#システム)。[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)トラフィックや[システム](../../../_/chapters/system.md#システム)のログを監視し、異常なアクティビティや攻撃パターンを特定することで、潜在的な[セキュリティ](./security.md#セキュリティ)侵害を検知する。

### IPS

**IPS**（侵入防御システム: Intrusion Prevention System）は、[IDS](ids)の機能に加えて、攻撃を防止するためのアクションを実行するための[セキュリティ](./security.md#セキュリティ)[システム](../../../_/chapters/system.md#システム)。[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)トラフィックや[システム](../../../_/chapters/system.md#システム)のログを監視し、悪意のある侵入や攻撃を検知して即座に対応を講じることで、[セキュリティ](./security.md#セキュリティ)の強化を図る。


## プロキシ

**プロキシ**(Proxy)は、[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)通信を仲介する中間[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)で、[クライアント](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)と[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)との間に配置され、通信制御やフィルタリング、アクセス制限などの機能を提供する。[インターネット](../../../../network/_/chapters/network.md#インターネット)上での匿名性を確保したり、フィルタリングやログの記録を行うといった[セキュリティ](./security.md#セキュリティ)上の役割の他、[キャッシュ](../../../../network/_/chapters/web.md#キャッシュ)の提供などを行う場合もある。社内[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)など[クライアント](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)側に設置されるものと、[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)側に設置される[リバースプロキシ](#リバースプロキシ)がある。

### リバースプロキシ

**リバースプロキシ**(Reverse Proxy)は、[システム](../../../_/chapters/system.md#システム)運用者などによって[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)側に設置される[プロキシ](#プロキシ)。不特定多数の[クライアント](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)に対するアクセス制御や、[サーバ](../../../_/chapters/system_processing_model.md#クライアントサーバシステム)の負荷分散、静的コンテンツの[キャッシュ](../../../../network/_/chapters/web.md#キャッシュ)などのために使用される。


## ブロックチェーン

**ブロックチェーン**は、分散型の[データベース](../../../../development/database/_/chapters/database.md#データベース)技術であり、複数の[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)間で共有されるデータの連鎖（チェーン）を指す用語。オンライン取引の記録やデータの透明性・信頼性・[セキュリティ](./security.md#セキュリティ)を確保するために使用される。

ブロックチェーンは複数のノード（[コンピュータ](../../../../computer/_/chapters/computer.md#コンピュータ)）で分散された[データベース](../../../../development/database/_/chapters/database.md#データベース)で、各ノードは[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)に参加し、取引やデータの承認を行う。この分散構造により、信頼性や[可用性](../../../_/chapters/system_performance_evaluation.md#可用性)が高まり、[単一障害点](../../../_/chapters/system_architecture.md#単一障害点)のリスクが低減される。新しいデータは前のブロックに追加される形で連続的に追跡され、一度追加されたデータは改ざんや削除ができないため不変性が保たれる。また、ブロックチェーンは公開型であり、[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)上のすべての参加者が同じ情報を共有するため、透明性が確保される。


## CSP

**CSP**(Content Security Policy)は、[Web](../../../../network/_/chapters/web.md#web)[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の[セキュリティ](./security.md#セキュリティ)を向上させるメカニズムで、[XSS](./cyber_attack.md#xss)やデータの盗聴などの攻撃を軽減するために使用される。[レスポンスヘッダー](../../../../network/_/chapters/web.md#レスポンスメッセージ)にポリシーを設定することで、特定の[リソース](../../../../network/_/chapters/web.md#リソース)の読み込みや実行を制御する。特定の[ドメイン](../../../../network/_/chapters/internet_layer.md#ドメイン名)からの[リソース](../../../../network/_/chapters/web.md#リソース)（[JavaScript](../../../../programming/_/chapters/programming_language.md#javascript)やCSS、画像など）のみ読み込みを許可したり、インラインスクリプトの実行を制限したりすることができる。


## CORS

**CORS**(Cross-Origin Resource Sharing)は、[Webブラウザ](../../../../network/_/chapters/web.md#webブラウザ)の[セキュリティ](./security.md#セキュリティ)メカニズムの一部で、異なるオリジン（[ドメイン](../../../../network/_/chapters/internet_layer.md#ドメイン名)、[プロトコル](../../../../network/_/chapters/network_architecture.md#プロトコル)、[ポート番号](../../../../network/_/chapters/address_on_network.md#ポート番号)）間での[リソース](../../../../network/_/chapters/web.md#リソース)共有を制御するための仕組み。同一オリジンポリシーによって、異なるオリジン間での直接的なアクセスは制御されているが、CORSを使用することでオリジン間の[リソース](../../../../network/_/chapters/web.md#リソース)共有を明示的に許可することができる。

CORSはクロスドメインの[API](../../../../computer/software/_/chapters/operating_system.md#api)呼び出しやAjax[リクエスト](../../../_/chapters/system_processing_model.md#リクエスト)を制御するために広く使用されている。


## 認証

**認証**は、ユーザが正当な権限を持っていることを確認するための手段。パスワードや[多要素認証](#多要素認証)、[生体認証](#生体認証)などにより、ユーザの身元を確認する。

### 多要素認証

**多要素認証**(**MFA**: Multi-Factor Authentication)は、ユーザのなりすましなどを防ぐため、複数の要素を組み合わせて[認証](#認証)を行う方法。パスワードや所有物に基づく認証（スマートフォンやワンタイムパスワード、[USB](../../../../computer/hardware/_/chapters/bus.md#usb)キーなど）、[生体認証](#生体認証)を組み合わせることで、不正アクセスのリスクを軽減できる。

### 生体認証

**生体認証**(Biometric Authentication)は、個人の生態的な特徴を使用して[認証](#認証)を行う技術。指紋認証や顔認証、虹彩認証、声紋認証などが一般的。パスワードやピンコードといった従来の[認証](#認証)手法と比べて高い[セキュリティ](./security.md#セキュリティ)と利便性を提供し、なりすましや不正アクセスのリスクを軽減できる。一方で、生体認証技術の実装には適切な[センサ](../../../../basics/measurement_and_control/_/chapters/control_theory.md#センサ)や[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)が必要であり、使用者の状態や環境条件に左右されるというデメリットもある。


## デジタルフォレンジック

**デジタルフォレンジック**は、サイバー[セキュリティ](./security.md#セキュリティ)における[インシデント](./security.md#インシデント)対応を含めた、被害及び加害[システム](../../../_/chapters/system.md#システム)の調査全般を指す用語。サイバー犯罪の捜査や企業の内部調査、[セキュリティ](./security.md#セキュリティ)[インシデント](./security.md#インシデント)調査といった分野で活用される。

### ハニーポット

**ハニーポット**は、攻撃者を誘い込み、行動や目的などを観測するための[システム](../../../_/chapters/system.md#システム)。[セキュリティ](./security.md#セキュリティ)研究や[インシデント](./security.md#インシデント)対応の一環として使用されることが多く、対話型ハニーポットや仮想ハニーポット、分散型ハニーポットなどの種類がある。ただし、攻撃者の攻撃を受けることを前提としているため、十分な[セキュリティ](./security.md#セキュリティ)知識を持ち、慎重な計画や設計を行う必要がある。
