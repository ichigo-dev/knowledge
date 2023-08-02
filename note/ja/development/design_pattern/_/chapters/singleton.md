# 『Singleton』ノート

（最終更新： 2023-08-02）


## 目次

1. [Singletonパターン](#singletonパターン)
	1. [Singleton](#singleton)


## Singletonパターン

**Singletonパターン**は、ある[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)が絶対に1つしか存在しないことを保証する[デザインパターン](./design_pattern.md#デザインパターン)。[システム](../../../../system/_/chapters/system.md#システム)中で1つしか存在しないものを[プログラム](../../../../programming/_/chapters/programming.md#プログラム)で表現したい場合に用いられる。

Singletonパターンでは、[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)の[コンストラクタ](../../../../programming/_/chapters/object_oriented.md#コンストラクタ)を[private](../../../../programming/_/chapters/object_oriented.md#private)に隠蔽し、[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)の生成および取得のための[staticメソッド](../../../../programming/_/chapters/object_oriented.md#staticメソッド)を用意しておく。Singletonパターンは[Singleton](#singleton)の役のみで構成される。

### Singleton

**Singleton**は、[Singletonパターン](#singletonパターン)において、唯一の[インスタンス](../../../../programming/_/chapters/object_oriented.md#インスタンス)を生成・管理・取得するためのインタフェースを持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。
