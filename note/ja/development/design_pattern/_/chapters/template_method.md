# 『Template Method』ノート

（最終更新： 2023-08-01）


## 目次

1. [Template Methodパターン](#template-methodパターン)
	1. [AbstractClass](#abstractclass)
	1. [ConcreteClass](#concreteclass)


## Template Methodパターン

**Template Methodパターン**は、ある処理の大まかな[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)をあらかじめ[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)に定めておき、その具体的な設計を[サブクラス](../../../../programming/_/chapters/object_oriented.md#子クラス)に任せる[デザインパターン](./design_pattern.md#デザインパターン)。[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)は、処理の流れを定義した**テンプレートメソッド**を持ち、この[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)の中で[抽象メソッド](../../../../programming/_/chapters/object_oriented.md#抽象メソッド)の呼び出しを行う。

### AbstractClass

**AbstractClass**（抽象クラス）は、[Template Methodパターン](#template-methodパターン)において、[テンプレートメソッド](#template-methodパターン)を実装する役割を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。

### ConcreteClass

**ConcreteClass**（具象クラス）は、[Template Methodパターン](#template-methodパターン)において、[抽象メソッド](../../../../programming/_/chapters/object_oriented.md#抽象メソッド)を具体的に実装する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。ConcreteClassに実装された[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)は[テンプレートメソッド](#template-methodパターン)を介して呼び出される。
