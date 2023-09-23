# 『Command』ノート

（最終更新： 2023-09-23）


## 目次

1. [Commandパターン](#commandパターン)
	1. [Command](#command)
	1. [ConcreteCommand](#concretecommand)
	1. [Receiver](#receiver)
	1. [Invoker](#invoker)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Commandパターン

**Commandパターン**（**Eventパターン**）は、命令をひとつの[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)として表現することにより、命令の集まりから履歴の管理や、複数の命令をまとめた新しい命令の作成をできるようにした[デザインパターン](./design_pattern.md#デザインパターン)。さらに、命令自体を[関数](../../../../programming/_/chapters/function.md#関数)の[引数](../../../../programming/_/chapters/function.md#引数)として渡したり、実行の遅延や[待ち行列](../../../../basics/applied_mathematics/_/chapters/waiting_queue_theory.md#待ち行列)の利用、取り消し操作などを実現することができる。

Commandパターンは、[Command](#command)、[ConcreteCommand](#concretecommand)、[Receiver](#receiver)、[Invoker](#invoker)から構成される。

### Command

**Command**（命令）は、[Commandパターン](#commandパターン)において、命令が持つ[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。

### ConcreteCommand

**ConcreteCommand**（具体的命令）は、[Commandパターン](#commandパターン)において、[Command](#command)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[フィールド](../../../../programming/_/chapters/object_oriented.md#フィールド)に別の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を持っておき、具体的な処理を任せることもできる。

### Receiver

**Receiver**（受信者）は、[Commandパターン](#commandパターン)において、[Command](#command)が命令を実行するときの処理対象となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)。

### Invoker

**Invoker**（起動者）は、[Commandパターン](#commandパターン)において、[Command](#command)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を呼び出し、実際に命令を実行する役。[Command](#command)の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を組み合わせたり、履歴を管理するための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を提供したりする。


## サンプルプログラム

### Java

クリップボードを利用した様々なコマンドを利用可能なテキストエディタの実装を考える。

```java
import java.util.ArrayDeque;

//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        Editor editor = new Editor();
        EditorToolbar toolbar = new EditorToolbar(editor);

        editor.appendText("Hello, world!");
        System.out.println("text: " + editor.getText());
        System.out.println("clipboard: " + editor.getClipboard());

        System.out.println("==== copy ====");
        toolbar.copy();
        System.out.println("text: " + editor.getText());
        System.out.println("clipboard: " + editor.getClipboard());

        System.out.println("==== paste ====");
        toolbar.paste();
        System.out.println("text: " + editor.getText());
        System.out.println("clipboard: " + editor.getClipboard());

        System.out.println("==== undo ====");
        toolbar.undo();
        System.out.println("text: " + editor.getText());
        System.out.println("clipboard: " + editor.getClipboard());

        System.out.println("==== cut ====");
        toolbar.cut();
        System.out.println("text: " + editor.getText());
        System.out.println("clipboard: " + editor.getClipboard());
    }
}

//------------------------------------------------------------------------------
// Command
//------------------------------------------------------------------------------
abstract class Command
{
    protected Editor editor;
    private String backup;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Command( Editor editor )
    {
        this.editor = editor;
    }

    //--------------------------------------------------------------------------
    // バックアップの取得
    //--------------------------------------------------------------------------
    protected void backup()
    {
        this.backup = editor.getText();
    }

    //--------------------------------------------------------------------------
    // 戻す
    //--------------------------------------------------------------------------
    public void undo()
    {
        this.editor.setText(this.backup);
    }

    public abstract boolean execute();
}

//------------------------------------------------------------------------------
// ConcreteCommand
//------------------------------------------------------------------------------
class CopyCommand extends Command
{
    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public CopyCommand( Editor editor )
    {
        super(editor);
    }

    //--------------------------------------------------------------------------
    // コマンドの実行
    //--------------------------------------------------------------------------
    public boolean execute()
    {
        this.editor.setClipboard(this.editor.getText());
        return true;
    }
}

class PasteCommand extends Command
{
    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public PasteCommand( Editor editor )
    {
        super(editor);
    }

    //--------------------------------------------------------------------------
    // コマンドの実行
    //--------------------------------------------------------------------------
    public boolean execute()
    {
        if( this.editor.getClipboard().length() <= 0 )
        {
            return false;
        }

        this.backup();
        this.editor.appendText(this.editor.getClipboard());
        return true;
    }
}

class CutCommand extends Command
{
    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public CutCommand( Editor editor )
    {
        super(editor);
    }

    //--------------------------------------------------------------------------
    // コマンドの実行
    //--------------------------------------------------------------------------
    public boolean execute()
    {
        if( this.editor.getClipboard().length() <= 0 )
        {
            return false;
        }

        this.backup();
        this.editor.setClipboard(this.editor.getText());
        this.editor.setText("");
        return true;
    }
}

//------------------------------------------------------------------------------
// Receiver
//------------------------------------------------------------------------------
class Editor
{
    private String text;
    private String clipboard;
    private ArrayDeque<Command> history;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Editor()
    {
        this.text = "";
        this.clipboard = "";
        this.history = new ArrayDeque<Command>();
    }

    //--------------------------------------------------------------------------
    // テキストを取得
    //--------------------------------------------------------------------------
    public String getText()
    {
        return this.text;
    }

    //--------------------------------------------------------------------------
    // テキストを設定
    //--------------------------------------------------------------------------
    public void setText( String text )
    {
        this.text = text;
    }

    //--------------------------------------------------------------------------
    // テキストを追加
    //--------------------------------------------------------------------------
    public void appendText( String text )
    {
        this.text += text;
    }

    //--------------------------------------------------------------------------
    // クリップボードを取得
    //--------------------------------------------------------------------------
    public String getClipboard()
    {
        return this.clipboard;
    }

    //--------------------------------------------------------------------------
    // クリップボードを設定
    //--------------------------------------------------------------------------
    public void setClipboard( String text )
    {
        this.clipboard = text;
    }

    //--------------------------------------------------------------------------
    // コマンド履歴を追加
    //--------------------------------------------------------------------------
    public void pushHistory( Command command )
    {
        this.history.push(command);
    }

    //--------------------------------------------------------------------------
    // コマンド履歴を取得
    //--------------------------------------------------------------------------
    public Command popHistory()
    {
        Command cmd = this.history.pop();
        return cmd;
    }
}

//------------------------------------------------------------------------------
// Invoker
//------------------------------------------------------------------------------
class EditorToolbar
{
    private Editor editor;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public EditorToolbar( Editor editor )
    {
        this.editor = editor;
    }

    //--------------------------------------------------------------------------
    // 戻す
    //--------------------------------------------------------------------------
    public void undo()
    {
        Command cmd = this.editor.popHistory();
        cmd.undo();
    }

    //--------------------------------------------------------------------------
    // コマンドの実行
    //--------------------------------------------------------------------------
    private void executeCommand( Command cmd )
    {
        cmd.execute();
        this.editor.pushHistory(cmd);
    }

    //--------------------------------------------------------------------------
    // コピー
    //--------------------------------------------------------------------------
    public void copy()
    {
        this.executeCommand(new CopyCommand(this.editor));
    }

    //--------------------------------------------------------------------------
    // ペースト
    //--------------------------------------------------------------------------
    public void paste()
    {
        this.executeCommand(new PasteCommand(this.editor));
    }

    //--------------------------------------------------------------------------
    // カット
    //--------------------------------------------------------------------------
    public void cut()
    {
        this.executeCommand(new CutCommand(this.editor));
    }
}
```
