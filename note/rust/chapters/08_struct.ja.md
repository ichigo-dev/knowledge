# 構造体


## 目次

- [名前付きフィールド型構造体](#名前付きフィールド型構造体)
- [タプル型構造体](#タプル型構造体)
- [ユニット型構造体](#ユニット型構造体)
- [構造体のメソッド定義](#構造体のメソッド定義)
	- [型関連関数](#型関連関数)
- [型関連定数](#型関連定数)
- [ジェネリック構造体](#ジェネリック構造体)
	- [定数パラメータを持つジェネリック構造体](#定数パラメータを持つジェネリック構造体)
- [一般的なトレイトの自動実装](#一般的なトレイトの自動実装)
- [内部可変性](#内部可変性)


## 名前付きフィールド型構造体

**名前付きフィールド型構造体**の定義は以下のように行う。

```rust
struct Author
{
	name: String,
	books: Vec<String>,
}
```

定義した構造体の型の値を生成するには、次のような**構造体式**を用いる。

```rust
let author = Author {
	name: "".to_string(),
	books: Vec::new(),
};
```

構造体そのものやそのフィールドはデフォルトでプライベートとなっているため、外部モジュールから利用したい場合は明示的に `pub` キーワードをつける必要がある。


## タプル型構造体

**タプル型構造体**は、構造体名を持つタプルのようなもので、格納されている値は**要素**と呼ぶ。

```rust
struct Bounds(usize, usize);

let image_bounds = Bounds(1024, 768);
```

タプル型構造体は、**ニュータイプ**を作るのに適している。ニュータイプとは構成要素が1つだけの構造体で、型の持つ意味を明確にしたり、より厳密な型チェックを行う目的などで用いる。

```rust
struct Ascii(Vec<u8>);
```


## ユニット型構造体

**ユニット型構造体**は、要素を宣言しない構造体であり、ユニット型 `()` と同様にメモリを消費しない。

```rust
struct Onesuch;
```

ユニット型構造体はトレイトを扱う際に役に立つ。


## 構造体のメソッド定義

構造体に対して**メソッド**を定義するには、 `impl` ブロックを用いる。 `impl` ブロック内では特別なパラメータとして、メソッドを追加する対象の型を表す `Self` を用いることができる。

```rust
struct Queue
{
	older: Vec<char>,
	younger: Vec<char>,
}

impl Queue
{
	pub fn push( &mut self, c: char )
	{
		self.younger.push(c);
	}

	//	この関数の返り値としてSelfを用いることもできる
	pub fn pop( &mut self ) -> Option<char>
	{
		if self.older.is_empty()
		{
			if self.younger.is_empty()
			{
				return None;
			}

			use std::mem::swap;
			swap(&mut self.older, &mut self.younger);
			self.older.reverse();
		}

		self.older.pop()
	}
}
```

メソッドは別名**関連関数**とも呼ばれ、 `impl` ブロックの外で記述された**自由関数**と区別される。

メソッドは最初の引数として、呼び出される対象を `self` という特別な名前で与える必要がある。

### 型関連関数

`impl` ブロック内には、 `self` を引数として取らない**型関連関数**を定義することもできる。

```rust
impl Queue
{
	pub fn new() -> Queue
	{
		Queue { older: Vec::new(), younger: Vec::new() }
	}
}

let mut q = Queue::new();
q.push('*');
```


## 型関連定数

**型関連定数**を用いると、型そのものに値を関連付けることができる。

```rust
pub struct Position
{
	x: f32,
	y: f32,
}

impl Position
{
	const ZERO: Position = Position { x: 0.0, y: 0.0 };
	const UNIT: Position = Position { x: 1.0, y: 0.0 };
	const NAME: &'static str = "Position";
	const ID: u32 = 1;
}
```


## ジェネリック構造体

Rustの構造体は**ジェネリック**にすることで、構造体の定義をテンプレートとして任意の型をプラグインできる。構造体をジェネリックにするには、**型パラメータ**を `<T>` のように記述する。

```rust
struct Queue<T>
{
	older: Vec<T>,
	younger: Vec<T>,
}

impl<T> Queue<T>
{
	pub fn new() -> Queue<T>
	{
		Queue { older: Vec::new(), younger: Vec::new() }
	}
}
```

`impl<T>` としなかった場合、 `Queue<T>` の `T` は型の名前として認識される。つまり、次のような特定の方に対するメソッド定義のように扱われる。

```rust
impl Queue<f64>
{
	fn sum( &self ) -> f64
	{
		/* ... */
	}
}
```

### 定数パラメータを持つジェネリック構造体

ジェネリック構造体は定数をパラメータとすることもできる。

```rust
struct Polynomial<const N: usize>
{
	coefficients: [f64; N],
}

impl<const N: usize> Polynomial<N>
{
	fn new( coefficients: [f64; N] ) -> Polynomial<N>
	{
		Polynomial { coefficients }
	}
}
```


## 一般的なトレイトの自動実装

独自の型に対して、CopyやClone、Debug、PartialEqなどの標準的なトレイトを自動実装するには、 `#[derive]` 属性をつける。

```rust
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point
{
	x: f64,
	y: f64,
}
```


## 内部可変性

Rustでは**内部可変性**パターンを用いることで、制限付きで不変オブジェクトを安全に可変に変更することができる。内部可変性を用いるには、CellやRefCellを利用する。これらを使うことによって、 `mut` ではないメソッドからでも `.get()` や `.set()` で値にアクセスすることができる。

```rust
use std::cell::Cell;

pub struct Robot
{
	hardware_error_count: Cell<u32>,
}

impl Robot
{
	pub fn add_hardware_error( &self )
	{
		let n = self.hardware_error_count.get();
		self.hardware_error_count.set(n + 1);
	}
}
```

RefCellには不変参照や可変参照を返すメソッドがある。通常Rustの参照が安全に使われていることはコンパイル時にチェックされるが、RefCellでは実行時にチェックされるので、パニックを起こさないように実装者が意識しなければならない。

```rust
use std::cell::RefCell;

let ref_cell: RefCell<String> = RefCell::new("hello".to_string());

//	共有参照を取得
let r = ref_cell.borrow();
let count = r.len();
assert_eq!(count, 5);

//	すでに借用されているため、パニックを起こす
let mut w = ref_cell.borrow_mut();
w.push_str(" world");
```

CellやRefCellはスレッド安全ではないことに注意する必要がある。スレッド安全な内部可変性を用いるには、Mutexなどを利用する。
