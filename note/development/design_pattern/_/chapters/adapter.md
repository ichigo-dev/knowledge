# 『Adapter』ノート

（最終更新： 2023-09-23）


## 目次

1. [Adapterパターン](#adapterパターン)
	1. [Target](#target)
	1. [Adaptee](#adaptee)
	1. [Adapter](#adapter)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Adapterパターン

**Adapterパターン**（**Wrapperパターン**）は、既存の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)や[コンポーネント](../../../../computer/software/_/chapters/package.md#コンポーネント)を別のインタフェースに変換するための[デザインパターン](./design_pattern.md#デザインパターン)。このパターンは、既存の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)を利用したいが、そのインタフェースが[プログラム](../../../../programming/_/chapters/programming.md#プログラム)との互換性がない場合に非常に有用である。また、[スーパクラス](../../../../programming/_/chapters/object_oriented.md#親クラス)に欠けている機能を、対象となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)をラップすることで動的に追加するという使い方もできる（[Decoratorパターン](./decorator.md#decoratorパターン)のような使い方）。

Adapterパターンは、[Target](#target)、[Adaptee](#adaptee)、[Adapter](#adapter)から構成される。

### Target

**Target**（対象）は、[Adapterパターン](#adapterパターン)において、今必要となっている[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)（[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)）を定める役割を持つ。[継承](../../../../programming/_/chapters/object_oriented.md#継承)を用いた場合は[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)、委譲を用いた場合は[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)がこの役を担う。

### Adaptee

**Adaptee**は、[Adapterパターン](#adapterパターン)において、既存の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)の実装を提供する役割を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。このAdapteeが持つ[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を[Target](#target)の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)に適合させるために[Adapter](#adapter)が必要となる。

### Adapter

**Adapter**は、[Adapterパターン](#adapterパターン)において、[Adaptee](#adaptee)の提供する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を利用して[Target](#target)の要件を満たす役割を持つ。[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)を用いた[Adapterパターン](#adapterパターン)の場合には、[継承](../../../../programming/_/chapters/object_oriented.md#継承)を使って[Adaptee](#adaptee)を利用する。[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を用いた[Adapterパターン](#adapterパターン)の場合には、[Adaptee](#adaptee)の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を[メンバ変数](../../../../programming/_/chapters/object_oriented.md#プロパティ)として保持し、それを介して[Adaptee](#adaptee)の機能を利用する（委譲）。


## サンプルプログラム

### Java

[システム](../../../../system/_/chapters/system.md#システム)内での消費税計算に、既存の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)のインタフェースを修正して利用したいケースを考える。

まずは、[継承](../../../../programming/_/chapters/object_oriented.md#継承)を使った例。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // 消費税の計算
        TaxCalculatorInterface calculator = new TaxCalculator(0.08);
        System.out.println(calculator.calculate(1000));
        System.out.println(calculator.calculateTaxAmount(1000));
    }
}

//------------------------------------------------------------------------------
// Target
//------------------------------------------------------------------------------
interface TaxCalculatorInterface
{
    public abstract double calculateTaxAmount( double amount );
    public abstract double calculate( double amount );
}

//------------------------------------------------------------------------------
// Adaptee（既存の税計算クラス）
//------------------------------------------------------------------------------
class TaxUtility
{
    private double taxRate;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TaxUtility( double taxRate )
    {
        this.taxRate = taxRate;
    }

    //--------------------------------------------------------------------------
    // 消費税の計算
    //--------------------------------------------------------------------------
    public double getTax( double amount )
    {
        return amount * this.taxRate;
    }

    //--------------------------------------------------------------------------
    // 消費税を含めた合計金額の計算
    //--------------------------------------------------------------------------
    public double getTotalWithTax( double amount )
    {
        return amount + this.getTax(amount);
    }
}

//------------------------------------------------------------------------------
// Adapter（既存のクラスを継承）
//------------------------------------------------------------------------------
class TaxCalculator extends TaxUtility implements TaxCalculatorInterface
{
    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TaxCalculator( double taxRate )
    {
        super(taxRate);
    }

    //--------------------------------------------------------------------------
    // 消費税の計算
    //--------------------------------------------------------------------------
    public double calculateTaxAmount( double amount )
    {
        return super.getTax(amount);
    }

    //--------------------------------------------------------------------------
    // 消費税を含めた合計金額の計算
    //--------------------------------------------------------------------------
    public double calculate( double amount )
    {
        return super.getTotalWithTax(amount);
    }
}
```

次に、委譲を使った例。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // 消費税の計算
        AbstractTaxCalculator calculator = new TaxCalculator(0.08);
        System.out.println(calculator.calculate(1000));
        System.out.println(calculator.calculateTaxAmount(1000));
    }
}

//------------------------------------------------------------------------------
// Target
//------------------------------------------------------------------------------
abstract class AbstractTaxCalculator
{
    public abstract double calculateTaxAmount( double amount );
    public abstract double calculate( double amount );
}

//------------------------------------------------------------------------------
// Adaptee（既存の税計算クラス）
//------------------------------------------------------------------------------
class TaxUtility
{
    private double taxRate;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TaxUtility( double taxRate )
    {
        this.taxRate = taxRate;
    }

    //--------------------------------------------------------------------------
    // 消費税の計算
    //--------------------------------------------------------------------------
    public double getTax( double amount )
    {
        return amount * this.taxRate;
    }

    //--------------------------------------------------------------------------
    // 消費税を含めた合計金額の計算
    //--------------------------------------------------------------------------
    public double getTotalWithTax( double amount )
    {
        return amount + this.getTax(amount);
    }
}

//------------------------------------------------------------------------------
// Adapter（既存のクラスを委譲）
//------------------------------------------------------------------------------
class TaxCalculator extends AbstractTaxCalculator
{
    private TaxUtility utility;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TaxCalculator( double taxRate )
    {
        this.utility = new TaxUtility(taxRate);
    }

    //--------------------------------------------------------------------------
    // 消費税の計算
    //--------------------------------------------------------------------------
    public double calculateTaxAmount( double amount )
    {
        return this.utility.getTax(amount);
    }

    //--------------------------------------------------------------------------
    // 消費税を含めた合計金額の計算
    //--------------------------------------------------------------------------
    public double calculate( double amount )
    {
        return this.utility.getTotalWithTax(amount);
    }
}
```
