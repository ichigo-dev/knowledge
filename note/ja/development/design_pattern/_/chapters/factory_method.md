# 『Factory Method』ノート

（最終更新： 2023-08-02）


## 目次

1. [Factory Methodパターン](#factory-methodパターン)
	1. [Product](#product)
	1. [Creator](#creator)
	1. [ConcreteProduct](#concreteproduct)
	1. [ConcreteCreator](#concretecreator)


## Factory Method

**Factory Methodパターン**は、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成のための枠組みをあらかじめ[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)に定めておき、その具体的な実装を[サブクラス](../../../../programming/_/chapters/object_oriented.md#子クラス)に任せる[デザインパターン](./design_pattern.md#デザインパターン)。[Template Methodパターン](./template_method.md#template-methodパターン)を[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成の場面に適応させたパターン。

Factory Methodパターンは大きく分けて、[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)側（抽象的な骨組み、フレームワーク）の[Creator](#creator)および[Product](#product)と、[サブクラス](../../../../programming/_/chapters/object_oriented.md#子クラス)側（具体的な肉付け）の[ConcreteCreator](#concretecreator)および[ConcreteProduct](#concreteproduct)から構成される。

### Product

**Product**（製品）は、[Factory Methodパターン](#factory-methodパターン)のフレームワーク側で、生成される[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)が持つべきインタフェース（[API](../../../../computer/software/_/chapters/operating_system.md#api)）を定める[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)。具体的な実装は[ConcreteProduct](#concreteproduct)が持つ。

### Creator

**Creator**（作成者）は、[Factory Methodパターン](#factory-methodパターン)のフレームワーク側で、[Product](#product)役を生成する[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)。具体的な実装は[ConcreteCreator](#concretecreator)が持つ。

Creatorは実際に生成される[ConcreteProduct](#concreteproduct)については何も知らず、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成のメソッドを呼び出せば[Product](#product)が生成されることのみ知っている。 `new` による実際の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成を、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成のための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)呼び出しで代替することで、具体的な[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)名による束縛から[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)を解放することができる。

### ConcreteProduct

**ConcreteProduct**（具体的製品）は、[Factory Methodパターン](#factory-methodパターン)の具体的な肉付けをする側で、[Product](#product)の実装を行う[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)。

### ConcreteCreator

**ConcreteCreator**（具体的作成者）は、[Factory Methodパターン](#factory-methodパターン)の具体的な肉付けをする側で、[Creator](#creator)の実装を行う[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)。
