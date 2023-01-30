# 『クラス』

（最終更新： 2023-01-30）


## 目次

1. [オブジェクト指向](#オブジェクト指向)
	1. [オブジェクト](#オブジェクト)
	1. [クラス](#クラス)
	1. [インスタンス](#インスタンス)
	1. [プロトタイプ](#プロトタイプ)
1. [オブジェクト指向の特性](#オブジェクト指向の特性)
	1. [カプセル化](#カプセル化)
	1. [ポリモーフィズム](#ポリモーフィズム)
	1. [ダックタイピング](#ダックタイピング)
	1. [DI](#di)


## オブジェクト指向

**オブジェクト指向**はプログラミングパラダイムのひとつで、非常に多くのプログラミング言語に取り入れられている概念である。相互に作用するオブジェクトを組み合わせることでプログラムを設計する手法で、クラスベースのオブジェクト指向に則ったプログラミング言語が多い（JavaScriptのように、プロトタイプベースのオブジェクト指向もある）。

```cpp
// C++


#include <string>

// クラス宣言
class Goblin
{
    public:

        // メソッド
        int level_up( int diff_ )
        {
            this->m_level += diff_;
            return this->m_level;
        }

    private:

        // プロパティ
        int m_index = 1;
        std::string m_name = "goblin";
        int m_level = 1;
};


int main()
{
    // インスタンス生成
    Goblin goblin;
    int level = goblin.level_up(10);
    printf("Level: %d\n", level);

    return 0;
}
```

```php
<?php

// PHP

// クラス宣言
class Goblin
{
    // メソッド
    public function level_up( $diff_ )
    {
        $this->m_level += $diff_;
        return $this->m_level;
    }

    // プロパティ
    private $m_index = 1;
    private $m_name = "goblin";
    private $m_level = 1;
}

// インスタンス生成
$goblin = new Goblin();
$level = $goblin->level_up(10);
echo("Level: " . $level);

?>
```

```javascript
// JavaScript

// プロトタイプの初期化用
function Goblin()
{
    // プロパティ
    this.m =
    {
        index : 1,
        name : "goblin",
        level : 1,
    };
}

// メソッド
Goblin.prototytpe.level_up = function(diff_)
{
    this.m.level += diff_;
    return this.m.level;
};

// オブジェクト生成
let goblin = Goblin();
let level = goblin.level_up(10);
console.log("Level: " + level);
```

### オブジェクト

**オブジェクト**は、あらゆるものをコンピュータが扱うデータとして抽象化したもの。オブジェクトはプロパティやメソッドを持っており、他のオブジェクトとの相互に作用しあって成り立っている。**プロパティ**（**属性**、**メンバ変数**、**フィールド変数**）はオブジェクトが持つデータや情報のことであり、**メソッド**（**操作**、**メンバ関数**）はオブジェクトの機能や振る舞いのこと。

### クラス

**クラス**は、オブジェクトを生成するための設計図のようなもの。オブジェクトの実体はクラスを元にして生成される。同じクラスから生成されたオブジェクトは、共通のプロパティとメソッドを持つ（ただし、それぞれのオブジェクトが持つプロパティは独立しており、オブジェクトごとに固有）。クラスは実体を持たない設計図であるため、そのままではプロパティやメソッドを利用することはできない。

### インスタンス

**インスタンス**はオブジェクトの別の呼び方。クラスという概念からオブジェクトという実体を生成することを、**インスタンス化**という。

### プロトタイプ

**プロトタイプ**は、プロトタイプベースのオブジェクト指向において用いられる概念。オブジェクトを生成する際に、そのもととなるプロトタイプオブジェクトを指定することで、プロトタイプオブジェクトの共通の機能を利用することができる。このようにして、あるオブジェクトからプロトタイプオブジェクトの機能を参照することを**プロトタイプチェーン**と呼ぶ。

プロトタイプベースのオブジェクト指向にはクラスの概念はなく、全てが実体（オブジェクト、プロトタイプオブジェクト）から成る。クラスベースにおける設計図であるクラスは静的なものであり、一度定義されたクラスはプログラム中で変更されることはない。一方でプロトタイプは、設計図自体をプログラム中で拡張したり変更したりすることができる。


## オブジェクト指向の特性

### カプセル化

**カプセル化**は、オブジェクトが持つ情報を隠蔽することによって、不正な操作ができないようにする仕組みのこと。オブジェクトが持つメソッドやプロパティは、アクセス指定子によってアクセスできる範囲を制限できる。

カプセル化には、オブジェクトの外部からのアクセスを制限することで、オブジェクト内部のデータをを保護して、直接書き換えられないようにする目的がある。オブジェクト内部のデータに直接アクセスできないようにする代わりに、保護されたデータを間接的に操作できるインタフェースとなるようなメソッドを用意しておくという使い方が多い。

```cpp
// C++

#include <string>

class Goblin
{
    // オブジェクト外部からもアクセス可能
    public:

        int level_up( int diff_ )
        {
            this->m_level += diff_;
            return this->m_level;
        }

    // オブジェクト内部からのみアクセス可能（カプセル化されている）
    private:

        int m_index = 1;
        std::string m_name = "goblin";
        int m_level = 1;
};


int main()
{
    Goblin goblin;

    // 公開されたメソッドを通してカプセル化されたプロパティを更新
    int level = goblin.level_up(10);
    printf("Level: %d\n", level);

    return 0;
}
```

```php
<?php

// PHP

class Goblin
{
    // オブジェクト外部からもアクセス可能
    public function level_up( $diff_ )
    {
        $this->m_level += $diff_;
        return $this->m_level;
    }

    // オブジェクト内部からのみアクセス可能（カプセル化されている）
    private $m_index = 1;
    private $m_name = "goblin";
    private $m_level = 1;
}

$goblin = new Goblin();

// 公開されたメソッドを通してカプセル化されたプロパティを更新
$level = $goblin->level_up(10);
echo("Level: " . $level);

?>
```

### ポリモーフィズム

**ポリモーフィズム**（**ポリモルフィズム**、**多様性**）は、同じ識別子のメソッドで複数の異なる振る舞いを定義することができる性質。

例えば、異なるクラスが同じ名前のメソッドを共通して持つことによって、そのメソッドを通して、暗黙的に複数のオブジェクトを切り替えることができる。この性質を利用することで、オブジェクトを利用している側のソースコードを変更することなくクラスを切り替えることが可能となり、メンテナンス性の向上に繋がる。インタフェースや仮想関数の機能を用いることで、クラスのメソッドのシグネチャを共通化することができ、ポリモーフィズムの性質を利用しやすくなる。

```cpp
// C++

#include <string>

class Goblin
{
    public:

        int level_up( int diff_ )
        {
            this->m_level += diff_;
            return this->m_level;
        }

    private:

        int m_index = 1;
        std::string m_name = "goblin";
        int m_level = 1;
};

class Dragon
{
    public:

        int level_up( int diff_ )
        {
            this->m_level += diff_;
            return this->m_level;
        }

        // ポリモーフィズムの例
        // 引数の数が異なる同じ識別子のメソッドを定義
        int level_up( int diff_, int power_ )
        {
            this->m_level += diff_;
            this->m_power += power_ * diff_;
            return this->m_level;
        }

    private:

        int m_index = 2;
        std::string m_name = "dragon";
        int m_level = 1;
        int m_power = 10;
};

int main()
{
    Goblin goblin;
    Dragon dragon;

    // ポリモーフィズムの例
    // 違うクラスだが、同じメソッドを利用できる
    int goblin_level = goblin.level_up(10);

    int dragon_level = dragon.level_up(3);
    dragon_level = dragon.level_up(3, 10);

    return 0;
}
```

```php
<?php

// PHP

class Goblin
{
    public function level_up( $diff_ )
    {
        $this->m_level += $diff_;
        return $this->m_level;
    }

    private $m_index = 1;
    private $m_name = "goblin";
    private $m_level = 1;
}

class Dragon
{
    public function level_up( $diff_ )
    {
        $this->m_level += $diff_;
        $this->m_power += 10 * $diff_;
        return $this->m_level;
    }

    private $m_index = 2;
    private $m_name = "dragon";
    private $m_level = 1;
    private $m_power = 10;
}

// ダックタイピングの例
// 第一引数に受け取るオブジェクトがlevel_upメソッドを実装していればよい
function empower_monster( $monster_, $diff_ )
{
    $level = $monster_->level_up($diff_);
    echo("Level up: " . $level);
    return $level;
}

$goblin = new Goblin();
$dragon = new Dragon();
empower_monster($goblin, 10);
empower_monster($dragon, 3);

?>
```

### ダックタイピング

**ダックタイピング**は、関数やメソッドが、特定のインタフェース（プロパティやメソッド）を持つ全てのオブジェクトを引数として受け取ることができるような性質のこと。ポリモーフィズムの恩恵のひとつで、呼び出される関数（メソッド）は、引数として渡されたデータがどのクラスのオブジェクトであるかを意識する必要がない。

### DI

**DI**(Dependency Injection: **依存性の注入**)は、引数として受け取るオブジェクトがどのクラスのものかに依存せずに、関数やメソッドを実行するという考え方。ダックタイピングにより、共通のインタフェースを持つオブジェクト全てを引数として受け取れるようになり、オブジェクトへの依存性を関数側から縛るのではなく、外部から指定できるようになる。
