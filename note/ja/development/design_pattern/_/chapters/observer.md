# 『Observer』ノート

（最終更新： 2023-08-04）


## 目次

1. [Observerパターン](#observerパターン)
	1. [Subject](#subject)
	1. [ConcreteSubject](#concretesubject)
	1. [Observer](#observer)
	1. [ConcreteObserver](#concreteobserver)


## Observerパターン

**Observerパターン**は、観察者となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が、観察対象となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)からの状態変化の通知を受けて、それに対する処理を行う[デザインパターン](./design_pattern.md#デザインパターン)。

Observerパターンは、[Subject](#subject)、[ConcreteSubject](#concretesubject)、[Observer](#observer)、[ConcreteObserver](#concreteobserver)から構成される。

### Subject

**Subject**（観察対象者）は、[Observerパターン](#observerパターン)において、[Observer](#observer)の観察対象となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。[Observer](#observer)を保持する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)、[Observer](#observer)への通知[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)などを定義する。[Observer](#observer)は複数保持していても良い。

### ConcreteSubject

**ConcreteSubject**（具体的な観察対象者）は、[Observerパターン](#observerパターン)において、[Subject](#subject)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Observer](#observer)への通知は、自身に保持している[Observer](#observer)[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を介して行う。

### Observer

**Observer**（観察者）は、[Observerパターン](#observerパターン)において、[Subject](#subject)の状態変化を監視するための[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。[Subject](#subject)からの通知を受信するための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義する。

### ConcreteObserver

**ConcreteObserver**（具体的な観察者）は、[Observerパターン](#observerパターン)において、[Observer](#observer)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Subject](#subject)からの状態受信用の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)が呼ばれると、その呼び出し元の状態をもとに処理を行う。
