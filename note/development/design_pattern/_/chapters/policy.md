# 『Policy』ノート

（最終更新： 2023-10-04）


## 目次

1. [Policyパターン](#policyパターン)
	1. [Context](#context)
	1. [PolicyInterface](#policyinterface)
	1. [Policy](#policy)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Policyパターン

**Policyパターン**は、[条件分岐](../../../../programming/_/chapters/control_flow.md#条件分岐)を部品化し、これらの部品を組み合わせることで条件をカスタマイズすることができるようにする[デザインパターン](./design_pattern.md#デザインパターン)。

Policyパターンは、[Context](#context)、[PolicyInterface](#policyinterface)、[Policy](#policy)から構成される。

### Context

**Context**は、[Policyパターン](#policyパターン)において、[PolicyInterface](#policyinterface)を実装する[Policy](#policy)を組み合わせることで、新しい条件を構築したり、構築された条件を判定に利用する役割。

### PolicyInterface

**PolicyInterface**は、[Policyパターン](#policyパターン)において、[Policy](#policy)が実装するべき[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定めた[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。条件式を包み込む[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義する。

### Policy

**Policy**は、[Policyパターン](#policyパターン)において、[PolicyInterface](#policyinterface)を実装した、条件そのものを表す[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。


## サンプルプログラム

### Java

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        User goldUser = new User(10000, 0.1);
        User normalUser = new User(400, 0.2);
    }
}

//------------------------------------------------------------------------------
// システム利用者
//------------------------------------------------------------------------------
class User
{
    private int purchaseAmount;
    private double returnRate;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    User( int purchaseAmount, double returnRate )
    {
        this.purchaseAmount = purchaseAmount < 0 ? 0 : purchaseAmount;
        this.returnRate = ( returnRate < 0 || returnRate > 1 ) ? 0 : returnRate;
    }

    //--------------------------------------------------------------------------
    // 購入額の取得
    //--------------------------------------------------------------------------
    int getPurchaseAmount()
    {
        return this.purchaseAmount;
    }

    //--------------------------------------------------------------------------
    // 返品率の取得
    //--------------------------------------------------------------------------
    double getReturnRate()
    {
        return this.returnRate;
    }
}

//------------------------------------------------------------------------------
// PolicyInterface
//------------------------------------------------------------------------------
interface UserRankRule
{
    boolean ok( User user );
}

//------------------------------------------------------------------------------
// Policy
//------------------------------------------------------------------------------
class purchaseAmountRule implements UserRankRule
{
    //--------------------------------------------------------------------------
    // 条件判定
    //--------------------------------------------------------------------------
    boolean ok( User user )
    {
        return user.getPurchaseAmount() > 1000;
    }
}

class returnRateRule implements UserRankRule
{
    //--------------------------------------------------------------------------
    // 条件判定
    //--------------------------------------------------------------------------
    boolean ok( User user )
    {
        return user.getReturnRate() < 0.3;
    }
}

//------------------------------------------------------------------------------
// Context
//------------------------------------------------------------------------------
class GoldRankPolicy
{
    private Set<UserRankRule> rules;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    GoldRankPolicy()
    {
        this.rules = new HashSet();
    }
}
```
