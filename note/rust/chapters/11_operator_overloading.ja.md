# 演算子オーバーロード


## 目次

- [独自型への演算子オーバーロード](#独自型への演算子オーバーロード)
	- [演算子オーバーロードに用いるトレイト](#演算子オーバーロードに用いるトレイト)
- [算術演算子のオーバーロード](#算術演算子のオーバーロード)


## 独自型への演算子オーバーロード

Rustでは、独自の方に対して組み込みトレイトを実装することで、算術演算などの演算が使えるようになる。この機能を**演算子オーバーロード**という。

### 演算子オーバーロードに用いるトレイト

| カテゴリ             | トレイト               | 演算子                       |
|----------------------|------------------------|------------------------------|
| 単項演算子           | std::osp::Neg          | \-x                          |
|                      | std::osp::Not          | \!x                          |
| 算術演算子           | std::osp::Add          | x \+ y                       |
|                      | std::osp::Sub          | x \- y                       |
|                      | std::osp::Mul          | x \* y                       |
|                      | std::osp::Div          | x / y                        |
|                      | std::osp::Rem          | x % y                        |
| ビット演算子         | std::osp::BitAnd       | x & y                        |
|                      | std::osp::BitOr        | x \| y                       |
|                      | std::osp::BitXor       | x ^ y                        |
|                      | std::osp::Shl          | x << y                       |
|                      | std::osp::Shr          | x >> y                       |
| 複合代入演算子       | std::osp::AddAssign    | x \+= y                      |
|                      | std::osp::SubAssign    | x \-= y                      |
|                      | std::osp::MulAssign    | x \*= y                      |
|                      | std::osp::DivAssign    | x /= y                       |
|                      | std::osp::RemAssign    | x %= y                       |
| 複合代入ビット演算子 | std::osp::BitAndAssign | x &= y                       |
|                      | std::osp::BitOrAssign  | x \|= y                      |
|                      | std::osp::BitXorAssign | x ^= y                       |
|                      | std::osp::ShlAssign    | x <<= y                      |
|                      | std::osp::ShrAssign    | x >>= y                      |
| 比較                 | std::cmp::PartialEq    | x == y, x != y               |
|                      | std::cmp::PartialOrd   | x < y, x <= y, x > y, x >= y |
| インデックス         | std::ops::Index        | x[y], &x[y]                  |
|                      | std::ops::IndexMut     | x[y] = z, &mut x[y]          |


## 算術演算子のオーバーロード

独自型が `+` 演算子をサポートするには、Addを実装する。

```rust
use std::ops::Add;

impl Add for Complex<T>
    where
        T: Add<Output = T>,
{
    type Output = Self;

    fn add( self, rhs: Self ) -> Self
    {
        Complex
        {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
```

また、さらに制約を緩めて左右のオペランドの型が異なる演算をサポートするようにもできる。

```rust
use std::ops::Add;

impl<L, R> Add for Complex<L>
    where
        L: Add<R>,
{
    type Output = Complex<L::Output>;

    fn add( self, rhs: Complex<R> ) -> Self::Output
    {
        Complex
        {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
```
