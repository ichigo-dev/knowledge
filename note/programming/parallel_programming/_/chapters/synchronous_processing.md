# 『同期処理』ノート

（最終更新： 2023-09-02）


## 目次

1. [レースコンディション](#レースコンディション)
	1. [クリティカルセクション](#クリティカルセクション)
1. [同期処理](#同期処理)
	1. [スピンロック](#スピンロック)
	1. [ミューテックス](#ミューテックス)
	1. [セマフォ](#セマフォ)
	1. [条件変数](#条件変数)
	1. [バリア同期](#バリア同期)
	1. [Readers-Writerロック](#readers-writerロック)
1. [アトミック処理](#アトミック処理)
	1. [Compare and Swap](#compare-and-swap)
	1. [Test and Set](#test-and-set)
	1. [Load-Link/Store-Conditional](#load-linkstore-conditional)
1. [排他制御](#排他制御)


## レースコンディション

**レースコンディション**（**競合状態**）は、複数の[プロセス](./concurrency_and_parallelism.md#プロセス)が並行して共有リソースにアクセスした結果引き起こされる、予期しない異常な状態。並行[プログラミング](../../../_/chapters/programming.md#プログラミング)においては、いかにレースコンディションを引き起こさずに正しく[プログラミング](../../../_/chapters/programming.md#プログラミング)するかが課題の1つとなる。

### クリティカルセクション

**クリティカルセクション**は、[レースコンディション](#レースコンディション)を引き起こす[プログラム](../../../_/chapters/programming.md#プログラミング)の[コード](../../../_/chapters/programming.md#ソースコード)ブロック。クリティカルセクションを正しく実装するためには、[排他制御](#排他制御)などの[同期処理](#同期処理)を適切に使用する必要がある。


## 同期処理

**同期処理**は、複数の[プロセス](./concurrency_and_parallelism.md#プロセス)が協調して動作する際に、[クリティカルセクション](#クリティカルセクション)の実行順序やタイミングを制御するための手段。[レースコンディション](#レースコンディション)やデータの不整合を回避し、安定した並行処理を実現するために用いられる。ただし、過度な同期処理の適用は性能低下を引き起こす場合もあるため、注意が必要となる。同期処理機構には、[ミューテックス](#ミューテックス)や[セマフォ](#セマフォ)、[条件変数](#条件変数)などがある。

### スピンロック

**スピンロック**は、共有リソースのロックが獲得できるまで、空きができているかをポーリングして確認する方法で、[ミューテックス](#ミューテックス)の実装などに用いられる。スピンロック用の[API](../../../../computer/software/_/chapters/middleware.md#api)は典型的に、ロック獲得用とロック解放用の2つが提供されており、それぞれ以下のような処理を行う。

- ロック獲得用: 共有変数への[ポインタ](../../../_/chapters/data_type.md#ポインタ型)を受け取り、[TAS](#test-and-set)を用いてロックを獲得できるまでループする
- ロック解放用: 共有変数への[ポインタ](../../../_/chapters/data_type.md#ポインタ型)を受け取り、[tas_release](#test-and-set)を呼び出す

単純なスピンロックの実装は以下のようになる。

```c
void spinlock_acquire( bool *look )
{
    while( test_and_set(lock) );
}

void spinlock_release( bool *lock )
{
    tas_release(lock);
}
```

[アトミック処理](#アトミック処理)は実行速度上のペナルティが大きいので、[TAS](#test-and-set)をループで呼び出すのではなく、事前にロックの状態をループでチェックしておいてから最後に[TAS](#test-and-set)を呼び出す**Test and Test and Set**(**TTAS**)を利用した実装が多い。この場合の実装は以下のようになる。

```c
void spinlock_acquire( volatile bool *lock )
{
    for(;;)
    {
        while( *lock );
        if( test_and_set(lock) == false )
        {
            break;
        }
    }
}

void spinlock_release( bool *lock )
{
    tas_release(lock);
}
```

### ミューテックス

**ミューテックス**(**mutex**: Mutual exclusion)は、[クリティカルセクション](#クリティカルセクション)を実行可能な[プロセス](./concurrency_and_parallelism.md#プロセス)の数を1つに制限する[同期処理](#同期処理)。共有リソースへのアクセスを制御するためのフラグを使用し、ロックの獲得・解放を行う。ミューテックスは、同時アクセスプロセス数を1にした[セマフォ](#セマフォ)と同じである（**バイナリセマフォ**）。

単純なミューテックスの実装は以下のようになる。

```c
bool lock = false;

void some_func()
{
    retry:
        if( test_and_set(&lock) == false )
        {
            // ロックの獲得
            lock = true;

            // クリティカルセクション
        }
        else
        {
            goto retry;
        }

    // ロックの解放
    tas_releae(&lock);
}
```

### セマフォ

**セマフォ**(**semaphore**)は、[ミューテックス](#ミューテックス)をより一般化したものであり、同時に複数の[プロセス](./concurrency_and_parallelism.md#プロセス)が共有リソースにアクセスできるようにする[同期処理](#同期処理)。複数の[プロセス](./concurrency_and_parallelism.md#プロセス)が共有リソースにアクセスできるため、[ミューテックス](#ミューテックス)では防げた[レースコンディション](#レースコンディション)が防げなくなる可能性がある。

単純なセマフォの実装は以下のようになる。

```c
void semaphore_acquire( volatile int *cnt )
{
    for(;;)
    {
        while( *cnt >= NUM );
        __sync_fetch_and_add(cnt, 1);
        if( *cnt <= NUM )
        {
            break;
        }
        __sync_fetch_and_sub(cnt, 1);
    }
}

void semaphore_release( int *cnt )
{
    __sync_fetch_and_sub(cnt, 1);
}
```

### 条件変数

**条件変数**は、[プロセス](./concurrency_and_parallelism.md#プロセス)間で共有しているリソースが特定の条件を満たすまで待機する[同期処理](#同期処理)。[ミューテックス](#ミューテックス)で保護されるデータ構造が特定条件を満たすまで効率的に待機するためのメカニズムを提供し、実行時の[オーバヘッド](../../../../system/_/chapters/system_performance_evaluation.md#オーバヘッド)も削減することができる。

### バリア同期

**バリア同期**は、[並行処理](./concurrency_and_parallelism.md#並行処理)において、複数の[プロセス](./concurrency_and_parallelism.md#プロセス)を特定のポイントで同期するための[同期処理](#同期処理)。複数の[プロセス](./concurrency_and_parallelism.md#プロセス)で段階別に処理を行う際に有効で、全ての[プロセス](./concurrency_and_parallelism.md#プロセス)が1ステップ目の計算を終了するのを待ち、それが終わったら次のステップに進む、といった使い方がされる。

### Readers-Writerロック

**Readers-Writerロック**は、[レースコンディション](#レースコンディション)の原因となる書き込みに対してのみ[排他制御](#排他制御)を行うような[同期処理](#同期処理)。読み込みのみを行う[プロセス](./concurrency_and_parallelism.md#プロセス)(Reader)と、読み込みと書き込みを行う[プロセス](./concurrency_and_parallelism.md#プロセス)に分類し、以下の制約を満たすように制御を行う。

- ロックを獲得中のReaderが同時に複数存在可能
- ロックを獲得中のWriterが同時に最大1つのみ存在可能
- ReaderとWriterが同時にロック状態にならない


## アトミック処理

**アトミック処理**（**不可分操作**）は、それ以上は分割不可能な処理で、その処理の途中状態を[システム](../../../../system/_/chapters/system.md#システム)的に観測することができず、もしその処理が失敗した場合は完全に処理前の状態に復元される。現代的な[同期処理](#同期処理)のほとんどはこのアトミック処理を利用している。

### Compare and Swap

**Compare and Swap**(**CAS**)は、[アトミック処理](#アトミック処理)のひとつで、共有変数の値を比較して条件が成立する場合に新しい値を設定するという操作。この操作は次のようなステップで構成される。

1. 共有変数の現在の値を読み込む
1. 読み込んだ値と目的の値を比較する
1. もし読み込んだ値と目的の値が一致する場合、新しい値を共有変数に設定する

CASの意味を示した[ソースコード](../../../_/chapters/programming.md#ソースコード)は以下の通りとなる（ただしこの[ソースコード](../../../_/chapters/programming.md#ソースコード)は一般的に[アトミック](#アトミック処理)ではない）。

```c
bool compare_and_swap( uint64_t* p, uint64_t val, uint64_t newval )
{
    if( *p != val )
    {
        return false;
    }
    *p = newval;
    return true;
}
```

### Test and Set

**Test and Set**(**TAS**)は、[アトミック処理](#アトミック処理)のひとつで、共有変数の値を読み込んで、その値を設定するという操作。この操作は次のようなステップで構成される。

1. 共有変数の現在の値を読み込む
1. 読み込んだ値を保存する
1. 新しい値を共有変数に設定する

TASの意味を示した[ソースコード](../../../_/chapters/programming.md#ソースコード)は以下の通りとなる（ただしこの[ソースコード](../../../_/chapters/programming.md#ソースコード)は一般的に[アトミック](#アトミック処理)ではない）。

```c
bool test_and_set( bool *p )
{
    if( *p )
    {
        return true;
    }
    else
    {
        *p = true;
        return false;
    }
}

void tas_release( bool *p )
{
    *p = false;
}
```

### Load-Link/Store-Conditional

**Load-Link/Store-Conditional**(**LL/SC**)は、[アトミック処理](#アトミック処理)のひとつで、[共有メモリ](../../../../computer/linux/_/chapters/process_and_job.md#共有メモリ)へのアクセス制御やロックフリーな構造の実装に利用される。この操作は次のようなステップで構成される。

1. **Load-Link**: [共有メモリ](../../../../computer/linux/_/chapters/process_and_job.md#共有メモリ)の値を読み込み、リンクを設定する
1. 読み込んだ値に対して任意の操作を行う（この間、他の[プロセス](./concurrency_and_parallelism.md#プロセス)も値にアクセスすることができる）
1. **Store-Conditional**: [共有メモリ](../../../../computer/linux/_/chapters/process_and_job.md#共有メモリ)の値を更新する前に、リンクが変化していないかを確認し、変化していなければ更新を行う

Store-Conditionalにおいて、書き込みが失敗した場合にのみ、再度Load-Linkしてから書き込みを試みれば、見かけ上[アトミック](#アトミック処理)な処理を実現できる。


## 排他制御

**排他制御**は、複数の[プロセス](./concurrency_and_parallelism.md#プロセス)が協調して動作する際に、共有リソースや共有データへのあ同時アクセスを制限し、[レースコンディション](#レースコンディション)やデータの不整合を防ぐための手法。排他制御は、正確なタイミングで適切な[同期処理](#同期処理)を使用することによって実現できる。
