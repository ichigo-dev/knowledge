# 『Visitor』ノート

（最終更新： 2023-08-09）


## 目次

1. [Visitorパターン](#visitorパターン)
	1. [Visitor](#visitor)
	1. [ConcreteVisitor](#concretevisitor)
	1. [Acceptor](#acceptor)
	1. [ConcreteAcceptor](#concreteacceptor)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Visitorパターン

**Visitorパターン**は、データ構造とそれに対する処理を分離することを目的とした[デザインパターン](./design_pattern.md#デザインパターン)。訪問者となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が、データ構造の個々の要素を訪問し、その訪問先で公開されている資源を利用して処理を実行して回る。

Visitorパターンは、[Visitor](#visitor)、[ConcreteVisitor](#concretevisitor)、[Acceptor](#acceptor)、[ConcreteAcceptor](#concreteacceptor)から構成される。

### Visitor

**Visitor**（訪問者）は、[Visitorパターン](#visitorパターン)において、具体的なデータ構造の要素である[ConcreteAcceptor](#concreteacceptor)を訪問して処理を行う[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)構造の具象要素を引数として取る一連の訪問[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を宣言する。

### ConcreteVisitor

**ConcreteVisitor**（具体的訪問者）は、[Visitorパターン](#visitorパターン)において、[Visitor](#visitor)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。

### Acceptor

**Acceptor**（データ構造）は、[Visitorパターン](#visitorパターン)において、[Visitor](#visitor)の訪問先であるデータ構造要素に対する受入れ口となる[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)や[抽象クラス](../../../../programming/_/chapters/object_oriented.md#抽象クラス)。[Visitorパターン](#visitorパターン)を利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)は、Acceptorの受入れ用の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を介して[Visitor](#visitor)の訪問用の[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を呼び出す（**ダブルディスパッチ**）。

### ConcreteAcceptor

**ConcreteAcceptor**（具体的データ構造）は、[Visitorパターン](#visitorパターン)において、[Acceptor](#acceptor)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。


## サンプルプログラム

### Java

仮想的な[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)を表現し、[Visitorパターン](#visitorパターン)により各要素のレポートを出力したい例を考える。

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
        // 仮想ファイルシステムを構築
        DirectoryElement root = new DirectoryElement("root");
        DirectoryElement documents = new DirectoryElement("Documents");
        DirectoryElement images = new DirectoryElement("Images");
        FileElement textFile = new FileElement("document.txt");
        FileElement pngFile = new FileElement("image.png");

        documents.addChild(textFile);
        images.addChild(pngFile);
        root.addChild(documents);
        root.addChild(images);

        // Visitorで内容を描画
        FileSystemVisitor visitor = new FilePrinterVisitor();
        root.accept(visitor, 0);
    }
}

//------------------------------------------------------------------------------
// Visitor
//------------------------------------------------------------------------------
interface FileSystemVisitor
{
    public abstract void visit( DirectoryElement directory, int depth );
    public abstract void visit( FileElement file, int depth );
}

//------------------------------------------------------------------------------
// ConcreteVisitor
//------------------------------------------------------------------------------
class FilePrinterVisitor implements FileSystemVisitor
{
    private int displayIndent;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public FilePrinterVisitor()
    {
        this.displayIndent = 4;
    }

    //--------------------------------------------------------------------------
    // 訪問メソッド
    //--------------------------------------------------------------------------
    public void visit( DirectoryElement directory, int depth )
    {
        String indent = " ".repeat(this.displayIndent * depth);
        System.out.println(indent + directory.getName() + "/");
    }

    public void visit( FileElement file, int depth )
    {
        String indent = " ".repeat(this.displayIndent * depth);
        System.out.println(indent + file.getName());
    }
}

//------------------------------------------------------------------------------
// Acceptor
//------------------------------------------------------------------------------
abstract class FileSystemElement
{
    protected String name;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public FileSystemElement( String name )
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

    public abstract void accept(FileSystemVisitor visitor, int depth);
}

//------------------------------------------------------------------------------
// ConcreteAcceptor
//------------------------------------------------------------------------------
class DirectoryElement extends FileSystemElement
{
    private List<FileSystemElement> children;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public DirectoryElement( String name )
    {
        super(name);
        this.children = new ArrayList<FileSystemElement>();
    }

    //--------------------------------------------------------------------------
    // 子要素を追加
    //--------------------------------------------------------------------------
    public void addChild( FileSystemElement child )
    {
        this.children.add(child);
    }

    //--------------------------------------------------------------------------
    // 子要素を取得
    //--------------------------------------------------------------------------
    public List<FileSystemElement> getChildren()
    {
        return this.children;
    }

    //--------------------------------------------------------------------------
    // Visitorの受け入れ
    //--------------------------------------------------------------------------
    public void accept( FileSystemVisitor visitor, int depth )
    {
        visitor.visit(this, depth);

        // 子要素にもVisitorを適用
        for( int i = 0; i < this.children.size(); i++ )
        {
            FileSystemElement element = this.children.get(i);
            element.accept(visitor, depth + 1);
        }
    }
}

class FileElement extends FileSystemElement
{
    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public FileElement( String name )
    {
        super(name);
    }

    //--------------------------------------------------------------------------
    // Visitorの受け入れ
    //--------------------------------------------------------------------------
    public void accept( FileSystemVisitor visitor, int depth )
    {
        visitor.visit(this, depth);
    }
}
```
