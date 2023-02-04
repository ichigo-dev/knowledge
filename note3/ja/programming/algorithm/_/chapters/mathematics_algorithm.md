# 『数学アルゴリズム』

（最終更新： 2023-02-03）


## 目次

1. [素数判定](#素数判定)
	1. [単純な素数判定](#単純な素数判定)
	1. [高速な素数判定](#高速な素数判定)
1. [約数列挙](#約数列挙)
1. [素因数分解](#素因数分解)
1. [最大公約数の算出](#最大公約数の算出)
	1. [単純な最大公約数の算出](#単純な最大公約数の算出)
	1. [ユークリッドの互除法](#最大公約数の算出)
	1. [3個以上の最大公約数](#3個以上の最大公約数)


## 素数判定

自然数 $N$ が素数であるかどうかを判定するプログラムを考える。

### 単純な素数判定

自然数 $N$ が素数であるためには、$N$ が $2 \leq x \leq N-1$ を満たすすべての整数 $x$ で割り切れないことを確かめればよい。

- 計算量: $O(N)$

```rust
mod utility;
use utility::input;

fn main()
{
    let n: usize = input("N");
    let mut answer = true;

    if n < 2
    {
        answer = false;
    }
    else
    {
        for i in 2..(n - 1)
        {
            if n % i == 0
            {
                answer = false;
                break;
            }
        }
    }

    println!("Answer: {}", answer);
}
```

### 高速な素数判定

自然数 $N$ について、 $2 \leq x \leq N-1$ ではなく $2 \leq x \leq \sqrt{N}$ の範囲を調べて割り切れる整数 $x$ がなければ素数と言える。この事実は、背理法により証明することができる。

- 計算量: $O(\sqrt{N})$

```rust
mod utility;
use utility::input;

fn main()
{
    let n: usize = input("N");
    let mut answer = true;

    if n < 2
    {
        answer = false;
    }
    else
    {
        let sqrt_n: usize = (n as f32).sqrt().ceil() as usize;
        for i in 2..sqrt_n
        {
            if n % i == 0
            {
                answer = false;
                break;
            }
        }
    }

    println!("Answer: {}", answer.sort());
}
```


## 約数列挙

```rust
mod utility;
use utility::input;

fn main()
{
    let n: usize = input("N");
    let mut answer = Vec::new();

    let sqrt_n: usize = (n as f32).sqrt().ceil() as usize;
    for i in 1..sqrt_n
    {
        if n % i == 0
        {
            answer.push(i);

            if n / i != i
            {
                answer.push(n / i);
            }
        }
    }

    answer.sort();
    println!("Answer: {:?}", answer);
}
```


## 素因数分解

```rust
mod utility;
use utility::input;

fn main()
{
    let mut n: usize = input("N");
    let mut answer = Vec::new();

    let sqrt_n: usize = (n as f32).sqrt().ceil() as usize;
    for i in 2..sqrt_n
    {
        while n % i == 0
        {
            answer.push(i);
            n /= i;
        }
    }

    if n > 1
    {
        answer.push(n);
    }

    println!("Answer: {:?}", answer);
}
```


## 最大公約数の算出

### 単純な最大公約数の算出

```rust
mod utility;
use utility::input;

fn main()
{
    let a: usize = input("A");
    let b: usize = input("B");
    let min = if a < b { a } else { b };
    let mut answer = 1;

    for i in 2..min
    {
        if a % i == 0 && b % i == 0
        {
            answer = i;
        }
    }

    println!("Answer: {}", answer);
}
```

### ユークリッドの互除法

**ユークリッドの互除法**は、2つの自然数の最大公約数を求める効率的なアルゴリズム。この方法では、次の手順で最大公約数を求める。

1. 2つの自然数のうち大きい方を、大きい方を小さい方で割った余りに置き換える。
1. 1の操作を繰り返し、片方が0になったら終了する。このとき、もう一方の数が最大公約数となる。

- 計算量: $O(\log(A+B))$

```rust
mod utility;
use utility::input;

fn gcd( mut a_: usize, mut b_: usize ) -> usize
{
    while a_ > 0 && b_ > 0
    {
        if a_ > b_ { a_ %= b_ }
        else { b_ %= a_ }
    }

    if a_ >= 1 { a_ } else { b_ }
}

fn main()
{
    let a: usize = input("A");
    let b: usize = input("B");
    let answer = gcd(a, b);

    println!("Answer: {}", answer);
}
```

`gcd` 関数は、再帰呼出しを利用すると次のように書くこともできる。

```rust
fn gcd( a_: usize, b_: usize ) -> usize
{
    if b_ == 0 { a_ }
    else { gcd(b_, a_ % b_) }
}
```

### 3個以上の最大公約数

3個の自然数の最大公約数は、2つの自然数の最大公約数を求めた後、その答えともうひとつの数との最大公約数を求めたものになる。3個以上の自然数の場合も同様の操作を繰り返すことで求めることができる。

```rust
mod utility;
use utility::input_vec;

fn gcd( a_: usize, b_: usize ) -> usize
{
    if b_ == 0 { a_ }
    else { gcd(b_, a_ % b_) }
}

fn main()
{
    let a: Vec<usize> = input_vec("A");
    let len = a.len();
    if len < 2 { return; }

    let mut answer = gcd(a[0], a[1]);
    for i in 2..len
    {
        answer = gcd(answer, a[i]);
    }

    println!("Answer: {}", answer);
}
```
