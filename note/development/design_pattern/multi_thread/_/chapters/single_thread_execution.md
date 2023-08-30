# 『Single Thread Execution』ノート

（最終更新： 2023-08-29）


## 目次

1. [Single Thread Executionパターン](#single_thread_executionパターン)


## Single Thread Executionパターン

**Single Thread Executionパターン**は、複数の[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)から同時に実行されると問題になるような箇所に対して[排他制御](../../../../../programming/parallel_programming/_/chapters/synchronous_processing.md#排他制御)を行い、1つの[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)からのみ実行されるようにする[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。
