# 『Immutable』ノート

（最終更新： 2023-09-21）


## 目次

1. [Immutableパターン](#immutableパターン)
	1. [Immutable](#immutable)
1. [サンプルプログラム](#サンプルプログラム)
	1. [C++](#c)


## Immutableパターン

**Immutableパターン**は、データへの書き込みを禁止することで、複数の[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)から共有リソースにアクセスしても問題ないようにする[デザインパターン](../../../_/chapters/design_pattern.md#デザインパターン)。

Immutableパターンは[Immutable](#immutable)の役のみで構成される。

### Immutable

**Immutable**は、[Immutableパターン](#immutableパターン)において、変更不可能な[変数](../../../../../programming/_/chapters/variable.md#変数)や[クラス](../../../../../programming/_/chapters/object_oriented.md#クラス)。[クラス](../../../../../programming/_/chapters/object_oriented.md#クラス)を使用する場合は、setter（[メンバ変数](../../../../../programming/_/chapters/object_oriented.md#プロパティ)の変更用の[メソッド](../../../../../programming/_/chapters/object_oriented.md#メソッド)）を持たず、getter（[メンバ変数](../../../../../programming/_/chapters/object_oriented.md#プロパティ)の取得用の[メソッド](../../../../../programming/_/chapters/object_oriented.md#メソッド)）のみを持つように設計する。また、[メンバ変数](../../../../../programming/_/chapters/object_oriented.md#プロパティ)は全て[コンストラクタ](../../../../../programming/_/chapters/object_oriented.md#コンストラクタ)で設定する。


## サンプルプログラム

### C++

[Immutableパターン](#immutableパターン)を実現するためには、取得用の[メソッド](../../../../../programming/_/chapters/object_oriented.md#メソッド)(getter)のみを持つ[クラス](../../../../../programming/_/chapters/object_oriented.md#クラス)を用意し、それを各[スレッド](../../../../../computer/software/_/chapters/operating_system.md#スレッド)から参照するようにする。

```cpp
#include <thread>

//------------------------------------------------------------------------------
// Immutable
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
