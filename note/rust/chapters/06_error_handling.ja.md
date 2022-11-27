# エラー処理


## 目次

- [パニック](#パニック)
	- [スタックの巻き戻し](#スタックの巻き戻し)
	- [アボート](#アボート)
- [Result](#result)
	- [エラーのキャッチ](#エラーのキャッチ)
	- [Result型のエイリアス](#result型のエイリアス)
	- [エラーの表示](#エラーの表示)
	- [エラーの伝搬](#エラーの伝搬)
	- [複数種類のエラーへの対応](#複数種類のエラーへの対応)
	- [main内でのエラー処理](#main内でのエラー処理)
	- [カスタムエラー型](#カスタムエラー型)


## パニック

プログラム中で**パニック**が起こるのは、次の例のようにプログラム自身のバグに起因すると思われる問題が発生したときである。

- 配列の範囲外へのアクセス
- 整数のゼロ除算
- ErrであるResultに対する `.expect()` の呼び出し
- アサートの失敗

また、プログラマ自身が任意のタイミングで `panic!()` マクロを用いてパニックを起こすこともできる。

パニックが発生した場合、Rustはスタックを巻き戻すか、アボートするかのどちらかを行う。デフォルトではスタックが巻き戻される。

### スタックの巻き戻し

スタックの巻き戻しは次の手順で行われる。

1. エラーメッセージを端末に表示する。
1. スタックを巻き戻す。作られた値やローカル変数、実行中の関数が使っている引数が、作られたのとは逆順にドロップされていく。
1. スレッドを終了する。パニックを起こしたのがメインスレッドだった場合にはプロセスが終了する（終了コード0）。

パニックはスレッド単位で発生するので、あるスレッドがパニックを起こしても、他のスレッドは通常の動作を続けることができる。子スレッドが起こしたパニックを親スレッドから見つけて扱うこともできる。

また、スタックの巻き戻しを**キャッチ**して、スレッドを殺さずに実行を続けることもできる（これには `std::panic::catch_unwind()` を用いる）。

### アボート

パニックを巻き戻している最中に、 `.drop()` メソッド内で2つ目のパニックが起きた場合には致命的な状態とみなされ、巻き戻しが中止してプロセス全体を強制終了する。

また、コンパイル時にパニックした際の動作をカスタマイズでき、1つ目のパニックで強制的にアボートさせることもできる。


## Result

Rustには例外がない代わりに、失敗する可能性のある関数は返り値として `Result` を返す。このような関数は、呼び出すたびにエラー処理を記述する必要がある。

### エラーのキャッチ

最もしっかりと `Result` を扱うには、match式を用いる。

```rust
match get_weather(hometown)
{
	Ok(report) => { display_weather(hometown, &report); },
	Err(err) => { println!("error get_weather(): {}", err); },
}
```

match式を用いると冗長になることが多いので、 `Result` には次のような便利なメソッドが用意されている。

- `result.is_ok()` , `result.is_err()` : `Result` が成功だったかエラーだったかを示すbool値を返す。
- `result.ok()` : 成功した場合の値を `Option<T>` として返す。成功していれば `Some(value)` が、失敗していれば `None` が返され、エラーの値は捨てられる。
- `result.err()` : 失敗した場合の値を `Option<T>` として返す。
- `result.unwrap_or(fallback)` : 成功した場合には成功値を返し、失敗だった場合にはデフォルトの `fallback` を返し、エラーは捨てる。
- `result.unwrap_or_else(fallback_fn)` : ほぼ上述のメソッドと同じであるが、失敗だった場合にデフォルトのクロージャを実行した結果を返す。 `fallback` の計算コストを支払いたくない場合に用いる。
- `result.unwrap()` : 成功だった場合には成功値を返し、失敗だった場合にはパニックを起こす。失敗することがないということが予めわかっている場合によく用いる。
- `result.expect(message)` : ほぼ `unwrap()` と同様で、パニックしたときのメッセージを指定できる。

これらのメソッドは（ `is_ok()` と `is_err()` を除きすべてが）、動作対象となる `result` を消費してしまう。そのため `result` を使いまわしたい場合には、 `reuslt` の参照を取得して、参照に対してこれらのメソッドを実行する。この場合は、 `result.as_ref().ok()` のように書く。

### Result型のエイリアス

多くのモジュールでは、以下の例のような `Result` に対するエイリアスが定義されている。

```rust
use std::error::Error;

pub type Result<T> = result::Result<T, Error>;
```

### エラーの表示

- `println()` : エラーを `{}` フォーマット指定子で表示すると短いメッセージが表示され、 `{:?}` フォーマット指定子で表示するとデバッグ表示となる。
- `err.to_string()` : エラーメッセージを `String` 型で返す。
- `err.source()` : エラーの原因となったエラーがあれば `Option` 型で返す。

入手可能なすべてのエラー原因を表示したい場合、次のような関数を用いるとよい。

```rust
use std::error::Error;

fn print_error( mut err: &dyn Error )
{
	//	発生したエラーを表示
	let _ = writeln!(stderr(), "error: {}", err);

	//	原因となったエラーを再帰的に表示
	while let Some(source) = err.source()
	{
		let _ = writeln!(stderr(), "caused by: {}", source);
		err = source;
	}
}
```

ここでは、 `writeln!()` でエラーが起こったとしてももともとのエラーを表示したい。そこで、 `Result` の値を無視するために `let _ = ...` のようにして警告が出ないようにしている。

### エラーの伝搬

多くの場合、関数の中で起きたエラーの処理は関数の呼び出し元に任せたい。Rustにはこれをサポートする?演算子がある。

```rust
let weather = get_weather(hometown)?;
```

?演算子の動作は次の通り。

- 成功した場合は、 `Reuslt` の成功値を取り出す。
- 失敗した場合は、呼び出し元の関数から即時にリターンして、呼び出し連鎖の上流の関数にエラーを渡す。このため、?演算子は返り値が `Reuslt` 型の関数内部でしか利用できない。

### 複数種類のエラーへの対応

同じ関数内で複数の種類のエラーが発生するというケースは大いにあり得る。しかし、エラーの型が違えばその関数はコンパイルできない。Rustでは、標準ライブラリのエラー型は、「すべてのエラー」を表す `Box<dyn std::error::Error + Send + Sync + 'static>` に変換できる。そのため、次のような型エイリアスを定義しておき、この型を用いることで複数種類のエラーにも対応できるようになる。

```rust
type GenericError = Box<dyn std::error::Error + Send + Sync + 'Static>;
type GenericResult<T> = Result<T, GenericError>;
```

### main内でのエラー処理

エラーを伝搬させていくと、最終的にはmain関数にたどり着く。main内ではmatch式などを用いてエラーを正確に処理するのが望ましい。

### カスタムエラー型

独自のエラー定義は以下のように行うことができる。

```rust
#[derive(Debug, Clone)]
pub struct MyError
{
	pub message: String,
	pub line: usize,
	pub column: usize,
}
```

このエラーを利用したいときは、以下のように記述する。

```rust
return Err(MyError {
	message: "my error message",
	line: current_line,
	column: current_column,
});
```

これを標準のエラー型のように扱えるようにするには `std::error::Error` トレイトと、エラーの表示方法を決める `std::fmt::Display` トレイトを実装する必要がある。

```rust
use std::fmt;

impl std::error::Error for MyError {}

impl fmt::Display for MyError
{
	fn fmt( &self, f: &mut fmt::Formatter ) -> Result<(), fmt::Error>
	{
		write!(f, "{} ({}:{})", self.message, self.line, self.column);
	}
}
```
