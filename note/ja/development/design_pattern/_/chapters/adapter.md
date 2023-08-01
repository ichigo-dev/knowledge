# 『Adapter』ノート

（最終更新： 2023-08-01）


## 目次

1. [Adapterパターン](#adapterパターン)
	1. [Target](#target)
	1. [Client](#client)
	1. [Adaptee](#adaptee)
	1. [Adapter](#adapter)


## Adapterパターン

**Adapterパターン**（**Wrapperパターン**）は、既存の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)や[コンポーネント](../../../../computer/software/_/chapters/package.md#コンポーネント)を別のインタフェースに変換するための[デザインパターン](./design_pattern.md#デザインパターン)。

### Target

**Target**（対象）は、[Adapterパターン](#adapterパターン)において、今必要となっている[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)（[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)）を定める役割を持つ。[継承](../../../../programming/_/chapters/object_oriented.md#継承)を用いた場合は[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)、委譲を用いた場合は[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)がこの役を担う。

### Client

**Client**（依頼者）は、[Adapterパターン](#adapterパターン)において、[Target](#target)の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を利用する役割を持つ。メインとなる[プログラム](../../../../programming/_/chapters/programming.md#プログラム)や[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)がこの役を担う場合が多い。

### Adaptee

**Adaptee**は、[Adapterパターン](#adapterパターン)において、既存の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)の実装を提供する役割を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。このAdapteeが持つ[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を[Target](#target)の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)に適合させるために[Adapter](#adapter)が必要となる。

### Adapter

**Adapter**は、[Adapterパターン](#adapterパターン)において、[Adaptee](#adaptee)の提供する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を利用して[Target](#target)の要件を満たす役割を持つ。[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)を用いた[Adapterパターン](#adapterパターン)の場合には、[継承](../../../../programming/_/chapters/object_oriented.md#継承)を使って[Adaptee](#adaptee)を利用する。[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を用いた[Adapterパターン](#adapterパターン)の場合には、[Adaptee](#adaptee)の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を[メンバ変数](../../../../programming/_/chapters/object_oriented.md#プロパティ)として保持し、それを介して[Adaptee](#adaptee)の機能を利用する（委譲）。
