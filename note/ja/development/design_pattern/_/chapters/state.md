# 『State』ノート

（最終更新： 2023-08-13）


## 目次

1. [Stateパターン](#stateパターン)
	1. [State](#state)
	1. [ConcreteState](#concretestate)
	1. [Context](#context)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Stateパターン

**Stateパターン**は、ある者についての各状態を[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)で表現する[デザインパターン](./design_pattern.md#デザインパターン)。通常は条件（状態）に一致するか否かの処理は単純な[条件分岐](../../../../programming/_/chapters/control_flow.md#条件分岐)により実装可能であるが、それが複雑な条件となる場合や、同じ[条件分岐](../../../../programming/_/chapters/control_flow.md#条件分岐)を複数個所で繰り返し利用するような場合、メンテナンス性を向上させるためにこのパターンが用いられる。ただし、状態の数が少ない、あるいは状態の更新頻度が低い場合にはこのパターンの適用は過剰な可能性がある。

Stateパターンは、[State](#state)、[ConcreteState](#concretestate)、[Context](#context)から構成される。

### State

**State**（状態）は、[Stateパターン](#stateパターン)において、状態そのものを表す[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。状態ごとに振る舞いが異なるような[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義する。

### ConcreteState

**ConcreteState**（具体的な状態）は、[Stateパターン](#stateパターン)において、[State](#state)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。具体的な状態を1[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)につき1状態で表し、1つの状態を表すのに複数の[インスタンス](../../../../programming/_/chapters/object_oriented.md#クラス)は必要ないため、[Singletonパターン](./singleton.md#singletonパターン)を適用する。

### Context

**Context**（状況判断）は、[Stateパターン](#stateパターン)において、現在の状態を保持し、このパターンを利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)へのインタフェースを定義する。状態を変更する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を持ち、その状態に固有の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)は全てContextを介して呼び出される。


## サンプルプログラム

### Java

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
    }
}

//------------------------------------------------------------------------------
// State
//------------------------------------------------------------------------------
abstract class InputMode
{
    protected TextEditor editor;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public InputMode( TextEditor editor )
    {
        this.editor = editor;
    }

    public abstract void appendText( String text );
}

//------------------------------------------------------------------------------
// ConcreteState
//------------------------------------------------------------------------------
class NormalMode extends InputMode
{
    //--------------------------------------------------------------------------
    // コンストラクタを隠蔽
    //--------------------------------------------------------------------------
    private NormalMode( TextEditor editor )
    {
        super(editor);
    }

    //--------------------------------------------------------------------------
    // 文字列の追加
    //--------------------------------------------------------------------------
    public void appendText( String text )
    {
        this.editor.appendText(text);
    }
}

//------------------------------------------------------------------------------
// Context
//------------------------------------------------------------------------------
class TextEditor
{
    private State state;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TextEditor()
    {
        this.state = 
    }
}
```
