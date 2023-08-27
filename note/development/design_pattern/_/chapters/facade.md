# 『Facade』ノート

（最終更新： 2023-08-09）


## 目次

1. [Facadeパターン](#facadeパターン)
	1. [Facade](#facade)
	1. [Class](#class)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Facadeパターン

**Facadeパターン**は、複雑な内部処理を隠蔽し、利用者にシンプルなインタフェースを提供する[デザインパターン](./design_pattern.md#デザインパターン)。複雑な[API](../../../../computer/software/_/chapters/operating_system.md#api)呼び出しの適切な実行順を利用者に意識させないという目的もある。また、複雑だがよく使われる処理に対してエイリアスとして使うことも可能。ただし、**神オブジェクト**（複雑すぎる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)）になってしまう可能性があるので、注意が必要。

Facadeパターンは、[Facade](#facade)、[Class](#class)から構成される。

### Facade

**Facade**（正面）は、[Facadeパターン](#facadeパターン)において、複雑な処理を構成している[Class](#class)をシンプルなインタフェースで提供する役割を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。各[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を適切な順番、使い方で呼び出す。

### Class

**Class**（各処理）は、[Facadeパターン](#facadeパターン)において、[Facade](#facade)から呼び出されて処理を行う[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Facade](#facade)を意識せず、[Facade](#facade)を呼び出すこともない。


## サンプルプログラム

### Java

[Facadeパターン](#facadeパターン)を用いて、テンプレートメールを簡単に生成できるようにする例を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // テンプレートメールの作成
        Email systemEmail = EmailFacade.createSystemEmail();
        System.out.println(systemEmail.getContent());

        Email welcomeEmail = EmailFacade.createWelcomeEmail();
        System.out.println(welcomeEmail.getContent());
    }
}

//------------------------------------------------------------------------------
// Facade
//------------------------------------------------------------------------------
class EmailFacade
{
    //--------------------------------------------------------------------------
    // コンストラクタを隠蔽
    //--------------------------------------------------------------------------
    private EmailFacade()
    {
    }

    //--------------------------------------------------------------------------
    // システムメールを作成
    //--------------------------------------------------------------------------
    public static Email createSystemEmail()
    {
        Email email = new Email();
        email.setFromAddress("system@example.com");
        email.setTitle("System message");
        email.setBody("");
        return email;
    }

    //--------------------------------------------------------------------------
    // Welcomeを作成
    //--------------------------------------------------------------------------
    public static Email createWelcomeEmail()
    {
        Email email = new Email();
        email.setFromAddress("system@example.com");
        email.setTitle("Welcome to our system");
        email.setBody("Thank you for using our system!");
        return email;
    }
}

//------------------------------------------------------------------------------
// Class
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
}
```
