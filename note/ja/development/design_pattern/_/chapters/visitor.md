# 『Visitor』ノート

（最終更新： 2023-08-04）


## 目次

1. [Visitorパターン](#visitorパターン)
	1. [Visitor](#visitor)
	1. [ConcreteVisitor](#concretevisitor)
	1. [Acceptor](#acceptor)
	1. [ConcreteAcceptor](#concreteacceptor)


## Visitorパターン

**Visitorパターン**は、データ構造とそれに対する処理を分離することを目的とした[デザインパターン](./design_pattern.md#デザインパターン)。訪問者となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が、データ構造の個々の要素を訪問し、その訪問先で公開されている資源を利用して処理を実行して回る。

Visitorパターンは、[Visitor](#visitor)、[ConcreteVisitor](#concretevisitor)、[Acceptor](#acceptor)、[ConcreteAcceptor](#concreteacceptor)から構成される。

### Visitor

**Visitor**（訪問者）は、[Visitorパターン](#visitorパターン)において、具体的なデータ構造の要素である[ConcreteAcceptor](#concreteacceptor)を訪問して処理を行う[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。

### ConcreteVisitor

**ConcreteVisitor**（具体的訪問者）は、[Visitorパターン](#visitorパターン)において、[Visitor](#visitor)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。

### Acceptor

**Acceptor**（データ構造）は、[Visitorパターン](#visitorパターン)において、[Visitor](#visitor)の訪問先であるデータ構造要素に対する受入れ口となる[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。[Visitorパターン](#visitorパターン)を利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)は、Acceptorの受入れ用の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を介して[Visitor](#visitor)の訪問用の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を呼び出す（**ダブルディスパッチ**）。

### ConcreteAcceptor

**ConcreteAcceptor**（具体的データ構造）は、[Visitorパターン](#visitorパターン)において、[Acceptor](#acceptor)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。
