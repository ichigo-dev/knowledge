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

ユーザのランクに応じて処理を切り換えるような[システム](../../../../system/_/chapters/system.md#システム)において、ランクの条件判定を[Policyパターン](#policyパターン)により行う場合を考える。

```java
import java.util.Set;
import java.util.HashSet;

//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        User goldUser = new User(10000, 0.1);
        User silverUser = new User(10000, 0.5);
        User normalUser = new User(400, 0.2);

        // ゴールド会員の条件
        RankPolicy goldRankPolicy = new RankPolicy();
        goldRankPolicy.add(new purchaseAmountRule());
        goldRankPolicy.add(new returnRateRule());

        //  シルバー会員の条件
        RankPolicy silverRankPolicy = new RankPolicy();
        silverRankPolicy.add(new purchaseAmountRule());

        System.out.println("==========");
        System.out.println("Gold Rank:");
        System.out.println(goldRankPolicy.comply_with_all(goldUser));
        System.out.println(goldRankPolicy.comply_with_all(silverUser));
        System.out.println(goldRankPolicy.comply_with_all(normalUser));
        System.out.println("==========");
        System.out.println("Silver Rank:");
        System.out.println(silverRankPolicy.comply_with_all(goldUser));
        System.out.println(silverRankPolicy.comply_with_all(silverUser));
        System.out.println(silverRankPolicy.comply_with_all(normalUser));
        System.out.println("==========");
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
    public User( int purchaseAmount, double returnRate )
    {
        this.purchaseAmount = purchaseAmount < 0 ? 0 : purchaseAmount;
        this.returnRate = ( returnRate < 0 || returnRate > 1 ) ? 0 : returnRate;
    }

    //--------------------------------------------------------------------------
    // 購入額の取得
    //--------------------------------------------------------------------------
    public int getPurchaseAmount()
    {
        return this.purchaseAmount;
    }

    //--------------------------------------------------------------------------
    // 返品率の取得
    //--------------------------------------------------------------------------
    public double getReturnRate()
    {
        return this.returnRate;
    }
}

//------------------------------------------------------------------------------
// PolicyInterface
//------------------------------------------------------------------------------
interface UserRankRule
{
    public boolean ok( User user );
}

//------------------------------------------------------------------------------
// Policy
//------------------------------------------------------------------------------
class purchaseAmountRule implements UserRankRule
{
    //--------------------------------------------------------------------------
    // 条件判定
    //--------------------------------------------------------------------------
    public boolean ok( User user )
    {
        return user.getPurchaseAmount() > 1000;
    }
}

class returnRateRule implements UserRankRule
{
    //--------------------------------------------------------------------------
    // 条件判定
    //--------------------------------------------------------------------------
    public boolean ok( User user )
    {
        return user.getReturnRate() < 0.3;
    }
}

//------------------------------------------------------------------------------
// Context
//------------------------------------------------------------------------------
class RankPolicy
{
    private Set<UserRankRule> rules;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public RankPolicy()
    {
        this.rules = new HashSet<UserRankRule>();
    }

    //--------------------------------------------------------------------------
    // Policyの追加
    //--------------------------------------------------------------------------
    public void add( UserRankRule rule )
    {
        this.rules.add(rule);
    }

    //--------------------------------------------------------------------------
    // 全ての条件を判定
    //--------------------------------------------------------------------------
    public boolean comply_with_all( User user )
    {
        for( UserRankRule rule : this.rules )
        {
            if( rule.ok(user) == false )
            {
                return false;
            }
        }
        return true;
    }
}
```
