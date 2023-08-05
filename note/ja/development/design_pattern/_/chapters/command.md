# 『Command』ノート

（最終更新： 2023-08-05）


## 目次

1. [Commandパターン](#commandパターン)
	1. [Command](#command)
	1. [ConcreteCommand](#concretecommand)
	1. [Receiver](#receiver)
	1. [Invoker](#invoker)


## Commandパターン

**Commandパターン**（**Eventパターン**）は、命令をひとつの「もの」として表現することにより、命令の集まりから履歴の管理や、複数の命令をまとめた新しい命令の作成をできるようにした[デザインパターン](./design_pattern#デザインパターン)。

Commandパターンは、[Command](#command)、[ConcreteCommand](#concretecommand)、[Receiver](#receiver)、[Invoker](#invoker)から構成される。

### Command

**Command**（命令）は、[Commandパターン](#commandパターン)において、命令が持つ[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。

### ConcreteCommand

**ConcreteCommand**（具体的命令）は、[Commandパターン](#commandパターン)において、[Command](#command)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)の具体的な実装を持つ[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。

### Receiver

**Receiver**（受信者）は、[Commandパターン](#commandパターン)において、[Command](#command)が命令を実行するときの処理対象となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)。

### Invoker

**Invoker**（起動者）は、[Commandパターン](#commandパターン)において、[Command](#command)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を呼び出し、実際に命令を実行する役。[Command](#command)の[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)を組み合わせたり、履歴を管理するための[メソッド](../../../../programming/_/chapters/object_oriented.md#メソッド)を提供したりする。
