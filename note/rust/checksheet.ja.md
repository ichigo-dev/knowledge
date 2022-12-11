# 『Rust』チェックシート


## Rustの基礎知識

- [Rust](./chapters/01_basic_knowledge_of_rust.ja.md#rustの概要)
- [未定義動作](./chapters/01_basic_knowledge_of_rust.ja.md#未定義動作)
	- [ダングリングポインタ](./chapters/01_basic_knowledge_of_rust.ja.md#未定義動作)
	- [メモリの多重解放（多重フリー）](./chapters/01_basic_knowledge_of_rust.ja.md#未定義動作)
	- [nullポインタの参照解決](./chapters/01_basic_knowledge_of_rust.ja.md#未定義動作)
- [並列プログラム](./chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
- [データ競合](./chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
- [同期機構](./chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
	- [排他ロック](./chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
	- [条件変数](./chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
	- [チャネル](./chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
	- [アトミック変数](./chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
- [ゼロオーバーヘッド原則](./chapters/01_basic_knowledge_of_rust.ja.md#実行速度)
	- [ガベージコレクタ（GC）](./chapters/01_basic_knowledge_of_rust.ja.md#実行速度)
- [ゼロコスト抽象化](./chapters/01_basic_knowledge_of_rust.ja.md#実行速度)
	- [静的ディスパッチ](./chapters/01_basic_knowledge_of_rust.ja.md#実行速度)
- [rustup](./chapters/01_basic_knowledge_of_rust.ja.md#rust開発を支えるツール)
- [Cargo](./chapters/01_basic_knowledge_of_rust.ja.md#rust開発を支えるツール)
- [rustc](./chapters/01_basic_knowledge_of_rust.ja.md#rust開発を支えるツール)
- [rustdoc](./chapters/01_basic_knowledge_of_rust.ja.md#rust開発を支えるツール)
- [ユニットテスト](./chapters/01_basic_knowledge_of_rust.ja.md#ユニットテスト)


## Rustの基本的なデータ型

- [静的型付け言語](./chapters/02_data_type.ja.md#rustの型システム)
- [動的型付け言語](./chapters/02_data_type.ja.md#rustの型システム)
- [型推論](./chapters/02_data_type.ja.md#型推論)
- [ジェネリック](./chapters/02_data_type.ja.md#ジェネリック)
	- [ダックタイピング](./chapters/02_data_type.ja.md#ジェネリック)
- [固定長数値型](./chapters/02_data_type.ja.md#固定長数値)
	- [整数](./chapters/02_data_type.ja.md#整数)
		- [2の補数表現](./chapters/02_data_type.ja.md#整数)
		- [バイト値](./chapters/02_data_type.ja.md#整数)
		- [リテラル](./chapters/02_data_type.ja.md#整数)
		- [暗黙の型変換](./chapters/02_data_type.ja.md#整数)
	- [浮動小数点](./chapters/02_data_type.ja.md#浮動小数点)
		- [単精度](./chapters/02_data_type.ja.md#浮動小数点)
		- [倍精度](./chapters/02_data_type.ja.md#浮動小数点)
		- [INFINITY](./chapters/02_data_type.ja.md#浮動小数点)
		- [NEG_INFINITY](./chapters/02_data_type.ja.md#浮動小数点)
		- [NAN](./chapters/02_data_type.ja.md#浮動小数点)
		- [MIN](./chapters/02_data_type.ja.md#浮動小数点)
		- [MAX](./chapters/02_data_type.ja.md#浮動小数点)
	- [ラップ演算](./chapters/02_data_type.ja.md#ラップ演算)
	- [飽和演算](./chapters/02_data_type.ja.md#飽和演算)
		- [クランプ](./chapters/02_data_type.ja.md#飽和演算)
	- [オーバーフロー演算](./chapters/02_data_type.ja.md#オーバーフロー演算)
- [真偽値型](./chapters/02_data_type.ja.md#真偽値型)
	- [比較演算](./chapters/02_data_type.ja.md#真偽値型)
- [文字](./chapters/02_data_type.ja.md#文字)
- [タプル](./chapters/02_data_type.ja.md#タプル)
	- [ユニット型](./chapters/02_data_type.ja.md#タプル)
- [ポインタ型](./chapters/02_data_type.ja.md#ポインタ型)
	- [ポインタアドレス](./chapters/02_data_type.ja.md#ポインタ型)
	- [参照](./chapters/02_data_type.ja.md#参照)
		- [共有参照](./chapters/02_data_type.ja.md#参照)
		- [可変参照](./chapters/02_data_type.ja.md#参照)
	- [Box](./chapters/02_data_type.ja.md#box)
		- [ヒープ](./chapters/02_data_type.ja.md#box)
		- [移動（move）](./chapters/02_data_type.ja.md#box)
	- [rawポインタ](./chapters/02_data_type.ja.md#rawポインタ)
- [配列](./chapters/02_data_type.ja.md#配列)
- [ベクタ](./chapters/02_data_type.ja.md#ベクタ)
	- [アロケート](./chapters/02_data_type.ja.md#ベクタ)
- [スライス](./chapters/02_data_type.ja.md#スライス)
	- [ファットポインタ](./chapters/02_data_type.ja.md#スライス)
- [文字列型](./chapters/02_data_type.ja.md#文字列型)
	- [文字列リテラル](./chapters/02_data_type.ja.md#文字列リテラル)
	- [生文字列](./chapters/02_data_type.ja.md#文字列リテラル)
- [バイトリテラル](./chapters/02_data_type.ja.md#バイトリテラル)
- [文字列](./chapters/02_data_type.ja.md#文字列)
- [型エイリアス](./chapters/02_data_type.ja.md#型エイリアス)


## 所有権と移動

- [メモリ管理](./chapters/03_ownership_and_move.ja.md#メモリ管理)
	- [ガベージコレクション](./chapters/03_ownership_and_move.ja.md#メモリ管理)
- [所有権](./chapters/03_ownership_and_move.ja.md#所有権)
	- [スタックフレーム](./chapters/03_ownership_and_move.ja.md#所有権)
	- [ヒープ](./chapters/03_ownership_and_move.ja.md#所有権)
	- [ツリー構造](./chapters/03_ownership_and_move.ja.md#所有権)
	- [生存期間](./chapters/03_ownership_and_move.ja.md#所有権)
	- [プリミティブ型](./chapters/03_ownership_and_move.ja.md#所有権)
	- [参照カウント](./chapters/03_ownership_and_move.ja.md#所有権)
	- [参照の借用](./chapters/03_ownership_and_move.ja.md#所有権)
- [移動](./chapters/03_ownership_and_move.ja.md#移動)
	- [移動](./chapters/03_ownership_and_move.ja.md#移動)
	- [未初期化状態](./chapters/03_ownership_and_move.ja.md#移動)
	- [ディープコピー](./chapters/03_ownership_and_move.ja.md#移動)
- [コピー型](./chapters/03_ownership_and_move.ja.md#コピー型)
- [参照カウント](./chapters/03_ownership_and_move.ja.md#rcとarc)
	- [Rc](./chapters/03_ownership_and_move.ja.md#rcとarc)
	- [Arc（アトミックな参照カウント）](./chapters/03_ownership_and_move.ja.md#rcとarc)


## 参照とライフタイム

- [参照](./chapters/04_reference_and_lifetime.ja.md#参照と借用)
	- [借用](./chapters/04_reference_and_lifetime.ja.md#参照と借用)
	- [値渡し](./chapters/04_reference_and_lifetime.ja.md#参照と借用)
	- [参照渡し](./chapters/04_reference_and_lifetime.ja.md#参照と借用)
	- [共有参照](./chapters/04_reference_and_lifetime.ja.md#参照と借用)
	- [可変参照](./chapters/04_reference_and_lifetime.ja.md#参照と借用)
	- [ファットポインタ](./chapters/04_reference_and_lifetime.ja.md#ファットポインタ)
	- [トレイトオブジェクト](./chapters/04_reference_and_lifetime.ja.md#ファットポインタ)
- [生存期間（ライフタイム）](./chapters/04_reference_and_lifetime.ja.md#ライフタイム)
	- [ボローチェッカー](./chapters/04_reference_and_lifetime.ja.md#ライフタイムのルール)
	- [生存期間パラメータ](./chapters/04_reference_and_lifetime.ja.md#staticな生存期間)
	- ['static生存期間](./chapters/04_reference_and_lifetime.ja.md#staticな生存期間)

## 式


## エラー処理


## クレートとモジュール


## 構造体


## 列挙型とパターン


## トレイトとジェネリクス


## 演算子オーバーロード


## ユーティリティトレイト


## クロージャ


## イテレータ


## コレクション


## 文字列とテキスト


## 並列性
