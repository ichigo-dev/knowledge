# 『Prototype』ノート

（最終更新： 2023-08-06）


## 目次

1. [Prototypeパターン](#prototypeパターン)
	1. [Prototype](#prototype)
	1. [ConcretePrototype](#concreteprototype)


## Prototypeパターン

(**Prototypeパターン**は、[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)から[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成するのではなく、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)から別の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を複製して生成する[デザインパターン](./design_pattern.md#デザインパターン)。このパターンを使用することで、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)が余分な依存関係を含まずに[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を複製できたり（その[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)の持つ[フィールド](../../../../programming/_/chapters/object_oriented.md#プロパティ)を直接記述しなくてもよいため）、同じ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を持つ異なる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に対しても同じような複製処理を記述できたりするようになる。

Prototypeパターンは、[Prototype](#prototype)、[ConcretePrototype](#concreteprototype)から構成される。

### Prototype

**Prototype**（原型）は、[Prototypeパターン](#prototypeパターン)において、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を複製して新しい[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を作るための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定める[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。この[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)は多くの場合、既存の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)の[フィールド](../../../../programming/_/chapters/object_oriented.md#プロパティ)をすべてコピーした新しい[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を返す。

### ConcretePrototype

**ConcretePrototype**（具体的な原型）は、[Prototypeパターン](#prototypeパターン)において、[Prototype](#prototype)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を実装して、実際に[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を複製する処理を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。
