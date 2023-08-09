# 『Chain of Responsibility』ノート

（最終更新： 2023-08-09）


## 目次

1. [Chain of Responsibilityパターン](#chain-of-responsibilityパターン)
	1. [Handler](#handler)
	1. [ConcreteHandler](#concretehandler)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Chain of Responsibilityパターン

**Chain of Responsibilityパターン**は、ある要求の受け取り対象となる複数の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を鎖のように繋ぎ、要求が発生した際に各[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を渡り歩いて、いずれかの段階で処理されることを表現する[デザインパターン](./design_pattern.md#デザインパターン)。このパターンを利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)は、要求を処理する[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の存在を意識することなく、連鎖関係の一番最初にいる（受付となる）[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に対して要求を投げればよくなるため、役割の分離がしやすくなる。

Chain of Responsibilityパターンは、[Handler](#handler)、[ConcreteHandler](#concretehandler)から構成される。

### Handler

**Handler**は、[Chain of Responsibilityパターン](#chain-of-responsibility)において、要求を処理する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義した[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)や[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)。自身で処理できない要求の場合に受け流す先（[ConcreteHandler](#concretehandler)）を[フィールド](../../../../programming/_/chapters/object_oriented.md#フィールド)として持つ。

### ConcreteHandler

**ConcreteHandler**は、[Chain of Responsibilityパターン](#chain-of-responsibility)において、[Handler](#handler)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。実際に要求を自身で処理するか、次の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に要求を渡す。


## サンプルプログラム

### Java

任意の[ミドルウェア](../../../../computer/software/_/chapters/middleware.md#ミドルウェア)を設定できるような[サーバ](../../../../computer/_/chapters/computer.md#サーバ)の実装を考える。

```java
import java.util.HashMap;
import java.util.Map;

//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // サーバの初期化
        Server server = new Server();
        server.register("admin@example.com", "password");
        server.register("user@example.com", "password");

        // ミドルウェアの設定
        Middleware middleware = Middleware.init
        (
            new ThrottlingMiddleware(2),
            new AuthMiddleware(server),
            new RoleCheckMiddleware()
        );
        server.setMiddleware(middleware);

        // ログイン処理
        server.login("user@example.com", "bad_password");
        server.login("admin@example.com", "password");
        server.login("user@example.com", "password");
    }
}

//------------------------------------------------------------------------------
// Handler
//------------------------------------------------------------------------------
abstract class Middleware
{
    private Middleware next;

    //--------------------------------------------------------------------------
    // ミドルウェアの初期化
    //--------------------------------------------------------------------------
    public static Middleware init( Middleware first, Middleware... chain )
    {
        Middleware head = first;
        for( int i = 0; i < chain.length; i++ )
        {
            head.next = chain[i];
            head = chain[i];
        }
        return first;
    }

    //--------------------------------------------------------------------------
    // 次のハンドラによるチェック
    //--------------------------------------------------------------------------
    protected boolean checkNext( String email, String password )
    {
        if( next == null )
        {
            return true;
        }
        return next.check(email, password);
    }

    public abstract boolean check( String email, String password );
}

//------------------------------------------------------------------------------
// ConcreteHandler
//------------------------------------------------------------------------------
class ThrottlingMiddleware extends Middleware
{
    private int requestPerMinute;
    private int requestCnt;
    private long currentTime;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public ThrottlingMiddleware( int requestPerMinute )
    {
        this.requestPerMinute = requestPerMinute;
        this.currentTime = System.currentTimeMillis();
    }

    //--------------------------------------------------------------------------
    // チェック
    //--------------------------------------------------------------------------
    public boolean check( String email, String password )
    {
        if( System.currentTimeMillis() > this.currentTime + 60000 )
        {
            this.requestCnt = 0;
            this.currentTime = System.currentTimeMillis();
        }

        this.requestCnt++;

        if( this.requestCnt > this.requestPerMinute )
        {
            System.out.println("Request limit exceeded.");
            return false;
        }
        return this.checkNext(email, password);
    }
}

class AuthMiddleware extends Middleware
{
    private Server server;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public AuthMiddleware( Server server )
    {
        this.server = server;
    }

    //--------------------------------------------------------------------------
    // チェック
    //--------------------------------------------------------------------------
    public boolean check( String email, String password )
    {
        if( this.server.hasEmail(email) == false )
        {
            System.out.println("This email is not registered.");
            return false;
        }
        if( this.server.isValidPassword(email, password) == false )
        {
            System.out.println("Wrong password.");
            return false;
        }
        return this.checkNext(email, password);
    }
}

class RoleCheckMiddleware extends Middleware
{
    //--------------------------------------------------------------------------
    // チェック
    //--------------------------------------------------------------------------
    public boolean check( String email, String password )
    {
        if( email.equals("admin@example.com") )
        {
            System.out.println("Administrator logined.");
            return true;
        }
        System.out.println("User logined.");
        return checkNext(email, password);
    }
}

//------------------------------------------------------------------------------
// サーバ
//------------------------------------------------------------------------------
class Server
{
    private Map<String, String> users;
    private Middleware middleware;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Server()
    {
        this.users = new HashMap<String, String>();
    }

    //--------------------------------------------------------------------------
    // ミドルウェアの設定
    //--------------------------------------------------------------------------
    public void setMiddleware( Middleware middleware )
    {
        this.middleware = middleware;
    }

    //--------------------------------------------------------------------------
    // ログイン
    //--------------------------------------------------------------------------
    public boolean login( String email, String password )
    {
        if( this.middleware.check(email, password) )
        {
            return true;
        }
        return false;
    }

    //--------------------------------------------------------------------------
    // ユーザ登録
    //--------------------------------------------------------------------------
    public void register( String email, String password )
    {
        this.users.put(email, password);
    }

    //--------------------------------------------------------------------------
    // メールアドレスの登録チェック
    //--------------------------------------------------------------------------
    public boolean hasEmail( String email )
    {
        return this.users.containsKey(email);
    }

    //--------------------------------------------------------------------------
    // パスワードチェック
    //--------------------------------------------------------------------------
    public boolean isValidPassword( String email, String password )
    {
        return this.users.get(email).equals(password);
    }
}
```
