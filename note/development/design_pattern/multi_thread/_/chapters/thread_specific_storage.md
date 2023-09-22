# 『Thread-Specific Storage』ノート

（最終更新： 2023-09-22）


## 目次

1. [Thread-Specific Storageパターン](#thread-specific-storageパターン)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Thread-Specific Storageパターン

**Thread-Specific Storageパターン**（**Thread Local Storageパターン**）は、複数の[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)を用いる[プログラム](../../../../../programming/_/chapters/programming.md#プログラム)において、各[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)が個別のデータ領域を持つことで、[排他処理](../../../../../programming/parallel_programming/_/chapters/synchronous_processing.md#排他処理)を行わずに並列実行を可能にする[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。

[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)ごとに個別のログファイルを管理するといった、[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)固有のデータの隔離に利用される。


## サンプルプログラム

### C++

[C++](../../../../../programming/_/chapters/programming_language.md#c)では、 `thread_local` キーワードを用いることで[Thread-Specific Storageパターン](#thread-specific-storageパターン)を利用することができる。

```cpp
#include <thread>

thread_local int g_data = 0;

//------------------------------------------------------------------------------
// スレッドで実行される処理
//------------------------------------------------------------------------------
void execute( int num_ )
{
    // 各スレッドが固有のg_dataを管理する
    g_data += num_;
    std::printf("data: %d\n", g_data);
}

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    std::thread t1(execute, 100);
    std::thread t2(execute, 5);
    std::thread t3(execute, 80);

    t1.join();
    t2.join();
    t3.join();

    return 0;
}
```
