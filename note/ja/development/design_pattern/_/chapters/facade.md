# 『Facade』ノート

（最終更新： 2023-08-04）


## 目次

1. [Facadeパターン](#facadeパターン)
	1. [Facade](#facade)
	1. [Class](#class)


## Facadeパターン

**Facadeパターン**は、複雑な内部処理を隠蔽し、利用者にシンプルなインタフェースを提供する[デザインパターン](./design_pattern.md#デザインパターン)。複雑な[API](../../../../computer/software/_/chapters/operating_system.md#api)呼び出しの適切な実行順を利用者に意識させないという目的もある。また、複雑だがよく使われる処理に対してエイリアスとして使うことも可能。

Facadeパターンは、[Facade](#facade)、[Class](#class)から構成される。

### Facade

**Facade**（正面）は、[Facadeパターン](#facadeパターン)において、複雑な処理を構成している[Class](#class)をシンプルなインタフェースで提供する役割を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。各[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を適切な順番、使い方で呼び出す。

### Class

**Class**（各処理）は、[Facadeパターン](#facadeパターン)において、[Facade](#facade)から呼び出されて処理を行う[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Facade](#facade)を意識せず、[Facade](#facade)を呼び出すこともない。
