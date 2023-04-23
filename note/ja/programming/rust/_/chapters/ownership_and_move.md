# 『所有権と移動』ノート

（最終更新： 2023-04-23）


## 目次

1. [所有権](#所有権)
1. [移動](#移動)
1. [コピー型](#コピー型)
1. [RcとArc](#rcとarc)


## 所有権

**所有権**は、[Rust](./rust.md#rust)が持つ[メモリ管理](../../../_/chapters/memory_management.md#プログラムのメモリ管理)のための機能で、確保した[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)を使用が終わった時点で自動的に破棄する機能。[C++](../../../_/chapters/programming_language.md#c)においても、[オブジェクト](../../../_/chapters/object_oriented.md#オブジェクト)に所有されている[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)に対して[ポインタ](../../../_/chapters/data_type.md#ポインタ型)を作った場合に、[オブジェクト](../../../_/chapters/object_oriented.md#オブジェクト)が[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)領域を破棄する前に、[ポインタ](../../../_/chapters/data_type.md#ポインタ型)を破棄する義務がある、という考え方のことを所有権という。このような考え方をもとに実装された[C++](../../../_/chapters/programming_language.md#c)の機能をスマートポインタという。[Rust](./rust.md#rust)では所有権の考え方が言語そのものに組み込まれており、すべてのデータに対して[コンパイラ](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイラ)が所有権をチェックすることによって[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)のクリーンアップを強制する。[Rust](./rust.md#rust)におけるすべての値は、その[ライフタイム](./reference_and_lifetime.md#ライフタイム)を決定する唯一の所有者を持ち、所有者が解放（ドロップ）されたときに所有されていた値もドロップされる。

[ベクタ](./data_type.md#ベクタ)や文字列などの可変長の[変数](../../../_/chapters/variable.md#変数)を扱う際には、その[変数](../../../_/chapters/variable.md#変数)のファットポインタが[スタックフレーム](../../../_/chapters/data_type.md#スタック)上に保持され、その[ポインタ](../../../_/chapters/data_type.md#ポインタ型)の先のバッファだけが[ヒープ](../../../../computer/hardware/_/chapters/memory.md#ヒープ領域)上に取られる。このような状態を、バッファが[スタックフレーム](../../../../computer/hardware/_/chapters/memory.md#スタック領域)上の[ポインタ](../../../_/chapters/data_type.md#ポインタ型)に所有されているという。

複雑な所有関係にある要素（[ベクタ](./data_type.md#ベクタ)の中に文字列が格納されている場合など）では、所有者と所有される値はツリー構造をなす。[変数](../../../_/chapters/variable.md#変数)が[スコープ](../../../_/chapters/control_flow.md#スコープ)から外れると、それより下のツリー全体が順次ドロップされる。

[Rust](./rust.md#rust)の所有権には他にも以下のような特徴がある。

- 値を1つの所有者から別の所有者へmove（移動）することができる
- 整数、浮動小数点数、文字などの[プリミティブ型](../../../_/chapters/data_type.md#プリミティブ型)（[ヒープ](../../../../computer/hardware/_/chapters/memory.md#ヒープ領域)上に領域を確保する必要のない固定長変数）については、所有権のルールが適用されない
- [標準ライブラリ](../../../../computer/software/_/chapters/package.md#標準ライブラリ)の `Rc` や `Arc` を利用することで、参照カウントが利用できる
- 値への参照の[借用](./reference_and_lifetime.md#参照と借用)（[参照](./data_type.md#参照)を[関数](../../../_/chapters/function.md#関数)の[引数](../../../_/chapters/function.md#引数)として渡す操作）ができる


## 移動

**移動**(**move**)は、[Rust](./rust.md#rust)が持つ[メモリ管理](../../../_/chapters/memory_management.md#プログラムのメモリ管理)のための機能で、ある所有者によって所有されている値を、別の所有者に渡す操作。[Rust](./rust.md#rust)ではほとんどの[型](../../../_/chapters/data_type.md#型)が、[変数](../../../_/chapters/variable.md#変数)への値の代入、[関数](../../../_/chapters/function.md#関数)への[引数](../../../_/chapters/function.md#引数)の受け渡し、[関数](../../../_/chapters/function.md#関数)からの[返り値](../../../_/chapters/function.md#返り値)の返却の際に、データは[ディープコピー](../../../_/chapters/data_type.md#ディープコピー)されずに移動が発生する。[ディープコピー](../../../_/chapters/data_type.md#ディープコピー)が必要な場合は明示的に変数に対して `clone()` メソッドを呼ぶ。

[Rust](./rust.md#rust)の[コンパイラ](../../../../basics/information_theory/_/chapters/compiler_theory.md#コンパイラ)はこれらの移動を推測することができ、[機械語](../../../_/chapters/programming.md#機械語)レベルで見たときに最終的に値が移動される場所にはじめから[オブジェクト](../../../_/chapters/object_oriented.md#オブジェクト)が作られることも多い。

[ベクタ](./data_type.md#ベクタ)のようにインデックスにより参照される値については、インデックスを指定してその要素だけを移動するということができない。このような操作を実現したい場合は、次のような3つの方法がある。

```rust
let mut v = Vec::new();
for i in 1..100
{
    v.push(i.to_string());
}

// 1. ベクタの最後の要素をポップして取り出す
let x = v.pop().expect("vector empty");

// 2. ベクタの指定したインデックスの場所から要素を取り出し、代わりに最後の要素をそこに入れる
let index = 8;
let y = v.swap_remove(index);

// 3. 取り出した値の代わりに別の値を入れる
let index = 10;
let z = std::mem::replace(&mut v[index], "substitute".to_string());
```

特に `Option<T>` を要素として持つ[ベクタ](./data_type.md#ベクタ)から値を取り出したい場合には、値を移動して代わりに `None` を入れるという使い方が多い。これを実現するための `take()` メソッドも用意されている。

[コレクション型](../../../_/chapters/data_type.md#コンテナ型)は一般に、すべての要素をループで消費するための[メソッド](../../../_/chapters/object_oriented.md#メソッド)を用意している。次のようにしてループに[ベクタ](./data_type.md#ベクタ)を直接渡すと、[ベクタ](./data_type.md#ベクタ)は `v` から移動されて `v` は[未初期化状態](../../../_/chapters/variable.md#参照)となり、繰り返しごとにループは次の要素を[変数](../../../_/chapters/variable.md#変数) `s` に移動する。

```rust
let v = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];

for mut s in v
{
    s.push('!');
    println!("{}", s);
}
```


## コピー型

**コピー型**は、[Rust](./rust.md#rust)において、[移動](#移動)の代わりに[ディープコピー](../../../_/chapters/data_type.md#ディープコピー)が行われるような[型](../../../_/chapters/data_type.md#型)。[ヒープ領域](../../../../computer/hardware/_/chapters/memory.md#ヒープ領域)のリソースを伴う値は、[ディープコピー](../../../_/chapters/data_type.md#ディープコピー)するのに大きなコストが必要になるが、[プリミティブ型](../../../_/chapters/data_type.md#プリミティブ型)のように[移動](#移動)を意識するメリットが少ない[型](../../../_/chapters/data_type.md#型)については、[移動](#移動)のルールを適用しないためにコピー型とする場合がある。

[変数](../../../_/chapters/variable.md#変数)がコピー型であるかを判断する簡単な指標として、値をドロップする際になにか特別なことをしなければいけない[型](../../../_/chapters/data_type.md#型)はコピー型ではない、と考えるとよい。

独自定義の[型](../../../_/chapters/data_type.md#型)については、デフォルトでコピー型ではないものの、[フィールド](../../../_/chapters/object_oriented.md#プロパティ)がすべてコピー型である場合には次のように属性を付与することでコピー型にすることができる。

```rust
#[derive(Copy, Clone)]
struct Size
{
    height: usize,
    width: usize,
}
```


## RcとArc

[Rust](./rust.md#rust)では値が唯一の所有者を持っているが、複数の所有者のすべてがその値を使い終わるまで生存してほしいような値に対しては、 `Rc` や `Arc` といった参照カウントの[ポインタ型](./data_type.md#ポインタ型)を利用することができる。

`Arc` はアトミックな参照カウンタであり、複数の[スレッド](../../../../computer/software/_/chapters/operating_system.md#スレッド)間で値を直接共有しても安全なようにできている。だたし、そのような必要がない場合については、 `Rc` の方が高速である。

```rust
use std::rc::Rc;

let s: Rc<String> = Rc::new("Hello, world".to_string());
let t: Rc<String> = s.clone();
let u: Rc<String> = s.clone();
```

このような[コード](../../../_/chapters/programming.md#ソースコード)を書いた場合、[スタックフレーム](../../../../computer/hardware/_/chapters/memory.md#スタック領域)上では[ヒープ領域](../../../../computer/hardware/_/chapters/memory.md#ヒープ領域)の `Rc` の参照カウントを指す[ポインタ](../../../_/chapters/data_type.md#ポインタ型)だけが保持される。 `Rc` は強い参照カウントを持ち、 `Rc<T>` がクローンされると参照カウントが[インクリメント](../../../_/chapters/operation.md#インクリメント)される。 `Rc` 自身は `String` 型と同様に、値への[ポインタ](../../../_/chapters/data_type.md#ポインタ型)とバッファの容量、長さからなるワード列でできている。

`Rc` [ポインタ](../../../_/chapters/data_type.md#ポインタ型)に所有される値は不変である必要があるため、注意が必要。
