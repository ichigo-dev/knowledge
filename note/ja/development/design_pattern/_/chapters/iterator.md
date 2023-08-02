# 『Iterator』ノート

（最終更新： 2023-08-02）


## 目次

1. [Iteratorパターン](#iteratorパターン)
	1. [Aggregate](#aggregate)
	1. [Iterator](#iterator)
	1. [ConcreteAggregate](#concreteaggregate)
	1. [ConcreteIterator](#concreteiterator)


## Iteratorパターン

**Iteratorパターン**は、集合体の要素に順番にアクセスし、全体をスキャンしていくような処理を行うための[デザインパターン](./design_pattern.md#デザインパターン)。iterateという英単語には何かを「繰り返す」という意味があり、Iteratorは日本語で**反復子**とも呼ばれる。

Iteratorパターンは、集合体とアクセス方法を分離することで、集合体の内部構造に依存せずに要素にアクセスすることができるようにすることを目的としている。このパターンを利用することで得られるメリットとして、次のようなものが考えられる。

- 集合体の内部構造を知る必要がなく、簡潔に[コード](../../../../programming/_/chapters/programming.md#ソースコード)を記述することができる
- 要素を順番に取り出すことができるため、検索や並び替えなど、要素に対する様々な操作が行いやすくなる
- 複数の集合体で同じIteratorを使用することができ、[コード](../../../../programming/_/chapters/programming.md#ソースコード)の再利用性が高まる

Iteratorパターンは、[Aggregate](#aggregate)、[ConcreteAggregate](#concreteaggregate)、[Iterator](#iterator)、[ConcreteIterator](#concreteiterator)から構成される。

### Aggregate

**Aggregate**（集合体）は、[Iteratorパターン](#iteratorパターン)において、[Iterator](#iterator)インスタンスを生成する役割を持つ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。Aggregateは集合体そのものの役割を持っており、[Iterator](#iterator)[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成するような[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を持つ。

### Iterator

**Iterator**（反復子）は、[Iteratorパターン](#iteratorパターン)において、集合体に含まれる要素を順番に取り出す責任を持つ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。次の要素や前の要素にアクセスするための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)や、要素の存在チェックを行う[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)、要素を削除する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)などを持つ。

### ConcreteAggregate

**ConcreteAggregate**（具体的な集合体）は、[Iteratorパターン](#iteratorパターン)において、[Aggregate](#aggregate)を実装した具体的な[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。

### ConcreteIterator

**ConcreteIterator**（具体的な反復子）は、[Iteratorパターン](#iteratorパターン)において、[Iterator](#iterator)を実装した具体的な[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。
