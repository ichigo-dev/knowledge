# 『State』ノート

（最終更新： 2023-08-15）


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

時間を計測するストップウォッチの実装において、ストップウォッチの状態を[Stateパターン](#stateパターン)で管理するケースを考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // ストップウォッチによる時間の計測
        Stopwatch stopwatch = new Stopwatch();
        System.out.println(stopwatch.getTime());

        try
        {
            stopwatch.start();
            Thread.sleep(10);
            stopwatch.stop();
            System.out.println(stopwatch.getTime());

            stopwatch.start();
            Thread.sleep(20);
            stopwatch.stop();
            System.out.println(stopwatch.getTime());
        }
        catch( Exception e )
        {
            System.out.println(e);
        }

        stopwatch.reset();
        System.out.println(stopwatch.getTime());
    }
}

//------------------------------------------------------------------------------
// State
//------------------------------------------------------------------------------
interface StopwatchState
{
    public abstract void start( StopwatchContext context );
    public abstract void stop( StopwatchContext context );
    public abstract void reset( StopwatchContext context );
    public abstract int getTime( StopwatchContext context );
}

//------------------------------------------------------------------------------
// ConcreteState
//------------------------------------------------------------------------------
class InitialState implements StopwatchState
{
    private static InitialState instance;

    //--------------------------------------------------------------------------
    // コンストラクタをprivateに隠蔽
    //--------------------------------------------------------------------------
    private InitialState()
    {
    }

    //--------------------------------------------------------------------------
    // インスタンス取得
    //--------------------------------------------------------------------------
    public static StopwatchState getInstance()
    {
        if( InitialState.instance == null )
        {
            InitialState.instance = new InitialState();
        }
        return InitialState.instance;
    }

    //--------------------------------------------------------------------------
    // 記録の開始
    //--------------------------------------------------------------------------
    public void start( StopwatchContext context )
    {
        context.setState(MeasuringState.getInstance());
        context.setStartTime((int)System.currentTimeMillis());
    }

    //--------------------------------------------------------------------------
    // 記録の終了
    //--------------------------------------------------------------------------
    public void stop( StopwatchContext context )
    {
    }

    //--------------------------------------------------------------------------
    // 記録のリセット
    //--------------------------------------------------------------------------
    public void reset( StopwatchContext context )
    {
    }

    //--------------------------------------------------------------------------
    // 記録の取得
    //--------------------------------------------------------------------------
    public int getTime( StopwatchContext context )
    {
        return 0;
    }
}

class MeasuringState implements StopwatchState
{
    private static MeasuringState instance;

    //--------------------------------------------------------------------------
    // コンストラクタをprivateに隠蔽
    //--------------------------------------------------------------------------
    private MeasuringState()
    {
    }

    //--------------------------------------------------------------------------
    // インスタンス取得
    //--------------------------------------------------------------------------
    public static StopwatchState getInstance()
    {
        if( MeasuringState.instance == null )
        {
            MeasuringState.instance = new MeasuringState();
        }
        return MeasuringState.instance;
    }

    //--------------------------------------------------------------------------
    // 記録の開始
    //--------------------------------------------------------------------------
    public void start( StopwatchContext context )
    {
    }

    //--------------------------------------------------------------------------
    // 記録の終了
    //--------------------------------------------------------------------------
    public void stop( StopwatchContext context )
    {
        context.setState(ResultState.getInstance());
        context.setTime(this.calculateTime(context));
    }

    //--------------------------------------------------------------------------
    // 記録のリセット
    //--------------------------------------------------------------------------
    public void reset( StopwatchContext context )
    {
    }

    //--------------------------------------------------------------------------
    // 記録の取得
    //--------------------------------------------------------------------------
    public int getTime( StopwatchContext context )
    {
        return this.calculateTime(context);
    }

    //--------------------------------------------------------------------------
    // 記録の計算
    //--------------------------------------------------------------------------
    private int calculateTime( StopwatchContext context )
    {
        int now = (int)System.currentTimeMillis();
        return now - context.getStartTime() + context.getInnerTime();
    }
}

class ResultState implements StopwatchState
{
    private static ResultState instance;

    //--------------------------------------------------------------------------
    // コンストラクタをprivateに隠蔽
    //--------------------------------------------------------------------------
    private ResultState()
    {
    }

    //--------------------------------------------------------------------------
    // インスタンス取得
    //--------------------------------------------------------------------------
    public static StopwatchState getInstance()
    {
        if( ResultState.instance == null )
        {
            ResultState.instance = new ResultState();
        }
        return ResultState.instance;
    }

    //--------------------------------------------------------------------------
    // 記録の開始
    //--------------------------------------------------------------------------
    public void start( StopwatchContext context )
    {
        context.setState(MeasuringState.getInstance());
        context.setStartTime((int)System.currentTimeMillis());
    }

    //--------------------------------------------------------------------------
    // 記録の終了
    //--------------------------------------------------------------------------
    public void stop( StopwatchContext context )
    {
    }

    //--------------------------------------------------------------------------
    // 記録のリセット
    //--------------------------------------------------------------------------
    public void reset( StopwatchContext context )
    {
        context.setState(InitialState.getInstance());
        context.setTime(0);
    }

    //--------------------------------------------------------------------------
    // 記録の取得
    //--------------------------------------------------------------------------
    public int getTime( StopwatchContext context )
    {
        return context.getInnerTime();
    }
}

//------------------------------------------------------------------------------
// Context
//------------------------------------------------------------------------------
interface StopwatchContext
{
    public abstract void setState( StopwatchState state );
    public abstract int getInnerTime();
    public abstract int getStartTime();
    public abstract void setTime( int time );
    public abstract void setStartTime( int startTime );
    public abstract void start();
    public abstract void stop();
    public abstract void reset();
    public abstract int getTime();
}

//------------------------------------------------------------------------------
// Contextの実装
//------------------------------------------------------------------------------
class Stopwatch implements StopwatchContext
{
    private int time;
    private int startTime;
    private StopwatchState state;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Stopwatch()
    {
        this.state = InitialState.getInstance();
    }

    //--------------------------------------------------------------------------
    // 状態の設定
    //--------------------------------------------------------------------------
    public void setState( StopwatchState state )
    {
        this.state = state;
    }

    //--------------------------------------------------------------------------
    // 時間の取得
    //--------------------------------------------------------------------------
    public int getInnerTime()
    {
        return this.time;
    }

    //--------------------------------------------------------------------------
    // 開始時間の取得
    //--------------------------------------------------------------------------
    public int getStartTime()
    {
        return this.startTime;
    }

    //--------------------------------------------------------------------------
    // 時間の設定
    //--------------------------------------------------------------------------
    public void setTime( int time )
    {
        this.time = time;
    }

    //--------------------------------------------------------------------------
    // 開始時間の設定
    //--------------------------------------------------------------------------
    public void setStartTime( int startTime )
    {
        this.startTime = startTime;
    }

    //--------------------------------------------------------------------------
    // 記録の開始
    //--------------------------------------------------------------------------
    public void start()
    {
        this.state.start(this);
    }

    //--------------------------------------------------------------------------
    // 記録の終了
    //--------------------------------------------------------------------------
    public void stop()
    {
        this.state.stop(this);
    }

    //--------------------------------------------------------------------------
    // 記録のリセット
    //--------------------------------------------------------------------------
    public void reset()
    {
        this.state.reset(this);
    }

    //--------------------------------------------------------------------------
    // 記録の取得
    //--------------------------------------------------------------------------
    public int getTime()
    {
        return this.state.getTime(this);
    }
}
```
