# 『Active Object』ノート

（最終更新： 2023-09-22）


## 目次

1. [Active Objectパターン](#active-objectパターン)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Active Objectパターン

**Active Objectパターン**（**Actorパターン**）は、処理をアクティブな[オブジェクト](../../../../../programming/_/chapters/object_oriented.md#オブジェクト)（アクター）に[カプセル化](../../../../../programming/_/chapters/object_oriented.md#カプセル化)し、[メソッド](../../../../../programming/_/chapters/object_oriented.md#メソッド)の呼び出しと実行を[非同期](../../../../../programming/parallel_programming/_/chapters/asynchronous_processing.md#非同期処理)に行う[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。


## サンプルプログラム

### C++

```cpp
#include <functional>
#include <thread>
#include <queue>
#include <mutex>
#include <condition_variable>

//------------------------------------------------------------------------------
// Message
//------------------------------------------------------------------------------
struct Message
{
    std::function<void()> func;
};

//------------------------------------------------------------------------------
// ActiveObject
//------------------------------------------------------------------------------
class ActiveObject
{
    public:

        //----------------------------------------------------------------------
        // コンストラクタ
        //----------------------------------------------------------------------
        ActiveObject()
        {
            this->m_is_terminate = false;
            this->m_thread = std::thread(this->m_run);
        }

        //----------------------------------------------------------------------
        // デストラクタ
        //----------------------------------------------------------------------
        ~ActiveObject()
        {
            this->stop();
        }

        //----------------------------------------------------------------------
        // メソッド呼び出しをキューに追加
        //----------------------------------------------------------------------
        void enqueue( std::function<void()> func_ )
        {
            this->m_mtx.lock();
            this->m_queue.push({func_});
            this->m_cv.notify_one();
            this->m_mtx.unlock();
        }

        //----------------------------------------------------------------------
        // 停止
        //----------------------------------------------------------------------
        void stop()
        {
            if( this->m_thread.joinable() )
            {
                this->enqueue([this]
                {
                    this->m_is_terminate = true;
                });
                this->m_thread.join();
            }
        }

    private:

        //----------------------------------------------------------------------
        // メッセージの処理ループ
        //----------------------------------------------------------------------
        std::function<void()> m_run = [this]()
        {
            while( true )
            {
                Message message;
                {
                    std::unique_lock<std::mutex> lock(this->m_mtx);
                    if( this->m_queue.empty() )
                    {
                        if( this->m_is_terminate )
                        {
                            return;
                        }
                        this->m_cv.wait(lock);
                    }

                    message = this->m_queue.front();
                    this->m_queue.pop();
                }
                message.func();
            }
        };

        std::thread m_thread;
        std::queue<Message> m_queue;
        std::mutex m_mtx;
        std::condition_variable m_cv;
        bool m_is_terminate;
};

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    ActiveObject active_object;

    active_object.enqueue([]
    {
        std::printf("Task 1\n");
    });
    active_object.enqueue([]
    {
        std::printf("Task 2\n");
    });
    active_object.stop();

    return 0;
}
```
