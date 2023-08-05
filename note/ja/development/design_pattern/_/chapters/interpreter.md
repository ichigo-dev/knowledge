# 『Interpreter』ノート

（最終更新： 2023-08-05）


## 目次

1. [Interpreterパターン](#interpreterパターン)
	1. [AbstractExpression](#abstractexpression)
	1. [TerminalExpression](#terminalexpression)
	1. [NonterminalExpression](#nonterminalexpression)
	1. [Context](#context)


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
