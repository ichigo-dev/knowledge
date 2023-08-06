# 『Iterator』ノート

（最終更新： 2023-08-06）


## 目次

1. [Iteratorパターン](#iteratorパターン)
	1. [Aggregate](#aggregate)
	1. [Iterator](#iterator)
	1. [ConcreteAggregate](#concreteaggregate)
	1. [ConcreteIterator](#concreteiterator)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Iteratorパターン

**Iteratorパターン**は、集合体の要素に順番にアクセスし、全体をスキャンしていくような処理を行うための[デザインパターン](./design_pattern.md#デザインパターン)。iterateという英単語には何かを「繰り返す」という意味があり、Iteratorは日本語で**反復子**とも呼ばれる。

Iteratorパターンは、集合体とアクセス方法を分離することで、集合体の内部構造（[リスト](../../../../programming/_/chapters/data_type.md#リスト)や[スタック](../../../../programming/_/chapters/data_type.md#スタック)、[ツリー](../../../../programming/_/chapters/data_type.md#木)など）に依存せずに要素にアクセスすることができるようにすることを目的としている。このパターンを利用することで得られるメリットとして、次のようなものが考えられる。

- 集合体の内部構造を知る必要がなく、簡潔に[コード](../../../../programming/_/chapters/programming.md#ソースコード)を記述することができる
- 要素を順番に取り出すことができるため、検索や並び替えなど、要素に対する様々な操作が行いやすくなる
- 複数の集合体で同じ[Iterator](#iterator)を使用することができ、[コード](../../../../programming/_/chapters/programming.md#ソースコード)の再利用性が高まる
- 複数の[Iterator](#iterator)が同じ[コレクション](../../../../programming/_/chapters/data_type.mdコンテナ型)を互いに独立して探索することができる

Iteratorパターンは、[Aggregate](#aggregate)、[ConcreteAggregate](#concreteaggregate)、[Iterator](#iterator)、[ConcreteIterator](#concreteiterator)から構成される。このパターンを利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)は、[ConcreteAggregate](#concreteaggregate)や[ConcreteIterator](#concreteiterator)に依存することがなくなり、具体的な[反復処理](../../../../programming/_/chapters/control_flow.md#反復)の[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)を置き換えることができる。

### Aggregate

**Aggregate**（集合体）は、[Iteratorパターン](#iteratorパターン)において、[Iterator](#iterator)インスタンスを生成する役割を持つ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。Aggregateは集合体そのものの役割を持っており、[Iterator](#iterator)[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成するような[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を持つ。

### Iterator

**Iterator**（反復子）は、[Iteratorパターン](#iteratorパターン)において、集合体に含まれる要素を順番に取り出す責任を持つ[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。次の要素や前の要素にアクセスするための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)や、要素の存在チェックを行う[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)、要素を削除する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)などを持つ。

### ConcreteAggregate

**ConcreteAggregate**（具体的な集合体）は、[Iteratorパターン](#iteratorパターン)において、[Aggregate](#aggregate)を実装した具体的な[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。このパターンを利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)からの要求があるたびに、新しい[ConcreteIterator](#concreteiterator)の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を返す。

### ConcreteIterator

**ConcreteIterator**（具体的な反復子）は、[Iteratorパターン](#iteratorパターン)において、[Iterator](#iterator)を実装した具体的な[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Aggregate](#aggregate)を探索するための特定の[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)を実装し、探索の進行状況を把握している。


## サンプルプログラム

### Java

ある部署（[Aggregate](#aggregate)）に所属する従業員を順番に取得する[Iterator](#iterator)の実装を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // 集合体の生成
        Department department = new Department(5);

        department.append(new Employee("Smith"));
        department.append(new Employee("Johnson"));
        department.append(new Employee("Williams"));
        department.append(new Employee("Brown"));
        department.append(new Employee("Jones"));

        // Iteratorを生成し、順次要素を取り出して処理を実行
        Iterator<Employee> iterator = department.iterator();
        while( iterator.hasNext() )
        {
            Employee employee = (Employee)iterator.next();
            System.out.println(employee.getName());
        }
    }
}

//------------------------------------------------------------------------------
// Aggregate
//------------------------------------------------------------------------------
interface Aggregate<T>
{
    public abstract Iterator<T> iterator();
}

//------------------------------------------------------------------------------
// Iterator
//------------------------------------------------------------------------------
interface Iterator<T>
{
    public abstract boolean hasNext();
    public abstract T next();
}

//------------------------------------------------------------------------------
// Employee（具体的な集合体の要素）
//------------------------------------------------------------------------------
class Employee
{
    private String name;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Employee( String name )
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
}

//------------------------------------------------------------------------------
// Department (ConcreteAggregate)
//------------------------------------------------------------------------------
class Department implements Aggregate<Employee>
{
    private Employee[] employees;
    private int last;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Department( int maxsize )
    {
        this.employees = new Employee[maxsize];
    }

    //--------------------------------------------------------------------------
    // インデックス指定で従業員を取得
    //--------------------------------------------------------------------------
    public Employee get( int index )
    {
        return this.employees[index];
    }

    //--------------------------------------------------------------------------
    // 従業員を追加
    //--------------------------------------------------------------------------
    public void append( Employee employee )
    {
        this.employees[this.last] = employee;
        this.last++;
    }

    //--------------------------------------------------------------------------
    // 従業員数を取得
    //--------------------------------------------------------------------------
    public int length()
    {
        return this.last;
    }

    //--------------------------------------------------------------------------
    // イテレータを取得
    //--------------------------------------------------------------------------
    public Iterator<Employee> iterator()
    {
        return new DepartmentIterator(this);
    }
}

//------------------------------------------------------------------------------
// DepartmentIterator (ConcreteIterator)
//------------------------------------------------------------------------------
class DepartmentIterator implements Iterator<Employee>
{
    private Department department;
    private int index;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public DepartmentIterator( Department department )
    {
        this.department = department;
    }

    //--------------------------------------------------------------------------
    // 次の要素があるかチェック
    //--------------------------------------------------------------------------
    public boolean hasNext()
    {
        return index < this.department.length();
    }

    //--------------------------------------------------------------------------
    // 次の要素を取得
    //--------------------------------------------------------------------------
    public Employee next()
    {
        Employee employee = this.department.get(this.index);
        this.index++;
        return employee;
    }
}
```
