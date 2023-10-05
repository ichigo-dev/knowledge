# 『Value Object』ノート

（最終更新： 2023-10-05）


## 目次

1. [Value Objectパターン](#value-objectパターン)
	1. [ValueObject](#valueobject)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Value Objectパターン

**Value Objectパターン**は、通常は単なる値（[プリミティブ](../../../../programming/_/chapters/data_type.md#プリミティブ型)）として表されるものの役割を明確にし、[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)として実装する[デザインパターン](./design_pattern.md#デザインパターン)。Value Objectは[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)同士を比較することができ、内部に持つ値は不変であることが望ましい。

Value Objectパターンは[ValueObject](#valueobject)の役のみで構成される。

### ValueObject

**Value Object**は、[Value Objectパターン](#value-objectパターン)において、値そのものを表す[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)。[メンバ変数](../../../../programming/_/chapters/object_oriented.md#プロパティ)として値そのものを持っており、この値は不変となるようにする場合が多い。値を変更するようなの[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を必要とする場合には、自身の新しい[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成して返却するように設計する。


## サンプルプログラム

### Java

お金を扱う[システム](../../../../system/_/chapters/system.md#システム)について、通貨は通常 `int` や `float` などの数値型で扱うことができるが、これらを[Value Object](#value-objectパターン)で実現する場合を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        Money wallet = new Money(1000);
        Money bank = new Money(30000);
        Money total = wallet.add(bank);

        System.out.println("==========");
        System.out.println("Wallet:");
        System.out.println(wallet.getValue());
        System.out.println("==========");
        System.out.println("Bank:");
        System.out.println(bank.getValue());
        System.out.println("==========");
        System.out.println("Total:");
        System.out.println(total.getValue());
        System.out.println("==========");
    }
}

//------------------------------------------------------------------------------
// Value Object
//------------------------------------------------------------------------------
class Money
{
    private final int value;

    //--------------------------------------------------------------------------
    // デフォルトコンストラクタを隠蔽
    //--------------------------------------------------------------------------
    private Money()
    {
        this.value = 0;
    }

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Money( int value )
    {
        this.value = value;
    }

    //--------------------------------------------------------------------------
    // 値の取得
    //--------------------------------------------------------------------------
    public int getValue()
    {
        return this.value;
    }

    //--------------------------------------------------------------------------
    // 値の加算（新しいMoneyオブジェクトを返す）
    //--------------------------------------------------------------------------
    public Money add( Money other )
    {
        return new Money(this.getValue() + other.getValue());
    }

    //--------------------------------------------------------------------------
    // 値の比較
    //--------------------------------------------------------------------------
    public boolean equals( Money other )
    {
        return this.getValue() == other.getValue();
    }
}
```
