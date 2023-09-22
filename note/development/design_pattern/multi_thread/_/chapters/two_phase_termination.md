# 『Two-Phase Termination』ノート

（最終更新： 2023-09-22）


## 目次

1. [Two-Phase Terminationパターン](#two-phase-terminationパターン)
	1. [TerminationRequest](#terminationrequest)
	1. [Terminator](#terminator)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Two-Phase Terminationパターン

**Two-Phase Terminationパターン**は、外部から[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)に対して終了要求を送り、[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)が自身の都合の良いタイミングで処理を終えることで、安全に[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)を終了する[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。

Two-Phase Terminationパターンは、[TerminationRequest](#terminationrequest)と[Terminator](#terminator)から構成される。

### TerminationRequest

**TerminationRequest**は、[Two-Phase Terminationパターン](#two-phase-terminationパターン)において、[Terminator](#terminator)へ[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)の終了要求を出す役。[Terminator](#terminator)は即座に[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)を終了するのではなく、都合の良いタイミングまで処理を進めてから終了処理に進む。

### Terminator

**Terminator**は、[Two-Phase Terminationパターン](#two-phase-terminationパターン)において、[TerminationRequest](#terminationrequest)の終了要求を受けて、実際に終了処理を行う役。[Terminator](#terminator)は終了要求を受け取るための[メソッド](../../../../../programming/_/chapters/object_oriented.md#メソッド)と自身が終了要求を受けたかどうかを表すフラグを持っている。


## サンプルプログラム

### C++

外部から[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)の終了要求があった時に、終了フラグを立てておき、タイミングを見て[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)の終了処理を行う例を考える。

```cpp
#include <thread>
#include <functional>

//------------------------------------------------------------------------------
// Terminator
//------------------------------------------------------------------------------
class Thread
{
    public:

        //----------------------------------------------------------------------
        // コンストラクタ
        //----------------------------------------------------------------------
        Thread()
        {
            this->m_is_terminate = false;
            this->m_thread = new std::thread(this->m_run);
        }

        //----------------------------------------------------------------------
        // デストラクタ
        //----------------------------------------------------------------------
        ~Thread()
        {
            this->m_thread->join();
        }

        //----------------------------------------------------------------------
        // 終了を要求
        //----------------------------------------------------------------------
        void request_terminate()
        {
            this->m_is_terminate = true;
            std::printf("terminated\n");
        }

    private:

        //----------------------------------------------------------------------
        // スレッドの実行処理
        //----------------------------------------------------------------------
        std::function<void()> m_run = [this]
        {
            while( true )
            {
                std::printf("running...\n");
                std::this_thread::sleep_for(std::chrono::milliseconds(100));

                if( this->m_is_terminate )
                {
                    return;
                }
            }
        };

        std::thread* m_thread;
        bool m_is_terminate;
};

//------------------------------------------------------------------------------
// main (TerminationRequest)
//------------------------------------------------------------------------------
int main()
{
    Thread th;
    std::this_thread::sleep_for(std::chrono::milliseconds(500));
    th.request_terminate();
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    return 0;
}
```
