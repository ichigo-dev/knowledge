# 『Bridge』ノート

（最終更新： 2023-08-07）


## 目次

1. [Bridgeパターン](#bridgeパターン)
	1. [Abstraction](#abstraction)
	1. [RefinedAbstraction](#refinedabstraction)
	1. [Implementor](#implementor)
	1. [ConcreteImplementor](#concreteimplementor)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Bridgeパターン

**Bridgeパターン**は、機能の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層と実装の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層を分離して[継承](../../../../programming/_/chapters/object_oriented.md#継承)させ、それぞれを独立して管理することで、拡張性を高める[デザインパターン](./design_pattern.md#デザインパターン)。

[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層を分離しないアンチパターンとして、ある[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)を実装した[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)がすでに存在するとする。その[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)に機能を追加する目的で、その[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)を[継承](../../../../programming/_/chapters/object_oriented.md#継承)した別の[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)を新しく実装したとき、元の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)の[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)の実装と同じものを再度作らなければならなくなる（新しく作った[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)から、[継承](../../../../programming/_/chapters/object_oriented.md#継承)元の[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)を実装した[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)は見えない）。そのため、機能を追加するたびに[具象クラス](../../../../programming/_/chapters/object_oriented.md#具象クラス)の数が増えてしまう。

Bridgeパターンでは、[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)を用いずに、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)の移譲を利用することでこのような問題を解決している。

Bridgeパターンは、[Abstraction](#abstraction)、[RefinedAbstraction](#refinedabstraction)、[Implementor](#implementor)、[ConcreteImplementor](#concreteimplementor)から構成される。

### Abstraction

**Abstraction**（抽象化）は、[Bridgeパターン](#bridgeパターン)の機能の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層において、[Implementor](#implementor)を介して具体的な実装を呼び出す抽象化[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)（[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)ではない）。メイン[プログラム](../../../../programming/_/chapters/programming.md#プログラム)等が実際に利用するのは、Abstractionやそれを[継承](../../../../programming/_/chapters/object_oriented.md#継承)した[RefinedAbstraction](#refinedabstraction)となる。

Abstractionは、実装の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層の共通[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)（[Implementor](#implementor)）を実装した[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)（[ConcreteImplementor](#concreteimplementor)）を[フィールド](../../../../programming/_/chapters/object_oriented.md#プロパティ)として持つ。この[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を介して具体的な実装を呼び出すことで、[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)を用いずに抽象化を実現している。

### RefinedAbstraction

**RefinedAbstraction**（改善した抽象化）は、[Bridgeパターン](#bridgeパターン)の機能の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層において、[Abstraction](#abstraction)を[継承](../../../../programming/_/chapters/object_oriented.md#継承)して機能を拡充した[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Abstraction](#abstraction)から直接[継承](../../../../programming/_/chapters/object_oriented.md#継承)するだけでなく、別のRefinedAbstractionの機能を[継承](../../../../programming/_/chapters/object_oriented.md#継承)してさらに機能追加を行うこともできる。

### Implementor

**Implementor**（実装者）は、[Bridgeパターン](#bridgeパターン)の実装の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層において、[Abstraction](#abstraction)が利用する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義した[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。[Abstraction](#abstraction)はImplementatorで宣言された[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を介してのみ具体的な実装を利用することができる。

### ConcreteImplementor

**ConcreteImplementor**（具体的な実装者）は、[Bridgeパターン](#bridgeパターン)の実装の[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)階層において、[Implementor](#implementor)の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。ConcreteImplementorの[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)は[Abstraction](#abstraction)の[フィールド](../../../../programming/_/chapters/object_oriented.md#フィールド)として保持される。


## サンプルプログラム

### Java

[HTML](../../../../web_development/html/_/chapters/html.md#html)[コンポーネント](../../../../computer/software/_/chapters/package.md#コンポーネント)を生成する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)において、[コンポーネント](../../../../computer/software/_/chapters/package.md#コンポーネント)自体とそれを描画する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)を分離する場合を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // コンポーネントの作成と描画
        Component component = new ButtonComponent();
        component.setContent("Example");
        ComponentRenderer renderer = new ComponentRenderer(component);
        renderer.display();

        // コンポーネントレンダラーの拡張
        AdvancedComponentRenderer advancedRenderer = renderer.advanced();
        advancedRenderer.displayWithParen();
    }
}

//------------------------------------------------------------------------------
// Abstraction
//------------------------------------------------------------------------------
class ComponentRenderer
{
    protected Component component;

    //--------------------------------------------------------------------------
    // コンストラクタ（Componentを委譲）
    //--------------------------------------------------------------------------
    public ComponentRenderer( Component component )
    {
        this.component = component;
    }

    //--------------------------------------------------------------------------
    // コンポーネントを描画
    //--------------------------------------------------------------------------
    public void display()
    {
        System.out.println(this.component.build());
    }

    //--------------------------------------------------------------------------
    // 拡張
    //--------------------------------------------------------------------------
    public AdvancedComponentRenderer advanced()
    {
        return new AdvancedComponentRenderer(this.component);
    }
}

//------------------------------------------------------------------------------
// RefinedAbstraction
//------------------------------------------------------------------------------
class AdvancedComponentRenderer extends ComponentRenderer
{
    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public AdvancedComponentRenderer( Component component )
    {
        super(component);
    }

    //--------------------------------------------------------------------------
    // 括弧付きでコンポーネントを描画
    //--------------------------------------------------------------------------
    public void displayWithParen()
    {
        System.out.println("(" + this.component.build() + ")");
    }
}

//------------------------------------------------------------------------------
// Implementor
//------------------------------------------------------------------------------
interface Component
{
    public abstract void setContent( String content );
    public abstract String build();
}

//------------------------------------------------------------------------------
// ConcreteImplementor
//------------------------------------------------------------------------------
class ButtonComponent implements Component
{
    protected String content;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public ButtonComponent()
    {
        this.content = "";
    }

    //--------------------------------------------------------------------------
    // コンテンツの設定
    //--------------------------------------------------------------------------
    public void setContent( String content )
    {
        this.content = content;
    }

    //--------------------------------------------------------------------------
    // パーツの作成
    //--------------------------------------------------------------------------
    public String build()
    {
        return "<button>" + this.content + "</button>";
    }
}

class TextFieldComponent implements Component
{
    protected String content;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TextFieldComponent()
    {
        this.content = "";
    }

    //--------------------------------------------------------------------------
    // コンテンツの設定
    //--------------------------------------------------------------------------
    public void setContent( String content )
    {
        this.content = content;
    }

    //--------------------------------------------------------------------------
    // パーツの作成
    //--------------------------------------------------------------------------
    public String build()
    {
        return "<label><input>" + this.content + "</label>";
    }
}
```
