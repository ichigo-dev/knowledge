# 『Web開発』ノート

（最終更新： 2023-08-27）


## 目次

1. [静的サイト](#静的サイト)
1. [動的サイト](#動的サイト)
1. [CMS](#cms)
	1. [WordPress](#wordpress)
1. [SEO](#seo)
1. [ファビコン](#ファビコン)
1. [OGP](#ogp)
1. [PWA](#pwa)


## 静的サイト

**静的サイト**は、あらかじめ作成された[HTML](../../html/_/chapters/html.md#html)や[CSS](../../css/_/chapters/css.md#css)、[JavaScript](../../javascript/_/chapters/javascript.md#javascript)などの[ファイル](../../../computer/software/_/chapters/file_system.md#ファイル)から構成される[Webサイト](../../../network/_/chapters/web.md#web)。コンテンツは固定されており、ユーザが何度アクセスしても表示される内容は変わらない。[サーバ](../../../network/_/chapters/web.md#webサーバ)側の処理や[データベース](../../../development/database/_/chapters/database.md#データベース)を使用しないため、ホスティングやメンテナンスが容易である。


## 動的サイト

**動的サイト**は、ユーザの操作や[データベース](../../../development/database/_/chapters/database.md#データベース)の内容に基づいてコンテンツが生成される[Webサイト](../../../network/_/chapters/web.md#web)。ユーザがアクセスする度に異なるコンテンツが表示される場合がある。[サーバ](../../../network/_/chapters/web.md#webサーバ)側の処理や[データベース](../../../development/database/_/chapters/database.md#データベース)を使用しており、ユーザごとに個別の体験を提供することが可能となる。


## CMS

**CMS**(Contents Management System)は、[動的サイト](#動的サイト)を効率的に管理し、作成・編集・公開するための[ソフトウェア](../../../computer/software/_/chapters/software.md#応用ソフトウェア)。特別な技術知識がなくても、視覚的なエディタなどを利用してコンテンツを作成することができたり、様々なテンプレートが用意されていたりする。代表的なCMSとしては、[WordPress](#wordpress)がある。

### WordPress

**WordPress**は、[PHP](../../../programming/_/chapters/programming_language.md#php)で開発された[オープンソース](../../../computer/software/_/chapters/open_source_software.md#オープンソースソフトウェア)の[CMS](#cms)。世界中で広く利用されており、[Webサイト](../../../network/_/chapters/web.md#web)の40%以上がWordPressを使用して作られている。

代表的な機能として、カテゴリの管理やテーマによるデザインの切り替え、プラグインによる機能拡張などがある。


## SEO

**SEO**（**検索エンジン最適化**: Search Engine Optimization）は、[Webサイト](../../../network/_/chapters/web.md#web)が[検索エンジン](../../../network/_/chapters/web.md#検索エンジン)の検索結果で上位に表示されるように最適化するための取り組みやプロセス。つまり、特定のキーワードやフレーズに関連する検索結果で、自分の[Webサイト](../../../network/_/chapters/web.md#web)をより高い順位で表示させるための戦略や手法のこと。

自分の[Webサイト](../../../network/_/chapters/web.md#web)に関連するキーワードやフレーズの調査、コンテンツの最適化、メタデータ最適化、サイトの速度とパフォーマンス向上、レスポンシブ対応などの手法がある。


## ファビコン

**ファビコン**(Favicon: Favorites Icon)は、[Webサイト](../../../network/_/chapters/web.md#web)のタブやブックマークバーなど、Webブラウジングの様々な場所で表示される小さなアイコン。[Webサイト](../../../network/_/chapters/web.md#web)のブランディングや識別、[UX](../../../computer/software/_/chapters/software.md#ux)の向上などを目的としている。


## OGP

**OGP**(Open Graph Protocol)は、[Webサイト](../../../network/_/chapters/web.md#web)がSNSで適切に表示されるようにするためのメタデータセット。SNSで[Webサイト](../../../network/_/chapters/web.md#web)がシェアされた際に、[リンク](../../../network/_/chapters/web.md#ハイパーリンク)先のプレビュー情報を制御する。


## PWA

**PWA**（**プログレッシブWebアプリケーション**）は、[Webサイト](../../../network/_/chapters/web.md#web)をネイティブアプリと同じ感覚で使えるようにするための仕組み。ダウンロードやアップデートの必要がない点や、プッシュ通知ができる点で有用。