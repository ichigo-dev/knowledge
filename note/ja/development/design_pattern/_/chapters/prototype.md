# 『Prototype』ノート

（最終更新： 2023-08-03）


## 目次

1. [Prototypeパターン](#prototypeパターン)
	1. [Prototype](#prototype)
	1. [ConcretePrototype](#concreteprototype)
	1. [Client](#client)


## Prototypeパターン

**Prototypeパターン**は、[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)から[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成するのではなく、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)から別の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を複製して生成する[デザインパターン](./design_pattern.md#デザインパターン)。

Prototypeパターンは、[Prototype](#prototype)、[ConcretePrototype](#concreteprototype)、[Client](#client)から構成される。

### Prototype

**Prototype**（原型）は、[Prototypeパターン](#prototypeパターン)において、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を複製して新しい[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を作るための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定める[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。

### ConcretePrototype

**ConcretePrototype**（具体的な原型）は、[Prototypeパターン](#prototypeパターン)において、[Prototype](#prototype)を実装して、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を複製する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。

### Client

**Client**（利用者）は、[Prototypeパターン](#prototypeパターン)において、[ConcretePrototype](#concreteprototype)の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)によって複製した[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)や[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。
