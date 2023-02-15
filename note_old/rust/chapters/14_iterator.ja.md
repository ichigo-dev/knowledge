# イテレータ


## 目次

1. [IteratorとIntoIterator](#iteratorとintoiterator)
1. [イテレータの作成](#イテレータの作成)
	1. [IntoIteratorの実装](#IntoIteratorの実装)
	1. [クロージャからのイテレータの生成](#クロージャからのイテレータの生成)
	1. [drain](#drain)
	1. [その他のイテレータの生成方法](#その他のイテレータの生成方法)
1. [イテレータアダプタ](#イテレータアダプタ)
	1. [mapとfilter](#mapとfilter)
	1. [filter_mapとflat_map](#filter_mapとflat_map)
	1. [flatten](#flatten)


## IteratorとIntoIterator

**イテレータ**は配列のような集合的なデータ構造（コレクション、コンテナ）の各要素に対する処理の抽象化である。

RustにおけるイテレータはIteratorトレイトを実装している。

```rust
use std::iter::Iterator

trait Iterator
{
    type Item;

    fn next( &mut self ) -> Option<Self::Item>;

    /* ... */
}
```

IntoIteratorを実装した型は**イテレート可能**と呼ばれ、 `into_iter()` メソッドを呼び出すことでイテレータを得ることができる。

```rust
trait IntoIterator where Self::IntoIter: Iterator<Item=Self::Item>
{
    type Item;
    type IntoIter: Iterator;

    fn into_iter( self ) -> Self::Iterator;
}
```

イテレータは値を生成し、forループのような消費者が値を消費する。


## イテレータの作成

`iter()` や `iter_mut()` を使えばイテレータを作成することができる。

```rust
let v = vec![4, 29, 42];
let mut v_iter = v.iter();
assert_eq!(v_iter.next(), Some(&4));
assert_eq!(v_iter.next(), Some(&29));
assert_eq!(v_iter.next(), Some(&42));
assert_eq!(v_iter.next(), None);
assert_eq!(v_iter.next(), None);
```

### IntoIteratorの実装

IntoIteratorを実装した型は、 `into_iter()` を呼び出すことができる。

- 共有参照に対する `into_iter()` は、アイテムの共有参照を生成するイテレータを返す。
- 可変参照に対する `into_iter()` は、アイテムの可変参照を生成するイテレータを返す。
- コレクションの値に対する `into_iter()` は、アイテムを値で返すイテレータを返す。アイテムの所有権はコレクションから消費者に移る。

### クロージャからのイテレータの生成

`from_fn()` や `successors()` を用いれば、クロージャを元にイテレータを生成することができる。

```rust
let lengths: Vec<f64> =
    from_fn(|| Some((random::<f64>() - random::<f64>).abs()))
    .take(1000)
    .collect();
```

```rust
use enum::Complex;
use std::iter::successors;

fn escape_time( c: Complex<f64>, limit: usize ) -> Option<usize>
{
    let zero = Complex { re: 0.0, im: 0.0 };
    successors(Some(zero), |&z| { Some(z * z + c) })
        .take(limit)
        .enumerate()
        .find(|(_i, z)| z.norm_sqr() > 4.0)
        .map(|(_i, z)| i)
}
```

### drain

多くのコレクションが実装している `drain()` は、コレクションへの可変参照を引数として取り、値の所有権を消費者に引き渡すイテレータを返す。 

```rust
let mut outer = "Earth".to_string();
let inner = String::from_iter(outer.drain(1..4));

assert_eq!(outer, "Eh");
assert_eq!(inner, "art"
```


## イテレータアダプタ

イテレータが手に入れば、Iteratorトレイトが提供する多様な**アダプタメソッド**が利用できる。イテレータアダプタはオーバーヘッドのない抽象化であり、イテレータチェーン全体は1つの機械語コードに変換できる。

### mapとfilter

**mapアダプタ**は個々の要素に対してクロージャを適用するイテレータに変換する。**filterアダプタ**は、個々の要素のうちの一部を取り除いたイテレータに変換する。

```rust
let text = "  ponies  \n    giraffes\niguanas   \nsquid".to_string();
let v: Vec<&str> = text.lines()
    .map(str::trim)
    .filter(|s| *s != "iguanas")
    .collect();
assert_eq!(v, ["ponies", "giraffes", "squid"]);
```

### filter_mapとflat_map

**filter_mapアダプタ**はmapと似ているが、クロージャはアイテムを新しいアイテムに変換するか、繰り返しからドロップする。

```rust
use std::str::FromStr;

let text = "1\nfrond  .25  189\n3.1415 estuary\n";
for number in text
    .split_whitespace()
    .filter_map(|w| f64::from_str(w).ok())
{
    println!("{:4.2}", number.sqrt());
}

/*
    filterとmapで同じことをしようとすると以下のようになる

    for number in text
        .split_whitespace()
        .map(|w| f64::from_str(w))
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
    {
        println!("{:4.2}", number.sqrt());
    }
*/
```

**flat_mapアダプタ**は、mapやfilter_mapの延長にあるイテレータアダプタである。mapのクロージャはアイテムを1個だけ返し、filter_mapのクロージャはアイテムを1個か0個返すが、flat_mapのクロージャはアイテムを任意の個数返す。

```rust
let vec = vec!["Amusement"," ", "Creators"];
let amusement_creators = vec.iter()
    .flat_map(|s| s.chars())
    .collect::<String>();

assert_eq!(&amusement_creators, "Amusement Creators");
```

### flatten

**flattenアダプタ**は、イテレータの生成するアイテムがそれぞれイテレータであることを仮定して、それらをつなぎ合わせる。

```rust
assert_eq!(
	vec![None, Some("day"), None, Some("one")]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>(),
    vec!["day", "one"]
);
```
