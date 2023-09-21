# 『Single Thread Execution』ノート

（最終更新： 2023-09-21）


## 目次

1. [Single Thread Executionパターン](#single_thread_executionパターン)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Single Thread Executionパターン

**Single Thread Executionパターン**は、複数の[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)から同時に実行されると問題になるような箇所に対して[排他制御](../../../../../programming/parallel_programming/_/chapters/synchronous_processing.md#排他制御)を行い、1つの[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)からのみ実行されるようにする[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。複数の[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)が共有リソースに同時にアクセスすることで、[レースコンディション](../../../../../programming/parallel_programming/_/chapters/synchronous_processing.md#レースコンディション)が発生することを避け、データの整合性を保つことができる。


## サンプルプログラム

### C++

[Single Thread Executionパターン](#single_thread_executionパターン)を実現するためには、[ミューテックス](../../../../../programming/parallel_programming/_/chapters/synchronous_processing.md#ミューテックス)などの[同期機構](../../../../../programming/parallel_programming/_/chapters/synchronous_processing.md#同期処理)を利用する。

```cpp
#include <ostream>
#include <thread>
#include <mutex>

// カウンタ（共有リソース）
int shared_counter = 0;

// ミューテックス
std::mutex mtx;

//------------------------------------------------------------------------------
// カウンタのインクリメント
//------------------------------------------------------------------------------
void increment_shared_counter( const char* name_, int n_ )
{
    for( int i = 0; i < n_; i++ )
    {
        mtx.lock();
        shared_counter++;
        std::printf("====================\n");
        std::printf("Thread: %s\n", name_);
        std::printf("Counter: %d\n", shared_counter);
        std::printf("====================\n");
        mtx.unlock();

        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
}

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    std::thread t1([] { increment_shared_counter("Thread 1", 4); });
    std::thread t2([] { increment_shared_counter("Thread 2", 8); });

    t1.join();
    t2.join();

    return 0;
}
```
