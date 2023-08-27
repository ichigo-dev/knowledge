# 『Interpreter』ノート

（最終更新： 2023-08-18）


## 目次

1. [Interpreterパターン](#interpreterパターン)
	1. [AbstractExpression](#abstractexpression)
	1. [TerminalExpression](#terminalexpression)
	1. [NonterminalExpression](#nonterminalexpression)
	1. [Context](#context)
1. [サンプルプログラム](#サンプルプログラム)
	1. [Java](#java)


## Interpreterパターン

**Interpreterパターン**は、何らかの文法規則を持った文書（[プログラム](../../../../programming/_/chapters/programming.md#プログラム)）を解析し、その結果得られた手順（命令）に基づいて処理を実行していく[デザインパターン](./design_pattern.md#デザインパターン)。実行中の[プログラム](../../../../programming/_/chapters/programming.md#プログラム)とは別に、任意の形式によって処理ができる言語を考え、それを実行するためのパターン。

Interpreterパターンは、[AbstractExpression](#abstractexpression)、[TerminalExpression](#terminalexpression)、[NonterminalExpression](#nonterminalexpression)、[Context](#context)から構成される。

### AbstractExpression

**AbstractExpression**（抽象的な表現）は、[Interpreterパターン](#interpreterパターン)において、構文木のノード（[TerminalExpression](#terminalexpression)と[NonterminalExpression](#nonterminalexpression)）に共通の[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を定める役。

### TerminalExpression

**TerminalExpression**（終端となる表現）は、[Interpreterパターン](#interpreterパターン)において、[BNF](../../../../basics/information_theory/_/chapters/formal_language.md#bnf)の終端を表現する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[AbstractExpression](#abstractexpression)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ。

### NonterminalExpression

**NonterminalExpression**（非終端となる表現）は、[Interpreterパターン](#interpreterパターン)において、[BNF](../../../../basics/information_theory/_/chapters/formal_language.md#bnf)の非終端を表現する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[AbstractExpression](#abstractexpression)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ。

### Context

**Context**（文脈、前後関係）は、[Interpreterパターン](#interpreterパターン)において、インタプリタが構文解析を行うための情報を提供する役。


## サンプルプログラム

### Java

独自の命令（ここでは、ロボットの動き（前進・進行方向の変更）を制御するためのミニ言語をイメージしたもの）を持つミニ[プログラム](../../../../programming/_/chapters/programming.md#プログラム)を解析する例を考える。

```java
import java.util.ArrayList;
import java.util.StringTokenizer;

//------------------------------------------------------------------------------
// Client
//------------------------------------------------------------------------------
public class Client
{
    public static void main( String[] args )
    {
        // ミニプログラム言語の解析
        String text = """
                      program end
                      program go end
                      program right go right go right go right end
                      program repeat 4 go right end end
                      program repeat 4 repeat 3 go left end right end end
                      """;
        String[] split = text.split("\n");
        for( int i = 0; i < split.length; i++ )
        {
            Node node = new ProgramNode();
            node.parse(new Context(split[i]));
            System.out.println(node.toString());
        }
    }
}

//------------------------------------------------------------------------------
// AbstractExpression
//------------------------------------------------------------------------------
abstract class Node
{
    public abstract void parse( Context context );
    public abstract String toString();
}

//------------------------------------------------------------------------------
// TerminalExpression
//------------------------------------------------------------------------------
class PrimitiveCommandNode extends Node
{
    private String name;

    //--------------------------------------------------------------------------
    // 構文解析
    //--------------------------------------------------------------------------
    public void parse( Context context )
    {
        this.name = context.currentToken();
        context.skipToken(this.name);
    }

    //--------------------------------------------------------------------------
    // 文字列として取得
    //--------------------------------------------------------------------------
    public String toString()
    {
        return this.name;
    }
}

//------------------------------------------------------------------------------
// NonTerminalExpression
//------------------------------------------------------------------------------
class ProgramNode extends Node
{
    private Node commandListNode;

    //--------------------------------------------------------------------------
    // 構文解析
    //--------------------------------------------------------------------------
    public void parse( Context context )
    {
        context.skipToken("program");
        this.commandListNode = new CommandListNode();
        this.commandListNode.parse(context);
    }

    //--------------------------------------------------------------------------
    // 文字列として取得
    //--------------------------------------------------------------------------
    public String toString()
    {
        return "[program " + this.commandListNode.toString() + "]";
    }
}

class CommandListNode extends Node
{
    private ArrayList<Node> list;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public CommandListNode()
    {
        this.list = new ArrayList<Node>();
    }

    //--------------------------------------------------------------------------
    // 構文解析
    //--------------------------------------------------------------------------
    public void parse( Context context )
    {
        while( true )
        {
            if( context.currentToken().equals("end") )
            {
                context.skipToken("end");
                break;
            }
            else
            {
                Node commandNode = new CommandNode();
                commandNode.parse(context);
                this.list.add(commandNode);
            }
        }
    }

    //--------------------------------------------------------------------------
    // 文字列として取得
    //--------------------------------------------------------------------------
    public String toString()
    {
        return this.list.toString();
    }
}

class CommandNode extends Node
{
    private Node node;

    //--------------------------------------------------------------------------
    // 構文解析
    //--------------------------------------------------------------------------
    public void parse( Context context )
    {
        if( context.currentToken().equals("repeat") )
        {
            this.node = new RepeatCommandNode();
            this.node.parse(context);
        }
        else
        {
            this.node = new PrimitiveCommandNode();
            this.node.parse(context);
        }
    }

    //--------------------------------------------------------------------------
    // 文字列として取得
    //--------------------------------------------------------------------------
    public String toString()
    {
        return this.node.toString();
    }
}

class RepeatCommandNode extends Node
{
    private int number;
    private Node commandListNode;

    //--------------------------------------------------------------------------
    // 構文解析
    //--------------------------------------------------------------------------
    public void parse( Context context )
    {
        context.skipToken("repeat");
        number = context.currentNumber();
        context.nextToken();
        this.commandListNode = new CommandListNode();
        this.commandListNode.parse(context);
    }

    //--------------------------------------------------------------------------
    // 文字列として取得
    //--------------------------------------------------------------------------
    public String toString()
    {
        return "[repeat " + this.number + " " + this.commandListNode + "]";
    }
}

//------------------------------------------------------------------------------
// Context
//------------------------------------------------------------------------------
class Context
{
    private StringTokenizer tokenizer;
    private String currentToken;

    //--------------------------------------------------------------------------
    // コンストラクタ
    //--------------------------------------------------------------------------
    public Context( String text )
    {
        this.tokenizer = new StringTokenizer(text);
        this.nextToken();
    }

    //--------------------------------------------------------------------------
    // 現在のトークンを次に進める
    //--------------------------------------------------------------------------
    public String nextToken()
    {
        if( this.tokenizer.hasMoreTokens() )
        {
            this.currentToken = this.tokenizer.nextToken();
        }
        else
        {
            this.currentToken = null;
        }
        return this.currentToken;
    }

    //--------------------------------------------------------------------------
    // 現在のトークンを取得
    //--------------------------------------------------------------------------
    public String currentToken()
    {
        return this.currentToken;
    }

    //--------------------------------------------------------------------------
    // 指定したトークンをスキップ
    //--------------------------------------------------------------------------
    public void skipToken( String token )
    {
        if( token.equals(this.currentToken) == false )
        {
            System.out.println(token + " is expected, but " + this.currentToken + " is found.");
            return;
        }
        nextToken();
    }

    //--------------------------------------------------------------------------
    // 現在のトークンを数値としてパースして取得
    //--------------------------------------------------------------------------
    public int currentNumber()
    {
        int number = 0;
        try
        {
            number = Integer.parseInt(this.currentToken);
        }
        catch( Exception e )
        {
            System.out.println(e);
        }
        return number;
    }
}
```
