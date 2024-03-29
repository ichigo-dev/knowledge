# 『テクニック』ノート

（最終更新： 2023-10-20）


## 目次

1. [バリデーション](#バリデーション)
1. [リファクタリング](#リファクタリング)
1. [ガード節](#ガード節)
1. [SOLID](#solid)
	1. [単一責任の原則](#単一責任の原則)
	1. [開放閉鎖の原則](#開放閉鎖の原則)
	1. [リスコフの置換原則](#リスコフの置換原則)
	1. [インタフェース分離の原則](#インタフェース分離の原則)
	1. [依存性逆転の原則](#依存性逆転の原則)
1. [DRY原則](#dry原則)
1. [YAGNI原則](#yagni原則)
1. [驚き最小の原則](#驚き最小の原則)
1. [ラバーダッキング](#ラバーダッキング)
1. [CQS](#cqs)
	1. [CQRS](#cqrs)


## バリデーション

**バリデーション**は、入力されたデータが、あらかじめ規定された条件や仕様、形式に適合しているかを検証する操作。一般的には、あくまで形式的な妥当性の検証を行い、意味的に適切であるかという内容は判断しない。


## リファクタリング

**リファクタリング**は、[プログラム](./programming.md#プログラム)の動作や振る舞いを変えることなく、内部の設計や構造を見直すことで、[コード](./programming.md#ソースコード)を理解や修正がしやすいように整える作業。[システム](../../../system/_/chapters/system.md#システム)を長期的に運用したり、仕様変更に強くしたりするために重要な工程。


## ガード節

**ガード節**（**アーリーリターン**、**早期リターン**）は、[プログラミング](./programming.md#プログラミング)のテクニックのひとつで、ある条件が満たされていない場合に[コード](./programming.md#ソースコード)の実行を中断し、特定の制御を行う仕組み。主に[反復処理](./control_flow.md#反復)や[関数](./function.md#関数)の中で使用される。

例えば、条件を満たしている場合にポイントを付与する、以下のような[関数](./function.md#関数)を考える。

```js
function give_family_point( member_, point_ = 100 )
{
    if( member_.is_gold_rank() )
    {
        if( member_.has_children() )
        {
            if( member_.with_family() )
            {
                member_.give_point(point_);
            }
        }
    }
}
```

これは[条件分岐](./control_flow.md#条件分岐)の[ネスト](./control_flow.md#ネスト)が深く、非常に読みづらい[コード](./programming.md#ソースコード)となっている。そこで、ガード節を用いてこの[関数](./function.md#関数)を[リファクタリング](#リファクタリング)すると、以下のようになる。

```js
function give_family_point( member_, point_ = 100 )
{
    // ガード節によるアーリーリターン
    if( member_.is_gold_rank() === false ) return;
    if( member_.has_children() === false ) return;
    if( member_.with_family() === false ) return;

    member_.give_point(point_);
}
```

ガード節を用いると、可読性が向上する他に、条件ブロックと処理ブロックを分離できるというメリットがある。また、 `return` だけではなく、 `break` や `continue` を用いても同様の[リファクタリング](#リファクタリング)が可能。


## SOLID

**SOLID**(Single Responsibility Principle, Open-Closed Principle, Liskov Substitution Principle, Interface Segregation Principle, Dependency Inversion Principle)は、[オブジェクト指向](./object_oriented.md#オブジェクト指向)を用いた開発において、[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)をより柔軟にして保守をしやすくするための原則5つ（[単一責任の原則](#単一責任の原則)、[開放閉鎖の原則](#開放閉鎖の原則)、[リスコフの置換原則](#リスコフの置換原則)、[インタフェース分離の原則](#インタフェース分離の原則)、[依存性逆転の原則](#依存性逆転の原則)）をまとめた用語。

### 単一責任の原則

**単一責任の原則**(**SRP**: Single Responsibility Principle)は、1つの[クラス](./object_oriented.md#クラス)や[モジュール](../../../computer/software/_/chapters/package.md#モジュール)、[関数](./function.md#関数)などは単一の機能についての責任を持つべきである、という[SOLID](#solid)の原則のひとつ。複数の責務を持つ[クラス](./object_oriented.md#クラス)が存在すると、それらの責務が密接に関連してしまい、1つの責務を変更した際に他の責務にも影響を及ぼす可能性がある。

### 開放閉鎖の原則

**開放閉鎖の原則**(**OCP**: Open-Closed Principle)は、[クラス](./object_oriented.md#クラス)や[モジュール](../../../computer/software/_/chapters/package.md#モジュール)、[関数](./function.md#関数)などは、拡張のために開いていて、修正のために閉じているべきである、という[SOLID](#solid)の原則のひとつ。機能の追加が発生した場合に、既存の[コード](./programming.md#ソースコード)や[クラス](./object_oriented.md#クラス)には手を加える必要がなく（修正のために閉じており）、新しい[コード](./programming.md#ソースコード)の追加によって対応できる（拡張のために開いている）ことが望ましい。

### リスコフの置換原則

**リスコフの置換原則**(**LSP**: Liskov Substitution Principle)は、[親クラス](./object_oriented.md#親クラス)の[インスタンス](./object_oriented.md#インスタンス)が適用される[コード](./programming.md#ソースコード)に対して、[子クラス](./object_oriented.md#子クラス)の[インスタンス](./object_oriented.md#インスタンス)で置き換えても、問題なく動作するべきである、という[SOLID](#solid)の原則のひとつ。これは、[親クラス](./object_oriented.md#親クラス)の振る舞いを[子クラス](./object_oriented.md#子クラス)が壊してはいけない、ということと同義。

### インタフェース分離の原則

**インタフェース分離の原則**(**ISP**: Interface Segregation Principle)は、[インタフェース](./object_oriented.md#インタフェース)の利用者にとって不要な[メソッド](./object_oriented.md#メソッド)への依存を強制してはいけない、という[SOLID](#solid)の原則のひとつ。[インタフェース](./object_oriented.md#インタフェース)の実装先の[クラス](./object_oriented.md#クラス)が不要な[メソッド](./object_oriented.md#メソッド)を持たなければいけないような場合は、さらに[インタフェース](./object_oriented.md#インタフェース)を分割することを検討するのが望ましい。

### 依存性逆転の原則

**依存性逆転の原則**(**DIP**: Dependency Inversion Principle)は、[プログラム](./programming.md#プログラム)の重要な部分が、重要ではない部分に依存しないように設計するべきである、とする[SOLID](#solid)の原則のひとつ。上位の[モジュール](../../../computer/software/_/chapters/package.md#モジュール)はいかなるものも下位の[モジュール](../../../computer/software/_/chapters/package.md#モジュール)から持ち込まず、常に具象が抽象に依存するという関係性を保つことが望ましい。


## DRY原則

**DRY原則***(Don't Repeat Yourself)は，同じ[コード](./programming.md#ソースコード)を繰り返し記述するべきではない、という[プログラミング](./programming.md#プログラミング)における原則。これを厳守することにより、[コード](./programming.md#ソースコード)の可読性や保守性を向上させることができ、[バグ](./programming.md#バグ)の発生を防止することができる。

何度も使用する処理を[関数](./function.md#関数)にまとめたり、値をハードコーディングせずに[変数](./variable.md#変数)として定義することでDRY原則を適用することができる。


## YAGNI原則

**YAGNI原則**(You aren't going to need it)は、必要なもの以外は実装するべきではない、という[プログラミング](./programming.md#プログラミング)における原則。無駄な[コード](./programming.md#ソースコード)を書かないようにすることで開発コストを削減し、[システム](../../../system/_/chapters/system.md#システム)をシンプルに保つことができる。実際の開発において、将来の仕様を予想して先回りして作り込むというのは、現実にはほとんど使われないばかりか、[バグ](./programming.md#バグ)を引き起こす原因となってしまう場合もある。

YAGNI原則を破って実装された[コード](./programming.md#ソースコード)は、[デッドコード](./anti_patterns.md#デッドコード)となってしまうパターンも多い。


## 驚き最小の原則

**驚き最小の原則**は、[関数](./function.md#関数)などの仕様は、使用者の予想を裏切るような処理を可能な限り行わないようにするべきである、という[プログラミング](./programming.md#プログラミング)における原則。[命名](./coding_rule.md#命名規則)に関する原則であり、ひとつの[関数](./function.md#関数)が担う役割が十分に小さくない場合や、あまり一般的ではない単語を用いた場合に、この原則に反してしまう結果となる。


## ラバーダッキング

**ラバーダッキング**は、[プログラミング](./programming.md#プログラミング)において、問題が発生した際に第三者（あるいは架空の相手（ラバーダックはゴム製のアヒルのおもちゃのこと））にそれを説明することで頭の中を整理し、問題の解決を図る方法。


## CQS

**CQS**(Command Query Separation)は、[オブジェクト指向](./object_oriented.md#オブジェクト指向)の[クラス](./object_oriented.md#クラス)設計において、コマンド（変更操作）とクエリ（取得操作）を明確に分離するアプローチ。コマンドはデータの変更を行い、[副作用](./function.md#副作用)を持つことがあるが、[戻り値](./function.md#戻り値)を返さない。クエリはデータの取得を行い、[副作用](./function.md#副作用)を持たず、[戻り値](./function.md#戻り値)を返す。

### CQRS

**CQRS**(Command Query Responsibility Segregation)は、[CQS](#cqs)の考え方をベースとしており、データモデルとなる[クラス](./object_oriented.md#クラス)を変更操作用と取得操作用に分けるような設計。変更操作用の[クラス](./object_oriented.md#クラス)をコマンドモデル、取得操作用の[クラス](./object_oriented.md#クラス)をクエリモデルとし、クエリモデルは読み取り専用にする。
