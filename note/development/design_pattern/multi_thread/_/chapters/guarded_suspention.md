# 『Guarded Suspention』ノート

（最終更新： 2023-09-21）


## 目次

1. [Guarded Suspentionパターン](#guarded-suspentionパターン)
	1. [GuardedObject](#guardedobject)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Guarded Suspentionパターン

**Guarded Suspentionパターン**は、ある条件が整うまで[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)をブロックしておき、その条件が満たされた時に処理を実行する[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。類似の[Balkingパターン](./balking.md#balkingパターン)では、ガード条件が満たされていない場合に処理をスキップするが、このパターンでは待機する。

Guarded Suspentionパターンは[GuardedObject](#guardedobject)の役のみからなる。

### GuardedObject

**GuardedObject**は、ガードされた[メソッド](../../../../../programming/_/chapters/object_oriented.md#メソッド)を持つ[クラス](../../../../../programming/_/chapters/object_oriented.md#クラス)。ガード条件が満たされていればこの[メソッド](../../../../../programming/_/chapters/object_oriented.md#メソッド)を実行し、満たされていなければ満たされるまで待機する。


## サンプルプログラム

### C++

遅延してメッセージを共有[変数](../../../../../programming/_/chapters/variable.md#変数)に書き込む[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)と、共有[変数](../../../../../programming/_/chapters/variable.md#変数)にあるデータを定期的に監視してそれを読み込む[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)からなる[プログラム](../../../../../programming/_/chapters/programming.md#プログラム)を考える。

```cpp
#include <thread>
#include <mutex>

// 共有リソース
std::mutex g_mtx;
const char* g_data = nullptr;

//------------------------------------------------------------------------------
// データ書き込み用クラス
//------------------------------------------------------------------------------
class Writer
{
    public:

        //----------------------------------------------------------------------
        // 書き込み
        //----------------------------------------------------------------------
        void write( const char* msg_ )
        {
            std::this_thread::sleep_for(std::chrono::milliseconds(500));

            g_mtx.lock();
            std::printf("Data is ready\n");
            g_data = msg_;
            g_mtx.unlock();
        }
};

//------------------------------------------------------------------------------
// GuardedObject
//------------------------------------------------------------------------------
class Reader
{
    public:

        //----------------------------------------------------------------------
        // データの存在をチェックしてあれば読み込み、なければ待機する
        //----------------------------------------------------------------------
        void read()
        {
            while( true )
            {
                g_mtx.lock();
                if( g_data != nullptr )
                {
                    std::printf("Read data: %s\n", g_data);
                    g_mtx.unlock();
                    break;
                }
                else
                {
                    std::printf("Waiting data...\n");
                }
                std::this_thread::sleep_for(std::chrono::milliseconds(100));
                g_mtx.unlock();
            }
        }
};

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    Writer writer;
    Reader reader;
    std::thread th1([&] { writer.write("Hello, world"); });
    std::thread th2([&] { reader.read(); });

    th1.join();
    th2.join();

    return 0;
}
```

また、[C++](../../../../../programming/_/chapters/programming_language.md#c)には状態変数が実装されているため、ビジーループを使用せずに[Guarded Suspentionパターン](#guarded-suspentionパターン)を実装することができる。

```cpp
#include <thread>
#include <mutex>
#include <condition_variable>

// 共有リソース
std::mutex g_mtx;
std::condition_variable g_data_ready;
const char* g_data = nullptr;

//------------------------------------------------------------------------------
// データ書き込み用クラス
//------------------------------------------------------------------------------
class Writer
{
    public:

        //----------------------------------------------------------------------
        // 書き込み
        //----------------------------------------------------------------------
        void write( const char* msg_ )
        {
            std::this_thread::sleep_for(std::chrono::milliseconds(500));

            g_mtx.lock();
            std::printf("Data is ready\n");
            g_data = msg_;

            // データが書き込まれたことを通知
            g_data_ready.notify_all();
            g_mtx.unlock();
        }
};

//------------------------------------------------------------------------------
// GuardedObject
//------------------------------------------------------------------------------
class Reader
{
    public:

        //----------------------------------------------------------------------
        // データの存在をチェックしてあれば読み込み、なければ待機する
        //----------------------------------------------------------------------
        void read()
        {
            std::unique_lock<std::mutex> lock(g_mtx);

            //  データが書き込まれるまで待機
            while( g_data == nullptr )
            {
                std::printf("Waiting data...\n");
                g_data_ready.wait(lock);
            }
            std::printf("Read data: %s\n", g_data);
        }
};

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    Writer writer;
    Reader reader;
    std::thread th1([&] { writer.write("Hello, world"); });
    std::thread th2([&] { reader.read(); });

    th1.join();
    th2.join();

    return 0;
}
```
