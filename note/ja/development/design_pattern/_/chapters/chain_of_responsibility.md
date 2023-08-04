# 『Chain of Responsibility』ノート

（最終更新： 2023-08-04）


## 目次

1. [Chain of Responsibilityパターン](#chain-of-responsibilityパターン)
	1. [Handler](#handler)
	1. [ConcreteHandler](#concretehandler)


## Chain of Responsibilityパターン

**Chain of Responsibilityパターン**は、ある要求の受け取り対象となる複数の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を鎖のように繋ぎ、要求が発生した際に各[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を渡り歩いて、いずれかの段階で処理されることを表現する[デザインパターン](./design_pattern.md#デザインパターン)。このパターンを利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)は、要求を処理する[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の存在を意識することなく、連鎖関係の一番最初にいる（受付となる）[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に対して要求を投げればよくなるため、役割の分離がしやすくなる。

Chain of Responsibilityパターンは、[Handler](#handler)、[ConcreteHandler](#concretehandler)から構成される。

### Handler

**Handler**は、[Chain of Responsibilityパターン](#chain-of-responsibility)において、要求を処理する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義した[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。自身で処理できない要求の場合に受け流す先（[ConcreteHandler](#concretehandler)）を[フィールド](../../../../programming/_/chapters/object_oriented.md#フィールド)として持つ。

### ConcreteHandler

**ConcreteHandler**は、[Chain of Responsibilityパターン](#chain-of-responsibility)において、[Handler](#handler)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。実際に要求を自身で処理するか、次の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に要求を渡す。
