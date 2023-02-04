ネットワーク
↓
データベース
↓
Rust
↓
並行プログラミング
↓
人工知能

アルゴリズムは別リポジトリで


## メモ

### input

標準入力を任意の型にパースする関数。

```rust
use std::io::Write;

pub(crate) fn input<T: std::str::FromStr>( hint_: &str ) -> T
{
    print!("Input `{}` >> ", hint_);
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

pub(crate) fn input_vec<T: std::str::FromStr>( hint_: &str ) -> Vec<T>
{
    print!("Input `{}` >> ", hint_);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}
```
