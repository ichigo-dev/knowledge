# 『Decorator』ノート

（最終更新： 2023-08-08）


## 目次

1. [Decoratorパターン](#decoratorパターン)
	1. [Component](#component)
	1. [ConcreteComponent](#concretecomponent)
	1. [Decorator](#decorator)
	1. [ConcreteDecorator](#concretedecorator)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Decoratorパターン

**Decoratorパターン**は、中心となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に対して次々と機能をかぶせていき、より目的にあった[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に仕上げていく[デザインパターン](./design_pattern.md#デザインパターン)。

Decoratorパターンは、[Component](#component)、[ConcreteComponent](#concretecomponent)、[Decorator](#decorator)、[ConcreteDecorator](#concretedecorator)から構成される。

### Component

**Component**は、[Decoratorパターン](#decoratorパターン)において、デコレーションを施すためのコアとなる部分の[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。[ConcreteComponent](#concretecomponent)や[Decorator](#decorator)はこの[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の実装を持つ。

### ConcreteComponent

**ConcreteComponent**は、[Decoratorパターン](#decoratorパターン)において、デコレーションを施すためのコアとなる部分で、[Component](#component)を実装している[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Decorator](#decorator)によって変更できる基本的な[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義する。

### Decorator

**Decorator**（装飾者）は、[Decoratorパターン](#decoratorパターン)において、[Component](#component)と同じ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を持ち、デコレーションする対象となる[Component](#component)のインスタンスを[フィールド](../../../../programming/_/chapters/object_oriented.md#インタフェース)に含んでいる役。

### ConcreteDecorator

**ConcreteDecorator**（具体的な装飾者）は、[Decoratorパターン](#decoratorパターン)において、[Decorator](#decorator)の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。


## サンプルプログラム

### Java

テキストに対して様々なスタイルの適用を行うテキストフォーマッタの実装を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // テキストに対して様々なフォーマットを行う
        String text = "Hello, world!";
        TextFormatter plainTextFormatter = new PlainTextFormatter();
        System.out.println(plainTextFormatter.format(text));

        TextFormatter boldFormatter = new BoldTextDecorator(plainTextFormatter);
        System.out.println(boldFormatter.format(text));

        TextFormatter italicBoldFormatter = new ItalicTextDecorator
        (
            new BoldTextDecorator(plainTextFormatter)
        );
        System.out.println(italicBoldFormatter.format(text));

        TextFormatter underlineItalicBoldFormatter = new UnderlineTextDecorator
        (
            new ItalicTextDecorator
            (
                new BoldTextDecorator(plainTextFormatter)
            )
        );
        System.out.println(underlineItalicBoldFormatter.format(text));
    }
}

//------------------------------------------------------------------------------
// Component
//------------------------------------------------------------------------------
interface TextFormatter
{
    public abstract String format( String text );
}

//------------------------------------------------------------------------------
// ConcreteComponent
//------------------------------------------------------------------------------
class PlainTextFormatter implements TextFormatter
{
    //--------------------------------------------------------------------------
    // テキストのフォーマット
    //--------------------------------------------------------------------------
    public String format( String text )
    {
        return text;
    }
}

//------------------------------------------------------------------------------
// Decorator
//------------------------------------------------------------------------------
abstract class TextDecorator implements TextFormatter
{
    protected TextFormatter decoratedText;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TextDecorator( TextFormatter decoratedText )
    {
        this.decoratedText = decoratedText;
    }
}

//------------------------------------------------------------------------------
// ConcreteDecorator
//------------------------------------------------------------------------------
class BoldTextDecorator extends TextDecorator
{
    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public BoldTextDecorator( TextFormatter decoratedText )
    {
        super(decoratedText);
    }

    //--------------------------------------------------------------------------
    // テキストのフォーマット
    //--------------------------------------------------------------------------
    public String format( String text )
    {
        return "<b>" + decoratedText.format(text) + "</b>";
    }
}

class ItalicTextDecorator extends TextDecorator
{
    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public ItalicTextDecorator( TextFormatter decoratedText )
    {
        super(decoratedText);
    }

    //--------------------------------------------------------------------------
    // テキストのフォーマット
    //--------------------------------------------------------------------------
    public String format( String text )
    {
        return "<i>" + decoratedText.format(text) + "</i>";
    }
}

class UnderlineTextDecorator extends TextDecorator
{
    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public UnderlineTextDecorator( TextFormatter decoratedText )
    {
        super(decoratedText);
    }

    //--------------------------------------------------------------------------
    // テキストのフォーマット
    //--------------------------------------------------------------------------
    public String format( String text )
    {
        return "<u>" + decoratedText.format(text) + "</u>";
    }
}
```
