# クロージャ


## 目次

1. [クロージャと変数のキャプチャ](#クロージャと変数のキャプチャ)
	1. [借用するクロージャ](#借用するクロージャ)
	1. [盗むクロージャ](#盗むクロージャ)
1. [関数型とクロージャ型](#関数型とクロージャ型)
1. [クロージャと安全性](#クロージャと安全性)
	1. [殺すクロージャ](#殺すクロージャ)
	1. [FnOnce](#fnonce)
	1. [FnMut](#fnmut)


## クロージャと変数のキャプチャ

**クロージャ**（無名関数式）はその名の通り、名前のない関数である。クロージャは関数のコールバックなどに利用される。

クロージャは外部の関数に属するデータを使うことができる。

```rust
fn sort_by_statistic( cities: &mut Vec<City>, stat: Statistic )
{
    cities.sort_by_key(|city| -city.get_statistic(stat));
}
```

### 借用するクロージャ

前述の例では、クロージャが作られたときに自動的に `stat` への参照を借用する。 `stat` は借用と生存期間のルールに縛られる。

### 盗むクロージャ

以下の例では、 `stat` や `cities` が破棄されるよりも先に `thread::spawn` で生成されたスレッドが終了するとは限らない。そのため、 `cities` や `stat` の所有権をクロージャに借用させるのではなく移動するようにしている。

```rust
use std::thread;

fn start_sorting_thread( mut cities: Vec<City>, stat: Statistic )
    -> thread::JoinHandle<Vec<City>>
{
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };

    thread::spawn(move ||
    {
        cities.sort_by_key(key_fn);
        cities
    });
}
```


## 関数型とクロージャ型

関数とクロージャにも型がある。

```rust
let my_key_fn: fn( &City ) -> i64 = if user.prefs.by_population
    {
        city_population_descending
    }
    else
    {
        city_monster_attack_risk_descending
    };
```

関数はfn型であり、その値は関数の機械語コードのアドレスを表している。一方でクロージャは、Fn、FnMut、FnOnceトレイトのいずれかのインスタンスであり、fn型とは異なる。

```rust
fn( &City ) -> bool     // fn型（関数のみ）
Fn( &City ) -> bool     // Fnトレイト（関数とクロージャの両方）
```


## クロージャと安全性

### 殺すクロージャ

値をドロップするような以下のクロージャを2回呼び出すと、メモリの**二重解放**により未定義動作が起こる（実際にはコンパイルエラーとなる）。

```rust
let my_str = "hello".to_string();
let f = || drop(my_str);
```

### FnOnce

前述のように値をドロップするクロージャは、Fnではなく**FnOnce**トレイトを実装している。FnOnceはその名の通り、一度しか呼び出すことができない。

```rust
trait Fn() -> R
{
    fn call( &self ) -> R;
}

trait FnOnce() -> R
{
    fn call_once( self ) -> R;
}
```

### FnMut

mut参照を持つクロージャは**FnMut**トレイトを実装している。このようなクロージャはスレッド安全ではない。

```rust
trait Fn -> rust
{
    fn call( &self ) -> R;
}

trait FnMut -> rust
{
    fn call_mut( &mut self ) -> R;
}
```

Fn()はFnMut()のサブトレイトとなっており、FnMut()はFnOnce()のサブトレイトとなっている。RustではクロージャがFn()、FnMut()、FnOnce()のどれを実装するのかを自動で判断する。
