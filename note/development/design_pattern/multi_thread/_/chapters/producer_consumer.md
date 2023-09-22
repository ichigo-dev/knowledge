# 『Producer-Consumer』ノート

（最終更新： 2023-09-22）


## 目次

1. [Producer-Consumerパターン](#producer-consumerパターン)
	1. [Producer](#producer)
	1. [Consumer](#consumer)
	1. [Data](#data)
	1. [Channel](#channel)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Producer-Consumerパターン

**Producer-Consumerパターン**は、データを生成する[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)と、生成されたデータを消費する[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)が協力して、データを安全に共有し処理を行う[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。

Producer-Consumerパターン**は、[Producer](#producer)、[Consumer](#consumer)、[Data](#data)、[Channel](#channel)から構成される。

### Producer

**Producer**は、[Producer-Consumerパターン](#producer-consumerパターン)において、[Data](#data)を生成して[Channel](#channel)に追加する役。[Channel](#channel)の容量がいっぱいであれば、[Consumer](#consumer)が[Data](#data)を消費するまで待機する。

### Consumer

**Consumer**は、[Producer-Consumerパターン](#producer-consumerパターン)において、[Channel](#channel)から[Data](#data)を取得して処理をする役。[Channel](#channel)が空であれば、[Producer](#producer)が[Data](#data)を生成するまで待機する。

### Data

**Data**は、[Producer-Consumerパターン](#producer-consumerパターン)において、[Producer](#producer)によって生み出されて[Channel](#channel)に保持され、[Consumer](#consumer)によって取り出されて処理される役。

### Channel

**channel**は、[producer-consumerパターン](#producer-consumerパターン)において、[producer](#producer)によって生み出された[data](#data)を、[Consumer](#consumer)によって取り出されるまで一時的に保持する役。


## サンプルプログラム

### C++

[Channel](#channel)に対して100回データの生成を行う[Producer](#producer)と、それらのデータに対して処理を行う[Consumer](#consumer)からなる[プログラム](../../../../../programming/_/chapters/programming.md#プログラム)を考える。

```cpp
#include <thread>
#include <mutex>
#include <condition_variable>

//------------------------------------------------------------------------------
// Channel
//------------------------------------------------------------------------------
const int BUFFER_SIZE = 10;
int g_buffer[BUFFER_SIZE];

// カウンタ
int g_item_count = 0;

// 状態変数
std::mutex g_mtx;
std::condition_variable g_buffer_not_empty;
std::condition_variable g_buffer_not_full;

//------------------------------------------------------------------------------
// Producer
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

        // バッファが空ではなくなったことをConsumerに通知
        g_buffer_not_empty.notify_all();
    }
}

//------------------------------------------------------------------------------
// Consumer
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

        // バッファがいっぱいではなくなったことをProducerに通知
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
