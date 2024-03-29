# 『コーディングルール』ノート

（最終更新： 2023-10-02）


## 目次

1. [コーディングルール](#コーディングルール)
1. [命名規則](#命名規則)
	1. [パスカルケース](#パスカルケース)
	1. [キャメルケース](#キャメルケース)
	1. [スネークケース](#スネークケース)
	1. [アッパーケース](#アッパーケース)
	1. [ケバブケース](#ケバブケース)
	1. [ハンガリアン記法](#ハンガリアン記法)
	1. [技術駆動命名](#技術駆動命名)
	1. [連番命名](#連番命名)
1. [コーディングスタイル](#コーディングスタイル)
	1. [インデント](#インデント)
	1. [中括弧](#中括弧)
	1. [スペース](#スペース)


## コーディングルール

**コーディングルール**は、[変数](./variable.md#変数)名のつけ方や[コメント](./programming.md#コメント)の書き方、[制御構文](./control_flow.md#制御フロー)の書き方といった、[ソースコード](./programming.md#ソースコード)をつくる際に[プログラマ](./programming.md#プログラマ)が守るように定められたルール。[プログラミング言語](./programming.md#プログラミング言語)の仕様として定められていたり、構文上の[エラー](./programming.md#エラー)となることはないが、チーム内で[ソースコード](./programming.md#ソースコード)の一貫性を持たせるために設けることが多い。どのコーディングルールが正しいといった正解はなく、[プログラミング言語](./programming.md#プログラミング言語)ごとのコミュニティや開発チームの文化によって、コーディングルールも異なる。


## 命名規則

**命名規則**は、[識別子](./programming.md#識別子)の命名に関する[コーディングルール](#コーディングルール)。1つの[プログラム](./programming.md#プログラム)の中で複数の命名規則を組み合わせることも多い（[変数](./variable.md#変数)は[スネークケース](#スネークケース)、[関数](./function.md#関数)は[キャメルケース](#キャメルケース)を用いるといった具合）。

特に、複数の単語からなる[識別子](./programming.md#識別子)を使用する場合には、それらの単語の区切りをどのように表現するかによって様々な命名規則が存在する（[識別子](./programming.md#識別子)にスペースは使用できないため）。

### パスカルケース

**パスカルケース**は、[命名規則](#命名規則)の一種で、全ての単語の1文字目を大文字で始め、複数の単語からなる[識別子](./programming.md#識別子)については単語同士をそのままつなげるスタイル。[C#](./programming_language.md#c-1)などの一部の[プログラミング言語](./programming.md#プログラミング言語)で使われることがある。

```js
User
UserName
SetUserName
```

### キャメルケース

**キャメルケース**は、[命名規則](#命名規則)の一種で、複数の単語からなる[識別子](./programming.md#識別子)において、2つ目以降の単語の1文字目を大文字で始め、単語同士をそのままつなげるスタイル。[スネークケース](#スネークケース)と並んで、様々な[プログラミング言語](./programming.md#プログラミング言語)で広く使われている。

```js
user
userName
setUserName
```

### スネークケース

**スネークケース**は、[命名規則](#命名規則)の一種で、複数の単語からなる[識別子](./programming.md#識別子)において、全ての単語を小文字にし、単語同士をアンダースコアでつなげるというスタイル。[キャメルケース](#キャメルケース)と並んで、様々な[プログラミング言語](./programming.md#プログラミング言語)で広く使われている。

```js
user
user_name
set_user_name
```

### アッパーケース

**アッパーケース**は、[命名規則](#命名規則)の一種で、全ての単語を大文字にし、単語同士をアンダースコアでつなげるというスタイル。[定数](./variable.md#定数)名として用いられることが多い。

```js
USER
USER_NAME
SET_USER_NAME
```

### ケバブケース

**ケバブケース**は、[命名規則](#命名規則)の一種で、全ての単語を小文字にし、単語同士をハイフンでつなげるというスタイル。[プログラミング言語](./programming.md#プログラミング言語)で採用されることは少なく、HTMLのclass名や属性、CSSのプロパティなどに用いられる。

```js
user
user-name
set-user-name
```

### ハンガリアン記法

**ハンガリアン記法**は、[命名規則](#命名規則)の一種で、[識別子](./programming.md#識別子)の先頭や末尾に決まった意味のプレフィックス（接頭辞）やサフィックス（接尾辞）を付与するスタイル。流行していた時代もあったが、現代ではアンチパターンとされることもある。ただし、全ての[識別子](./programming.md#識別子)に対してではなく、一部の決まった[識別子](./programming.md#識別子)に対してハンガリアン記法を適用するのが有効な場合もあるため、適切に利用するとよい。

```js
// メンバ変数にプレフィックスとしてm_をつける
m_name
m_type

// グローバル変数にプレフィックスとしてg_をつける
g_counter
g_error_code

// 変数の型を表すプレフィックスをつける
i_number    // int型
c_alphabet  // char型
```

### 技術駆動命名

**技術駆動命名**は、[プログラミング言語](./programming.md#プログラミング言語)の[型](./data_type.md#型)名である `int` や、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)制御を表す `memory` 、 `flag` といった、[プログラミング](./programming.md#プログラミング)用語や[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)用語に基づいた技術ベースでの命名方法。[変数](./variable.md#変数)や[関数](./function.md#関数)の意図が読み取りづらく、[コード](./programming.md#ソースコード)の理解が難しくなるため、避けたほうが良い。また、ロジックが不必要に複雑化する危険性もある。

### 連番命名

**連番命名**は、 `001, 002, 003, ...` といった具合に番号付きの名前をつける命名方法。[変数](./variable.md#変数)や[関数](./function.md#関数)の意図が読み取りづらく、[コード](./programming.md#ソースコード)の理解が難しくなるため、避けたほうが良い。また、ロジックが不必要に複雑化する危険性もある。


## コーディングスタイル

**コーディングスタイル**は、[ソースコード](./programming.md#ソースコード)の記述に関する細かい決まりごと。

### インデント

**インデント**（字下げ）は、[ソースコード](./programming.md#ソースコード)中で[ブロック](./control_flow.md#ブロック)の構造が分かりやすくなるように、先頭に挿入する空白。インデントにスペースを用いるか、タブを用いるか、またインデントはスペースいくつ分がよいかといったスタイルの違いがある。よく用いられるのは、スペース2つやスペース4つ、タブなど。基本的にはインデントの付け方によって[プログラム](./programming.md#プログラム)の動作に影響はないが、[Python](./programming_language.md#python)のようにインデントが構造を記述する意味を持っている場合もある。

### 中括弧

[ブロック](./control_flow.md#ブロック)の始まりと終わりを示す中括弧の書き方に関するスタイルの違いがある。代表的なものは次の2つ。

```js
// オールマン
while( true )
{
    something();
}

// K&R
while( true ) {
    something();
}
```

### スペース

[変数](./variable.md#変数)への値の[代入](./variable.md#代入)や[算術演算](./operation.md#算術演算)において、[オペレータ](./operation.md#演算)と[オペランド](./operation.md#演算)の間にスペースを設けるか設けないかといったスタイルの違いがある。

```js
// パターン1
a = b * c + d;
e = (f + g) / h;

// パターン2
a=b*c+d;
e=(f+g)/h;
```

[関数定義](./function.md#関数定義)や[制御構文](./control_flow.md#制御構文)の中でのスペースの使い方にもスタイルの違いがある。

```js
// パターン1
function example_fn( x, y )
{
    result = 0;
    for( i=0; i<10; i++ )
    {
        if( result > 1000 ) break;
        result += x * y;
    }
    return result;
}
example_fn(1, 3);

// パターン2
function example_fn( x, y )
{
    result = 0;
    for (i = 0; i < 10; i++)
    {
        if (result > 1000) break;
        result += x * y;
    }
    return result;
}
example_fn( 1, 3 );

// パターン3
function example_fn(x,y)
{
    result=0;
    for(i=0;i<10;i++)
    {
        if(result>1000) break;
        result+=x*y;
    }
    return result;
}
example_fn(1,3);
```
