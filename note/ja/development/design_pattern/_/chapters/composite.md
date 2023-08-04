# 『Composite』ノート

（最終更新： 2023-08-04）


## 目次

1. [Compositeパターン](#compositeパターン)
	1. [Leaf](#leaf)
	1. [Composite](#composite)
	1. [Component](#component)


## Compositeパターン

**Compositeパターン**は、[木構造](../../../../basics/applied_mathematics/_/chapters/graph_theory.md#木)を持つデータに対して再帰的な処理を行うために、容器と中身を同一視する[デザインパターン](./design_pattern.md#デザインパターン)。例えば、[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)において、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)と[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)は、どちらも[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)の中に入ることができるもの（ディレクトリエントリ）としてまとめることができる。このとき、[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)はディレクトリエントリを入れるための容器でもあり、ディレクトリエントリ自身であることもできるため、容器と中身が同一視されている。

Compositeパターンは、[Leaf](#leaf)、[Composite](#composite)、[Component](#component)から構成される。

### Leaf

**Leaf**（葉）は、[Compositeパターン](#compositeパターン)において、中身のものを表す役。Leafの中には他のものを入れることはできない。[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)における[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)はこの役の実装と言える。

### Composite

**Composite**（複合体）は、[Compositeパターン](#compositeパターン)において、容器を表す役。Compositeの中には[Leaf](#leaf)や他のCompositeを入れることができる。[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)における[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)はこの役の実装と言える。

### Component

**Component**は、[Compositeパターン](#compositeパターン)において、[Leaf](#leaf)と[Composite](#composite)の役を同一視するための抽象化構造。例えば、[Leaf](#leaf)と[Composite](#composite)の共通の[スーパクラス](../../../../programming/_/chapters/object_oriented.md#親クラス)として実現することができる。
