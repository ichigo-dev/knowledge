# 『Rust』チェックシート


## Rustの基礎知識

- [Rust](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#rustの概要)
- [未定義動作](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#未定義動作)
	- [ダングリングポインタ](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#未定義動作)
	- [メモリの多重解放（多重フリー）](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#未定義動作)
	- [nullポインタの参照解決](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#未定義動作)
- [並列プログラム](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
- [データ競合](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
- [同期機構](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
	- [排他ロック](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
	- [条件変数](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
	- [チャネル](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
	- [アトミック変数](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#並列プログラム)
- [ゼロオーバーヘッド原則](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#実行速度)
	- [ガベージコレクタ（GC）](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#実行速度)
- [ゼロコスト抽象化](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#実行速度)
	- [静的ディスパッチ](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#実行速度)
- [rustup](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#rust開発を支えるツール)
- [Cargo](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#rust開発を支えるツール)
- [rustc](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#rust開発を支えるツール)
- [rustdoc](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#rust開発を支えるツール)
- [ユニットテスト](/note/rust/chapters/01_basic_knowledge_of_rust.ja.md#ユニットテスト)


## Rustの基本的なデータ型

- [静的型付け言語](/note/rust/chapters/02_data_type.ja.md#rustの型システム)
- [動的型付け言語](/note/rust/chapters/02_data_type.ja.md#rustの型システム)
- [型推論](/note/rust/chapters/02_data_type.ja.md#型推論)
- [ジェネリック](/note/rust/chapters/02_data_type.ja.md#ジェネリック)
	- [ダックタイピング](/note/rust/chapters/02_data_type.ja.md#ジェネリック)
- [固定長数値型](/note/rust/chapters/02_data_type.ja.md#固定長数値)
	- [整数](/note/rust/chapters/02_data_type.ja.md#整数)
		- [2の補数表現](/note/rust/chapters/02_data_type.ja.md#整数)
		- [バイト値](/note/rust/chapters/02_data_type.ja.md#整数)
		- [リテラル](/note/rust/chapters/02_data_type.ja.md#整数)
		- [暗黙の型変換](/note/rust/chapters/02_data_type.ja.md#整数)
	- [浮動小数点](/note/rust/chapters/02_data_type.ja.md#浮動小数点)
		- [単精度](/note/rust/chapters/02_data_type.ja.md#浮動小数点)
		- [倍精度](/note/rust/chapters/02_data_type.ja.md#浮動小数点)
		- [INFINITY](/note/rust/chapters/02_data_type.ja.md#浮動小数点)
		- [NEG_INFINITY](/note/rust/chapters/02_data_type.ja.md#浮動小数点)
		- [NAN](/note/rust/chapters/02_data_type.ja.md#浮動小数点)
		- [MIN](/note/rust/chapters/02_data_type.ja.md#浮動小数点)
		- [MAX](/note/rust/chapters/02_data_type.ja.md#浮動小数点)
	- [ラップ演算](/note/rust/chapters/02_data_type.ja.md#ラップ演算)
	- [飽和演算](/note/rust/chapters/02_data_type.ja.md#飽和演算)
		- [クランプ](/note/rust/chapters/02_data_type.ja.md#飽和演算)
	- [オーバーフロー演算](/note/rust/chapters/02_data_type.ja.md#オーバーフロー演算)
- [真偽値型](/note/rust/chapters/02_data_type.ja.md#真偽値型)
	- [比較演算](/note/rust/chapters/02_data_type.ja.md#真偽値型)
- [文字](/note/rust/chapters/02_data_type.ja.md#文字)
- [タプル](/note/rust/chapters/02_data_type.ja.md#タプル)
	- [ユニット型](/note/rust/chapters/02_data_type.ja.md#タプル)
- [ポインタ型](/note/rust/chapters/02_data_type.ja.md#ポインタ型)
	- [ポインタアドレス](/note/rust/chapters/02_data_type.ja.md#ポインタ型)
	- [参照](/note/rust/chapters/02_data_type.ja.md#参照)
		- [共有参照](/note/rust/chapters/02_data_type.ja.md#参照)
		- [可変参照](/note/rust/chapters/02_data_type.ja.md#参照)
	- [Box](/note/rust/chapters/02_data_type.ja.md#box)
		- [ヒープ](/note/rust/chapters/02_data_type.ja.md#box)
		- [移動（move）](/note/rust/chapters/02_data_type.ja.md#box)
	- [rawポインタ](/note/rust/chapters/02_data_type.ja.md#rawポインタ)
- [配列](/note/rust/chapters/02_data_type.ja.md#配列)
- [ベクタ](/note/rust/chapters/02_data_type.ja.md#ベクタ)
	- [アロケート](/note/rust/chapters/02_data_type.ja.md#ベクタ)
- [スライス](/note/rust/chapters/02_data_type.ja.md#スライス)
	- [ファットポインタ](/note/rust/chapters/02_data_type.ja.md#スライス)
- [文字列型](/note/rust/chapters/02_data_type.ja.md#文字列型)
	- [文字列リテラル](/note/rust/chapters/02_data_type.ja.md#文字列リテラル)
	- [生文字列](/note/rust/chapters/02_data_type.ja.md#文字列リテラル)
- [バイトリテラル](/note/rust/chapters/02_data_type.ja.md#バイトリテラル)
- [文字列](/note/rust/chapters/02_data_type.ja.md#文字列)
- [型エイリアス](/note/rust/chapters/02_data_type.ja.md#型エイリアス)


## 所有権と移動

- [メモリ管理](/note/rust/chapters/03_ownership_and_move.ja.md#メモリ管理)
	- [ガベージコレクション](/note/rust/chapters/03_ownership_and_move.ja.md#メモリ管理)
- [所有権](/note/rust/chapters/03_ownership_and_move.ja.md#所有権)
	- [スタックフレーム](/note/rust/chapters/03_ownership_and_move.ja.md#所有権)
	- [ヒープ](/note/rust/chapters/03_ownership_and_move.ja.md#所有権)
	- [ツリー構造](/note/rust/chapters/03_ownership_and_move.ja.md#所有権)
	- [生存期間](/note/rust/chapters/03_ownership_and_move.ja.md#所有権)
	- [プリミティブ型](/note/rust/chapters/03_ownership_and_move.ja.md#所有権)
	- [参照カウント](/note/rust/chapters/03_ownership_and_move.ja.md#所有権)
	- [参照の借用](/note/rust/chapters/03_ownership_and_move.ja.md#所有権)
- [移動](/note/rust/chapters/03_ownership_and_move.ja.md#移動)
	- [移動](/note/rust/chapters/03_ownership_and_move.ja.md#移動)
	- [未初期化状態](/note/rust/chapters/03_ownership_and_move.ja.md#移動)
	- [ディープコピー](/note/rust/chapters/03_ownership_and_move.ja.md#移動)
- [コピー型](/note/rust/chapters/03_ownership_and_move.ja.md#コピー型)
- [参照カウント](/note/rust/chapters/03_ownership_and_move.ja.md#rcとarc)
	- [Rc](/note/rust/chapters/03_ownership_and_move.ja.md#rcとarc)
	- [Arc（アトミックな参照カウント）](/note/rust/chapters/03_ownership_and_move.ja.md#rcとarc)


## 参照とライフタイム


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
