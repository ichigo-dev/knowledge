# 『Mediator』ノート

（最終更新： 2023-08-09）


## 目次

1. [Mediatorパターン](#mediatorパターン)
	1. [Mediator](#mediator)
	1. [ConcreteMediator](#concretemediator)
	1. [Colleague](#colleague)
	1. [ConcreteColleague](#concretecolleague)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Mediatorパターン

**Mediatorパターン**は、複雑に関連し合う複数の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)間の関係に仲介者を設け、その仲介者を介して処理を行うようにすることで、単純かつ明快なインタフェースを提供する[デザインパターン](./design_pattern.md#デザインパターン)。仲介者は、管轄下にある複数の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)からの問い合わせを受けて、適宜判断を行い、管轄下にあるオブジェクト全体、または一部に指示を出す。

Mediatorパターンは、[Mediator](#mediator)、[ConcreteMediator](#concretemediator)、[Colleague](#colleague)、[ConcreteColleague](#concretecolleague)から構成される。

### Mediator

**Mediator**は、[Mediatorパターン](#mediatorパターン)において、[Colleague](#colleague)からの問い合わせを引き受けて、それをもとに判断を下し、[Colleague](#colleague)へ指示を出す役割を持つ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。各[Colleague](#colleague)からの相談受付の窓口となる[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)や、管轄下に置く[Colleague](#colleague)を格納するための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)などを定義する。

### ConcreteMediator

**ConcreteMediator**は、[Mediatorパターン](#mediatorパターン)において、[Mediator](#mediator)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。実際に[Colleague](#colleague)からの相談を受けて判断を下し、それらに指示を出す。

### Colleague

**Colleague**は、[Mediatorパターン](#mediatorパターン)において、他のColleagueと関連性のある[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を定義するための[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。他のCollegueを制御したい場合は、[Mediator](#mediator)に相談し、[Mediator](#mediator)からの指示を受けるための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)や、自身が相談する[Mediator](#mediator)を格納するための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)などを定義する。

### ConcreteColleague

**ConcreteColleague**は、[Mediatorパターン](#mediatorパターン)において、[Colleague](#colleague)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。


## サンプルプログラム

ユーザ間でメッセージのやり取りを行う[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の例を考える。

### Java

```java
import java.util.ArrayList;
import java.util.List;

//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // メッセージのやりとり
        ChatMediator mediator = new ChatMediator();
        User user1 = new User("Smith", mediator);
        User user2 = new User("Johnson", mediator);
        User user3 = new User("Williams", mediator);
        user1.sendMessage("Hello", user2);
        user3.sendMessage("world", user1);
    }
}

//------------------------------------------------------------------------------
// Mediator
//------------------------------------------------------------------------------
interface ChatMediatorInterface
{
    public abstract void sendMessage( String message, User target, User from );
    public abstract void addUser( User user );
}

//------------------------------------------------------------------------------
// ConcreteMediator
//------------------------------------------------------------------------------
class ChatMediator implements ChatMediatorInterface
{
    private List<User> users;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public ChatMediator()
    {
        this.users = new ArrayList<User>();
    }

    //--------------------------------------------------------------------------
    // メッセージ送信
    //--------------------------------------------------------------------------
    public void sendMessage( String message, User target, User from )
    {
        for( int i = 0; i < this.users.size(); i++ )
        {
            User user = this.users.get(i);
            if( user == target )
            {
                user.receiveMessage(message, from);
            }
        }
    }

    //--------------------------------------------------------------------------
    // ユーザ追加
    //--------------------------------------------------------------------------
    public void addUser( User user )
    {
        this.users.add(user);
    }
}

//------------------------------------------------------------------------------
// Colleague
//------------------------------------------------------------------------------
interface UserInterface
{
    public abstract void sendMessage( String message, User target );
    public abstract void receiveMessage( String message, User from );
}

//------------------------------------------------------------------------------
// ConcreteColleague
//------------------------------------------------------------------------------
class User implements UserInterface
{
    private String name;
    private ChatMediator mediator;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public User( String name, ChatMediator mediator )
    {
        this.name = name;
        this.mediator = mediator;
        mediator.addUser(this);
    }

    //--------------------------------------------------------------------------
    // 名前取得
    //--------------------------------------------------------------------------
    public String getName()
    {
        return this.name;
    }

    //--------------------------------------------------------------------------
    // メッセージ送信
    //--------------------------------------------------------------------------
    public void sendMessage( String message, User target )
    {
        this.mediator.sendMessage(message, target, this);
    }

    //--------------------------------------------------------------------------
    // メッセージ受信
    //--------------------------------------------------------------------------
    public void receiveMessage( String message, User from )
    {
        System.out.println
        (
            this.getName() +
            " received message '" +
            message +
            "' from " +
            from.getName()
        );
    }
}
```
