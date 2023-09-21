# 『Guarded Suspention』ノート

（最終更新： 2023-09-21）


## 目次

1. [Guarded Suspentionパターン](#guarded-suspentionパターン)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Guarded Suspentionパターン

**Guarded Suspentionパターン**は、ある条件が整うまで[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)をブロックしておき、その条件が満たされた時に処理を実行する[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。このパターンを利用しない場合、[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)は常に条件が満たされているか否かをループ処理によって監視し続ける必要があり、共有リソースをロックする時間が長くなってしまう。


## サンプルプログラム

### C++

```cpp
#include <thread>
#include <mutex>
#include <condition_variable>

// 共有リソース
const int BUFFER_SIZE = 10;
int g_buffer[BUFFER_SIZE];
int g_item_count = 0;

// 状態変数
std::mutex g_mtx;
std::condition_variable g_buffer_not_empty;
std::condition_variable g_buffer_not_full;

//------------------------------------------------------------------------------
// タスクを生成してバッファに追加
//------------------------------------------------------------------------------
void producer()
{
    for( int i = 0; i < 100; i++ )
    {
        std::unique_lock<std::mutex> lock(g_mtx);

        // バッファがいっぱいであれば待機
        while( g_item_count == BUFFER_SIZE )
        {
            g_buffer_not_full.wait(lock);
        }
        g_buffer[g_item_count++] = i + 1;
        std::printf("Producer: %d\n", i + 1);

        // バッファが空ではなくなったことをconsumerのスレッドに通知
        g_buffer_not_empty.notify_all();
    }
}

//------------------------------------------------------------------------------
// バッファのタスクを処理
//------------------------------------------------------------------------------
void consumer()
{
    for( int i = 0; i < 100; i++ )
    {
        std::unique_lock<std::mutex> lock(g_mtx);

        // バッファが空であれば待機
        while( g_item_count == 0 )
        {
            g_buffer_not_empty.wait(lock);
        }
        int data = g_buffer[--g_item_count];
        std::printf("Consumer: %d\n", data);

        // バッファがいっぱいではなくなったことをproducerのスレッドに通知
        g_buffer_not_full.notify_all();
    }
}

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    std::thread t1(producer);
    std::thread t2(consumer);

    t1.join();
    t2.join();

    return 0;
}
```
