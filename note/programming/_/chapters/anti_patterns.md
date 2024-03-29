# 『アンチパターン』ノート

（最終更新： 2023-10-09）


## 目次

1. [アンチパターン](#アンチパターン)
1. [ハードコーディング](#ハードコーディング)
	1. [マジックナンバー](#マジックナンバー)
1. [スマートUI](#スマートui)
1. [デッドコード](#デッドコード)
1. [例外の握り潰し](#例外の握り潰し)


## アンチパターン

**アンチパターン**は、[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)開発において、よくある失敗のパターン。このようなパターンに当てはまる[プログラム](./programming.md#プログラム)は解析や改修が難しく、プロジェクトが失敗する可能性が高まってしまう。


## ハードコーディング

**ハードコーディング**（**ハードコード**）は、[プログラミング](./programming.md#プログラミング)の[アンチパターン](#アンチパターン)の一種で、[ソースコード](./programming.md#ソースコード)中に直接的に値を書き込む形でデータを挿入することを指す。ハードコーディングが多用された[プログラム](./programming.md#プログラム)は可読性や保守性が低下し、[バグ](./programming.md#バグ)を生み出しやすくなる。

### マジックナンバー

**マジックナンバー**は、[プログラミング](./programming.md#プログラミング)において、[ソースコード](./programming.md#ソースコード)中に[ハードコーディング](#ハードコーディング)された数値。その意味や意図が記述した本人以外には自明ではないため、可読性や保守性の低下を招くものとして、原則避けるべきとされている。


## スマートUI

**スマートUI**は、[GUI](../../../computer/software/_/chapters/software.md#gui)[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)や[Web](../../../network/_/chapters/web.md#web)[システム](../../../system/_/chapters/system.md#システム)の[プログラミング](./programming.md#プログラミング)における[アンチパターン](#アンチパターン)の一種で、表示関連の[ソースコード](./programming.md#ソースコード)中に表示以外のロジックが実装されている構造を指す。特に、複雑なロジックが表示関連の[コード](./programming.md#ソースコード)に紛れ込んでいると、デザイン変更により機能に不具合が発生するなど、保守性の低下を招く。


## デッドコード

**デッドコード**（**到達不能コード**）は、[プログラミング](./programming.md#プログラミング)における[アンチパターン](#アンチパターン)の一種で、どのような条件であっても決して実行されることのない[コード](./programming.md#ソースコード)。デッドコードは[コード](./programming.md#ソースコード)の可読性を低下させ、将来的な仕様変更により実行されてしまったときに[バグ](./programming.md#バグ)となる可能性が高い。


## 例外の握り潰し

**例外の握り潰し**は、[プログラミング](./programming.md#プログラミング)における[アンチパターン](#アンチパターン)の一種で、[例外](./programming.md#例外)が発生した場合に、何の処理も行わないことを指す。例外の握り潰しにより、[エラー](./programming.md#エラー)が発生した際に、その原因を分析することが難しくなる。
