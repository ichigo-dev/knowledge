# 『Composite』ノート

（最終更新： 2023-08-08）


## 目次

1. [Compositeパターン](#compositeパターン)
	1. [Component](#component)
	1. [Leaf](#leaf)
	1. [Composite](#composite)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Compositeパターン

**Compositeパターン**は、[木構造](../../../../basics/applied_mathematics/_/chapters/graph_theory.md#木)を持つデータに対して再帰的な処理を行うために、容器と中身を同一視する[デザインパターン](./design_pattern.md#デザインパターン)。例えば、[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)において、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)と[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)は、どちらも[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)の中に入ることができるもの（ディレクトリエントリ）としてまとめることができる。このとき、[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)はディレクトリエントリを入れるための容器でもあり、ディレクトリエントリ自身であることもできるため、容器と中身が同一視されている。

Compositeパターンは、[Component](#component)、[Leaf](#leaf)、[Composite](#composite)から構成される。

### Component

**Component**は、[Compositeパターン](#compositeパターン)において、[Leaf](#leaf)と[Composite](#composite)の役を同一視するための抽象化構造。[Leaf](#leaf)と[Composite](#composite)の共通の[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)や[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)として実現する。

### Leaf

**Leaf**（葉）は、[Compositeパターン](#compositeパターン)において、中身のものを表す役で、中に他のものを入れることはできない。[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)における[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)はこの役の実装と言える。Leafは終端となる[Component](#component)であるため、実際の処理はほとんどの場合ここで実行される。

### Composite

**Composite**（複合体、コンテナ）は、[Compositeパターン](#compositeパターン)において、容器を表す役で、中には[Leaf](#leaf)や他のCompositeを入れることができる。[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)における[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)はこの役の実装と言える。Compositeは処理の要求を受けると、[Component](#component)の[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を介して子に処理を委任する。


## サンプルプログラム

### Java

JSONをコンポーネントとして表現し、それを表示する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)を考える。

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
        // JSONコンポーネントを作成して表示
        JsonComposite root = new JsonComposite("root");

        JsonComposite person = new JsonComposite("person");
        person.add(new JsonLeaf("name", "Smith"));
        person.add(new JsonLeaf("age", "30"));

        JsonComposite address = new JsonComposite("address");
        address.add(new JsonLeaf("streat", "123 Main St"));
        address.add(new JsonLeaf("city", "New York"));

        person.add(address);
        root.add(person);

        root.display(0);
    }
}

//------------------------------------------------------------------------------
// Component
//------------------------------------------------------------------------------
abstract class JsonComponent
{
    protected int displayIndent = 4;
    public abstract void display( int depth );
}

//------------------------------------------------------------------------------
// Leaf
//------------------------------------------------------------------------------
class JsonLeaf extends JsonComponent
{
    private String key;
    private String value;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public JsonLeaf( String key, String value )
    {
        this.key = key;
        this.value = value;
    }

    //--------------------------------------------------------------------------
    // 表示
    //--------------------------------------------------------------------------
    public void display( int depth )
    {
        String indent = " ".repeat(this.displayIndent * depth);
        System.out.print(indent + "\"" + key + "\": \"" + value + "\"");
    }
}

//------------------------------------------------------------------------------
// Composite
//------------------------------------------------------------------------------
class JsonComposite extends JsonComponent
{
    private String key;
    private List<JsonComponent> children;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public JsonComposite( String key )
    {
        this.key = key;
        this.children = new ArrayList<>();
    }

    //--------------------------------------------------------------------------
    // リーフを追加
    //--------------------------------------------------------------------------
    public void add( JsonComponent component )
    {
        this.children.add(component);
    }

    //--------------------------------------------------------------------------
    // 表示
    //--------------------------------------------------------------------------
    public void display( int depth )
    {
        String indent = " ".repeat(this.displayIndent * depth);
        System.out.println(indent + "\"" + key + "\": {");
        for( int i = 0; i < this.children.size(); i++ )
        {
            // 子要素はComositeでもLeafでもdisplayを実装しているので、それを呼び
            // 出す（処理を委任）
            JsonComponent component = this.children.get(i);
            component.display(depth + 1);
            if( i < this.children.size() - 1 )
            {
                System.out.println(",");
            }
        }
        System.out.println("\n" + indent + "}");
    }
}
```
