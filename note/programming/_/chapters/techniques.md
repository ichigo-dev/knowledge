# 『テクニック』ノート

（最終更新： 2023-10-09）


## 目次

1. [リファクタリング](#リファクタリング)
1. [ガード節](#ガード節)
1. [DRY原則](#dry原則)
1. [YAGNI原則](#yagni原則)


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


## DRY原則

**DRY原則***(Don't Repeat Yourself)は，同じ[コード](./programming.md#ソースコード)を繰り返し記述するべきではない、という[プログラミング](./programming.md#プログラミング)における原則。これを厳守することにより、[コード](./programming.md#ソースコード)の可読性や保守性を向上させることができ、[バグ](./programming.md#バグ)の発生を防止することができる。

何度も使用する処理を[関数](./function.md#関数)にまとめたり、値をハードコーディングせずに[変数](./variable.md#変数)として定義することでDRY原則を適用することができる。


## YAGNI原則

**YAGNI原則**(You aren't going to need it)は、必要なもの以外は実装するべきではない、という[プログラミング](./programming.md#プログラミング)における原則。無駄な[コード](./programming.md#ソースコード)を書かないようにすることで開発コストを削減し、[システム](../../../system/_/chapters/system.md#システム)をシンプルに保つことができる。実際の開発において、将来の仕様を予想して先回りして作り込むというのは、現実にはほとんど使われないばかりか、[バグ](./programming.md#バグ)を引き起こす原因となってしまう場合もある。

YAGNI原則を破って実装された[コード](./programming.md#ソースコード)は、[デッドコード](./anti_patterns.md#デッドコード)となってしまうパターンも多い。
