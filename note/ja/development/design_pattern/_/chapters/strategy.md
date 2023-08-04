# 『Strategy』ノート

（最終更新： 2023-08-04）


## 目次

1. [Strategyパターン](#strategyパターン)
	1. [Strategy](#strategy)
	1. [ConcreteStrategy](#concretestrategy)
	1. [Context](#context)


## Strategyパターン

**Strategyパターン**は、同じ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を実装する交換可能な[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)を複数用意しておき、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)実行時に適切なものを選択できるようにする[デザインパターン](./design_pattern.md#デザインパターン)。

Strategyパターンは、[Strategy](#strategy)、[ConcreteStrategy](#concretestrategy)、[Context](#context)から構成される。

### Strategy

**Strategy**（戦略）は、[Strategyパターン](#strategyパターン)において、[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)を実装する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定める[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。

### ConcreteStrategy

**ConcreteStrategy**（具体的戦略）は、[Strategyパターン](#strategyパターン)において、[Strategy](#strategy)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を実装し、実際の[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)を持った[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Strategyパターン](#strategyパターン)ではConcreteStrategyが複数用意され、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)は用いる戦略を容易に切り替えることができる。

### Context

**Context**（文脈）は、[Strategyパターン](#strategyパターン)において、[ConcreteStrategy](#concretestrategy)の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を持ち、必要に応じてその[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)が持つアルゴリズムを利用する役。Contextが呼び出すのは、[Strategy](#strategy)の[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)に定義された[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)のみであり、それぞれの[ConcreteStrategy](#concretestrategy)に依存した実装にはなっていない。
