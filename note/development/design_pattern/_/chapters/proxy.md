# 『Proxy』ノート

（最終更新： 2023-08-15）


## 目次

1. [Proxyパターン](#proxyパターン)
	1. [Subject](#subject)
	1. [RealSubject](#realsubject)
	1. [Proxy](#proxy)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Proxyパターン

**Proxyパターン**は、本人の代理人となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が本人でもなくともできる処理を受け持ち、本人にしかできない場合にだけ処理を任せる[デザインパターン](./design_pattern.md#デザインパターン)。負荷の高い[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の代わりに一部の処理を代理人が行うことで、負荷を軽減することができる。

Proxyパターンは、[Subject](#subject)、[Proxy](#proxy)、[RealSubject](#realsubject)から構成される。

### Subject

**Subject**（主体）は、[Proxyパターン](#proxyパターン)において、[Proxy](#proxy)と[RealSubject](#realsubject)を同一視するための[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。これにより、[Proxyパターン](#proxyパターン)を利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)は、[Proxy](#proxy)と[RealSubject](#realsubject)の使い分けを意識する必要がなくなる。

### RealSubject

**RealSubject**（実際の主体）は、[Proxyパターン](#proxyパターン)において、[Proxy](#proxy)では手に負えない処理を要求された際に、[Proxy](#proxy)からの依頼を受けて処理を行う[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Proxy](#proxy)と同じく[Subject](#subject)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を実装する。

### Proxy

**Proxy**（代理人）は、[Proxyパターン](#proxyパターン)において、このパターンを利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)からの要求をできる限り処理する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。自身が処理できない処理に関しては、[RealSubject](#realsubject)に依頼する。Proxyは、[Subject](#subject)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を実装する。


## サンプルプログラム

### Java

[コンストラクタ](../../../../programming/_/chapters/object_oriented.md#コンストラクタ)で重い処理を実行する必要がある[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)において、一部の処理を[Proxy](#proxy)に委ねるような場合を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // Proxy経由でメソッドを呼び出し
        Printable printer = new PrinterProxy("Printer1");
        System.out.println(printer.getName());
        printer.setName("Printer2");
        System.out.println(printer.getName());
        printer.print("Hello, world!");
    }
}

//------------------------------------------------------------------------------
// Subject
//------------------------------------------------------------------------------
interface Printable
{
    public abstract void setName( String name );
    public abstract String getName();
    public abstract void print( String string );
}

//------------------------------------------------------------------------------
// RealSubject
//------------------------------------------------------------------------------
class Printer implements Printable
{
    private String name;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Printer( String name )
    {
        this.name = name;
        this.heavyJob();
    }

    //--------------------------------------------------------------------------
    // 名前の設定
    //--------------------------------------------------------------------------
    public void setName( String name )
    {
        this.name = name;
    }

    //--------------------------------------------------------------------------
    // 名前の取得
    //--------------------------------------------------------------------------
    public String getName()
    {
        return this.name;
    }

    //--------------------------------------------------------------------------
    // 表示
    //--------------------------------------------------------------------------
    public void print( String msg )
    {
        System.out.println("=== " + this.name + " ===");
        System.out.println(msg);
    }

    //--------------------------------------------------------------------------
    // 重い処理
    //--------------------------------------------------------------------------
    private void heavyJob()
    {
        try
        {
            System.out.println("Heavy job start");
            Thread.sleep(100);
            System.out.println("Heavy job end");
        }
        catch( Exception e )
        {
            System.out.println(e);
        }
    }
}

//------------------------------------------------------------------------------
// Proxy
//------------------------------------------------------------------------------
class PrinterProxy implements Printable
{
    private String name;
    private Printer real;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public PrinterProxy( String name )
    {
        this.name = name;
    }

    //--------------------------------------------------------------------------
    // 名前の設定
    //--------------------------------------------------------------------------
    public void setName( String name )
    {
        if( this.real != null )
        {
            this.real.setName(name);
        }
        this.name = name;
    }

    //--------------------------------------------------------------------------
    // 名前の取得
    //--------------------------------------------------------------------------
    public String getName()
    {
        return this.name;
    }

    //--------------------------------------------------------------------------
    // 表示
    //--------------------------------------------------------------------------
    public void print( String msg )
    {
        this.realize();
        this.real.print(msg);
    }

    //--------------------------------------------------------------------------
    // 実オブジェクトの生成
    //--------------------------------------------------------------------------
    private void realize()
    {
        if( this.real == null )
        {
            this.real = new Printer(this.name);
        }
    }
}
```
