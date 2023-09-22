# 『Future』ノート

（最終更新： 2023-09-22）


## 目次

1. [Futureパターン](#futureパターン)
	1. [Host](#host)
	1. [VirtualData](#virtualdata)
	1. [RealData](#realdata)
	1. [Future](#future)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Futureパターン

**Futureパターン**は、[非同期プログラミング](../../../../../programming/parallel_programming/_/chapters/asynchronous_processing.md#非同期処理)において、将来のある時点で結果が利用可能になる処理を表現して扱うための[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。準備に時間のかかるデータを別[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)で処理しつつ、そのデータが実際に必要になった段階で準備が完了するのを待機することで、効率よくリソースを利用することを目的としている。

Futureパターンは、[Host](#host)、[VirtualData](#virtualdata)、[RealData](#realdata)、[Future](#future)から構成される。

### Host

**Host**は、[Futureパターン](#futureパターン)において、新しい[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)を作成して[RealData](#realdata)を作成する役。[RealData](#realdata)を要求する[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)には[Future](#future)を返し、[RealData](#realdata)が準備できた時点でその値を利用できるようにする。

### VirtualData

**VirtualData**は、[Futureパターン](#futureパターン)において、[RealData](#realdata)と[Future](#future)を同一視して扱うための[インタフェース](../../../../../programming/_/chapters/object_oriented.md#インタフェース)。

### RealData

**RealData**は、[Futureパターン](#futureパターン)において、[Host](#host)によって処理された実際のデータを表す役。RealDataを用意するのには時間を要するため、[Future](#future)を引換券として[Host](#host)から返却する。

### Future

**Future**は、[Futureパターン](#futureパターン)において、[Host](#host)が[RealData](#realdata)を準備するまでの間、仮で渡される引換券のような役。[RealData](#realdata)が要求された場合は、それが準備できるまで待機する。


## サンプルプログラム

### C++

[C++](../../../../../programming/_/chapters/programming_language.md#c)では、 `promise` と `future` を利用することで容易に[Futureパターン](#futureパターン)を実現することができる。

```cpp
#include <future>
#include <thread>

//------------------------------------------------------------------------------
// RealDataを生成するための関数
//------------------------------------------------------------------------------
int calculate( int a_, int b_ )
{
    std::this_thread::sleep_for(std::chrono::milliseconds(2000));
    return a_ + b_;
}

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    std::promise<int> result_promise;
    std::future<int> future_result = result_promise.get_future();

    std::thread t([&result_promise]
    {
        int result = calculate(10, 20);
        result_promise.set_value(result);
    });

    std::printf("Waiting for the result...\n");
    int result = future_result.get();
    std::printf("Result: %d\n", result);

    t.join();
    return 0;
}
```
