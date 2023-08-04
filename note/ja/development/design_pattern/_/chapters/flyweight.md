# 『Flyweight』ノート

（最終更新： 2023-08-04）


## 目次

1. [Flyweightパターン](#flyweightパターン)
	1. [Flyweight](#flyweight)
	1. [FlyweightFactory](#flyweightfactory)


## Flyweightパターン

**Flyweightパターン**は、生成済みの[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)をできるだけ再利用（共有）し、無駄な[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の生成（[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)使用）を防止する仕組みを提供する[デザインパターン](./design_pattern.md#デザインパターン)。きほんてきには　イミュータブルな[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)に対してのみ適用できるパターン。

Flyweightパターンは、[Flyweight](#flyweight)、[FlyweightFactory](#flyweightfactory)から構成される。

### Flyweight

**Flyweight**（フライ級）は、[Flyweightパターン](#flyweightパターン)において、軽量化対象となる（通常の利用方法だと無駄な[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が生成される可能性のある）[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。

### FlyweightFactory

**FlyweightFactory**（フライ級の工場）は、[Flyweightパターン](#flyweightパターン)において、[Flyweight](#flyweight)を生成・管理する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Flyweight](#flyweight)の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が必要な場合は、このFlyweightFactory経由で取得する。FlyweightFactoryは生成した[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)をプールし、共有可能な[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が要求された場合はプールから[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を返却する。[Flyweight](#flyweight)を管理する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)が複数存在すると、再利用できる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を別個に管理してしまうため、[Singletonパターン](./singleton.md#singletonパターン)を適用する。
