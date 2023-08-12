# 『Memento』ノート

（最終更新： 2023-08-12）


## 目次

1. [Mementoパターン](#mementoパターン)
	1. [Originator](#originator)
	1. [Memento](#memento)
	1. [Caretaker](#caretaker)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Mementoパターン

**Mementoパターン**は、[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の任意の時点の状態を覚えておき、後でその状態に[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を戻すための工夫を提供する[デザインパターン](./design_pattern.md#デザインパターン)。テキストエディタや画像・動画の編集ソフトなどに実装されている、アンドゥ機能を提供することができる。

Mementoパターンは、[Originator](#originator)、[Memento](#memento)、[Caretaker](#caretaker)から構成される。

### Originator

**Originator**（作成者）は、[Mementoパターン](#mementoパターン)において、自分の状態を保存した[Memento](#memento)を作成したり、要求された[Memento](#memento)に状態を戻したりする役割を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。

### Memento

**Memento**（形見）は、[Mementoパターン](#mementoパターン)において、[Originator](#originator)の内部情報（[フィールド](../../../../programming/_/chapters/object_oriented.md#フィールド)）をスナップショットとして保持する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。Mementoの[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)は変更不可とし、[コンストラクタ](../../../../programming/_/chapters/object_oriented.md#コンストラクタ)を用いて一度だけデータを渡せるようにすることが多い。

### Caretaker

**Caretaker**（世話人）は、[Mementoパターン](#mementoパターン)において、[Memento](#memento)の履歴を保持し、[Originator](#originator)の状態を保存したり、ある時点の状態に戻したりするためタイミングを管理する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。どのタイミングで[Originator](#originator)の状態を獲得するべきか、また復元すべきかを知っており、[Mementoパターン](#mementoパターン)を利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)はCaretakerの[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を利用するだけで、状態管理のタイミングを意識する必要はない。


## サンプルプログラム

### Java

入力した文字列を保存しておき、後から復元できるようなテキストエディタの実装を考える。

```java
import java.util.ArrayDeque;

//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // エディタの編集履歴の管理
        TextEditor editor = new TextEditor();
        TextEditorMementoManager manager = new TextEditorMementoManager(editor);

        editor.appendText("Hello");
        manager.save();
        editor.appendText(", ");
        manager.save();
        editor.appendText("world");
        manager.save();
        editor.appendText("!");

        editor.printText();
        manager.load();
        editor.printText();
        manager.load();
        editor.printText();
        manager.load();
        editor.printText();
    }
}

//------------------------------------------------------------------------------
// Originator
//------------------------------------------------------------------------------
public class TextEditor
{
    private String text;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TextEditor()
    {
        this.text = new String();
    }

    //--------------------------------------------------------------------------
    // 文字列を追加
    //--------------------------------------------------------------------------
    public void appendText( String text )
    {
        this.text += text;
    }

    //--------------------------------------------------------------------------
    // 文字列を表示
    //--------------------------------------------------------------------------
    public void printText()
    {
        System.out.println(this.text);
    }

    //--------------------------------------------------------------------------
    // Mementoを作成
    //--------------------------------------------------------------------------
    public TextEditorMemento createMemento()
    {
        return new TextEditorMemento(this.text);
    }

    //--------------------------------------------------------------------------
    // Mementoを復元
    //--------------------------------------------------------------------------
    public void setMemento( TextEditorMemento memento )
    {
        this.text = memento.getText();
    }
}

//------------------------------------------------------------------------------
// Memento
//------------------------------------------------------------------------------
public class TextEditorMemento
{
    private String text;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TextEditorMemento( String text )
    {
        this.text = text;
    }

    //--------------------------------------------------------------------------
    // 文字列を取得
    //--------------------------------------------------------------------------
    public String getText()
    {
        return this.text;
    }
}

//------------------------------------------------------------------------------
// Caretaker
//------------------------------------------------------------------------------
public class TextEditorMementoManager
{
    private TextEditor editor;
    private ArrayDeque<TextEditorMemento> mementoList;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public TextEditorMementoManager( TextEditor editor )
    {
        this.editor = editor;
        this.mementoList = new ArrayDeque<TextEditorMemento>();
    }

    //--------------------------------------------------------------------------
    // 状態の保存
    //--------------------------------------------------------------------------
    public void save()
    {
        TextEditorMemento memento = this.editor.createMemento();
        this.mementoList.push(memento);
    }

    //--------------------------------------------------------------------------
    // 状態の復元
    //--------------------------------------------------------------------------
    public void load()
    {
        TextEditorMemento memento = this.mementoList.pop();
        this.editor.setMemento(memento);
    }
}
```
