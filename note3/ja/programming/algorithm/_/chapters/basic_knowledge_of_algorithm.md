# 『アルゴリズムの基礎知識』

（最終更新： 2023-02-03）


## 目次

1. [アルゴリズム](#アルゴリズム)
1. [ユーティリティ](#ユーティリティ)
	1. [input](#input)
	1. [input_vec](#input_vec)


## アルゴリズム

**アルゴリズム**は、問題を解くための手順のことで、特にコンピュータプログラムについて使われることが多い。適切なアルゴリズムを用いると、処理時間（時間計算量）が大幅に短縮できたり、メモリなどの処理に必要となる資源（空間計算量）を削減できたりする。アルゴリズムを学ぶことで、良いプログラムを作ることができるようになる。


## ユーティリティ

頻繁に使用するプログラムのうち、アルゴリズムに直接的に関係しないものを関数として以下のように定義するので、必要に応じてプログラムを読み替えること。

### input

標準入力を任意の型にパースする関数。

```rust
use std::io::Write;

pub(crate) fn input<T: std::str::FromStr>( name_: &str ) -> T
{
    print!("Input `{}` >> ", name_);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().parse().ok().unwrap()
}
```

### input_vec

スペース区切りの標準入力を任意の型にパースし、 `Vec` として返却する関数。

```rust
use std::io::Write;

pub(crate) fn input<T: std::str::FromStr>( name_: &str ) -> Vec<T>
{
    print!("Input `{}` >> ", name_);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}
```
