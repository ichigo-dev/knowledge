# 『オブジェクト指向』ノート

（最終更新： 2023-10-19）


## 目次

1. [オブジェクト指向](#オブジェクト指向)
	1. [オブジェクト](#オブジェクト)
	1. [メンバ](#メンバ)
	1. [プロパティ](#プロパティ)
	1. [メソッド](#メソッド)
	1. [プロパティチェーン](#プロパティチェーン)
	1. [メソッドチェーン](#メソッドチェーン)
	1. [クラス](#クラス)
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
	1. [完全コンストラクタ](#完全コンストラクタ)
	1. [デストラクタ](#デストラクタ)
1. [静的メンバ](#静的メンバ)
	1. [staticプロパティ](#staticプロパティ)
	1. [staticメソッド](#staticメソッド)
1. [生焼けオブジェクト](#生焼けオブジェクト)
1. [デメテルの法則](#デメテルの法則)


## オブジェクト指向

**オブジェクト指向**は、[プログラミングパラダイム](./programming.md#プログラミングパラダイム)のひとつで、非常に多くの[プログラミング言語](./programming.md#プログラミング言語)に取り入れられている概念である。相互に作用する[オブジェクト](#オブジェクト)を組み合わせることで[プログラム](./programming.md#プログラム)を設計する手法で、[クラス](#クラス)ベースのオブジェクト指向に則った[プログラミング言語](./programming.md#プログラミング言語)が多い（[JavaScript](./programming_language.md#javascript)のように、[プロトタイプ](#プロトタイプ)ベースのオブジェクト指向もある）。

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

**オブジェクト**は、あらゆるものを[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)が扱うデータとして抽象化したもの。オブジェクトは[プロパティ](#プロパティ)や[メソッド](#メソッド)を持っており、他のオブジェクトとの相互に作用しあって成り立っている。

### メンバ

**メンバ**は、[オブジェクト](#オブジェクト)が持つ[プロパティ](#プロパティ)や[メソッド](#メソッド)。

### プロパティ

**プロパティ**（**属性**、**メンバ変数**、**フィールド変数**）は、[オブジェクト](#オブジェクト)が持つデータや情報。

### メソッド

**メソッド**（**操作**、**メンバ関数**）は、[オブジェクト](#オブジェクト)の機能や振る舞い。

### プロパティチェーン

**プロパティチェーン**は、入れ子になっている[オブジェクト](#オブジェクト)の[プロパティ](#プロパティ)に次々にアクセスするための記法。一般的には、[プロパティ](#プロパティ)として扱われる[オブジェクト](#オブジェクト)とその[プロパティ](#プロパティ)を `.` 記号でつなぐことが多い。

### メソッドチェーン

**メソッドチェーン**は、[メソッド](#メソッド)の[戻り値](./function.md#戻り値)として返された[オブジェクト](#オブジェクト)に対して、次々と[メソッド](#メソッド)の呼び出しを行うための記法。一般的には、[メソッド](#メソッド)の後ろに `.` 記号をつけて更にその[戻り値](./function.md#戻り値)に対する[メソッド](#メソッド)の呼び出しを記述することが多い。

### クラス

**クラス**は、[オブジェクト](#オブジェクト)を生成するための設計図のようなもの。[オブジェクト](#オブジェクト)の実体はクラスを元にして生成される。同じクラスから生成された[オブジェクト](#オブジェクト)は、共通の[プロパティ](#プロパティ)と[メソッド](#メソッド)を持つ（ただし、それぞれの[オブジェクト](#オブジェクト)が持つ[プロパティ](#プロパティ)は独立しており、[オブジェクト](#オブジェクト)ごとに固有）。クラスは実体を持たない設計図であるため、そのままでは[プロパティ](#プロパティ)や[メソッド](#メソッド)を利用することはできない。

### インスタンス

**インスタンス**は、[クラス](#クラス)から生成された[オブジェクト](#オブジェクト)の実体。[クラス](#クラス)という概念から[オブジェクト](#オブジェクト)という実体を生成することを、**インスタンス化**という。

### プロトタイプ

**プロトタイプ**は、プロトタイプベースの[オブジェクト指向](#オブジェクト指向)において用いられる概念。[オブジェクト](#オブジェクト)を生成する際に、その元となる**プロトタイプオブジェクト**を指定することで、プロトタイプオブジェクトの共通の機能を利用することができる。このようにして、ある[オブジェクト](#オブジェクト)からプロトタイプオブジェクトの機能を参照することを**プロトタイプチェーン**と呼ぶ。

プロトタイプベースの[オブジェクト指向](#オブジェクト指向)には[クラス](#クラス)の概念はなく、全てが実体（[オブジェクト](#オブジェクト)、プロトタイプオブジェクト）から成る。[クラス](#クラス)ベースにおける設計図である[クラス](#クラス)は静的なものであり、一度定義された[クラス](#クラス)は[プログラム](./programming.md#プログラム)中で変更されることはない。一方でプロトタイプは、設計図自体を[プログラム](./programming.md#プログラム)中で拡張したり変更したりすることができる。


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

`private` は、[アクセス指定子](#アクセス指定子)のひとつで、[オブジェクト](#オブジェクト)の外部からの[メンバ](#メンバ)へのアクセスを制限することができる。[子クラス](#子クラス)から[親クラス](#親クラス)の[メンバ](#メンバ)へアクセスすることもできない。

### protected

`protected` は、[アクセス指定子](#アクセス指定子)のひとつで、[オブジェクト](#オブジェクト)の外部からの[メンバ](#メンバ)へのアクセスを制限することができる。[子クラス](#子クラス)から[親クラス](#親クラス)の[メンバ](#メンバ)へアクセスすることはできる。


## 継承

**継承**（**インヘリタンス**）は、継承元（親）となる[クラス](#クラス)の持つ[プロパティ](#プロパティ)や[メソッド](#メソッド)を引き継いだ、別の[クラス](#クラス)を定義できる仕組み。複数の類似した[クラス](#クラス)において、共通部分をまとめた[親クラス](#親クラス)を定義することで、コードの再利用が高まる。

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

**親クラス**（**スーパークラス**、**基底クラス**）は、ある[クラス](#クラス)の[継承](#継承)元となった[クラス](#クラス)。

### 子クラス

**子クラス**（**サブクラス**、**派生クラス**）は、別の[クラス](#クラス)を[継承](#継承)した[クラス](#クラス)。

### メソッドオーバライド

**メソッドオーバライド**は、[親クラス](#親クラス)で定義された[メソッド](#メソッド)と同じ[識別子](./programming.md#識別子)を持つ[メソッド](#メソッド)を[子クラス](#子クラス)側でも定義することで、動作が上書きできる機能。


## ポリモーフィズム

**ポリモーフィズム**（**ポリモルフィズム**、**多相性**）は、同じ[識別子](./programming.md#識別子)の[メソッド](#メソッド)で複数の異なる振る舞いを定義することができる性質。

例えば、異なる[クラス](#クラス)が同じ名前の[メソッド](#メソッド)を共通して持つことによって、その[メソッド](#メソッド)を通して、暗黙的に複数の[オブジェクト](#オブジェクト)を切り替えることができる。この性質を利用することで、[オブジェクト](#オブジェクト)を利用している側の[ソースコード](./programming.md#ソースコード)を変更することなく[クラス](#クラス)を切り替えることが可能となり、メンテナンス性の向上に繋がる。[インタフェース](#インタフェース)や[仮想関数](#抽象クラス)の機能を用いることで、[クラス](#クラス)の[メソッド](#メソッド)の[シグネチャ](./function.md#シグネチャ)を共通化することができ、ポリモーフィズムの性質を利用しやすくなる。

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

**ダックタイピング**は、[関数](./function.md#関数)や[メソッド](#メソッド)が、特定のインタフェース（[プロパティ](#プロパティ)や[メソッド](#メソッド)）を持つ全ての[オブジェクト](#オブジェクト)を[引数](./function.md#引数)として受け取ることができる性質。[ポリモーフィズム](#ポリモーフィズム)の恩恵のひとつで、呼び出される[関数](./function.md#関数)（[メソッド](#メソッド)）は、[引数](./function.md#引数)として渡されたデータがどの[クラス](#クラス)の[オブジェクト](#オブジェクト)であるかを意識する必要がない。

### DI

**DI**（Dependency Injection: **依存性の注入**）は、[引数](./function.md#引数)として受け取る[オブジェクト](#オブジェクト)がどの[クラス](#クラス)のものかに依存せずに、[関数](./function.md#関数)や[メソッド](#メソッド)を実行するという考え方。[ダックタイピング](#ダックタイピング)により、共通のインタフェースを持つ[オブジェクト](#オブジェクト)全てを[引数](./function.md#引数)として受け取れるようになり、[オブジェクト](#オブジェクト)への依存性を[関数](./function.md#関数)側から縛るのではなく、外部から指定できるようになる。


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

**抽象クラス**は、具体的な実装を持たず、[継承](#継承)されることを前提とした[クラス](#クラス)。[具象クラス](#具象クラス)からは直接[インスタンス](#インスタンス)を生成することはできない。[オーバライド](#メソッドオーバライド)されることが前提となる[メソッド](#メソッド)を**抽象メソッド**（C++では**仮想関数**）といい、[関数](./function.md#関数)の[シグネチャ](./function.md#シグネチャ)のみを記述する。

### 具象クラス

**具象クラス**は、[抽象クラス](#抽象クラス)を[継承](#継承)して[抽象メソッド](#抽象クラス)を[オーバライド](#メソッドオーバライド)し、具体的な実装を施した[クラス](#クラス)。

### インタフェース

**インタフェース**は、[クラス](#クラス)が定義するべき[メソッド](#メソッド)の[シグネチャ](./function.md#シグネチャ)を示したもので、具体的な実装は持たない。インタフェースを実装する[クラス](#クラス)は、インタフェースに定義されている[メソッド](#メソッド)を全て実装している必要がある（ `default` キーワードにより、インタフェースにデフォルトの実装を定義することができる[プログラミング言語](./programming.md#プログラミング言語)もある）。[継承](#継承)では1つの[親クラス](#親クラス)からしか性質を引き継げないが、1つの[クラス](#クラス)が複数のインタフェースを実装することは可能。

```java
// Java

// インタフェース
interface RaceMonster
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
class Slime implements RaceMonster
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
class Dragon implements RaceMonster, RaceDragon
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
interface RaceMonster
{
    public function level_up( $diff_ );
}

interface RaceDragon
{
    public function powerup();
}

// インタフェースを実装したクラス
class Slime implements RaceMonster
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
class Dragon implements RaceMonster, RaceDragon
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

`this` は、[クラス](#クラス)の[メソッド](#メソッド)内で用いることができ、[クラス](#クラス)の[インスタンス](#インスタンス)自身を指すキーワード（多くの[プログラミング言語](./programming.md#プログラミング言語)では `this` が用いられているが、 `self` など別のキーワードを用いる場合もある）。[インスタンス](#インスタンス)内の[プロパティ](#プロパティ)や[メソッド](#メソッド)を参照する際に利用する。

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

**コンストラクタ**は、[オブジェクト](#オブジェクト)の初期化を行うための特別な[メソッド](#メソッド)で、[インスタンス](#インスタンス)の生成時に自動的に実行される。[プロパティ](#プロパティ)の初期化や、[オブジェクト](#オブジェクト)ごとの特別な初期化処理が必要な場合などに用いる。

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
    Monster goblin("dragon");

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
$goblin = new Monster("dragon");

?>
```

### 完全コンストラクタ

**完全コンストラクタ**は、[オブジェクト](#オブジェクト)を[インスタンス化](#インスタンス)した時点で、全ての[プロパティ](#プロパティ)が正しく初期化されるような[コンストラクタ](#コンストラクタ)。[生焼けオブジェクト](#生焼けオブジェクト)を防止したり、[オブジェクト](#オブジェクト)に不正値が混入しないようにしたりするだけではなく、[コード](./programming.md#ソースコード)の可読性や保守性も向上させることができる。

完全コンストラクタを実装するには、[プロパティ](#プロパティ)の初期化に必要な値を[コンストラクタ](#コンストラクタ)の[引数](./function.md#引数)として取り、それらの値に対してバリデーションを行った上で[プロパティ](#プロパティ)に[代入](./variable.md#代入)する。

### デストラクタ

**デストラクタ**は、[オブジェクト](#オブジェクト)の終了処理を行うための特別な[メソッド](#メソッド)で、[インスタンス](#インスタンス)が破棄されるときに自動的に実行される。[ガベージコレクション](./programming.md#ガベージコレクション)を用いる[プログラミング言語](./programming.md#プログラミング言語)では、デストラクタが実行されるタイミングが予測できないため、複雑な処理を行おうとすると[バグ](./programming.md#バグ)の発生原因となってしまう場合がある。一方で、[C言語](./programming_language.md#c言語)のように[プログラマ](./programming.md#プログラマ)が[変数](./variable.md#変数)の生存期間を管理する言語においては、[インスタンス](#インスタンス)が管理する[ヒープ領域](../../../computer/hardware/_/chapters/memory.md#ヒープ領域)の解放を行うなど重要な役割を持っている。

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
    Monster dragon();

    // デストラクタが実行される
    return 0;
}
```


## 静的メンバ

**静的メンバ**は、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)上に静的に配置され、[インスタンス](#インスタンス)との直接的な関連を持たない[メンバ](#メンバ)。多くの[プログラミング言語](./programming.md#プログラミングン言語)では、 `static` キーワードをつけることで[メンバ](#メンバ)を静的メンバにすることができる。

### staticプロパティ

**staticプロパティ**は、[オブジェクト](#オブジェクト)に依存せずに独立した[変数](./variable.md#変数)のように扱える[プロパティ](#プロパティ)。[クラス](#クラス)に属する[グローバル変数](./variable.md#グローバル変数)のようなものとして扱われる。

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

**staticメソッド**（**静的メソッド**）は、[オブジェクト](#オブジェクト)の[メンバ](#メンバ)として定義されているものの、[インスタンス](#インスタンス)を必要としない[メソッド](#メソッド)。全ての[インスタンス](#インスタンス)で共通する処理や、[クラス](#クラス)に関連するような[関数](./function.md#関数)を使用したい場合に用いる。静的メソッドは[インスタンス](#インスタンス)に依存しないため、 `this` による[メンバ](#メンバ)へのアクセスができない。

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


## 生焼けオブジェクト

**生焼けオブジェクト**は、[インスタンス化](#インスタンス)されたものの、[コンストラクタ](#コンストラクタ)によって必要な初期化が行われていない[オブジェクト](#オブジェクト)。通常は[オブジェクト](#オブジェクト)を[インスタンス化](#インスタンス)しただけでは、[プロパティ](#プロパティ)には値が格納されていない状態であり、この状態で[オブジェクト](#オブジェクト)を使用すると、意図しない結果や[エラー](./programming.md#エラー)が発生する可能性がある。


## デメテルの法則

**デメテルの法則**は、[オブジェクト指向](#オブジェクト指向)における設計ガイドラインのひとつで、[オブジェクト](#オブジェクト)が[チェーン](#プロパティチェーン)するべきなのは、[オブジェクト](#オブジェクト)の直接の[プロパティ](#プロパティ)だけである、とする考え方。これにより、[クラス](#クラス)間の結合を弱くし、変更に対して強い構造にすることができる。
