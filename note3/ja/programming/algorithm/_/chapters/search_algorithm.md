# 『探索アルゴリズム』

（最終更新： 2023-02-03）


## 目次

1. [全探索](#全探索)
	1. [全探索の例](#全探索の例)
1. [ビット全探索](#ビット全探索)
	1. [ビット全探索の例](#ビット全探索の例)
1. [線形探索](#線形探索)
	1. [線形探索の例](#線形探索の例)
1. [二分探索](#二分探索)
	1. [二分探索の例](#二分探索の例)


## 全探索

**全探索**は、あり得るすべてのパターンをしらみつぶしに調べるアルゴリズム。最もシンプルなアルゴリズムのひとつで、問題を考える際にはまず全探索をしても現実的な時間で実行が終わるのかどうかを検討することが大切。

### 全探索の例

> 赤・青のカードが各1枚ずつあり、それぞれのカードに1以上 $N$ 以下の整数を1つ書き込む。このとき、カードに書かれた整数の合計が $S$ 以下となるような書き方がいくつあるかを出力するプログラムを考える。
>
> 例えば、$N = 3, S = 4$ の場合、6と出力すれば良い（1-1, 1-2, 1-3, 2-1, 2-2, 2-3, 3-1, 3-2, 3-3の組み合わせ）。

- 計算量: $O(N^2)$

```rust
mod utility;
use crate::utility::input;

fn main()
{
    let n: usize = input("N");
    let s: usize = input("S");
    let mut answer = 0;

    //  全探索
    for i in 1..(n + 1)
    {
        for j in 1..(n + 1)
        {
            let sum = i + j;

            if sum <= s
            {
                answer += 1;
            }
        }
    }

    println!("Answer: {}", answer);
}
```


## ビット全探索

**ビット全探索**は、ビット演算を利用することで全探索を行う方法。ビット全探索では、部分集合を全パターン列挙することができるため、組み合わせを全探索する際のテクニックとして使える。

### ビット全探索の例

> $N$ 枚のカードが並べられているとする。左から $i$ 番目 $(1 \geq i \geq N)$ のカードには整数 $A_i$ が書かれている。カードの中からいくつかを選んで、合計がちょうど $S$ となるような組み合わせがあるかを出力するプログラムを考える。
>
> 例えば、 $N = 3, S = 11, (A_1, A_2, A_3)=(2, 5, 9)$ の場合、 $A_1$ と $A_3$ の組み合わせが11となるので、真となる。

- 計算量: $O(2^N)$

```rust
module utility;
use crate::utility::{ input, input_vec };

fn main()
{
    let n: usize = input("N");
    let s: usize = input("S");
    let a: Vec<usize> = input_vec("A");
    let mut answer = false;
    assert_eq!(n, a.len());

    //  ビット全探索
    for bit in 0..(1 << n)
    {
        //  i番目のビットが1となっていれば加算していく
        let mut sum = 0;
        for i in 0..n
        {
            if bit & (1 << i) != 0
            {
                sum += a[i];
            }
        }

        //  合計がSと一致すれば真
        if sum == s
        {
            answer = true;
            break;
        }
    }

    println!("Answer: {:?}", answer);
}
```


## 線形探索

**線形探索**は、配列から検索したい値を見つけ出すためのアルゴリズムのひとつで、全ての要素を順番に検索したい値と比較する。

### 線形探索の例

> 1から $N$ までの整数列があり、その整数列から無作為に1つの整数 $S$ を選ぶ。このとき選ばれた $S$ が何であったかを求めるプログラムを考える。

- 計算量: $O(N)$
- ライブラリ: `rand = "0.6"`

```rust
mod utility;
use crate::utility::input;
use rand::Rng;

fn main()
{
    let n: usize = input("N");
    let v: Vec<usize> = (1..n).collect();

    //  乱数を生成
    let mut rng = rand::thread_rng();
    let s = rng.gen_range(1, n);

    //  各要素と検索したい値を比較
    for x in v
    {
        if x == s
        {
            println!("S: {}", x);
            return;
        }
    }

    unreachable!();
}
```


## 二分探索

**二分探索**は、昇順あるいは降順に並べられた配列から検索したい値を見つけ出すためのアルゴリズムのひとつで、中央の要素と検索したい値との大小比較を行い、検索したい値が中央の要素よりも前にあるか後ろにあるかを判断しながら絞り込んでいく。ソートされていない配列や、大小関係の定義されていない要素には使用できない。

### 二分探索の例

> 1から $N$ までの整数列があり、その整数列から無作為に1つの整数 $S$ を選ぶ。このとき選ばれた $S$ が何であったかを求めるプログラムを考える。

- 計算量: $O(\log_2{N})$
- ライブラリ: `rand = "0.6"`

```rust
mod utility;
use crate::utility::input;
use rand::Rng;

fn main()
{
    let n: usize = input("N");
    let v: Vec<usize> = (1..n).collect();

    //  乱数を生成
    let mut rng = rand::thread_rng();
    let s = rng.gen_range(1, n);

    //  上限、下限、中央のインデックス
    let mut low = 0;
    let mut high = v.len() - 1;
    let mut mid;

    while low <= high
    {
        //  中央のインデックスを更新
        mid = (low + high) / 2;

        //  検索する値
        match v[mid]
        {
            x if x == s =>
            {
                //  一致した場合は終了
                println!("S: {}", v[mid]);
                return;
            },

            //  上限、下限のインデックスを更新
            x if x > s => high = mid + 1,
            x if x < s => low = mid - 1,
            _ => unreachable!(),
        };
    }

    unreachable!();
}
```
