# 『Prototype』ノート

（最終更新： 2023-08-06）


## 目次

1. [Prototypeパターン](#prototypeパターン)
	1. [Prototype](#prototype)
	1. [ConcretePrototype](#concreteprototype)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Prototypeパターン

(**Prototypeパターン**は、[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)から[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成するのではなく、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)から別の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を複製して生成する[デザインパターン](./design_pattern.md#デザインパターン)。このパターンを使用することで、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)が余分な依存関係を含まずに[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を複製できたり（その[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)の持つ[フィールド](../../../../programming/_/chapters/object_oriented.md#プロパティ)を直接記述しなくてもよいため）、同じ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を持つ異なる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に対しても同じような複製処理を記述できたりするようになる。

Prototypeパターンは、[Prototype](#prototype)、[ConcretePrototype](#concreteprototype)から構成される。

### Prototype

**Prototype**（原型）は、[Prototypeパターン](#prototypeパターン)において、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を複製して新しい[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を作るための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定める[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。この[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)は多くの場合、既存の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)の[フィールド](../../../../programming/_/chapters/object_oriented.md#プロパティ)をすべてコピーした新しい[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を返す。

### ConcretePrototype

**ConcretePrototype**（具体的な原型）は、[Prototypeパターン](#prototypeパターン)において、[Prototype](#prototype)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を実装して、実際に[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を複製する処理を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。


## サンプルプログラム

### Java

図形を描画する[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)において、ある図形を複製して別の図形を作る、という機能を実装したい場合を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // 図形を作成
        Shape shape = new Circle();
        shape.x = 10;
        shape.y = 20;
        shape.color = "#ccc";

        // 図形の複製
        Shape shape_clone = shape.clone();
        System.out.println(shape_clone.x);
        System.out.println(shape_clone.y);
        System.out.println(shape_clone.color);
    }
}

//------------------------------------------------------------------------------
// Prototype
//------------------------------------------------------------------------------
abstract class Shape
{
    public int x;
    public int y;
    public String color;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Shape()
    {
    }

    public Shape( Shape target )
    {
        if( target != null )
        {
            this.x = target.x;
            this.y = target.y;
            this.color = target.color;
        }
    }

    public abstract Shape clone();
}

//------------------------------------------------------------------------------
// ConcretePrototype
//------------------------------------------------------------------------------
class Circle extends Shape
{
    public int radius;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Circle()
    {
    }

    public Circle( Circle target )
    {
        super(target);
        if( target != null )
        {
            this.radius = target.radius;
        }
    }

    public Circle clone()
    {
        return new Circle(this);
    }
}

//------------------------------------------------------------------------------
// ConcretePrototype
//------------------------------------------------------------------------------
class Rectangle extends Shape
{
    public int width;
    public int height;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Rectangle()
    {
    }

    public Rectangle( Rectangle target )
    {
        super(target);
        if( target != null )
        {
            this.width = target.width;
            this.height = target.height;
        }
    }

    public Rectangle clone()
    {
        return new Rectangle(this);
    }
}
```
