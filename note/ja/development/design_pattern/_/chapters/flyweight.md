# 『Flyweight』ノート

（最終更新： 2023-08-16）


## 目次

1. [Flyweightパターン](#flyweightパターン)
	1. [Flyweight](#flyweight)
	1. [FlyweightFactory](#flyweightfactory)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Flyweightパターン

**Flyweightパターン**は、生成済みの[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)をできるだけ再利用（共有）し、無駄な[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の生成（[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)使用）を防止する仕組みを提供する[デザインパターン](./design_pattern.md#デザインパターン)。基本的にはイミュータブルな[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)に対してのみ適用できるパターン。

Flyweightパターンは、[Flyweight](#flyweight)、[FlyweightFactory](#flyweightfactory)から構成される。

### Flyweight

**Flyweight**（フライ級）は、[Flyweightパターン](#flyweightパターン)において、軽量化対象となる（通常の利用方法だと無駄な[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が生成される可能性のある）[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。元の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の状態のうち、複数の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)で共有できる内因的な状態のみを含むような設計となる。

### FlyweightFactory

**FlyweightFactory**（フライ級の工場）は、[Flyweightパターン](#flyweightパターン)において、[Flyweight](#flyweight)を生成・管理する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Flyweight](#flyweight)の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が必要な場合は、このFlyweightFactory経由で取得する。FlyweightFactoryは生成した[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)をプールし、共有可能な[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が要求された場合はプールから[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を返却する。[Flyweight](#flyweight)を管理する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)が複数存在すると、再利用できる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を別個に管理してしまうため、[Singletonパターン](./singleton.md#singletonパターン)を適用する。


## サンプルプログラム

### Java

文字からスタンプを生成し、それを何度も使いまわすような[プログラム](../../../../programming/_/chapters/programming.md#プログラム)を考える。

```java
import java.util.ArrayList;
import java.util.List;
import java.util.HashMap;
import java.util.Map.Entry;

//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // スタンプを生成して表示
        StampFactory factory = StampFactory.getInstance();
        List<Stamp> stamps = new ArrayList<>();
        stamps.add(factory.getStamp('a'));
        stamps.add(factory.getStamp('b'));
        stamps.add(factory.getStamp('c'));
        stamps.add(factory.getStamp('d'));
        stamps.add(factory.getStamp('e'));
        stamps.add(factory.getStamp('f'));
        stamps.add(factory.getStamp('a'));
        stamps.add(factory.getStamp('a'));
        stamps.add(factory.getStamp('b'));
        stamps.add(factory.getStamp('d'));
        for( int i = 0; i < stamps.size(); i++ )
        {
            Stamp stamp = stamps.get(i);
            System.out.print(stamp.getUseCount() + ": ");
            stamp.print();
        }
        System.out.println("--------------------");
        factory.printAll();
    }
}

//------------------------------------------------------------------------------
// Flyweight
//------------------------------------------------------------------------------
class Stamp
{
    private char charname;
    private int useCount;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Stamp( char charname )
    {
        this.charname = charname;
        this.useCount = 0;
    }

    //--------------------------------------------------------------------------
    // 文字の表示
    //--------------------------------------------------------------------------
    public void print()
    {
        System.out.println(this.charname);
    }

    //--------------------------------------------------------------------------
    // 使用回数の取得
    //--------------------------------------------------------------------------
    public int getUseCount()
    {
        return this.useCount;
    }

    //--------------------------------------------------------------------------
    // 使用回数のインクリメント
    //--------------------------------------------------------------------------
    public void incrementUseCount()
    {
        this.useCount++;
    }
}

//------------------------------------------------------------------------------
// FlyweightFactory
//------------------------------------------------------------------------------
class StampFactory
{
    private static StampFactory instance;
    private HashMap<String, Stamp> pool;

    //--------------------------------------------------------------------------
    // コンストラクタをprivateに隠蔽
    //--------------------------------------------------------------------------
    private StampFactory()
    {
        this.pool = new HashMap<>();
    }

    //--------------------------------------------------------------------------
    // インスタンスの取得
    //--------------------------------------------------------------------------
    public static StampFactory getInstance()
    {
        if( StampFactory.instance == null )
        {
            StampFactory.instance = new StampFactory();
        }
        return StampFactory.instance;
    }

    //--------------------------------------------------------------------------
    // スタンプの生成
    //--------------------------------------------------------------------------
    public Stamp getStamp( char charname )
    {
        Stamp bc = this.pool.get("" + charname);

        // プールになければ生成
        if( bc == null )
        {
            bc = new Stamp(charname);
            this.pool.put("" + charname, bc);
        }

        bc.incrementUseCount();
        return bc;
    }

    //--------------------------------------------------------------------------
    // すべてのスタンプを表示
    //--------------------------------------------------------------------------
    public void printAll()
    {
        for( Entry<String, Stamp> entry : pool.entrySet() )
        {
            entry.getValue().print();
        }
    }
}
```
