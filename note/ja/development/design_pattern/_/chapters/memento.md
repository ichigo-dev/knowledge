# 『Memento』ノート

（最終更新： 2023-08-11）


## 目次

1. [Mementoパターン](#mementoパターン)
	1. [Originator](#originator)
	1. [Memento](#memento)
	1. [Caretaker](#caretaker)


## Mementoパターン

**Mementoパターン**は、[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の任意の時点の状態を覚えておき、後でその状態に[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を戻すための工夫を提供する[デザインパターン](./design_pattern.md#デザインパターン)。テキストエディタや画像・動画の編集ソフトなどに実装されている、アンドゥ機能を提供することができる。

Mementoパターンは、[Originator](#originator)、[Memento](#memento)、[Caretaker](#caretaker)から構成される。

### Originator

**Originator**（作成者）は、[Mementoパターン](#mementoパターン)において、自分の状態を保存した[Memento](#memento)を作成したり、要求された[Memento](#memento)に状態を戻したりする役割を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。

### Memento

**Memento**（形見）は、[Mementoパターン](#mementoパターン)において、[Originator](#originator)の内部情報（[フィールド](../../../../programming/_/chapters/object_oriented.md#フィールド)）をスナップショットとして保持する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。Mementoの[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)は変更不可とし、[コンストラクタ](../../../../programming/_/chapters/object_oriented.md#コンストラクタ)を用いて一度だけデータを渡せるようにすることが多い。

### Caretaker

**Caretaker**（世話人）は、[Mementoパターン](#mementoパターン)において、[Memento](#memento)の履歴を保持し、[Originator](#originator)の状態を保存したり、ある時点の状態に戻したりするためタイミングを管理する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。どのタイミングで[Originator](#originator)の状態を獲得するべきか、また復元すべきかを知っており、[Mementoパターン](#mementoパターン)を利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)はCaretakerの[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を利用するだけで、状態管理のタイミングを意識する必要はない。
