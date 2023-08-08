# 『Strategy』ノート

（最終更新： 2023-08-08）


## 目次

1. [Strategyパターン](#strategyパターン)
	1. [Strategy](#strategy)
	1. [ConcreteStrategy](#concretestrategy)
	1. [Context](#context)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Strategyパターン

**Strategyパターン**は、同じ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を実装する交換可能な[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)を複数用意しておき、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)実行時に適切なものを選択できるようにする[デザインパターン](./design_pattern.md#デザインパターン)。[条件分岐](../../../../programming/_/chapters/control_flow.md#条件分岐)などによって[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)に溶け込むような形で[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)を実装しがちであるが、Strategyパターンを利用すればより柔軟でメンテナンスのしやすい設計にすることができる。

Strategyパターンは、[Strategy](#strategy)、[ConcreteStrategy](#concretestrategy)、[Context](#context)から構成される。

### Strategy

**Strategy**（戦略）は、[Strategyパターン](#strategyパターン)において、[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)を実装する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定める[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。すべての[ConcreteStrategy](#concretestrategy)に共通の[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)であり、[Context](#context)が戦略を実行するために使用する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を宣言する。

### ConcreteStrategy

**ConcreteStrategy**（具体的戦略）は、[Strategyパターン](#strategyパターン)において、[Strategy](#strategy)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を実装し、実際の[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)を持った[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Strategyパターン](#strategyパターン)ではConcreteStrategyが複数用意され、[プログラム](../../../../programming/_/chapters/programming.md#プログラム)は用いる戦略を[Context](#context)を通じて容易に切り替えることができる。

### Context

**Context**（文脈）は、[Strategyパターン](#strategyパターン)において、[ConcreteStrategy](#concretestrategy)の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を持ち、必要に応じてその[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)が持つアルゴリズムを利用する役。Contextが呼び出すのは、[Strategy](#strategy)の[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)に定義された[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)のみであり、それぞれの[ConcreteStrategy](#concretestrategy)に依存した実装にはなっていない。


## サンプルプログラム

### Java

ソート[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)を柔軟に選択できるようなユーティリティの実装を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // 配列のソート
        Integer[] array = { 5, 1, 2, 8, 9, 3, 4, 6, 0, 7 };
        SortStrategy<Integer> strategy = new QuickSort<>();
        SortContext<Integer> context = new SortContext<>(strategy);
        context.sort(array);
        for( int i = 0; i < array.length; i++ )
        {
            System.out.println(array[i]);
        }
    }
}

//------------------------------------------------------------------------------
// Strategy
//------------------------------------------------------------------------------
interface SortStrategy<T extends Comparable<T>>
{
    public abstract void sort( T[] array );
}

//------------------------------------------------------------------------------
// ConcreteStrategy
//------------------------------------------------------------------------------
class BubbleSort<T extends Comparable<T>> implements SortStrategy<T>
{
    //--------------------------------------------------------------------------
    // ソート
    //--------------------------------------------------------------------------
    public void sort( T[] array )
    {
        int n = array.length;
        for( int i = 0; i < n - 1; i++ )
        {
            for( int j = 0; j < n - i - 1; j++ )
            {
                if( array[j].compareTo(array[j + 1]) > 0 )
                {
                    // スワップ
                    T temp = array[j];
                    array[j] = array[j + 1];
                    array[j + 1] = temp;
                }
            }
        }
    }
}

class QuickSort<T extends Comparable<T>> implements SortStrategy<T>
{
    //--------------------------------------------------------------------------
    // ソート
    //--------------------------------------------------------------------------
    public void sort( T[] array )
    {
        this.sort(array, 0, array.length - 1);
    }

    public void sort( T[] array, int low, int high )
    {
        if( low < high )
        {
            int pivotIndex = this.partition(array, low, high);

            // ピボットの左右を再帰的にソート
            this.sort(array, low, pivotIndex - 1);
            this.sort(array, pivotIndex + 1, high);
        }
    }

    //--------------------------------------------------------------------------
    // ピボット
    //--------------------------------------------------------------------------
    public int partition( T[] array, int low, int high )
    {
        // 最後の要素をピボットとする
        T pivot = array[high];
        int i = low - 1;

        // ピボットより小さい要素を左側に移動
        for( int j = low; j < high; j++ )
        {
            if( array[j].compareTo(pivot) < 0 )
            {
                i++;

                // スワップ
                T temp = array[i];
                array[i] = array[j];
                array[j] = temp;
            }
        }

        // ピボットの位置調整
        T temp = array[i + 1];
        array[i + 1] = array[high];
        array[high] = temp;

        // ピボットのインデックスを返却
        return i + 1;
    }
}

class MergeSort<T extends Comparable<T>> implements SortStrategy<T>
{
    //--------------------------------------------------------------------------
    // ソート
    //--------------------------------------------------------------------------
    public void sort( T[] array )
    {
        this.sort(array, 0, array.length - 1);
    }

    public void sort( T[] array, int low, int high )
    {

    }

    //--------------------------------------------------------------------------
    // マージ
    //--------------------------------------------------------------------------
    public void merge( T[] array, int low, int mid, int high )
    {

    }
}

//------------------------------------------------------------------------------
// Context
//------------------------------------------------------------------------------
class SortContext<T extends Comparable<T>>
{
    private SortStrategy<T> strategy;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public SortContext( SortStrategy<T> strategy )
    {
        this.strategy = strategy;
    }

    //--------------------------------------------------------------------------
    // ソート
    //--------------------------------------------------------------------------
    public void sort( T[] array )
    {
        this.strategy.sort(array);
    }
}
```
