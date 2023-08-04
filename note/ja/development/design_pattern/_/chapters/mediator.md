# 『Mediator』ノート

（最終更新： 2023-08-04）


## 目次

1. [Mediatorパターン](#mediatorパターン)
	1. [Mediator](#mediator)
	1. [ConcreteMediator](#concretemediator)
	1. [Collegue](#collegue)
	1. [ConcreteCollegue](#concretecollegue)


## Mediatorパターン

**Mediatorパターン**は、複雑に関連し合う複数の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)間の関係に仲介者を設け、その仲介者を介して処理を行うようにすることで、単純かつ明快なインタフェースを提供する[デザインパターン](./design_pattern.md#デザインパターン)。仲介者は、管轄下にある複数の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)からの問い合わせを受けて、適宜判断を行い、管轄下にあるオブジェクト全体、または一部に指示を出す。

Mediatorパターンは、[Mediator](#mediator)、[ConcreteMediator](#concretemediator)、[Collegue](#collegue)、[ConcreteCollegue](#concretecollegue)から構成される。

### Mediator

**Mediator**は、[Mediatorパターン](#mediatorパターン)において、[Collegue](#collegue)からの問い合わせを引き受けて、それをもとに判断を下し、[Collegue](#collegue)へ指示を出す役割を持つ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。各[Collegue](#collegue)からの相談受付の窓口となる[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)や、管轄下に億[Collegue](#collegue)を格納するための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)などを定義する。

### ConcreteMediator

**ConcreteMediator**は、[Mediatorパターン](#mediatorパターン)において、[Mediator](#mediator)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。実際に[Collegue](#collegue)からの相談を受けて判断を下し、それらに指示を出す。

### Collegue

**Collegue**は、[Mediatorパターン](#mediatorパターン)において、他の[Collegue](#collegue)と関連性のある[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を定義するための[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。他の[Collegue](#collegue)を制御したい場合は、[Mediator](#mediator)に相談し、[Mediator](#mediator)からの指示を受けるための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)や、自身が相談する[Mediator](#mediator)を格納するための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)などを定義する。

### ConcreteCollegue

**ConcreteCollegue**は、[Mediatorパターン](#mediatorパターン)において、[Collegue](#collegue)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。
