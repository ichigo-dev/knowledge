# 『Abstract Factory』ノート

（最終更新： 2023-08-07）


## 目次

1. [Abstract Factoryパターン](#abstract-factoryパターン)
	1. [AbstractProduct](#abstractproduct)
	1. [ConcreteProduct](#concreteproduct)
	1. [AbstractFactory](#abstractfactory)
	1. [ConcreteFactory](#concretefactory)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Abstract Factoryパターン

**Abstract Factoryパターン**は、関連性のある一連の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)群をまとめて生成する方法を提供する[デザインパターン](./design_pattern.md#デザインパターン)。このパターンを利用することで、関連する[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)のグループ単位での置き換えや追加が容易になる。[Factory Methodパターン](./factory_method.md#factory-methodパターン)が、単体の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成の枠組みを用意するのに対し、こちらは関連する[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)をまとめて管理することができる。

Abstract Factoryパターンは、[AbstractProduct](#abstractproduct)、[ConcreteProduct](#concreteproduct)、[AbstractFactory](#abstractfactory)、[ConcreteFactory](#concretefactory)から構成される。

### AbstractProduct

**AbstractProduct**（抽象的な製品）は、[Abstract Factoryパターン](#abstract-factoryパターン)において、[AbstractFactory](#abstractfactory)によって生成される抽象的な[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)のインタフェースを定義する[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)。AbstractProductは複数存在することができる。

### ConcreteProduct

**ConcreteProduct**（具体的な製品）は、[Abstract Factoryパターン](#abstract-factoryパターン)において、[AbstractProduct](#abstractproduct)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[AbstractProduct](#abstractproduct)が複数存在する場合は、それぞれに対応したConcreteProductの実装が必要となる。

### AbstractFactory

**AbstractFactory**（抽象的な工場）は、[Abstract Factoryパターン](#abstract-factoryパターン)において、グループ化された全ての[AbstractProduct](#abstract)を生成するためのインタフェースを定義する[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)。それぞれの[AbstractProduct](#abstractproduct)を生成するためのインタフェースを定義する。

### ConcreteFactory

**ConcreteFactory**（具体的な工場）は、[Abstract Factoryパターン](#abstract-factoryパターン)において、[AbstractFactory](#abstractfactory)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。


## サンプルプログラム

### Java

[HTML](../../../../web_development/html/_/chapters/html.md#html)[コンポーネント](../../../../computer/software/_/chapters/package.md#コンポーネント)を生成する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)において、いくつかのコンポーネントフレーバ（Simple、Richなど）を生成できるようにしたい場合を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // コンポーネント生成ファクトリから各種コンポーネントを生成
        ComponentFactory factory = new SimpleComponentFactory();
        Button button = factory.createButton("Done");
        TextField textField = factory.createTextField("Name");
        System.out.println(button.build());
        System.out.println(textField.build());
    }
}

//------------------------------------------------------------------------------
// AbstractProduct
//------------------------------------------------------------------------------
abstract class Button
{
    protected String text;

    //--------------------------------------------------------------------------
    // 文字列を設定
    //--------------------------------------------------------------------------
    public void setText( String text )
    {
        this.text = text;
    }

    public abstract String build();
}

//------------------------------------------------------------------------------
// ConcreteProduct
//------------------------------------------------------------------------------
class SimpleButton extends Button
{
    //--------------------------------------------------------------------------
    // パーツの構築
    //--------------------------------------------------------------------------
    public String build()
    {
        return "<button>" + this.text + "</button>";
    }
}

class RichButton extends Button
{
    //--------------------------------------------------------------------------
    // パーツの構築
    //--------------------------------------------------------------------------
    public String build()
    {
        return "<button class=\"rich\">" + this.text + "</button>";
    }
}

//------------------------------------------------------------------------------
// AbstractProduct
//------------------------------------------------------------------------------
abstract class TextField
{
    protected String label;

    //--------------------------------------------------------------------------
    // ラベルを設定
    //--------------------------------------------------------------------------
    public void setLabel( String label )
    {
        this.label = label;
    }

    public abstract String build();
}

//------------------------------------------------------------------------------
// ConcreteProduct
//------------------------------------------------------------------------------
class SimpleTextField extends TextField
{
    //--------------------------------------------------------------------------
    // パーツの構築
    //--------------------------------------------------------------------------
    public String build()
    {
        return "<label><input>" + this.label + "</label>";
    }
}

class RichTextField extends TextField
{
    //--------------------------------------------------------------------------
    // パーツの構築
    //--------------------------------------------------------------------------
    public String build()
    {
        return "<label class=\"rich\"><input>" + this.label + "</label>";
    }
}

//------------------------------------------------------------------------------
// AbstractFactory
//------------------------------------------------------------------------------
abstract class ComponentFactory
{
    public abstract Button createButton( String text );
    public abstract TextField createTextField( String label );
}

//------------------------------------------------------------------------------
// ConcreteFactory
//------------------------------------------------------------------------------
class SimpleComponentFactory extends ComponentFactory
{
    //--------------------------------------------------------------------------
    // ボタンパーツの作成
    //--------------------------------------------------------------------------
    public Button createButton( String text )
    {
        Button button = new SimpleButton();
        button.setText(text);
        return button;
    }

    //--------------------------------------------------------------------------
    // テキスト入力パーツの作成
    //--------------------------------------------------------------------------
    public TextField createTextField( String label )
    {
        TextField textField = new SimpleTextField();
        textField.setLabel(label);
        return textField;
    }
}

class RichComponentFactory extends ComponentFactory
{
    //--------------------------------------------------------------------------
    // ボタンパーツの作成
    //--------------------------------------------------------------------------
    public Button createButton( String text )
    {
        Button button = new RichButton();
        button.setText(text);
        return button;
    }

    //--------------------------------------------------------------------------
    // テキスト入力パーツの作成
    //--------------------------------------------------------------------------
    public TextField createTextField( String label )
    {
        TextField textField = new RichTextField();
        textField.setLabel(label);
        return textField;
    }
}
```
