# 『Balking』ノート

（最終更新： 2023-09-21）


## 目次

1. [Balkingパターン](#balkingパターン)
	1. [GuardedObject](#guardedobject)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Balkingパターン

**Balkingパターン**は、処理を要求したときに、ある条件が満たされている場合にのみ処理を行う[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。類似の[Balkingパターン](./balking.md#balkingパターン)では、ガード条件が満たされていない場合に処理をスキップするが、このパターンではスキップする。

Balkingパターンは[GuardedObject](#guardedobject)の役のみからなる。

### GuardedObject

**GuardedObject**は、ガードされた[メソッド](../../../../../programming/_/chapters/object_oriented.md#メソッド)を持つ[クラス](../../../../../programming/_/chapters/object_oriented.md#クラス)。ガード条件が満たされていればこの[メソッド](../../../../../programming/_/chapters/object_oriented.md#メソッド)を実行し、満たされていなければスキップする。


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
        // データの存在をチェックしてあれば読み込み、なければスキップする
        //----------------------------------------------------------------------
        bool read()
        {
            g_mtx.lock();
            if( g_data != nullptr )
            {
                std::printf("Read data: %s\n", g_data);
                g_mtx.unlock();
                return true;
            }
            else
            {
                std::printf("Waiting data...\n");
                g_mtx.unlock();
                return false;
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
    std::thread th2([&]
    {
        while( reader.read() == false )
        {
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
        }
    });

    th1.join();
    th2.join();

    return 0;
}
```
