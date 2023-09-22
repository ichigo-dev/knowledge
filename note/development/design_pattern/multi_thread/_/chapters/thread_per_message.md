# 『Thread-Per-Message』ノート

（最終更新： 2023-09-22）


## 目次

1. [Thread-Per-Messageパターン](#thread-per-messageパターン)
	1. [Host](#host)
	1. [Helper](#helper)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Thread-Per-Messageパターン

**Thread-Per-Messageパターン**は、時間のかかる処理要求に対して、1つ1つ[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)を割り当てることで、応答速度の向上を目的とする[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。

Thread-Per-Messageパターンは、[Host](#host)と[Helper](#helper)から構成される。

### Host

**Host**は、[Thread-Per-Messageパターン](#thread-per-messageパターン)において、処理要求を受けて新しい[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)を起動する[クラス](../../../../../programming/_/chapters/object_oriented.md#クラス)。また、起動された[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)は[Helper](#helper)を使用して要求を処理する。

### Helper

**Helper**は、[Thread-Per-Messageパターン](#thread-per-messageパターン)において、要求を処理する機能を[Host](#host)に提供する役。[Host](#host)によって起動された[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)は、Helperを利用して処理を実行する。


## サンプルプログラム

### C++

```cpp
#include <thread>
#include <mutex>

//------------------------------------------------------------------------------
// Helper
//------------------------------------------------------------------------------
class Helper
{
    public:

        //----------------------------------------------------------------------
        // コンストラクタ
        //----------------------------------------------------------------------
        Helper()
        {
            this->m_msg_cnt = 0;
        }

        //----------------------------------------------------------------------
        // 処理の実行
        //----------------------------------------------------------------------
        void handle( const char* message_ )
        {
            std::this_thread::sleep_for(std::chrono::milliseconds(500));

            this->m_mtx.lock();
            this->m_msg_cnt++;
            std::printf("%d: %s\n", this->m_msg_cnt, message_);
            this->m_mtx.unlock();
        }

    private:

        std::mutex m_mtx;
        int m_msg_cnt;
};

Helper g_helper;

//------------------------------------------------------------------------------
// Host
//------------------------------------------------------------------------------
class Host
{
    public:

        //----------------------------------------------------------------------
        // スレッドを起動して処理を実行
        //----------------------------------------------------------------------
        void request( const char* message_ )
        {
            std::thread ([=] { g_helper.handle(message_); }).detach();
        }
};

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    Host host;
    host.request("Hello");
    host.request("World");
    host.request("Thread-Per-Message");

    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    return 0;
}
```
