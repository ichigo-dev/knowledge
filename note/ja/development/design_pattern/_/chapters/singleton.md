# 『Singleton』ノート

（最終更新： 2023-08-06）


## 目次

1. [Singletonパターン](#singletonパターン)
	1. [Singleton](#singleton)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Singletonパターン

**Singletonパターン**は、ある[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)が絶対に1つしか存在しないことを保証する[デザインパターン](./design_pattern.md#デザインパターン)。[システム](../../../../system/_/chapters/system.md#システム)中で1つしか存在しないものを[プログラム](../../../../programming/_/chapters/programming.md#プログラム)で表現したい場合に用いられる。

Singletonパターンでは、[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)の[コンストラクタ](../../../../programming/_/chapters/object_oriented.md#コンストラクタ)を[private](../../../../programming/_/chapters/object_oriented.md#private)に隠蔽し、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)の生成および取得のための[staticメソッド](../../../../programming/_/chapters/object_oriented.md#staticメソッド)を用意しておく。Singletonパターンは[Singleton](#singleton)の役のみで構成される。

Singletonパターンを使用する際は、Singletonが[グローバル](../../../../programming/_/chapters/control_flow.md#グローバル)な状態であり、テストが難しくなることや、マルチスレッド環境において同時アクセスされることを想定したスレッドセーフな設計が必要となることに注意しなければいけない。Singletonパターンは非常に多くの問題を解決することができる一方で、依存関係を隠蔽してしまい発見が困難な不具合を生じる可能性があるなど、アンチパターンと見られる場合も多い。

### Singleton

**Singleton**は、[Singletonパターン](#singletonパターン)において、唯一の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成・管理・取得するためのインタフェースを持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[コンストラクタ](../../../../programming/_/chapters/object_oriented.md#コンストラクタ)は[private](../../../../programming/_/chapters/object_oriented.md#private)に隠蔽されており、呼び出されることはない。代わりに唯一の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を取得するための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を提供する。


## サンプルプログラム

### Java

ある[システム](../../../../system/_/chapters/system.md#システム)のコア部分（Application[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)）について、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)中に1つしか[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)が存在しないことを前提として設計する場合を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        Application app = Application.getInstance();
        app.start();
    }
}

//------------------------------------------------------------------------------
// Singleton
//------------------------------------------------------------------------------
class Application
{
    private static Application instance;
    private boolean isInitialized;

    //--------------------------------------------------------------------------
    // コンストラクタをprivateに隠蔽
    //--------------------------------------------------------------------------
    private Application()
    {
        this.isInitialized = false;
    }

    //--------------------------------------------------------------------------
    // インスタンス取得
    //--------------------------------------------------------------------------
    public static Application getInstance()
    {
        if( Application.instance == null )
        {
            Application.instance = new Application();
        }
        return Application.instance;
    }

    //--------------------------------------------------------------------------
    // アプリケーションの起動
    //--------------------------------------------------------------------------
    public void start()
    {
        if( this.isInitialized )
        {
            System.out.println("Application is already started.");
        }
        else
        {
            System.out.println("Application started.");
        }
        this.isInitialized = true;
    }
}
```

マルチスレッド対応のSingleton実装の例。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // 2つのスレッドからそれぞれApplicationを呼び出す
        Thread threadOne = new Thread(new ThreadOne());
        Thread threadTwo = new Thread(new ThreadTwo());
        threadOne.start();
        threadTwo.start();
    }

    //--------------------------------------------------------------------------
    // 1つ目のスレッド
    //--------------------------------------------------------------------------
    static class ThreadOne implements Runnable
    {
        public void run() 
        {
            System.out.println("ThreadOne");
            Application app = Application.getInstance();
            app.start();
        }
    }

    //--------------------------------------------------------------------------
    // 2つ目のスレッド
    //--------------------------------------------------------------------------
    static class ThreadTwo implements Runnable
    {
        public void run() 
        {
            System.out.println("ThreadTwo");
            Application app = Application.getInstance();
            app.start();
        }
    }
}

//------------------------------------------------------------------------------
// Singleton
//------------------------------------------------------------------------------
class Application
{
    private static volatile Application instance;
    private boolean isInitialized;

    //--------------------------------------------------------------------------
    // コンストラクタをprivateに隠蔽
    //--------------------------------------------------------------------------
    private Application()
    {
        this.isInitialized = false;
    }

    //--------------------------------------------------------------------------
    // インスタンス取得
    //--------------------------------------------------------------------------
    public static Application getInstance()
    {
        Application app = Application.instance;
        if( app != null )
        {
            return app;
        }
        synchronized( Application.class )
        {
            if( Application.instance == null )
            {
                Application.instance = new Application();
            }
            return Application.instance;
        }
    }

    //--------------------------------------------------------------------------
    // アプリケーションの起動
    //--------------------------------------------------------------------------
    public void start()
    {
        synchronized( Application.class )
        {
            if( this.isInitialized )
            {
                System.out.println("Application is already started.");
            }
            else
            {
                System.out.println("Application started.");
            }
            this.isInitialized = true;
        }
    }
}
```
