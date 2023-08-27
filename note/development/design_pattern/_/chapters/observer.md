# 『Observer』ノート

（最終更新： 2023-08-10）


## 目次

1. [Observerパターン](#observerパターン)
	1. [Subject](#subject)
	1. [ConcreteSubject](#concretesubject)
	1. [Observer](#observer)
	1. [ConcreteObserver](#concreteobserver)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Observerパターン

**Observerパターン**は、観察者となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が、観察対象となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)からの状態変化の通知を受けて、それに対する処理を行う[デザインパターン](./design_pattern.md#デザインパターン)。通知を発行する[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)と、それを受け取る[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)からなり、発行者は通知したい[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を登録（サブスクライブ）する必要がある。

Observerパターンは、[Subject](#subject)、[ConcreteSubject](#concretesubject)、[Observer](#observer)、[ConcreteObserver](#concreteobserver)から構成される。

### Subject

**Subject**（観察対象者）は、[Observerパターン](#observerパターン)において、[Observer](#observer)の観察対象となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。[Observer](#observer)を保持する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)、[Observer](#observer)への通知[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)などを定義する。[Observer](#observer)は複数保持していても良い。

### ConcreteSubject

**ConcreteSubject**（具体的な観察対象者）は、[Observerパターン](#observerパターン)において、[Subject](#subject)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Observer](#observer)への通知は、自身に保持している[Observer](#observer)[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を介して行う。

### Observer

**Observer**（観察者）は、[Observerパターン](#observerパターン)において、[Subject](#subject)の状態変化を監視するための[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。[Subject](#subject)からの通知を受信するための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義する。

### ConcreteObserver

**ConcreteObserver**（具体的な観察者）は、[Observerパターン](#observerパターン)において、[Observer](#observer)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Subject](#subject)からの状態受信用の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)が呼ばれると、その呼び出し元の状態をもとに処理を行う。


## サンプルプログラム

### Java

[コンポーネント](../../../../computer/software/_/chapters/package.md#コンポーネント)の変化を監視し、イベントが発生したときに処理を実行する例を考える。

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
        // コンポーネントの変化をオブザーバで監視
        EventObserver displayEventObserver = new DisplayEventObserver();
        EventObserver clickEventObserver = new ClickEventObserver();

        TextFieldComponent textField = new TextFieldComponent();
        textField.addObserver(displayEventObserver);

        ButtonComponent button = new ButtonComponent();
        button.addObserver(displayEventObserver);
        button.addObserver(clickEventObserver);

        textField.input("Hello, world!");
        button.click();
    }
}

//------------------------------------------------------------------------------
// Subject
//------------------------------------------------------------------------------
abstract class Component
{
    private List<EventObserver> observers;
    protected String value;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Component()
    {
        this.observers = new ArrayList<>();
    }

    //--------------------------------------------------------------------------
    //  オブザーバの追加
    //--------------------------------------------------------------------------
    public void addObserver( EventObserver observer )
    {
        this.observers.add(observer);
    }

    //--------------------------------------------------------------------------
    //  オブザーバの削除
    //--------------------------------------------------------------------------
    public void removeObserver( EventObserver observer )
    {
        this.observers.remove(observer);
    }

    //--------------------------------------------------------------------------
    //  オブザーバの削除
    //--------------------------------------------------------------------------
    public void notifyObservers()
    {
        for( int i = 0; i < this.observers.size(); i++ )
        {
            this.observers.get(i).update(this);
        }
    }

    //--------------------------------------------------------------------------
    //  値の取得
    //--------------------------------------------------------------------------
    public String getValue()
    {
        return this.value;
    }
}

//------------------------------------------------------------------------------
// ConcreteSubject
//------------------------------------------------------------------------------
class TextFieldComponent extends Component
{
    //--------------------------------------------------------------------------
    // 値の入力イベント
    //--------------------------------------------------------------------------
    public void input( String value )
    {
        this.value = value;
        this.notifyObservers();
    }
}

class ButtonComponent extends Component
{
    //--------------------------------------------------------------------------
    // ボタン押下イベント
    //--------------------------------------------------------------------------
    public void click()
    {
        this.value = "clicked";
        this.notifyObservers();
    }
}

//------------------------------------------------------------------------------
// Observer
//------------------------------------------------------------------------------
interface EventObserver
{
    public abstract void update( Component component );
}

//------------------------------------------------------------------------------
// ConcreteObserver
//------------------------------------------------------------------------------
class DisplayEventObserver implements EventObserver
{
    //--------------------------------------------------------------------------
    // 更新時処理
    //--------------------------------------------------------------------------
    public void update( Component component )
    {
        System.out.println("DisplayEventObserver: " + component.getValue());
    }
}

class ClickEventObserver implements EventObserver
{
    //--------------------------------------------------------------------------
    // 更新時処理
    //--------------------------------------------------------------------------
    public void update( Component component )
    {
        System.out.println("ClickEventObserver: clicked");
    }
}
```
