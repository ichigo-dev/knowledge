# トレイトとジェネリクス


## 目次

- [ポリモーフィズム](#ポリモーフィズム)
- [トレイト](#トレイト)
	- [トレイトオブジェクト](#トレイトオブジェクト)
	- [トレイトオブジェクトのメモリ配置](#トレイトオブジェクトのメモリ配置)
- [ジェネリクス](#ジェネリクス)
	- [ジェネリック関数と型パラメータ](#ジェネリック関数と型パラメータ)
- [トレイトの定義と実装](#トレイトの定義と実装)
	- [拡張トレイト](#拡張トレイト)
	- [孤児ルール](#孤児ルール)
	- [Self型](#self型)
	- [サブトレイト](#サブトレイト)
	- [完全修飾メソッド呼び出し](#完全修飾メソッド呼び出し)
- [関連型とジェネリックトレイト](#関連型とジェネリックトレイト)
	- [関連型](#関連型)
	- [ジェネリックトレイト](#ジェネリックトレイト)


## ポリモーフィズム

複数のクラスに同じ名前のメソッドを用意しておくことで、暗黙的に複数のインスタンスの処理を切り替えることができる性質を**ポリモーフィズム**または**多相性**という。Rustではトレイトとジェネリクスによってこのポリモーフィズムをサポートしている。


## トレイト

**トレイト**は、Rustにおける**インタフェース**や**抽象基底クラス**のようなもので、型がサポートする性質を意味している。

トレイトのメソッドを利用したい場合は、そのトレイトを `use` キーワードなどを用いてスコープに入れる必要がある。これはメソッド名の衝突を避けるためである（**完全修飾記法**を用いれば名前が衝突しても呼び出すことができる）。

### トレイトオブジェクト

ある特定のトレイトを実装したオブジェクトを**トレイトオブジェクト**と呼ぶ。変数のサイズはコンパイル時に定まっていなければならないが、トレイトは任意の型に実装可能なため、サイズが定まらない。そこで、実際にはトレイトを実装した型への参照を用いる。

```rust
use std::io::Write;

let mut buf: Vec<u8> = vec![];
let writer: &mut dyn Write = &mut buf;
```

トレイトオブジェクトはコンパイル時にサイズが決定しないため、実行時に**動的ディスパッチ**が行なわれる。**静的ディスパッチ**ではコンパイル時に呼び出される関数がわかっているが、動的ディスパッチでは実行時に仮想関数を呼び出すオーバヘッドがかかるため、**速度は遅くなる**。

### トレイトオブジェクトのメモリ配置

トレイトオブジェクトは、値へのポインタと、値の型を表すテーブル（**仮想テーブル**）へのポインタで構成されるファットポインタである。仮想テーブルはコンパイル時に一度だけ作られ、同じ型のすべてのオブジェクトによって共有される。トレイトオブジェクトのメソッドが呼び出されると、言語が自動的に仮想テーブルの値を参照してどの実装（機械語コードブロックのポインタ）を呼び出すか決定する。

C++では構造体自身に仮想テーブルへのポインタを格納しているが、Rustでは構造体にはフィールドしか入っていない。

```rust
use std::fs::File;
use std::io::Write;

fn say_hello( file: &mut dyn Write )
{
	/* ... */
}

//	FileはWriteトレイトを実装している
let mut local_file = File::create("hello.txt")?;

//	say_hello()の引数は&mut dyn Writeとする
say_hello(&mut local_file);
```


## ジェネリクス

**ジェネリクス**はC++のテンプレートに当たる機能で、様々な型の値に対してジェネリック関数やジェネリック型を扱うことができる。

### ジェネリック関数と型パラメータ

トレイトオブジェクトを引数として取る関数をジェネリック関数として記述すると、関数のシグネチャは次のようになる。

```rust
fn say_hell<W: Write>( file: &mut W ) { /* ... */ }
```

`<W: Write>` は**型パラメータ**で、この型パラメータによって関数がジェネリックになっている。このように型に対して与える条件を**制約**という。

コンパイラは実際にジェネリック関数を使用した型ごとにコンパイルを行う。このプロセスを**単相化**と呼ぶ。単相化の結果、**コードの肥大化**が起こる点には注意が必要である。

多くの場合は引数から実際に与えられた型パラメータを推論することができるが、引数を持たない関数の場合は明示的に呼び出したい型パラメータを指定する必要がある。

```rust
let v = (0 .. 1000).collect::<Vec<i32>>();    //	ターボフィッシュで型パラメータを指定
```

型パラメータとして複数のトレイトを実装していることを要求したい場合は次のように書く。

```rust
fn func<T: Debug + Hash + Eq>( val: T ) { /* ... */ }

//	型に対する制約が長くなってしまい可読性が損なわれることを防ぐための書き方
fn func<M, R>( x: M, y: R )
	where
		M: Mapper + Seralize,
		R: Reducer + Serialize,
{ /* ... */ }
```


## トレイトの定義と実装

トレイトの定義は以下のように行う。トレイトの定義には**デフォルトメソッド**を記述することもできる。

```rust
trait Visible
{
	fn display( &self, index: u8 );
	fn is_active( &self ) -> bool;

	fn display_all( &self )
	{
		for i in (0 .. self.value.len())
		{
			self.display(i);
		}
	}
}
```

このトレイトを型に対して実装する例は以下の通り。

```rust
impl Visible for Board
{
	fn display( &self, index: u8 )
	{
		println!("{}", self.value[index]);
	}

	fn is_active( &self ) -> bool
	{
		self.active
	}
}
```

トレイトには型関連関数を定義することもできる。

### 拡張トレイト

あるトレイトを実装する型全てに別のトレイトを実装すると、**拡張トレイト**となる。

```rust
use std::io::{ self, Write };

trait WriteHtml
{
	fn write_html( &mut self, html: &HtmlDocument ) -> io::Result<()>;
}

//	Writeトレイトを実装した型にWriteHtmlを実装
impl<W: Write> WriteHtml for W
{
	fn write_html( &mut self, html: &HtmlDocument ) -> io::Result<()>
	{
		/* ... */
	}
}
```

### 孤児ルール

トレイトを実装する際は、トレイトか型のどちらかはそのクレートで新しく定義されたものでなければならない。これを**孤児ルール**という。これは、 `impl Write for u8` のような実装を防ぐためである。

C++でも**単一定義の原則**（ODR: One Definition Rule）という同様の制約が設けられている。

### Self型

構造体のメソッド定義では、返り値として、その構造体自体を指す `Self` キーワードを用いることもできる。このキーワードはトレイトでも使うことができる。トレイトはどのような型に対して実装されるかがわからないので、その型自体を指したい場合は `Self` キーワードを使わなければならない。

### サブトレイト

あるトレイトを他のトレイトの拡張として宣言することもできる。下の例では、 `Sub` は `Super` の**サブトレイト**、 `Super` は `Sub` の**スーパートレイト**であり、 `Sub` を実装した型は `Super` も実装しなければならない。

```rust
trait Super
{
	fn super_method( &self );
}

trait Sub: Super
{
	fn sub_method( &self );
}
```

### 完全修飾メソッド呼び出し

下記の例は、ToStringトレイトのto_stringメソッドを実装したstr型に対する実装を呼び出す記法である。

```rust
"hello".to_string();
str::to_string("hello");
ToString::to_string("hello");
<str as ToString>::to_string("hello");
```

下3つは**修飾メソッド呼び出し**、最後の1つを**完全修飾メソッド呼び出し**という。


## 関連型とジェネリックトレイト

### 関連型

次のイテレータの例において、冒頭の `type Item;` は**関連型**である。関連型を用いることで、コンテナ型のトレイトのメソッドの出力の可読性を向上させることができる。

```rust
pub trait Iterator
{
	type Item;

	fn next( &mut self ) -> Option<Self::Item>
	{
		/* ... */
	}
}
```

### ジェネリックトレイト

トレイトは型パラメータをつけることでジェネリクスにすることできる。例えば、関連型は以下のようにジェネリックでも書くことができる（ただし、意味が異なるので注意）。

```rust
pub trait Iterator<Item>
{
	fn next( &mut self ) -> Option<Item>
	{
		/* ... */
	}
}
```

関連型では、「トレイト」と「関連型がとり得る型」が1対1の関係となる。一方、ジェネリックトレイトでは「トレイト」と「型変数がとり得る型」が1対多の関係となる。

ジェネリックトレイトにおいて、Iterator<u8>とIterator<char>は別のトレイトとして扱われるので、双方を任意の型Tに対して実装できる。

関連型を用いた場合はIteratorはIteratorというトレイトでしかないため、任意の型Tに対する実装はひとつに定まる。

つまり、関連型を用いるケースは「ある型に対して別の型が一意に定まる場合」であると言える。