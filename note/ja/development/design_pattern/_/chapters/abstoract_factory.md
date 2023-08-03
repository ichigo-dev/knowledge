# 『Abstract Factory』ノート

（最終更新： 2023-08-03）


## 目次

1. [Abstract Factoryパターン](#abstract-factoryパターン)
	1. [AbstractProduct](#abstractproduct)
	1. [ConcreteProduct](#concreteproduct)
	1. [AbstractFactory](#abstractfactory)
	1. [ConcreteFactory](#concretefactory)


## Abstract Factoryパターン

**Abstract Factoryパターン**は、関連のある一連の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)群をまとめて生成する方法を提供する[デザインパターン](./design_pattern.md#デザインパターン)。このパターンを利用することで、関連する[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)のグループ単位での置き換えや追加が容易になる。[Factory Methodパターン](./factory_method.md#factory-methodパターン)が、単体の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成の枠組みを用意するのに対し、こちらは関連する[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)をまとめて管理することができる。

Abstract Factoryパターンは、[AbstractProduct](#abstractproduct)、[ConcreteProduct](#concreteproduct)、[AbstractFactory](#abstractfactory)、[ConcreteFactory](#concretefactory)から構成される。

### AbstractProduct

**AbstractProduct**（抽象的な製品）は、[Abstract Factoryパターン](#abstract-factoryパターン)において、[AbstractFactory](#abstractfactory)によって生成される抽象的な[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)のインタフェースを定義する[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)。AbstractProductは複数存在することができる。

### ConcreteProduct

**ConcreteProduct**（具体的な製品）は、[Abstract Factoryパターン](#abstract-factoryパターン)において、[AbstractProduct](#abstractproduct)の具体的な実装を持つ[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)。[AbstractProduct](#abstractproduct)が複数存在する場合は、それぞれに対応したConcreteProductの実装が必要となる。

### AbstractFactory

**AbstractFactory**（抽象的な工場）は、[Abstract Factoryパターン](#abstract-factoryパターン)において、グループ化された全ての[AbstractProduct](#abstract)を生成するためのインタフェースを定義する[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)。それぞれの[AbstractProduct](#abstractproduct)を生成するためのインタフェースを定義する。

### ConcreteFactory

**ConcreteFactory**（具体的な工場）は、[Abstract Factoryパターン](#abstract-factoryパターン)において、[AbstractFactory](#abstractfactory)の具体的な実装を持つ[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)。
