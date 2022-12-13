# クラス


## 目次

1. [オブジェクト指向](#オブジェクト指向)
1. [クラスとインスタンス](#クラスとインスタンス)
1. [オブジェクト指向の特性](#オブジェクト指向の特性)
	1. [カプセル化](#カプセル化)
	1. [ポリモーフィズム](#ポリモーフィズム)
	1. [継承](#継承)
1. [this](#this)
1. [コンストラクタとデストラクタ](#コンストラクタとデストラクタ)
1. [静的メソッドと静的プロパティ](#静的メソッドと静的プロパティ)
1. [アクセス指定子](#アクセス指定子)
1. [抽象クラスと具象クラス](#抽象クラスと具象クラス)
1. [インタフェース](#インタフェース)


## オブジェクト指向

**オブジェクト指向**は最もよく用いられる**プログラミングパラダイム**（プログラミングの考え方）のひとつで、相互に作用するオブジェクトを組み合わせることでプログラムを設計する手法。クラスベースのオブジェクト指向に則ったプログラミング言語が多い（プロトタイプベースのオブジェクト指向もある）。

オブジェクト指向における**オブジェクト**とは、あらゆるモノをコンピュータが扱うデータとして抽象化したものを指す。オブジェクトはプロパティやそのオブジェクトに対するメソッドを持っており、他のオブジェクトと相互に作用しあって成り立つ。**プロパティ**（**属性**、**メンバ変数**、**フィールド変数**）はオブジェクトが持つデータや情報のことであり、**メソッド**（**操作**、**メンバ関数**）とはそのオブジェクトの機能や振る舞いのことである。


## クラスとインスタンス

**クラス**はオブジェクト指向において、オブジェクトを生成するための設計図のようなもので、オブジェクトの実体はクラスを元にして生成される。同じクラスから生成されたオブジェクトは共通のプロパティとメソッドを持っているが、それぞれのオブジェクトが持つ属性値は独立している。

クラスという概念からオブジェクトという実体を生成することを**インスタンス化**と呼び、オブジェクトのことを**インスタンス**ともいう。クラスは単なる概念であるため、そのままではプロパティやメソッドは使用することができず、インスタンスを生成する必要がある。

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


## オブジェクト指向の特性

### カプセル化

**カプセル化**とはオブジェクトが持つ情報を隠蔽することによって、不正な操作ができないようにするための仕組みのこと。オブジェクトが持つメソッドやプロパティはアクセス指定子によってアクセスできる範囲を制限できる。

オブジェクトの外部からのアクセスを制限することで、オブジェクト内部のデータを保護して直接書き換えられないようにする目的。代わりに、オブジェクト内部の保護されたデータを間接的に変更できるようにインタフェースとなるメソッドを用意しておくという使い方がよくされる。

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

**ポリモーフィズム**（多様性）とは、同じ識別子の関数（メソッド）で複数の異なる振る舞いを定義できる性質のこと。

例えば異なるクラスが同じ名前のメソッドを共通して持つことで、そのメソッドを通して暗黙的に複数のオブジェクトを切り替えることができるようにすることができる。この性質を利用することで、オブジェクトを利用している側のソースコードを変更することなくクラスを切り替えることが可能となり、メンテナンス性の向上につながる。

特定のプロパティやメソッドを持つ全てのオブジェクトを引数として受け取って関数やメソッドを実行できるという性質を**ダックタイピング**といい、これもポリモーフィズムの恩恵の1つである。関数（メソッド）側は渡されたオブジェクトがどのクラスのオブジェクトであるかを意識する必要がなくなり、オブジェクトに対する依存性を減らすことができる。このようにオブジェクトに対する依存性を引数として外部から指定するような考え方を**DI**（Dependency Injection、依存性の注入）という。

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

### 継承

**継承**（**インヘリタンス**）とは継承元（親）となるクラスの持つプロパティやメソッドを引き継いだ別のクラスを定義できるような仕組みのこと。これにより、複数の類似したクラスにおいて共通部分をまとめた**親クラス**（**スーパークラス**、**基底クラス**）を定義することができるようになる。親クラスの性質を受け継いだクラスのことを**子クラス**（**サブクラス**、**派生クラス**）という。継承の仕組みによりコードの再利用性が高まり、プログラムが変更に強くなるという利点がある。

親クラスが持つメソッドを別の処理に変更したい場合、同じ識別子でメソッドを定義することで**オーバライド**（上書き）できる。

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
class Goblin : public Monster
{
    private:

        int m_index = 1;
        std::string m_name = "goblin";
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
    Goblin goblin;
    Dragon dragon;

    // メソッドを継承しているのでget_levelが呼び出せる
    int goblin_level = goblin.get_level();
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
class Goblin extends Monster
{
    private $m_index = 1;
    private $m_name = "goblin";
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

$goblin = new Goblin();
$dragon = new Dragon();

// メソッドを継承しているのでget_levelが呼び出せる
$goblin_level = $goblin->get_level();
$dragon_level = $dragon->get_level();

?>
```


## this

`this` はクラスのメソッド内で用いることができるキーワードで、クラスのインスタンス自身を指す。インスタンス内のプロパティやメソッドにアクセスするときに使用する。

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

        int m_index = 0;
        std::string m_name = "";
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


## コンストラクタとデストラクタ

**コンストラクタ**はクラスの初期化を行うための特別なメソッドで、インスタンスの生成時に自動的に実行される。プロパティの初期化や、オブジェクトごとの特別な初期化処理などに用いる。

**デストラクタ**はクラスの終了処理を行うための特別なメソッドで、インスタンスが破棄されるときに自動的に実行される。ガベージコレクションを用いるプログラミング言語ではデストラクタが実行されるタイミングが予測できないため、複雑な処理を行おうとするとバグの発生原因となってしまう場合がある。一方で、C++のようにプログラマが変数の生存期間を管理する言語においては、インスタンスの後処理を行う上で重要な役割を果たす。

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
    // コンストラクタが実行される
    Monster goblin("goblin");

    // デストラクタが実行される
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

// コンストラクタが実行される
$goblin = new Monster("goblin");

?>
```

## 静的メソッドと静的プロパティ

**静的メソッド**（**staticメソッド**）はオブジェクトを必要としないメソッド。インスタンスを生成せずに、普通の関数のように直接呼び出すことができる。インスタンスに影響を受けないため、インスタンスの状態による副作用を受けない。一方で、静的メソッドはインスタンスに依存しないため、 `this` によるメンバへのアクセスができない。

**静的プロパティ**（**staticプロパティ**）も静的メソッドと同様、オブジェクトに依存せずに独立した変数のように扱えるプロパティ。クラスに属するグローバル変数のようなものといえる。

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
    // 静的メソッド呼び出し、静的プロパティへのアクセス
    Monster* monster = Monster::generate();
    printf("Monster index: %d\n", monster->get_index());
    printf("Last index: %d\n", Monster::g_last_index);

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

    // 静的プロパティ
    public static $g_last_index = 0;

    private $m_index = 0;
    private $m_name = "";
    private $m_level = 1;
}

$monster = Monster::generate();
echo("Monster index: " . $monster->get_index());
echo("Last index: " . Monster::g_last_index);

?>
```


## アクセス指定子

**アクセス指定子**はオブジェクトが持つプロパティやメソッドの公開範囲を設定するための機能。

`public` キーワードを指定した場合、オブジェクトのスコープ範囲であれば内部からでも外部からでもプロパティやメソッドにアクセスすることができる。

`private` キーワードを指定した場合、オブジェクトの外部からのプロパティやメソッドへのアクセスを制限できる。子クラスからもアクセスできなくなる。

`protected` キーワードを指定した場合、オブジェクトの外部からのプロパティやメソッドへのアクセスを制限できる。子クラスからはアクセスできる。

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

class Goblin : public Monster
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
    Goblin goblin;

    // level_upは外部からもアクセス可能
    int level = goblin.level_up(10);

    return 0;
}
```


## 抽象クラスと具象クラス

**抽象クラス**は具体的な実装を持たない、継承されることを前提としたクラス。具象クラスからは直接インスタンスを生成することはできない。オーバライドされることが前提となるメソッドを**仮想関数**といい、**シグニチャ**のみが記述される。

**具象クラス**は抽象クラスを継承してメソッドをオーバライドし、具体的な実装を行ったクラス。

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


## インタフェース

**インタフェース**はクラスのメソッドのシグニチャを定義したもので、具体的な実装は持たない。インタフェースを実装するクラスは、インタフェースに定義されているメソッドを全てオーバライドして具体的な実装を記述する必要がある（ `default` キーワードによりデフォルトの実装を定義することができるプログラミング言語もある）。継承では1つの親クラスからしか性質を引き継げないが、インタフェースを複数実装することは可能。

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
