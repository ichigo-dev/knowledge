# 『Worker Thread』ノート

（最終更新： 2023-09-22）


## 目次

1. [Worker Threadパターン](#worker-threadパターン)
	1. [Channel](#channel)
	1. [Worker](#worker)
	1. [Request](#request)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Worker Threadパターン

**Worker Threadパターン**（**Thread Poolパターン**）は、一般的に生成・破棄コストの高い[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)をあらかじめ起動しておき、それを使いまわす[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。[Producer-Consumerパターン](./producer_consumer.md#producer-consumerパターン)に基づき、キューを使用して[Worker](#worker)スレッドに処理させたいタスクを送受信する。

[Thread-Per-Message](./thread_per_message.md#thread-per-messageパターン)では処理要求がある度に新しい[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)を生成するが、Worker Threadパターンではこの際に発生するコストを抑えることができ、[スループット](../../../../../system/_/chapters/system_performance_evaluation.md#スループット)の向上が期待できる。

Worker Threadパターンは、[Channel](#channel)、[Worker](#worker)、[Request](#request)から構成される。

### Channel

**Channel**は、[Worker Threadパターン](#worker-threadパターン)において、[Worker](#worker)で処理したい[Request](#request)を一時的に保持しておく役。

### Worker

**Worker**は、[Worker Threadパターン](#worker-threadパターン)において、[Channel](#channel)から[Request](#request)を拾って実際に処理を行う役。[Channel](#channel)が空である場合は、新しい[Request](#request)が生成されるまで待機する。

### Request

**Request**は、[Worker Threadパターン](#worker-threadパターン)において、[Worker](#worker)によって処理されることを期待されるタスク。


## サンプルプログラム

### C++

```cpp
#include <deque>
#include <vector>
#include <functional>
#include <thread>
#include <mutex>
#include <condition_variable>

//------------------------------------------------------------------------------
// Channel
//------------------------------------------------------------------------------
class TaskQueue
{
    public:

        //----------------------------------------------------------------------
        // コンストラクタ
        //----------------------------------------------------------------------
        TaskQueue( int size_ )
        {
            this->m_size = size_;
        }

        //----------------------------------------------------------------------
        // タスクの追加
        //----------------------------------------------------------------------
        bool put( const std::function<void()> &task_ )
        {
            if( this->m_size <= this->m_deque.size() )
            {
                return false;
            }
            this->m_deque.emplace_back(task_);
            return true;
        }

        //----------------------------------------------------------------------
        // タスクの取得
        //----------------------------------------------------------------------
        bool get( std::function<void()>& task_ )
        {
            if( this->empty() )
            {
                return false;
            }
            task_ = std::move(this->m_deque.front());
            this->m_deque.pop_front();
            return true;
        }

        //----------------------------------------------------------------------
        // 空チェック
        //----------------------------------------------------------------------
        bool empty()
        {
            return this->m_deque.empty();
        }

    private:

        int m_size;
        std::deque<std::function<void()>> m_deque;
};

//------------------------------------------------------------------------------
// Worker
//------------------------------------------------------------------------------
class ThreadPool
{
    public:

        //----------------------------------------------------------------------
        // コンストラクタ
        //----------------------------------------------------------------------
        ThreadPool( int thread_cnt_, int queue_size_ )
        {
            this->m_is_terminate = false;
            this->m_queue = new TaskQueue(queue_size_);

            for( int i = 0; i < thread_cnt_; i++ )
            {
                this->m_threads.emplace_back(std::thread(this->m_run, i + 1));
            }
        }

        //----------------------------------------------------------------------
        // デストラクタ
        //----------------------------------------------------------------------
        ~ThreadPool()
        {
            this->m_mtx.lock();
            this->m_is_terminate = true;
            delete this->m_queue;
            this->m_mtx.unlock();

            // 各スレッドに停止を通知
            this->m_cv.notify_all();
            const int size = this->m_threads.size();
            for( int i = 0; i < size; i++ )
            {
                this->m_threads.at(i).join();
            }
        }

        //----------------------------------------------------------------------
        // タスクの追加
        //----------------------------------------------------------------------
        bool add( const std::function<void()> &func )
        {
            this->m_mtx.lock();
            if( this->m_queue->put(func) == false )
            {
                return false;
            }
            this->m_mtx.unlock();

            // タスクが空ではなくなったことをWorkerスレッドに通知
            this->m_cv.notify_all();
            return true;
        }

    private:

        //----------------------------------------------------------------------
        // 各Workerスレッドの実行処理
        //----------------------------------------------------------------------
        std::function<void(int)> m_run = [this]( int id_ )
        {
            while( true )
            {
                std::function<void()> func;
                {
                    std::unique_lock<std::mutex> lock(this->m_mtx);

                    // タスクがなければ待機
                    while( this->m_queue->empty() )
                    {
                        // 停止時
                        if( this->m_is_terminate )
                        {
                            return;
                        }
                        this->m_cv.wait(lock);
                    }

                    if( this->m_queue->get(func) == false )
                    {
                        continue;
                    }
                }

                // タスクの実行
                std::printf("===================\n");
                std::printf("Thread:  %d\n", id_);
                func();
                std::printf("===================\n");
            }
        };

        bool m_is_terminate;
        TaskQueue* m_queue;
        std::mutex m_mtx;
        std::condition_variable m_cv;
        std::vector<std::thread> m_threads;
};

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    ThreadPool tp(5, 100);
    for( int i = 0; i < 100; i++ )
    {
        // Request
        int request_cnt = i + 1;
        auto request = [=] { std::printf("Request: %d\n", request_cnt); };

        while( tp.add(request) == false )
        {
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
        }
    }

    std::this_thread::sleep_for(std::chrono::milliseconds(500));
    return 0;
}
```
