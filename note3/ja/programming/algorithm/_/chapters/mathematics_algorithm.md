# 『数学アルゴリズム』

（最終更新： 2023-02-03）


## 目次

1. [素数判定](#素数判定)
	1. [シンプルな素数判定](#シンプルな素数判定)


## 素数判定

自然数 $N$ が素数であるかどうかを判定するプログラムを考える。

### シンプルな素数判定

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

また、実際には $2 \leq x \leq N-1$ ではなく $2 \leq x \leq \sqrt{N}$ の範囲を調べて割り切れる整数 $x$ がなければ素数と言える。

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

    println!("Answer: {}", answer);
}
```
