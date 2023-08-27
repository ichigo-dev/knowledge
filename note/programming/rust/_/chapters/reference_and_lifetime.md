# 『参照とライフタイム』ノート

（最終更新： 2023-04-23）


## 目次

1. [参照と借用](#参照と借用)
	1. [共有参照](#共有参照)
	1. [可変参照](#可変参照)
1. [参照の使い方](#参照の使い方)
	1. [暗黙的な参照解決と参照の借用](#暗黙的な参照解決と参照の借用)
	1. [参照の代入](#参照の代入)
	1. [参照への参照](#参照への参照)
1. [ライフタイム](#ライフタイム)
	1. [ボローチェッカー](#ボローチェッカー)
	1. [staticライフタイム](#staticライフタイム)
	1. [共有と変更におけるライフタイム](#共有と変更におけるライフタイム)
	1. [バッファの再確保におけるライフタイム](#バッファの再確保におけるライフタイム)
	1. [構造体とライフタイム](#構造体とライフタイム)


## 参照と借用

[参照](./data_type.md#参照)は、[Rust](./rust.md#rust)が[所有権](./ownership_and_move.md#所有権)を持たない[ポインタ型](../../../_/chapters/data_type.md#ポインタ型)。[参照](./data_type.md#参照)は[参照](./data_type.md#参照)先よりも長生きしてはいけないので、これを強調するために[Rust](./rust.md#rust)ではある[参照](./data_type.md#参照)を作ることを**借用**と呼ぶ。

[関数](../../../_/chapters/function.md#関数)に[引数](../../../_/chapters/function.md#引数)を渡す時に[所有権](./ownership_and_move.md#所有権)を[移動](./ownership_and_move.md#移動)してしまうと、それ以降は[引数](../../../_/chapters/function.md#引数)に使った値が使えなくなってしまう（**値渡し**）。そのため、[関数](../../../_/chapters/function.md#関数)の[引数](../../../_/chapters/function.md#引数)として[参照](./data_type.md#参照)を用いるのが適切な場合も多い（**参照渡し**）。

### 共有参照

**共有参照**は、読み取り専用の[参照](./data_type.md#参照)で、[参照](./data_type.md#参照)先の値を書き換えることはできない。ある値に対する共有参照は同時に複数取ることができる。共有参照は、 `&T` のようにして記述する。

### 可変参照

**可変参照**は、読み書き可能な[参照](./data_type.md#参照)で、[参照](./data_type.md#参照)先の値を読み書きできる。可変参照は排他的であり、ある値に対して可変参照をとったときには他のあらゆる参照（[共有参照](#共有参照)も可変参照も）とることができない。可変参照は、 `&mut T` のようにして記述する。

```rust
// show()は値を読んでいるだけなので、共有参照を取るようにすれば十分
fn show_table( table: &Table )
{
    // コレクション型に対して繰り返し実行すると、普通は所有権が移動されて値が消費されてしまう
    // 共有参照に対して繰り返し実行すると、個々のエントリのキーと値に対する共有参照が作られる
    for (key, values) in table
    {
        println!("key = {}:", key);

        for value in values
        {
            println!(" {}", value);
        }
    }
}

// sort()は値を書き換えるので、可変参照を取る必要がある
fn sort_table( table: &mut Table )
{
    for (_key, values) in table
    {
        values.sort();
    }
}
```


## 参照の使い方

### 暗黙的な参照解決と参照の借用

[Rust](./rust.md#rust)では、 `&` [演算子](../../../_/chapters/operation.md#演算)によって[参照](./data_type.md#参照)を明示的に作り、 `*` [演算子](../../../_/chapters/operation.md#演算)で[参照](./data_type.md#参照)解決を行う。また、 `.` [演算子](../../../_/chapters/operation.md#演算)が必要に応じて暗黙に左側の[オペランド](../../../_/chapters/operation.md#演算)を[参照](./data_type.md#参照)解決する。さらに `.` [演算子](../../../_/chapters/operation.md#演算)は、必要であれば暗黙に左[オペランド](../../../_/chapters/operation.md#演算)への[参照](./data_type.md#参照)を借用する。

```rust
// .演算子による暗黙的な参照解決
struct Book
{
    name: &'static str,
    published: bool,
}
let book = Book { name: "my book", published: false };
let book_ref = &book;
assert_eq!(book_ref.name, "my book");    // assert_eq!((*book_ref).name, "my book");

// .演算子による暗黙的な参照の借用
let mut v = vec![5, 1, 4, 2, 3];
v.sort();                                // (&mut v).sort();
```

### 参照の代入

[変数](../../../_/chapters/variable.md#変数)に[参照](./data_type.md#参照)を[代入](../../../_/chapters/variable.md#代入)すると、その[変数](../../../_/chapters/variable.md#変数)は新しい場所を指すようになる。下記のような[コード](../../../_/chapters/programming.md#ソースコード)を書いた場合、[参照](./data_type.md#参照) `r` は最初 `x` を指しているが、[代入](../../../_/chapters/variable.md#代入)後は `y` を指すようになる。[C++](../../../_/chapters/programming_language.md#c)ではこのような書き方をすると、暗黙的に参照解決が行われてしまい、 `r` が指し示す先に `y` の[ポインタ](../../../_/chapters/data_type.md#ポインタ型)が代入されるという挙動になり、これは明らかに直感に反している。

```rust
let x = 10;
let y = 20;
let mut r = &x;
r = &y;
```

### 参照への参照

[Rust](./rust.md#rust)は[参照](./data_type.md#参照)への[参照](./data_type.md#参照)を許しており、 `.` [演算子](../../../_/chapters/operation.md#演算)は[型](../../../_/chapters/data_type.md#型)をチェックして何段でも[参照](./data_type.md#参照)解決を行ってくれる。

```rust
struct Position
{
    x: usize,
    y: usize,
}
let position = Position { x: 100, y: 200 };
let r = &position;
let rr = &r;
let rrr = &rr;

assert_eq!(rrr.y, 200);
```


## ライフタイム

**ライフタイム**（**生存期間**）は、[Rust](./rust.md#rust)[コンパイラ](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイラ)が[参照型](./data_type.md#参照)に対して割り当てるもので、[プログラム](../../../_/chapters/programming.md#プログラム)実行中にその[参照](./data_type.md#参照)が安全に利用できる期間を指す。

[ローカル変数](../../../_/chapters/variable.md#ローカル変数)の[参照](./data_type.md#参照)を借用して、その[変数](../../../_/chapters/variable.md#変数)を[スコープ](../../../_/chapters/control_flow.md#スコープ)の外に持ち出すと、[ダングリングポインタ](../../../_/chapters/memory_management.md#ダングリングポインタ)となる。ライフライムの成約により、このような[参照](./data_type.md#参照)が作られることがなくなっている。

```rust
{
    let r;
    {
        let x = 1;
        r = &x;
    }
    assert_eq!(*r, 1);       // スコープの外ではxにアクセスできない
}
```

ライフタイムのルールは以下のとおり。

- ある[変数](../../../_/chapters/variable.md#変数)の[参照](./data_type.md#参照)は、その[変数](../../../_/chapters/variable.md#変数)よりも長生きしてはならない
- ある[変数](../../../_/chapters/variable.md#変数)に格納した[参照](./data_type.md#参照)は、少なくともその[変数](../../../_/chapters/variable.md#変数)と同じだけ生きていなければならない（[変数](../../../_/chapters/variable.md#変数)のライフタイムは、その[変数](../../../_/chapters/variable.md#変数)から[借用](#参照と借用)した[参照](./data_type.md#参照)のライフタイムを包含していなければならない）

また、[Rust](./rust.md#rust)は[変数](../../../_/chapters/variable.md#変数)のライフタイムとして可能な限り短い期間を想定する。

### ボローチェッカー

**ボローチェッカー**は、[Rust](./rust.md#rust)[コンパイラ](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイラ)が[ライフタイム](#ライフタイム)を利用して[参照](./data_type.md#参照)の安全性を保証する機能。

### staticライフタイム

**'staticライフタイム**は、[プログラム](../../../_/chapters/programming.md#プログラム)の開始から終了まで存在する[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)リソースに割り当てられる[ライフタイム](#ライフタイム)。

ある `static` な[変数](../../../_/chapters/variable.md#変数)を更新する以下のような[関数](../../../_/chapters/function.md#関数)を考える。

```rust
static mut STASH: &i32 = &128;

fn update( p: &i32 )
{
    unsafe
    {
        STASH = p;
    }
}
```

このとき、[関数](../../../_/chapters/function.md#関数)の[シグネチャ](../../../_/chapters/function.md#シグネチャ)は**ライフタイムパラメータ**を省略している（実際は[Rust](./rust.md#rust)の[コンパイラ](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイラ)が自動的に補完してくれるので省略可能）。これを明示的に記述すると以下のようになる。

```rust
fn update<'a>( p: &'a i32 ) { /* ... */ }
```

`'a` は任意の[ライフタイム](#ライフタイム)であり、[引数](../../../_/chapters/function.md#引数) `p` が任意のライフタイムパラメータを持っていることを示す。しかし、static変数である `STASH` に対して任意の[ライフタイム](#ライフタイム)を持つ[変数](../../../_/chapters/variable.md#変数) `p` を[代入](../../../_/chapters/variable.md#代入)するのは、[ライフタイム](#ライフタイム)のルールに反している（[変数](../../../_/chapters/variable.md#変数) `p` は少なくとも `STASH` と同じだけの[ライフタイム](#ライフタイム)を持たなければならない）。そこで、[関数](../../../_/chapters/function.md#関数)の[引数](../../../_/chapters/function.md#引数)として取りうる値の[ライフタイム](#ライフタイム)を'staticライフタイムにすることでこの問題を解決できる。

```rust
fn update( p: &'static i32 ) { /* ... */ }
```

### 共有と変更におけるライフタイム

[ライフタイム](#ライフタイム)の制約により、[参照](./data_type.md#参照)が[スコープ](../../../_/chapters/control_flow.md#スコープ)から外れた[変数](../../../_/chapters/variable.md#変数)を指す、という問題が起きないことが保証されている。次の例は、[ダングリングポインタ](../../../_/chapters/memory_management.md#ダングリングポインタ)が発生する別のケースである。

```rust
let v = vec![1, 2, 3, 4, 5];
let r = &v;
let aside = v;    // ここで移動が発生する（vのライフタイムの終わり）
r[0];             // 未初期化状態となったvを使用しようとしている（rのライフタイムの終わり）
```

[Rust](./rust.md#rust)ではこのような[プログラム](../../../_/chapters/programming.md#プログラム)は[エラー](../../../_/chapters/programming.md#エラー)となる。 `v` の[参照](./data_type.md#参照)である `r` の[ライフタイム](#ライフタイム)が、 `v` の[ライフタイム](#ライフタイム)よりも長くなってはいけないというルールに反しているためである。

### バッファの再確保におけるライフタイム

[ベクタ](./data_type.md#ベクタ)において、その[変数](../../../_/chapters/variable.md#変数)のもともとの容量を超える要素が加えられようとするとき、[ベクタ](./data_type.md#ベクタ)はより容量の大きいバッファを再確保しようとする。このバッファの再確保により[ダングリングポインタ](../../../_/chapters/memory_management.md#ダングリングポインタ)が発生する例を考える。

```rust
// 他のスライスによってベクタを拡張する関数
fn extend( vec: &mut Vec<f32>, slice: &[f32] )
{
    for elem in slice
    {
        vec.push(*elem);
    }
}

let mut wave = Vec::new();
let head = vec![0.0, 1.0];
let tail = [0.0, -1.0];

extend(&mut wave, &head);
extend(&mut wave, &tail);

// ここでバッファの再確保が起きるとする
extend(&mut wave, &wave);
```

最後の `extend()` 呼び出し時にバッファの再確保が起きた場合に、第2[引数](../../../_/chapters/function.md#引数)で渡された[スライス](./data_type.md#スライス)の[参照](./data_type.md#参照)先がなくなってしまう。しかし、[Rust](./rust.md#rust)の[参照](./data_type.md#参照)のルールにより、 `wave` の[可変参照](#可変参照)と[共有参照](#共有参照)を同時に作ることはできない。これは言い換えると、[可変参照](#可変参照)の[ライフタイム](#ライフタイム)は他のいかなる[参照](./data_type.md#参照)の[ライフタイム](#ライフタイム)とも重なってはいけない、と説明できる。

### 構造体とライフタイム

構造体が[参照](./data_type.md#参照)を含む場合、[参照](./data_type.md#参照)の[ライフタイム](#ライフタイム)を指定する必要がある。次の例では、任意の[ライフタイム](#ライフタイム) `<'elt>` に対して、[ライフタイム](#ライフタイム)が `<'elt>` である[参照](./data_type.md#参照)を保持する構造体を定義している。

```rust
// greatestとleastの参照のライフタイムは一致しなければいけない
struct Extrema<'elt>
{
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'s>( slice: &'s [i32] ) -> Extrema<'s>
{
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len()
    {
        if slice[i] > *greatest { greatest = &slice[i]; }
        if slice[i] < *least { least = &slice[i]; }
    }
    Extrema { greatest, least }
}
```
