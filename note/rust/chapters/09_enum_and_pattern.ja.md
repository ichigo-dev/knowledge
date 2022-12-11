# 列挙型とパターン


## 目次

1. [列挙型](#列挙型)
	1. [データを保持する列挙型](#データを保持する列挙型)
	1. [ジェネリック列挙型](#ジェネリック列挙型)
1. [パターン](#パターン)
	1. [ワイルドカードパターン](#ワイルドカードパターン)
	1. [参照パターン](#参照パターン)
	1. [マッチガード](#マッチガード)


## 列挙型

**列挙型**または**enum**は、複数の異なる定数を1つの集合として定義するために用いられる。次の例では、3つの取りうる**ヴァリアント**（または**構成子**）を持つ列挙型を定義している。

```rust
enum Ordering
{
    Less,
    Equal,
    Greater,
}
```

このようなスタイルの列挙型はメモリ上では整数として表現されるため、使用する整数値を指定することもできる。

```rust
enum HttpStatus
{
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}
```

### データを保持する列挙型

列挙型のヴァリアントには、**タプル型ヴァリアント**や**構造体型ヴァリアント**を用いることができる。

```rust
// 普通の列挙型
enum TimeUnit
{
    Years,
    Months,
    Days,
    Hours,
    Minutes,
    Seconds,
}


// タプル型ヴァリアントを持つ列挙型
enum RoughTime
{
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

let rt1 = RoughTime::InThePast(TimeUnit::Years, 4);
let rt2 = RoughTime::InThePast(TimeUnit::Hours, 3);


// 構造体型ヴァリアントを持つ列挙型
struct Position2d
{
    x: u32,
    y: u32,
}

struct Position3d
{
    x: u32,
    y: u32,
    z: u32,
}

enum Position
{
    Position2d { x: u32, y: u32 },
    Position3d { x: u32, y: u32, z: u32 },
}

let p2d = Position::Position2d { x: 1, y: 30 };
```

### ジェネリック列挙型

下記は、標準ライブラリの中でもRustで最もよく使われる2つのデータ構造の例である。

```rust
enum Option<T>
{
    None,
    Some(T),
}

enum Reuslt<T, E>
{
    Ok(T),
    Err(E),
}
```


## パターン

match式は**パターンマッチ**を行う。パターンに、次の例の `units` や `count` のように単純な識別子が入っていると、パターン以降のコードでローカル変数となり、マッチ対象の値に入っていた値は新しい変数にコピーもしくは移動される。

```rust
enum RoughTime
{
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn rough_time_to_english( rt: RoughTime ) -> String
{
    match rt
    {
        RoughTime::InThePast(units, count) =>
        {
            format!("{} {} ago", count, units.plural())
        },
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(units, count) =>
        {
            format!("{} {} from now", count, units.plural())
        },
    }
}
```

### ワイルドカードパターン

パターンマッチではすべてのパターンを列挙する必要がある。パターンとして変数名を用いると、どのような値にもマッチするようになる。

```rust
match meadow.count_rabbits()
{
    0 => {},
    1 => println!("A rabbit is nosing around in the clover"),
    n => println!("There are {} rabbits hopping about in the meadow", n),
}
```

また、マッチした値に興味がない場合はアンダースコアで表される**ワイルドカードパターン**を用いる。

```rust
let caption = match photo.tagged_pet()
{
    Pet::Tyrannosaur => "RRRAAAAHHHHH",
    Pet::Samoyed => "*dog thoughts",
    _ => "I'm cute, love me",
}
```

### 参照パターン

matchの対象となる値はパターンにmoveされる。一方、matchの対象となる値がCopyを実装していた場合は、ドロップされない。

```rust
match account
{
    Account { name, language } =>
    {
        ui.greet(&name, &language);
    
        // accountはmoveされているのでエラー
        ui.show_settings(&account);
    },
}
```

値がmoveされては困る場合、 `ref` キーワードを用いて値を**借用**する。

```rust
match account
{
    Account { ref name, ref language } =>
    {
        ui.greet(name, language);
        ui.show_settings(&account);
    }
}
```

また、 `mut` キーワードを用いることで、可変参照を借用することもできる。

```rust
match line_result
{
    Err(ref err) => log_error(err),
    Ok(ref mut line) =>
    {
        trim_comments(line);
        handle(line);
    },
}
```

### マッチガード

matchの分岐には、**マッチガード**を用いることで条件を追加することができる。

```rust
match point_to_hex(click)
{
    None => { /* ... */ },
    Some(hex) if hex == current_hex => { /* ... */ },
    Some(hex) => { /* ... */ },
}
```
