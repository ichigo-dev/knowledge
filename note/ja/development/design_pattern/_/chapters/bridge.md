# 『Bridge』ノート

（最終更新： 2023-08-03）


## 目次

1. [Bridgeパターン](#bridgeパターン)


## Bridgeパターン

**Bridgeパターン**は、機能の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層と実装の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層を分離して[継承](../../../../programming/_/chapters/object_oriented.md#継承)させ、それぞれを独立して管理することで、拡張性を高める[デザインパターン](./design_pattern.md#デザインパターン)。

[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層を分離しないアンチパターンとして、ある[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)を実装した[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)がすでに存在するとする。その[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)に機能を追加する目的で、その[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)を[継承](../../../../programming/_/chapters/object_oriented.md#継承)した別の[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)を新しく実装したとき、元の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)の[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)の実装と同じものを再度作らなければならなくなる（新しく作った[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)から、[継承](../../../../programming/_/chapters/object_oriented.md#継承)元の[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)を実装した[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)は見えない）。そのため、機能を追加するたびに[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)の数が増えてしまう。

Bridgeパターンでは、[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)を用いずに、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)の移譲を利用することでこのような問題を解決している。

Bridgeパターンは、[Abstraction](#abstraction)、[RefinedAbstraction](#refinedabstraction)、[Implementor](#implementor)、[ConcreteImplementor](#concreteimplementor)から構成される。

### Abstraction

**Abstraction**（抽象化）は、[Bridgeパターン](#bridgeパターン)の機能の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層において、[Implementor](#implementor)を介して具体的な実装を呼び出す抽象化[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)（[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)ではない）。メイン[プログラム](../../../../programming/_/chapters/programming.md#プログラム)等が実際に利用するのは、Abstractionやそれを[継承](../../../../programming/_/chapters/object_oriented.md#継承)した[RefinedAbstraction](#refinedabstraction)となる。

Abstractionは、実装の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層の共通[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)（[Implementor](#implementor)）を実装した[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)（[ConcreteImplementor](#concreteimplementor)）を[フィールド](../../../../programming/_/chapters/object_oriented.md#プロパティ)として持つ。この[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を介して具体的な実装を呼び出すことで、[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)を用いずに抽象化を実現している。

### RefinedAbstraction

**RefinedAbstraction**（改善した抽象化）は、[Bridgeパターン](#bridgeパターン)の機能の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層において、[Abstraction](#abstraction)を[継承](../../../../programming/_/chapters/object_oriented.md#継承)して機能を拡充した[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Abstraction](#abstraction)から直接[継承](../../../../programming/_/chapters/object_oriented.md#継承)するだけでなく、RefinedAbstractionの機能を[継承](../../../../programming/_/chapters/object_oriented.md#継承)してさらに機能追加を行うこともできる。

### Implementor

**implementor**（実装者）は、[Bridgeパターン](#bridgeパターン)の実装の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層において、[Abstraction](#abstraction)が利用する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義した[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。
