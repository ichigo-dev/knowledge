# 『Immutable』ノート

（最終更新： 2023-09-21）


## 目次

1. [Immutableパターン](#immutableパターン)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Immutableパターン

**Immutableパターン**は、データへの書き込みを禁止することで、複数の[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)から共有リソースにアクセスしても問題ないようにする[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。


## サンプルプログラム

### C++

[Immutableパターン](#immutableパターン)を実現するためには、取得用の[メソッド](../../../../../programming/_/chapters/object_oriented.md#メソッド)(getter)のみを持つ[クラス](../../../../../programming/_/chapters/object_oriented.md#クラス)を用意し、それを各[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)から参照するようにする。

```cpp
#include <ostream>
#include <thread>

//------------------------------------------------------------------------------
// アプリケーション情報（Immutable）
//------------------------------------------------------------------------------
class AppInfo
{
    public:

        //----------------------------------------------------------------------
        // コンストラクタ
        //----------------------------------------------------------------------
        AppInfo( const char* version_, const char* name_ )
        {
            this->m_version = version_;
            this->m_name = name_;
        }

        //----------------------------------------------------------------------
        // バージョンの取得
        //----------------------------------------------------------------------
        const char* get_version()
        {
            return this->m_version;
        }

        //----------------------------------------------------------------------
        // 名前の取得
        //----------------------------------------------------------------------
        const char* get_name()
        {
            return this->m_name;
        }

    private:

        const char* m_version;
        const char* m_name;
};

//------------------------------------------------------------------------------
// アプリケーション情報の表示
//------------------------------------------------------------------------------
void print_app_info( AppInfo app_info_ )
{
    std::printf("%s: %s\n", app_info_.get_name(), app_info_.get_version());
}

//------------------------------------------------------------------------------
// main
//------------------------------------------------------------------------------
int main()
{
    AppInfo info = AppInfo("1.0.0", "App name");
    std::thread t1(print_app_info, info);
    std::thread t2(print_app_info, info);

    t1.join();
    t2.join();

    return 0;
}
```
