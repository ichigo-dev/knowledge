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


```

通信方式

コネクション型とコネクションレス型
コネクション型は、通信をする前にコネクションを確立する通信方式。

コネクションレス型は、相手がいるかどうかを確認せずに送信者の都合でデータを送り付ける方式。

通信相手の数による通信方式の分類

ユニキャストは、1対1の通信のことを指す。1を意味する「Uni」と、投げるを意味する「Cast」を組み合わせた言葉。従来の電話が代表例。

ブロードキャストは、すべてのホストを対象としてデータを送信する。不特定多数に向かって一斉配信を行うテレビ放送が代表例。

マルチキャストは、特定のグループ内での通信を指す。限定された複数のホストが接続するビデオ会議が代表例。

エニーキャストは、特定のグループ内のいずれか1つのコンピュータとの通信を指す。ネットワーク上で、条件を満たしたホストの中からどれか1つと通信を行う。

```


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
