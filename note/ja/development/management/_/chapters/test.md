# 『テスト』ノート

（最終更新： 2023-06-14）


## 目次

1. [単体テスト](#単体テスト)
1. [結合テスト](#結合テスト)
1. [総合テスト](#総合テスト)
1. [チューニング](#チューニング)


## 単体テスト

**単体テスト**（**ユニットテスト**）は、[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)開発において最小単位の[コード](../../../../programming/_/chapters/programming.md#ソースコード)（[関数](../../../../programming/_/chapters/function.md#関数)や[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)など）を個別にテストする手法。個々の[コンポーネント](../../../../computer/software/_/chapters/package.md#コンポーネント)が正確に動作し、期待通りの結果を返すかどうかを検証する。このテストは自動化されている場合が多く、[依存関係](../../../../computer/software/_/chapters/package.md#依存関係)を最小限に抑えて細かい粒度で実施される。


## 結合テスト

**結合テスト**は、[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)開発において複数の[コンポーネント](../../../../computer/software/_/chapters/package.md#コンポーネント)や[モジュール](../../../../computer/software/_/chapters/package.md#モジュール)を組み合わせてテストする手法。個々の[コンポーネント](../../../../computer/software/_/chapters/package.md#コンポーネント)が単独で正常に動作することを確認した後に、それらを組み合わせて連携や相互作用をテストする。複数の[コンポーネント](../../../../computer/software/_/chapters/package.md#コンポーネント)を実際の環境に近い状態で組み合わせてテストを行い、データや制御の流れが正しく動作することを確認する。


## 総合テスト

**総合テスト**（**システムテスト**）は、[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)開発において[システム](../../../../system/_/chapters/system.md#システム)全体をテストするための手法。[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)が要求仕様を満たし、全体的な機能や性能が期待通りに動作するかを確認スルために行われる。[システム](../../../../system/_/chapters/system.md#システム)が他の[システム](../../../../system/_/chapters/system.md#システム)や外部環境と連携して動作する場合のテストも含まれ、ユーザの視点から実際のシナリオを想定したテストが実施される。


## チューニング

**チューニング**は、[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)の性能や効率を向上させるためのプロセス。[システム](../../../../system/_/chapters/system.md#システム)のパフォーマンスを最適化し、リソースの効率的な利用を促進することを目的に実施される。これは、[ハードウェア](../../../../computer/hardware/_/chapters/hardware.md#ハードウェア)や[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)、[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)、[データベース](../../../database/_/chapters/database.md#データベース)など、様々な要素に対して行われる。リソースの使用状況をモニタリングツールなどで監視して、ボトルネックが発生している場所を特定し、[アルゴリズム](../../../../programming/_/chapters/algorithm.md#アルゴリズム)の最適化やリソース（[CPU](../../../../computer/hardware/_/chapters/processor.md#cpu)、[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)など）の最適化などを行う。
