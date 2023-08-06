# 『Template Method』ノート

（最終更新： 2023-08-06）


## 目次

1. [Template Methodパターン](#template-methodパターン)
	1. [AbstractClass](#abstractclass)
	1. [ConcreteClass](#concreteclass)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Template Methodパターン

**Template Methodパターン**は、ある処理の大まかな[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)をあらかじめ[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)に定めておき、その具体的な設計を[サブクラス](../../../../programming/_/chapters/object_oriented.md#子クラス)に任せる[デザインパターン](./design_pattern.md#デザインパターン)。[スーパークラス](../../../../programming/_/chapters/object_oriented.md#親クラス)は、処理の流れを定義した**テンプレートメソッド**を持ち、この[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)の中で[抽象メソッド](../../../../programming/_/chapters/object_oriented.md#抽象メソッド)の呼び出しを行う。

Template Methodパターンは、[AbstractClass](#abstractclass)、[ConcreteClass](#concreteclass)から構成される。

### AbstractClass

**AbstractClass**（抽象クラス）は、[Template Methodパターン](#template-methodパターン)において、処理の流れを決定する役割を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)の各ステップに対応する[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)と、これらの[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を特定の順序で呼び出す[テンプレートメソッド](#template-methodパターン)を宣言する。

### ConcreteClass

**ConcreteClass**（具象クラス）は、[Template Methodパターン](#template-methodパターン)において、[抽象メソッド](../../../../programming/_/chapters/object_oriented.md#抽象メソッド)を具体的に実装する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。ConcreteClassに実装された[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)は[テンプレートメソッド](#template-methodパターン)を介して呼び出される。


## サンプルプログラム

### Java

[Template Methodパターン](#template-methodパターン)により、[ストレージ](../../../../computer/hardware/_/chapters/hardware.md#記憶装置)への書き込み処理を共通化する例を考える。

```java
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.io.OutputStream;
import java.net.URL;
import java.net.HttpURLConnection;

//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        StorageWriter textFileWriter = new TextFileWriter();
        textFileWriter.write("test.txt", "Hello, world");

        StorageWriter networkStorageWriter = new NetworkStorageWriter();
        networkStorageWriter.write("https://example.com", "Hello, world");
    }
}

//------------------------------------------------------------------------------
// AbstractClass
//------------------------------------------------------------------------------
abstract class StorageWriter
{
    //--------------------------------------------------------------------------
    // テンプレートメソッド
    //--------------------------------------------------------------------------
    public final void write( String target, String content )
    {
        this.open(target);
        this.writeContent(content);
        this.close();
    }

    public abstract void open( String target );
    public abstract void writeContent( String content );
    public abstract void close();
}

//------------------------------------------------------------------------------
// ConcreteClass
//------------------------------------------------------------------------------
class TextFileWriter extends StorageWriter
{
    private FileWriter writer;

    //--------------------------------------------------------------------------
    // ファイルオープン
    //--------------------------------------------------------------------------
    public void open( String target )
    {
        try
        {
            File file = new File(target);
            this.writer = new FileWriter(file);
        }
        catch( IOException e )
        {
            System.out.println(e);
        }
    }

    //--------------------------------------------------------------------------
    // ファイル書き込み
    //--------------------------------------------------------------------------
    public void writeContent( String content )
    {
        try
        {
            this.writer.write(content);
        }
        catch( IOException e )
        {
            System.out.println(e);
        }
    }

    //--------------------------------------------------------------------------
    // ファイルクローズ
    //--------------------------------------------------------------------------
    public void close()
    {
        try
        {
            this.writer.close();
        }
        catch( IOException e )
        {
            System.out.println(e);
        }
    }
}

//------------------------------------------------------------------------------
// ConcreteClass
//------------------------------------------------------------------------------
class NetworkStorageWriter extends StorageWriter
{
    private HttpURLConnection connection;
    private OutputStream stream;

    //--------------------------------------------------------------------------
    // オープン
    //--------------------------------------------------------------------------
    public void open( String target )
    {
        try
        {
            URL url = new URL(target);
            this.connection = (HttpURLConnection)url.openConnection();
            this.connection.setRequestMethod("POST");
            this.connection.setDoOutput(true);
            this.stream = this.connection.getOutputStream();
        }
        catch( Exception e )
        {
            System.out.println(e);
        }
    }

    //--------------------------------------------------------------------------
    // 書き込み
    //--------------------------------------------------------------------------
    public void writeContent( String content )
    {
        try
        {
            this.stream.write(content.getBytes());
        }
        catch( Exception e )
        {
            System.out.println(e);
        }
    }

    //--------------------------------------------------------------------------
    // クローズ
    //--------------------------------------------------------------------------
    public void close()
    {
        try
        {
            this.stream.close();
            this.connection.disconnect();
        }
        catch( Exception e )
        {
            System.out.println(e);
        }
    }
}
```
