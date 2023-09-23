# 『Factory Method』ノート

（最終更新： 2023-09-23）


## 目次

1. [Factory Methodパターン](#factory-methodパターン)
	1. [Product](#product)
	1. [Creator](#creator)
	1. [ConcreteProduct](#concreteproduct)
	1. [ConcreteCreator](#concretecreator)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)

## Factory Methodパターン

**Factory Methodパターン**は、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成のための枠組みをあらかじめ[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)に定めておき、その具体的な実装を[サブクラス](../../../../programming/_/chapters/object_oriented.md#子クラス)に任せる[デザインパターン](./design_pattern.md#デザインパターン)。[Template Methodパターン](./template_method.md#template-methodパターン)を[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成の場面に適応させたパターン。

[コンストラクタ](../../../../programming/_/chapters/object_oriented.md#コンストラクタ)の呼び出しを抽象化することにより、生成される可能性のある複数種類の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブエジェクト)に適合した[プログラム](../../../../programming/_/chapters/programming.md#プログラム)を作成することが可能になる。また、[コンストラクタ](../../../../programming/_/chapters/object_oriented.md#コンストラクタ)の代わりとなるインタフェースを用意することで、毎回新しい[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成するだけでなく、キャッシュやプールを利用することも容易となる。

Factory Methodパターンは大きく分けて、[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)側（抽象的な骨組み、フレームワーク）の[Creator](#creator)および[Product](#product)と、[サブクラス](../../../../programming/_/chapters/object_oriented.md#子クラス)側（具体的な肉付け）の[ConcreteCreator](#concretecreator)および[ConcreteProduct](#concreteproduct)から構成される。

### Product

**Product**（製品）は、[Factory Methodパターン](#factory-methodパターン)のフレームワーク側で、[Creator](#creator)によって生成される[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)に共通する[API](../../../../computer/software/_/chapters/operating_system.md#api)を定める[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)や[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。

### Creator

**Creator**（作成者）は、[Factory Methodパターン](#factory-methodパターン)のフレームワーク側で、[Product](#product)役を生成する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を持つ[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)や[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。

Creatorは実際に生成される[ConcreteProduct](#concreteproduct)については何も知らず、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成のメソッドを呼び出せば[Product](#product)が生成されることのみ知っている。 `new` による実際の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成を、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)生成のための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)呼び出しで代替することで、具体的な[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)名による束縛から[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)を解放することができる。

### ConcreteProduct

**ConcreteProduct**（具体的製品）は、[Factory Methodパターン](#factory-methodパターン)の具体的な肉付けをする側で、[Product](#product)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。

### ConcreteCreator

**ConcreteCreator**（具体的作成者）は、[Factory Methodパターン](#factory-methodパターン)の具体的な肉付けをする側で、[Creator](#creator)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。ConcreteCreatorが生成する[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)([ConcreteProduct](#concreteproduct))は、[Product](#product)を[継承](../../../../programming/_/chapters/object_oriented.md#継承)もしくは実装している必要がある。


## サンプルプログラム

### Java

テキスト形式の文書を出力する[システム](../../../../system/_/chapters/system.md#システム)について、将来的に別形式の文書での出力に対応することを見据えた実装を考える。

```java
//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // 文書を生成
        DocumentFactory factory = new TextDocumentFactory();
        Document document = factory.create("Hello, world");
        System.out.println(document.getContent());
    }
}

//------------------------------------------------------------------------------
// Product
//------------------------------------------------------------------------------
abstract class Document
{
    public abstract String getContent();
}

//------------------------------------------------------------------------------
// Creator
//------------------------------------------------------------------------------
abstract class DocumentFactory
{
    public abstract Document create( String content );
}

//------------------------------------------------------------------------------
// ConcreteProduct
//------------------------------------------------------------------------------
class TextDocument extends Document
{
    private String content;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TextDocument( String content )
    {
        this.content = content;
    }

    //--------------------------------------------------------------------------
    // 内容を取得
    //--------------------------------------------------------------------------
    public String getContent()
    {
        return this.content;
    }
}

//------------------------------------------------------------------------------
// ConcreteCreator
//------------------------------------------------------------------------------
public class TextDocumentFactory extends DocumentFactory
{
    //--------------------------------------------------------------------------
    // 文書を生成
    //--------------------------------------------------------------------------
    public Document create( String content )
    {
        return new TextDocument(content);
    }
}
```
