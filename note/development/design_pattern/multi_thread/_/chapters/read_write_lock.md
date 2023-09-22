# 『Read-Write Lock』ノート

（最終更新： 2023-09-22）


## 目次

1. [Read-Write Lockパターン](#read-write-lockパターン)
	1. [Reader](#reader)
	1. [Writer](#writer)
	1. [SharedResource](#sharedresource)
	1. [ReadWriteLock](#readwritelock)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Read-Write Lockパターン

**Read-Write Lockパターン**は、共有リソースに対して、単一の書き込み[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)、もしくは複数の読み込み[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)がアクセスすることができるという[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。また、読み込みと書き込みは同時にできないようにすることで、[レースコンディション](../../../../../programming/parallel_programming/_/chapters/synchronous_processing.md#レースコンディション)を防ぎながらパフォーマンスを向上させることができる。

Read-Write Lockパターンは、[Reader](#reader)、[Writer](#writer)、[SharedResource](#sharedresource)、[ReadWriteLock](#readwritelock)から構成される。

### Reader

**Reader**は、[Read-Write Lockパターン](#read-write-lockパターン)において、[SharedResource](#sharedresource)に対する読み込みを行う[クラス](../../../../../programming/_/chapters/object_oriented.md#クラス)や[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)。同時に複数のReaderが[SharedResource](#sharedresource)にアクセスすることができる。

### Writer

**Writer**は、[Read-Write Lockパターン](#read-write-lockパターン)において、[SharedResource](#sharedresource)に対する書き込みを行う[クラス](../../../../../programming/_/chapters/object_oriented.md#クラス)や[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)。同時にひとつのWriterが[SharedResource](#sharedresource)にアクセスでき、書き込み中は[Reader](#reader)も読み込みができない。

### SharedResource

**SharedResource**は、[Read-Write Lockパターン](#read-write-lockパターン)において、[Reader](#reader)と[Writer](#writer)によって共有されている資源。SharedResourceは、内部状態を変更する処理(Write)と、内部状態を変更しない処理(Read)を提供する。

### ReadWriteLock

**ReadWriteLock**は、[Read-Write Lockパターン](#read-write-lockパターン)において、[SharedResource](#sharedresource)が読み込み処理と書き込み処理を実現するためのロックを提供する役。


## サンプルプログラム

### C++

```cpp
#include <thread>
#include <mutex>
#include <condition_variable>

//------------------------------------------------------------------------------
// ReadWriteLock
//------------------------------------------------------------------------------
class ReadWriteLock
{
    public:

        //----------------------------------------------------------------------
        // コンストラクタ
        //----------------------------------------------------------------------
        ReadWriteLock()
        {
            this->m_read_count = 0;
            this->m_write_count = 0;
        }

        //----------------------------------------------------------------------
        // Reader用ロックの獲得
        //----------------------------------------------------------------------
        void acquire_read_lock()
        {
            std::unique_lock<std::mutex> lock(m_mtx);

            // Writer用ロックが獲得されていれば待機
            while( this->m_write_count > 0 )
            {
                this->m_read_condition.wait(lock);
            }
            this->m_read_count++;
        }

        //----------------------------------------------------------------------
        // Reader用ロックの解放
        //----------------------------------------------------------------------
        void release_read_lock()
        {
            std::unique_lock<std::mutex> lock(m_mtx);
            this->m_read_count--;

            // Reader用ロックが全て開放されたらWriterに通知
            if( this->m_read_count == 0 )
            {
                this->m_write_condition.notify_one();
            }
        }

        //----------------------------------------------------------------------
        // Writer用ロックの獲得
        //----------------------------------------------------------------------
        void acquire_write_lock()
        {
            std::unique_lock<std::mutex> lock(m_mtx);
            this->m_write_count++;

            // 他のロックが獲得されていれば待機
            while( this->m_read_count > 0 || this->m_write_count > 1 )
            {
                this->m_write_condition.wait(lock);
            }
        }

        //----------------------------------------------------------------------
        // Writer用ロックの解放
        //----------------------------------------------------------------------
        void release_write_lock()
        {
            std::unique_lock<std::mutex> lock(m_mtx);
            this->m_write_count--;

            // 他のロックが獲得されていれば待機
            this->m_write_condition.notify_one();
            this->m_read_condition.notify_all();
        }

    private:

        std::mutex m_mtx;
        std::condition_variable m_read_condition;
        std::condition_variable m_write_condition;
        int m_read_count;
        int m_write_count;
};

// SharedResource
int g_data = 0;
ReadWriteLock g_rw_lock;

//------------------------------------------------------------------------------
// Reader
//------------------------------------------------------------------------------
void reader()
{
    g_rw_lock.acquire_read_lock();
    std::printf("Reader: %d\n", g_data);
    g_rw_lock.release_read_lock();
}

//------------------------------------------------------------------------------
// Writer
//------------------------------------------------------------------------------
void writer()
{
    g_rw_lock.acquire_write_lock();
    std::printf("Writer\n");
    g_data++;
    g_rw_lock.release_write_lock();
}

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    std::thread t1(reader);
    std::thread t2(writer);
    std::thread t3(reader);

    t1.join();
    t2.join();
    t3.join();

    return 0;
}
```
