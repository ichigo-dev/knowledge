# 『Decorator』ノート

（最終更新： 2023-08-04）


## 目次

1. [Decoratorパターン](#decoratorパターン)
	1. [Component](#component)
	1. [ConcreteComponent](#concretecomponent)
	1. [Decorator](#decorator)
	1. [ConcreteDecorator](#cocnretedecorator)


## Decoratorパターン

**Decoratorパターン**は、中心となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に対して次々と機能をかぶせていき、より目的にあった[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に仕上げていく[デザインパターン](./design_pattern.md#デザインパターン)。

Decoratorパターンは、[Component](#component)、[ConcreteComponent](#concretecomponent)、[Decorator](#decorator)、[ConcreteDecorator](#concretedecorator)から構成される。

### Component

**Component**は、[Decoratorパターン](#decoratorパターン)において、デコレーションを施すためのコアとなる部分の[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。

### ConcreteComponent

**ConcreteComponent**は、[Decoratorパターン](#decoratorパターン)において、デコレーションを施すためのコアとなる部分で、[Component](#component)を実装している役。

### Decorator

**Decorator**（装飾者）は、[Decoratorパターン](#decoratorパターン)において、[Component](#component)と同じ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を持ち、デコレーションする対象となる[Component](#component)のインスタンスを[フィールド](../../../../programming/_/chapters/object_oriented.md#インタフェース)に含んでいる役。

### ConcreteDecorator

**ConcreteDecorator**（具体的な装飾者）は、[Decoratorパターン](#decoratorパターン)において、[Decorator](#decorator)を実装する役。
