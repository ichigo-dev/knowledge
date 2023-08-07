# 『Builder』ノート

（最終更新： 2023-08-07）


## 目次

1. [Builderパターン](#builderパターン)
	1. [Builder](#builder)
	1. [ConcreteBuilder](#concretebuilder)
	1. [Director](#director)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Builderパターン

**Builderパターン**は、[コンストラクタ](../../../../programming/_/chapters/object_oriented.md#コンストラクタ)に対して数多くの[パラメータ](../../../../programming/_/chapters/function.md#引数)をセットする必要がある場合や、多くのオプションを持つ場合に、段階を踏んで[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を組み上げていく[デザインパターン](./design_pattern.md#デザインパターン)。[パラメータ](../../../../programming/_/chapters/function.md#引数)が多い場合、その順番を正確に覚えることが難しくなったり、複数のオプションのうち限られたもののみを利用したい場合に冗長になったりするが、Builderパターンを用いることで簡潔な記述ができるようになる。また、このパターンの適用により[コンストラクタ](../../../../programming/_/chapters/object_oriented.md#コンストラクタ)の肥大化や複雑化を軽減することができる。

Builderパターンは、[Builder](#builder)、[ConcreteBuilder](#concretebuilder)、[Director](#director)から構成される。

### Builder

**Builder**（建築者）は、[Builderパターン](#builderパターン)において、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成するためのインタフェースを定める役。Builderには、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)の各部分を作るための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)が用意される。

### ConcreteBuilder

**ConcreteBuilder**（具体的建築者）は、[Builderパターン](#builderパターン)において、[Builder](#builder)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。実際の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成で呼び出される[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)がここで定義される。また、最終的に構築された結果を得るための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)も必要となる。

### Director

**Director**（監督者）は、[Builderパターン](#builderパターン)において、[Builder](#builder)のインタフェースを使って[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成する役。[ConcreteBuilder](#concretebuilder)に依存した実装は行わず、[Builder](#builder)の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)のみを使用する。このパターンを利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)が直接この役を担ったり、頻出する[Builder](#builder)の構築ステップをまとめた[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)として実現したりする。


## サンプルプログラム

### Java

メール送信機能の実装に[Builderパターン](#builderパターン)を適用する例を考える。

```java
//------------------------------------------------------------------------------
// Client (Director)
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // メールの作成
        Email email = Email.builder()
            .to("smith@example.com")
            .from("johnson@example.com")
            .title("Hello")
            .body("Hello, world")
            .build();
        System.out.println(email.getContent());
    }
}

//------------------------------------------------------------------------------
// EmailBuilderによって構築されるEmailクラス
//------------------------------------------------------------------------------
class Email
{
    private String toAddress;
    private String fromAddress;
    private String title;
    private String body;

    //--------------------------------------------------------------------------
    // 送信先の設定
    //--------------------------------------------------------------------------
    public void setToAddress( String address )
    {
        this.toAddress = address;
    }

    //--------------------------------------------------------------------------
    // 送信元の設定
    //--------------------------------------------------------------------------
    public void setFromAddress( String address )
    {
        this.fromAddress = address;
    }

    //--------------------------------------------------------------------------
    // 件名の設定
    //--------------------------------------------------------------------------
    public void setTitle( String content )
    {
        this.title = content;
    }

    //--------------------------------------------------------------------------
    // 本文の設定
    //--------------------------------------------------------------------------
    public void setBody( String content )
    {
        this.body = content;
    }

    //--------------------------------------------------------------------------
    // 内容の取得
    //--------------------------------------------------------------------------
    public String getContent()
    {
        return this.title + ": " + this.body;
    }

    //--------------------------------------------------------------------------
    // EmailBuilderの取得
    //--------------------------------------------------------------------------
    public static EmailBuilder builder()
    {
        return new EmailBuilder();
    }
}

//------------------------------------------------------------------------------
// Builder
//------------------------------------------------------------------------------
interface EmailBuilderInterface
{
    public abstract EmailBuilderInterface to( String address );
    public abstract EmailBuilderInterface from( String address );
    public abstract EmailBuilderInterface title( String content );
    public abstract EmailBuilderInterface body( String content );
}

//------------------------------------------------------------------------------
// ConcreteBuilder
//------------------------------------------------------------------------------
class EmailBuilder implements EmailBuilderInterface
{
    private Email email;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public EmailBuilder()
    {
        this.email = new Email();
    }

    //--------------------------------------------------------------------------
    // 送信先の設定
    //--------------------------------------------------------------------------
    public EmailBuilder to( String address )
    {
        this.email.setToAddress(address);
        return this;
    }

    //--------------------------------------------------------------------------
    // 送信元の設定
    //--------------------------------------------------------------------------
    public EmailBuilder from( String address )
    {
        this.email.setFromAddress(address);
        return this;
    }

    //--------------------------------------------------------------------------
    // 件名の設定
    //--------------------------------------------------------------------------
    public EmailBuilder title( String content )
    {
        this.email.setTitle(content);
        return this;
    }

    //--------------------------------------------------------------------------
    // 本文の設定
    //--------------------------------------------------------------------------
    public EmailBuilder body( String content )
    {
        this.email.setBody(content);
        return this;
    }

    //--------------------------------------------------------------------------
    // Emailの生成
    //--------------------------------------------------------------------------
    public Email build()
    {
        return this.email;
    }
}
```
