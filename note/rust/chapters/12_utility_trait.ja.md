# ユーティリティトレイト


## 目次

- [言語拡張トレイト](#言語拡張トレイト)
	- [Drop](#drop)
	- [Clone](#clone)
	- [DerefとDerefMut](#derefとderefmut)
	- [FromとInto](#fromとinto)
- [マーカトレイト](#マーカトレイト)
	- [Sized](#sized)
	- [Copy](#copy)
- [共有語彙としてのトレイト](#共有語彙としてのトレイト)
	- [Default](#defalt)
	- [AsRefとAsMut](#asrefとasmut)
	- [BorrowとBorrowMut](#borrowとborrowmut)
	- [TryFromとTryInto](#tryfromとtryinto)
	- [ToOwned](#toowned)


## 言語拡張トレイト

### Drop

Rustでは所有者がいなくなった値は**ドロップ**される。ドロップは、変数がスコープから出たとき、式文の最後、ベクタの後ろの要素を捨てて短くしたとき、などに起きる。

通常はドロップ時に値の保持するヒープストレージを自動的に解放するが、プログラマが独自にドロップ直前の挙動をカスタマイズすることができる。

**Drop**トレイトを実装した型は、drop()の処理を実行してから値を解放する。

```rust
trait Drop
{
    fn drop( &mut self );
}
```

### Clone

**Clone**トレイトは、自身のコピーを作ることができる型のトレイトである。独自定義の構造体のフィールドすべてがCloneトレイトを実装していれば、 `#[derive(Clone)]` として自動的にCloneトレイトを実装することができる。

```rust
trait Clone: Sized
{
    fn clone( &self ) -> Self;
    fn clone_from( &mut self, source: &Self )
    {
        *self = source.clone()
    }
}
```

### DerefとDerefMut

**Dref**トレイトや**DerefMut**トレイトを実装することで、その型に対する `*` や、**参照解決型変換**の動作を指定できる。

```rust
use std::ops::{ Deref, DerefMut };

trait Deref
{
	type Target: ?Sized;
	fn deref( &self ) -> &Self::Target;
}

trait DerefMut: Deref
{
	fn deref_mut( &mut self ) -> &mut Self::Target;
}
```

TargetはSelfが所有しているか参照している

### FromとInto

**From**トレイトと**Into**トレイトを実装することで、ある型の値を消費して別の型を返す変換を定義できる。

```rust
trait Into<T>: Sized
{
    fn into( self ) -> T;
}

trait From<T>: Sized
{
    fn from( other: T ) -> Self;
}
```


## マーカトレイト

### Sized

**Sized**型は、その型の値のメモリ上でのサイズが常に同じになるような型である。例えば、 `u64` は8バイト、 `(f32, f32, f32)` は12バイトとなる。

`T: Sized` のような制約をつけることで、型 `T` のサイズがコンパイル時に確定していることを要求できる。

Rustでは**unsized**な型（strや `[T]` 、dyn型など）を変数に格納したり、引数として渡したりすることはできず、2ワード長のファットポインタを介して扱うことしかできない。ファットポインタは、静的には失われた情報を動的な情報で補っている。

Rustコンパイラにとって、型はSizedであることが暗黙のデフォルトになっている。つまり、 `struct S<T> { /* ... */ }` のように書いたとき、 `struct S<T: Sized> { /* ... */ }` として解釈されるということ。Sizedであることを制約したくなければ、明示的に `struct S<T: ?Sized> { /* ... */ }` と書く必要がある。

### Copy

Rustではほとんどの型にとって代入は値をコピーするのではなく、移動する操作となる。しかし、 `u8` のようなプリミティブ型は**Copy**マーカトレイトを実装しており、このルールは適用されない。

Copyは浅いバイト単位のコピーだけでコピーが可能な型にのみ制限されており、また、Dropトレイトを実装している型はCopyにすることはできない。


## 共有語彙としてのトレイト

### Default

**Default**を実装することで、独自定義の型のデフォルトの値を指定することができる。

```rust
tarit Default
{
    fn default() -> Self;
}
```

独自定義の型のすべてのフィールドがDefaultを実装しているのであれば、 `#[derive(Default)]` として自動的に実装することもできる。

### AsRefとAsMut

ある型が**AsRef**を実装しているなら、その型から&Tを効率的に借用することができる。

```rust
trait AsRef<T: ?Sized>
{
    fn as_ref( &self ) -> &T;
}

trait AsMut<T: ?Sized>
{
    fn as_mut( &mut self ) -> &mut T;
}
```

### BorrowとBorrowMut

**Borrow**を実装しているなら、borrowメソッドで&Tを効率的に借用することができる。AsRefと似ているが、Borrowには次のような制限がある。&Tのハッシュ値や比較が、元の値と同じように行える場合だけBorrowを実装するべきだ、というものである。これはRustのコンパイラによって強制されるものではない。

```rust
trait Borrow<Borrowed: ?Size>
{
    fn borrow( &self ) -> &Borrowed;
}

trait BorrowMut<Borrowed: ?Sized>: Borrow<Borrowed>
{
    fn borrow_mut( &mut self ) -> &mut Borrowed;
}
```

### TryFromとTryInto

**TryFrom**や**TryInto**は、失敗する可能性のある型変換を定義するためのトレイト。例として、i32をi64に変換する場合のように、情報が欠落する可能性がある変換はこのトレイトを実装している。

```rust
trait TryFrom<T>: Sized
{
    type Error;
    fn try_from( value: T ) -> Result<Self, Self::Error>;
}

trait TryInto<T>: Sized
{
    type Error;
    fn try_into( self ) -> Result<T, Self::Error>;
}
```

### ToOwned

**ToOwned**トレイトは参照から所有された値へ変換するための少し緩和された方法を提供している。Cloneは正確にSelfを返さなければいかにが、ToOwnedでは&Selfから借用できるものなら何でも返すことができる。

```rust
trait ToOwned
{
    type Owned: Borrow<Self>;
    fn to_owned( &self ) -> Self::Owned;
}
```
