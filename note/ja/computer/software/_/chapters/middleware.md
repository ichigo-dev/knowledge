# 『ミドルウェア』ノート

（最終更新： 2023-03-09）


## 目次

1. [ミドルウェア](#ミドルウェア)
	1. [データベース管理システム](#データベース管理システム)
	1. [API](#api)


## ミドルウェア

**ミドルウェア**は、[OS](./operating_system.md#オペレーティングシステム)と[アプリケーションソフトウェア](./software.md#応用ソフトウェア)の中間のような処理・動作を行う[ソフトウェア](./software.md#ソフトウェア)。様々な[アプリケーション](./software.md#応用ソフトウェア)[プログラム](../../../../programming/_/chapters/programming.md#プログラム)に対して特定の共通化された機能を提供する。[OS](./operating_system.md#オペレーティングシステム)や[ハードウェア](../../../hardware/_/chapters/hardware.md#ハードウェア)の違いを吸収し、様々な[プラットフォーム](./software.md#プラットフォーム)で動作する[アプリケーション](./software.md#応用ソフトウェア)の開発を助けるといったメリットもある。

### データベース管理システム

**データベース管理システム**(**DBMS**: Database Management System)は、[データベース](../../../../development/database/_/chapters/database.md#データベース)を管理してデータに対するアクセス要求に応える[ソフトウェア](./software.md#ソフトウェア)。データの形式や利用手順を標準化することで、特定の[アプリケーションソフトウェア](./software.md#応用ソフトウェア)から独立させることができる。

### API

**API**(Application Programming Interface)は、[アプリケーション](./software.md#応用ソフトウェア)から[OS](./operating_system.md#オペレーティングシステム)や他の[ソフトウェア](./software.md#ソフトウェア)の機能を利用するための、[関数](../../../../programming/_/chapters/function.md#関数)などのインタフェース。例えば、[OS](./operating_system.md#オペレーティングシステム)の[カーネル](./operating_system.md#カーネル)の機能を提供されるAPIを[システムコール](./operating_system.md#システムコール)、[Web](../../../../network/_/chapters/web.md#web)を通じて提供されるAPIを[Web API](../../../../network/_/chapters/web.md#web-api)という。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)
