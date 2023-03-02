# 『クラス』

（最終更新： 2023-03-02）


## 目次

1. [オブジェクト指向](#オブジェクト指向)
	1. [オブジェクト](#オブジェクト)
	1. [メンバ](#メンバ)
	1. [プロパティ](#プロパティ)
	1. [メソッド](#メソッド)
	1. [クラス](#クラス-1)
	1. [インスタンス](#インスタンス)
	1. [プロトタイプ](#プロトタイプ)
1. [カプセル化](#カプセル化)
	1. [アクセス指定子](#アクセス指定子)
	1. [public](#public)
	1. [private](#private)
	1. [protected](#protected)
1. [継承](#継承)
	1. [親クラス](#親クラス)
	1. [子クラス](#子クラス)
	1. [メソッドオーバライド](#メソッドオーバライド)
1. [ポリモーフィズム](#ポリモーフィズム)
	1. [ダックタイピング](#ダックタイピング)
	1. [DI](#di)
1. [抽象化](#抽象化)
	1. [抽象クラス](#抽象クラス)
	1. [具象クラス](#具象クラス)
	1. [インタフェース](#インタフェース)
1. [this](#this)
1. [特別なメソッド](#特別なメソッド)
	1. [コンストラクタ](#コンストラクタ)
	1. [デストラクタ](#デストラクタ)
1. [静的メンバ](#静的メンバ)
	1. [staticプロパティ](#staticプロパティ)
	1. [staticメソッド](#staticメソッド)


## オブジェクト指向

**オブジェクト指向**は、[プログラミングパラダイム](./basic_knowledge_of_programming.md#プログラミングパラダイム)のひとつで、非常に多くの[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)に取り入れられている概念である。相互に作用する[オブジェクト](#オブジェクト)を組み合わせることで[プログラム](./basic_knowledge_of_programming.md#プログラム)を設計する手法で、[クラス](#クラス-1)ベースのオブジェクト指向に則った[プログラミング言語](./basic_knowledge_of_programming.md#プログラミング言語)が多い（[JavaScript](./programming_language.md#javascript)のように、[プロトタイプ](#プロトタイプ)ベースのオブジェクト指向もある）。

```cpp
// C++

#include <string>

// クラス宣言
class Dragon
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
        std::string m_name = "dragon";
        int m_level = 1;
};


int main()
{
    // インスタンス生成
    Dragon dragon;
    int level = dragon.level_up(10);
    printf("Level: %d\n", level);

    return 0;
}
```

```php
<?php

// PHP

// クラス宣言
class Dragon
{
    // メソッド
    public function level_up( $diff_ )
    {
        $this->m_level += $diff_;
        return $this->m_level;
    }

    // プロパティ
    private $m_index = 1;
    private $m_name = "dragon";
    private $m_level = 1;
}

// インスタンス生成
$dragon= new Dragon();
$level = $dragon->level_up(10);
echo("Level: " . $level);

?>
```

```javascript
// JavaScript

// プロトタイプの初期化用
function Dragon()
{
    // プロパティ
    this.m =
    {
        index : 1,
        name : "dragon",
        level : 1,
    };
}

// メソッド
Dragon.prototytpe.level_up = function(diff_)
{
    this.m.level += diff_;
    return this.m.level;
};

// オブジェクト生成
let dragon = Dragon();
let level = dragon.level_up(10);
console.log("Level: " + level);
```

### オブジェクト

**オブジェクト**は、あらゆるものを[コンピュータ](../../../computer/_/chapters/basic_knowledge_of_computer.md#コンピュータ)が扱うデータとして抽象化したもの。オブジェクトは[プロパティ](#プロパティ)や[メソッド](#メソッド)を持っており、他のオブジェクトとの相互に作用しあって成り立っている。

### メンバ

**メンバ**は、[オブジェクト](#オブジェクト)が持つ[プロパティ](#プロパティ)や[メソッド](#メソッド)。

### プロパティ

**プロパティ**（**属性**、**メンバ変数**、**フィールド変数**）は、[オブジェクト](#オブジェクト)が持つデータや情報。

### メソッド

**メソッド**（**操作**、**メンバ関数**）は、[オブジェクト](#オブジェクト)の機能や振る舞い。

### クラス

**クラス**は、[オブジェクト](#オブジェクト)を生成するための設計図のようなもの。[オブジェクト](#オブジェクト)の実体はクラスを元にして生成される。同じクラスから生成された[オブジェクト](#オブジェクト)は、共通の[プロパティ](#プロパティ)と[メソッド](#メソッド)を持つ（ただし、それぞれの[オブジェクト](#オブジェクト)が持つ[プロパティ](#プロパティ)は独立しており、[オブジェクト](#オブジェクト)ごとに固有）。クラスは実体を持たない設計図であるため、そのままでは[プロパティ](#プロパティ)や[メソッド](#メソッド)を利用することはできない。

### インスタンス

**インスタンス**は、[クラス](#クラス-1)から生成された[オブジェクト](#オブジェクト)の実体。[クラス](#クラス-1)という概念から[オブジェクト](#オブジェクト)という実体を生成することを、**インスタンス化**という。

### プロトタイプ

**プロトタイプ**は、プロトタイプベースの[オブジェクト指向](#オブジェクト指向)において用いられる概念。[オブジェクト](#オブジェクト)を生成する際に、その元となる**プロトタイプオブジェクト**を指定することで、プロトタイプオブジェクトの共通の機能を利用することができる。このようにして、ある[オブジェクト](#オブジェクト)からプロトタイプオブジェクトの機能を参照することを**プロトタイプチェーン**と呼ぶ。

プロトタイプベースの[オブジェクト指向](#オブジェクト指向)には[クラス](#クラス-1)の概念はなく、全てが実体（[オブジェクト](#オブジェクト)、プロトタイプオブジェクト）から成る。[クラス](#クラス-1)ベースにおける設計図である[クラス](#クラス-1)は静的なものであり、一度定義された[クラス](#クラス-1)は[プログラム](./basic_knowledge_of_programming.md#プログラム)中で変更されることはない。一方でプロトタイプは、設計図自体を[プログラム](./basic_knowledge_of_programming.md#プログラム)中で拡張したり変更したりすることができる。


## カプセル化

**カプセル化**は、[オブジェクト](#オブジェクト)が持つ情報を隠蔽することによって、不正な操作ができないようにする仕組み。[オブジェクト](#オブジェクト)が持つ[メソッド](#メソッド)や[プロパティ](#プロパティ)は、[アクセス指定子](#アクセス指定子)によってアクセスできる範囲を制限できる。

カプセル化には、[オブジェクト](#オブジェクト)の外部からのアクセスを制限することで、[オブジェクト](#オブジェクト)内部のデータを保護して、直接書き換えられないようにする目的がある。[オブジェクト](#オブジェクト)内部のデータに直接アクセスできないようにする代わりに、保護されたデータを間接的に操作できるインタフェースとなるような[メソッド](#メソッド)を用意しておくという使い方が多い。

```cpp
// C++

#include <string>

class Dragon
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
        std::string m_name = "dragon";
        int m_level = 1;
};


int main()
{
    Dragon dragon;

    // 公開されたメソッドを通してカプセル化されたプロパティを更新
    int level = dragon.level_up(10);
    printf("Level: %d\n", level);

    return 0;
}
```

```php
<?php

// PHP

class Dragon
{
    // オブジェクト外部からもアクセス可能
    public function level_up( $diff_ )
    {
        $this->m_level += $diff_;
        return $this->m_level;
    }

    // オブジェクト内部からのみアクセス可能（カプセル化されている）
    private $m_index = 1;
    private $m_name = "dragon";
    private $m_level = 1;
}

$dragon = new Dragon();

// 公開されたメソッドを通してカプセル化されたプロパティを更新
$level = $dragon->level_up(10);
echo("Level: " . $level);

?>
```

### アクセス指定子

**アクセス指定子**は、[オブジェクト](#オブジェクト)が持つ[プロパティ](#プロパティ)や[メソッド](#メソッド)の公開範囲を指定するためのキーワード。これにより、[オブジェクト](#オブジェクト)の内部からしかアクセスできないデータと、[オブジェクト](#オブジェクト)の外側からでもアクセスできるデータを切り分けることができる。

```cpp
// C++

class Monster
{
    // 外部にも公開されるメンバ
    public:

        int level_up( int diff_ )
        {
            return this->level_up_inner(diff_);
        }

    // 外部からは見えないが、子クラスからは見えるメンバ
    protected:

        int level_up_inner( int idff_ )
        {
            this->m_level += diff_;
            return this->m_level;
        }

        int m_level = 1;

    // 外部からも子クラスからも見えないメンバ
    private:

        int m_index = 0;
};

class Dragon : public Monster
{
    public:

        void power_up()
        {
            // level_up_innerは子クラスからもアクセス可能
            this->level_up_inner(1);
        }
};

int main()
{
    Dragon dragon;

    // level_upは外部からもアクセス可能
    int level = dragon.level_up(10);

    return 0;
}
```

### public

`public` は、[アクセス指定子](#アクセス指定子)のひとつで、[オブジェクト](#オブジェクト)の[スコープ](./control_flow.md#スコープ)範囲であれば、内部からでも外部からでもアクセスすることができる[メンバ](#メンバ)となる。

### private

`private` は、[アクセス指定子](#アクセス指定子)のひとつで、[オブジェクト](#オブジェクト)の外部からの[メンバ](#メンバ)へのアクセスを制限することができる。[子クラス](子クラス)から[親クラス](#親クラス)の[メンバ](#メンバ)へアクセスすることもできない。

### protected

`protected` は、[アクセス指定子](#アクセス指定子)のひとつで、[オブジェクト](#オブジェクト)の外部からの[メンバ](#メンバ)へのアクセスを制限することができる。[子クラス](#子クラス)から[親クラス](#親クラス)の[メンバ](#メンバ)へアクセスすることはできる。


## 継承

**継承**（**インヘリタンス**）は、継承元（親）となる[クラス](#クラス-1)の持つ[プロパティ](#プロパティ)や[メソッド](#メソッド)を引き継いだ、別の[クラス](#クラス-1)を定義できる仕組み。複数の類似した[クラス](#クラス-1)において、共通部分をまとめた[親クラス](#親クラス)を定義することで、コードの再利用が高まる。

```cpp
// C++

#include <string>

class Monster
{
    public:

        int level_up( int diff_ )
        {
            this->m_level += diff_;
            return this->m_level;
        }

        int get_level()
        {
            return this->m_level;
        }

    private:

        int m_index = 0;
        std::string m_name = "";
        int m_level = 1;
};


// Monsterクラスを継承
class Slime : public Monster
{
    private:

        int m_index = 1;
        std::string m_name = "slime";
};

// Monsterクラスを継承
class Dragon : public Monster
{
    public:

        // メソッドオーバライド
        int level_up( int diff_ )
        {
            this->m_level += diff_;
            this->m_power += 10 * diff_;
            return this->m_level;
        }

    private:

        int m_index = 2;
        std::string m_name = "dragon";
        int m_power = 10;
};

int main()
{
    Slime slime;
    Dragon dragon;

    // メソッドを継承しているのでget_levelが呼び出せる
    int slime_level = slime.get_level();
    int dragon_level = dragon.get_level();

    return 0;
}
```

```php
<?php

// PHP

class Monster
{
    public function level_up( $diff_ )
    {
        $this->m_level += $diff_;
        return $this->m_level;
    }

    private $m_index = 0;
    private $m_name = "";
    private $m_level = 1;
}

// Monsterクラスを継承
class Slime extends Monster
{
    private $m_index = 1;
    private $m_name = "slime";
}

// Monsterクラスを継承
class Dragon extends Monster
{
    // メソッドオーバライド
    public function level_up( $diff_ )
    {
        $this->m_level += $diff_;
        $this->m_power += 10 * $diff_;
        return $this->m_level;
    }

    private $m_index = 2;
    private $m_name = "dragon";
    private $m_power = 10;
}

$slime = new Slime();
$dragon = new Dragon();

// メソッドを継承しているのでget_levelが呼び出せる
$slime_level = $slime->get_level();
$dragon_level = $dragon->get_level();

?>
```

### 親クラス

**親クラス**（**スーパークラス**、**基底クラス**）は、ある[クラス](#クラス-1)の[継承](#継承)元となった[クラス](#クラス-1)。

### 子クラス

**子クラス**（**サブクラス**、**派生クラス**）は、別の[クラス](#クラス-1)を[継承](#継承)した[クラス](#クラス-1)。

### メソッドオーバライド

**メソッドオーバライド**は、[親クラス](#親クラス)で定義された[メソッド](#メソッド)と同じ[識別子](./basic_knowledge_of_programming.md#識別子)を持つ[メソッド](#メソッド)を[子クラス](#子クラス)側でも定義することで、動作が上書きできる機能。


## ポリモーフィズム

**ポリモーフィズム**（**ポリモルフィズム**、**多相性**）は、同じ[識別子](./basic_knowledge_of_programming.md#識別子)の[メソッド](#メソッド)で複数の異なる振る舞いを定義することができる性質。

例えば、異なる[クラス](#クラス-1)が同じ名前の[メソッド](#メソッド)を共通して持つことによって、その[メソッド](#メソッド)を通して、暗黙的に複数の[オブジェクト](#オブジェクト)を切り替えることができる。この性質を利用することで、[オブジェクト](#オブジェクト)を利用している側の[ソースコード](./basic_knowledge_of_programming.md#ソースコード)を変更することなく[クラス](#クラス-1)を切り替えることが可能となり、メンテナンス性の向上に繋がる。[インタフェース](#インタフェース)や[仮想関数](#抽象クラス)の機能を用いることで、[クラス](#クラス-1)の[メソッド](#メソッド)の[シグネチャ](./function.md#シグネチャ)を共通化することができ、ポリモーフィズムの性質を利用しやすくなる。

```cpp
// C++

#include <string>

class Slime
{
    public:

        int level_up( int diff_ )
        {
            this->m_level += diff_;
            return this->m_level;
        }

    private:

        int m_index = 1;
        std::string m_name = "slime";
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
    Slime slime;
    Dragon dragon;

    // ポリモーフィズムの例
    // 違うクラスだが、同じメソッドを利用できる
    int slime_level = slime.level_up(10);

    int dragon_level = dragon.level_up(3);
    dragon_level = dragon.level_up(3, 10);

    return 0;
}
```

```php
<?php

// PHP

class Slime
{
    public function level_up( $diff_ )
    {
        $this->m_level += $diff_;
        return $this->m_level;
    }

    private $m_index = 1;
    private $m_name = "slime";
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

$slime= new Slime();
$dragon = new Dragon();
empower_monster($goblin, 10);
empower_monster($dragon, 3);

?>
```

### ダックタイピング

**ダックタイピング**は、関数やメソッドが、特定のインタフェース（プロパティやメソッド）を持つ全てのオブジェクトを引数として受け取ることができるような性質のこと。ポリモーフィズムの恩恵のひとつで、呼び出される関数（メソッド）は、引数として渡されたデータがどのクラスのオブジェクトであるかを意識する必要がない。

### DI

**DI**(Dependency Injection: **依存性の注入**)は、引数として受け取るオブジェクトがどのクラスのものかに依存せずに、関数やメソッドを実行するという考え方。ダックタイピングにより、共通のインタフェースを持つオブジェクト全てを引数として受け取れるようになり、オブジェクトへの依存性を関数側から縛るのではなく、外部から指定できるようになる。


## 抽象化

```cpp
// C++

#include <string>

// 抽象クラス（仮想関数を持つ）
class Monster
{
    public:

        // 仮想関数
        virtual int level_up( int diff_ ) = 0;

    private:

        int m_index = 0;
        std::string m_name = "";
};

// 具象クラス
class Goblin : public Monster
{
    public:

        // 仮想関数をオーバライド
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
```

```php
<?php

// PHP

// 抽象クラス

abstruct class Monster
{
    abstruct public function level_up( $diff_ );

    private $m_index = 0;
    private $m_name = "";
}

// 具象クラス
class Goblin extends Monster
{
    // オーバライド
    public function level_up( $diff_ )
    {
        $this->m_level += $diff_;
        return $this->m_level;
    }

    private $m_index = 1;
    private $m_name = "goblin";
    private $m_level = 1;
}

?>
```

### 抽象クラス

**抽象クラス**は、具体的な実装を持たず、継承されることを前提としたクラス。具象クラスからは直接インスタンスを生成することはできない。オーバライドされることが前提となるメソッドを**抽象メソッド**（C++では**仮想関数**）といい、関数のシグネチャのみを記述する。

### 具象クラス

**具象クラス**は、抽象クラスを継承して抽象メソッドをオーバライドし、具体的な実装を施したクラス。

### インタフェース

**インタフェース**は、クラスが定義するべきメソッドのシグネチャを示したもので、具体的な実装は持たない。インタフェースを実装するクラスは、インタフェースに定義されているメソッドを全て実装している必要がある（ `default` キーワードにより、インタフェースにデフォルトの実装を定義することができるプログラミング言語もある）。継承では1つの親クラスからしか性質を引き継げないが、1つのクラスが複数のインタフェースを実装することは可能。

```java
// Java

// インタフェース
interface RaceBase
{
    public int level_up( int diff_ );
}

interface RaceDragon
{
    public void powerup();

    default public void bark()
    {
        System.out.println("Growl!!");
    }
}
 
// インタフェースを実装したクラス
class Goblin implements RaceBase
{
    public void level_up( int diff_ )
    {
        System.out.println("Level up!");
        this.m_level += diff_;
        return this.m_level;
    }

    private int m_level = 1;
}

// 複数のインタフェースを実装したクラス
class Dragon implements RaceBase, RaceDragon
{
    public void level_up( int diff_ )
    {
        System.out.println("Level up!");
        this.m_level += diff_;
        return this.m_level;
    }

    public void powerup()
    {
        System.out.println("Power up!");
        this.m_power += 100;
    }

    private int m_level = 1;
    private int m_power = 100;
}
```

```php
<?php

// PHP

// インタフェース
interface RaceBase
{
    public function level_up( $diff_ );
}

interface RaceDragon
{
    public function powerup();
}

// インタフェースを実装したクラス
class Goblin implements RaceBase
{
    public function level_up( $diff_ )
    {
        echo("Level up!");
        $this->m_level += $diff_;
        return $this->m_level;
    }

    private $m_level = 1;
}

// 複数のインタフェースを実装したクラス
class Dragon implements RaceBase, RaceDragon
{
    public function level_up( $diff_ )
    {
        echo("Level up!");
        $this->m_level += $diff_;
        return $this->m_level;
    }

    public function powerup()
    {
        echo("Power up!");
        $this->m_power += 100;
    }

    private $m_level = 1;
    private $m_power = 100;
}

?>
```


## this

`this` は、クラスのメソッド内で用いることができ、クラスのインスタンス自身を指すキーワード（多くのプログラミング言語では `this` が用いられているが、 `self` など別のキーワードを用いる場合もある）。インスタンス内のプロパティやメソッドにアクセスする際に利用する。

```cpp
// C++

#include <string>

class Monster
{

    public:

        int get_level()
        {
            // thisでインスタンスのプロパティへアクセス
            return this->m_level;
        }

    private:

        int m_level = 1;
};
```

```php
<?php

// PHP

class Monster
{
    public function get_level()
    {
        // thisでインスタンスのプロパティへアクセス
        return $this->m_level;
    }

    private $m_level = 1;
}

?>
```


## 特別なメソッド

### コンストラクタ

**コンストラクタ**は、クラスの初期化を行うための特別なメソッドで、インスタンスの生成時に自動的に実行される。プロパティの初期化や、オブジェクトごとの特別な初期化処理が必要な場合などに用いる。

```cpp
// C++

#include <string>

class Monster
{
    public:

        // コンストラクタ
        Monster( char* name_ )
        {
            this->m_name = name_;
            this->m_data = new int[1024];

            printf("Fight with %s!\n", name_);
        }
 
    private:

        int m_index = 0;
        std::string m_name = "";
        int m_level = 1;
        int* m_data;
};

int main()
{
    // コンストラクタの実行
    Monster goblin("goblin");

    return 0;
}
```

```php
<?php

// PHP

class Monster
{
    // コンストラクタ
    function __construct( $name_ )
    {
        $m_name = $name_;

        echo("Fight with " . name_ . "!");
    }

    private $m_index = 0;
    private $m_name = "";
    private $m_level = 1;
}

// コンストラクタの実行
$goblin = new Monster("goblin");

?>
```

### デストラクタ

**デストラクタ**は、クラスの終了処理を行うための特別なメソッドで、インスタンスが破棄されるときに自動的に実行される。ガベージコレクションを用いるプログラミング言語では、デストラクタが実行されるタイミングが予測できないため、複雑な処理を行おうとするとバグの発生原因となってしまう場合がある。一方で、C++のようにプログラマが変数の生存期間を管理する言語においては、インスタンスが管理するヒープ領域の解放を行うなど重要な役割を持っている。

```cpp
// C++

#include <string>

class Monster
{
    public:

        // デストラクタ
        ~Monster()
        {
            delete this->m_data;

            printf("Beat %s.\n", this->m_name.c_str());
        }


    private:

        int m_index = 0;
        std::string m_name = "";
        int m_level = 1;
        int* m_data;
};

int main()
{
    Monster goblin();

    // デストラクタが実行される
    return 0;
}
```


## 静的メンバ

**静的メンバ**は、メモリ上に静的に配置され、インスタンスとの直接的な関連を持たない。多くのプログラミング言語では、 `static` キーワードをつけることでメンバを静的にすることができる。

### staticプロパティ

**staticプロパティ**は、オブジェクトに依存せずに独立した変数のように扱えるプロパティ。クラスに属するグローバル変数のようなものとして扱われる。

```cpp
// C++

#include <string>

class Monster
{
    public:

        int get_index()
        {
            return this->m_index;
        }

        // 静的メソッド
        static Monster* generate()
        {
            Monster* monster = new Monster();
            monster->m_index = ++Monster::g_last_index;
            return monster;
        }

    private:

        int m_index = 0;
        std::string m_name = "";
        int m_level = 1;
};

int main()
{
    // 静的メソッド呼び出し
    Monster* monster = Monster::generate();
    printf("Monster index: %d\n", monster->get_index());

    return 0;
}
```

```php
<?php

// PHP

class Monster
{
    public function get_index()
    {
        return $this->m_index;
    }

    // 静的メソッド
    public static function generate()
    {
        $monster = new Monster();
        $monster->m_index = ++self::$g_last_index;
        return $monster;
    }

    private $m_index = 0;
    private $m_name = "";
    private $m_level = 1;
}

// 静的メソッド呼び出し
$monster = Monster::generate();
echo("Monster index: " . $monster->get_index());

?>
```

### staticメソッド

**staticメソッド**（**静的メソッド**）は、オブジェクトのメンバとして定義されているものの、インスタンスを必要としないメソッド。全てのインスタンスで共通する処理や、クラスに関連するような関数を使用したい場合に用いる。静的メソッドはインスタンスに依存しないため、 `this` によるメンバへのアクセスができない。

```cpp
// C++

#include <string>

class Monster
{
    public:

        // 静的プロパティ
        static int g_last_index;

    private:

        int m_index = 0;
        std::string m_name = "";
        int m_level = 1;
};

// 静的プロパティの初期化
int Monster::g_last_index = 0;

int main()
{
    // 静的プロパティへのアクセス
    Monster* monster = new Monster();
    printf("Last index: %d\n", Monster::g_last_index);

    return 0;
}
```

```php
<?php

// PHP

class Monster
{
    // 静的プロパティ
    public static $g_last_index = 0;

    private $m_index = 0;
    private $m_name = "";
    private $m_level = 1;
}

// 静的プロパティへのアクセス
$monster = new Monster();
echo("Last index: " . Monster::g_last_index);

?>
```
