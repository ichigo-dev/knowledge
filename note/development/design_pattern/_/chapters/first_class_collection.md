# 『First Class Collection』ノート

（最終更新： 2023-10-05）


## 目次

1. [First Class Collectionパターン](#first-class-collectionパターン)
	1. [FirstClassCollection](#firstclasscollection)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## First Class Collectionパターン

**First Class Collectionパターン**は、[プログラミング言語](../../../../programming/_/chapters/programming.md#プログラミング言語)で提供されている、組み込みの[リスト](../../../../programming/_/chapters/data_type.md#リスト)や[ハッシュテーブル](../../../../programming/_/chapters/data_type.md#ハッシュテーブル)などの[コンテナ型](../../../../programming/_/chapters/data_type.md#コンテナ型)を[プリミティブ](../../../../programming/_/chapters/data_type.md#プリミティブ型)とみなし、それをラップした[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)を実装する[デザインパターン](./design_pattern.md#デザインパターン)。[ソースコード](../../../../programming/_/chapters/programming.md#ソースコード)のあちこちに[コンテナ型](../../../../programming/_/chapters/data_type.md#コンテナ型)に関する操作が散らばることを防いだり、許可する操作を限定することで、不具合の発生を防止する目的で利用される。

First Class Collectionパターンは[FirstClassCollection](#firstclasscollection)の役のみで構成される。

### FirstClassCollection

**FirstClassCollection**は、[First Class Collectionパターン](#first-class-collectionパターン)において、既存の[コンテナ型](../../../../programming/_/chapters/data_type.md#コンテナ型)を包み込んだ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。


## サンプルプログラム

### Java

```java
import java.util.Deque;
import java.util.ArrayDeque;

//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        SearchHistory hist = new SearchHistory();

        hist.push("hello");
        hist.push("world");
        hist.push("keyword");

        System.out.println(hist.pop());
        System.out.println(hist.pop());
    }
}

//------------------------------------------------------------------------------
// FirstClassCollection
//------------------------------------------------------------------------------
class SearchHistory
{
    Deque<String> histories;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public SearchHistory()
    {
        this.histories = new ArrayDeque<String>();
    }

    //--------------------------------------------------------------------------
    // 履歴の追加
    //--------------------------------------------------------------------------
    public void push( String word )
    {
        this.histories.push(word);
    }

    //--------------------------------------------------------------------------
    // 履歴の取り出し
    //--------------------------------------------------------------------------
    public String pop()
    {
        return this.histories.pop();
    }
}
```
