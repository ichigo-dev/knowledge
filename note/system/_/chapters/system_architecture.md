# 『システム構成』ノート

（最終更新： 2023-06-27）


## 目次

1. [冗長化](#冗長化)
1. [単一障害点](#単一障害点)
1. [TSS](#tss)
1. [シンプレックスシステム](#シンプレックスシステム)
1. [デュアルシステム](#デュアルシステム)
1. [デュプレックスシステム](#デュプレックスシステム)
	1. [ホットスタンバイ](#ホットスタンバイ)
	1. [ウォームスタンバイ](#ウォームスタンバイ)
	1. [コールドスタンバイ](#コールドスタンバイ)
1. [バックアップサイト](#バックアップサイト)
1. [マルチプロセッサシステム](#マルチプロセッサシステム)
	1. [対称マルチプロセッサシステム](#対称マルチプロセッサシステム)
	1. [非対称マルチプロセッサシステム](#非対称マルチプロセッサシステム)
	1. [密結合マルチプロセッサシステム](#密結合マルチプロセッサシステム)
	1. [疎結合マルチプロセッサシステム](#疎結合マルチプロセッサシステム)
	1. [クラスタコンピューティング](#クラスタコンピューティング)
1. [ダンデムシステム](#ダンデムシステム)
1. [オンプレミス](#オンプレミス)
1. [クラウドコンピューティング](#クラウドコンピューティング)
	1. [SaaS](#saas)
	1. [PaaS](#paas)
	1. [IaaS](#iaas)
	1. [FaaS](#faas)
	1. [DaaS](#daas)
1. [グリッドコンピューティング](#グリッドコンピューティング)
1. [エッジコンピューティング](#エッジコンピューティング)
1. [仮想化技術](#仮想化技術)
	1. [仮想マシン](#仮想マシン)
	1. [ホストOS型](#ホストos型)
	1. [ハイパーバイザ型](#ハイパーバイザ型)
	1. [コンテナ型](#コンテナ型)
	1. [シンプロビジョニング](#シンプロビジョニング)
	1. [ライブマイグレーション](#ライブマイグレーション)
1. [VDI](#vdi)
1. [ハイパフォーマンスコンピューティング](#ハイパフォーマンスコンピューティング)


## 冗長化

**冗長性**(Redundancy)あるいは**多重性**は[システム](./system.md#システム)に障害が発生した場合に備えて冗長な予備装置を運用する仕組み。[システム](./system.md#システム)に冗長性を持たせることを**冗長化**という。メインの[サーバ](./system_processing_model.md#クライアントサーバシステム)（運用系）に加えて障害時に備えた予備の[サーバ](./system_processing_model.md#クライアントサーバシステム)（待機系）を常に運用しておくのは、冗長化の例である。


## 単一障害点

**単一障害点**(**SPOF**: Single Point Of Failure)は、[システム](./system.md#システム)を構成する要素のうちそこが停止すると[システム](./system.md#システム)全体が停止してしまう部分。[可用性](./system_performance_evaluation.md#可用性)の高い[システム](./system.md#システム)を構成するには、個々の構成要素を[冗長化](#冗長化)して[フェールオーバ](./reliability_design.md#フェールオーバ)の機構を用意しておくなど、単一障害点を減らすことが重要となる。


## TSS

**TSS**（**タイムシェアリングシステム**）は、1台の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)を複数人で同時に使用するための構成。一定時間ごとに[CPU](../../../computer/hardware/_/chapters/processor.md#cpu)の使用権を切り替えながら処理を実行する。


## シンプレックスシステム

**シンプレックスシステム**は、オンラインシステムにおいて、予備機を持たずに1つの[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)で処理を行う構成。処理能力や信頼性は劣るが、運用コストが低くなる。


## デュアルシステム

**デュアルシステム**は、オンラインシステムにおいて、処理するための機器を二重化した構成で、2つの[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)は同じ処理を行う。2系統の演算結果をクロスチェックしながら処理を進めるため信頼性が高く、一方の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)が故障した場合でももう一方で処理を継続できる。


## デュプレックスシステム

**デュプレックスシステム**は、処理系を2つ用意しておき、一方をメインのオンライン処理用、もう一方を[バッチ処理](./system_processing_model.md#バッチ処理)や待機系として用いる構成。信頼性は[デュアルシステム](#デュアルシステム)に比べると劣るが、コスト面では有利である。

### ホットスタンバイ

**ホットスタンバイ**は、運用系の[システム](./system.md#システム)と全く同じ環境にデータを常に同期し続け、障害発生時に即座に待機系（予備の[サーバ](./system_processing_model.md#クライアントサーバシステム)など）を運用系と切り替える手法。保守性を高めることができる一方でコストが大きくなる。

### ウォームスタンバイ

**ウォームスタンバイ**は、待機系の[システム](./system.md#システム)を最小限のリソースで起動しておき、障害発生時に運用系と切り換える手法。切り替えの際には運用系に比べてスペックが落ちるため、切り替え時にリソースを追加する必要がある場合もある。

### コールドスタンバイ

**コールドスタンバイ**は、通常時は待機系の[システム](./system.md#システム)に電源を供給せず、障害発生時に待機系の[システム](./system.md#システム)を起動してリソースなどを用意した後に運用系と切り換える手法。最もコストが低いが復旧までに時間がかかり、保守性が下がってしまう。


## バックアップサイト

**バックアップサイト**は、[システム](./system.md#システム)が稼働不可能な状態となった場合などに備えておくための機器や場所。障害や災害が発生した際に、復旧作業を行うための場所として用いられる。


## マルチプロセッサシステム

**マルチプロセッサシステム**は、複数の[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)を同時に使用することで、[システム](./system.md#システム)全体の処理能力と[耐障害性](./reliability_design.md#フォールトトレランス)を向上させる構成。

### 対称マルチプロセッサシステム

**対称マルチプロセッサシステム**(**SMP**: Symmetric Multiple Processor)は、全ての[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)に対して均一に処理を振り分けて並列処理を行う構成。

### 非対称マルチプロセッサシステム

**非対称マルチプロセッサシステム**(**ASMP**: Asymmetirc Multiple Processor)は、各[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)が個別の役割を持っており、それぞれにかかる負荷が対称とならないような構成。

### 密結合マルチプロセッサシステム

**密結合マルチプロセッサシステム**(**TCMP**: Tightly Coupled Multiprocessor)は、[主記憶](../../../computer/hardware/_/chapters/hardware.md#主記憶装置)を共有しながら1つの[OS](../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)によって制御されるような構成。1つの[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)パッケージに複数の独立した[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)コアが搭載されている**マルチコアプロセッサ**は代表的なTCMPのひとつ。

### 疎結合マルチプロセッサシステム

**疎結合マルチプロセッサシステム**(**LCMP**: Loosely Coupled Multiprocessor)または**クラスタシステム**は、複数の[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)がそれぞれの[主記憶装置](../../../computer/hardware/_/chapters/hardware.md#主記憶装置)を持ち、別々の[OS](../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)によって制御されるような構成。多くの場合、単体で完結して動作する独立した複数の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)を、通信インタフェースを介して連動させて全体をひとつの[システム](./system.md#システム)とする構成となっている。

### クラスタコンピューティング

**クラスタコンピューティング**は、複数の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)を接続して単一の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)として使用する方式で、単に**クラスタ**、**クラスタリング**とも呼ばれる。負荷分散やHPCの手法として用いられる場合が多い。


## ダンデムシステム

**ダンデムシステム**は、それぞれ機能の異なる複数の専用[プロセッサ](../../../computer/hardware/_/chapters/processor.md#プロセッサ)を直列に接続した構成。メッセージ処理専用のフロントエンドプロセッサ、[データベース](../../../development/database/_/chapters/database.md#データベース)管理専用のバックエンドプロセッサなどからなる。


## オンプレミス

**オンプレミス**は、各企業や個人が独自に[サーバ](./system_processing_model.md#クライアントサーバシステム)などのインフラ環境を構築し、利用・運用する方式。


## クラウドコンピューティング

**クラウドコンピューティング**は、[インターネット](../../../network/_/chapters/network_architecture.md#インターネット)を介して、[アプリケーション](../../../computer/software/_/chapters/software.md#応用ソフトウェア)や[データベース](../../../development/database/_/chapters/database.md#データベース)、[ストレージ](../../../computer/hardware/_/chapters/hardware.md#記憶装置)などを利用する形態やサービス。場所や端末を選ばずに[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)に接続することができる。クラウドは[仮想化技術](#仮想化技術)により資源を自動的に提供する仕組みの応用である。

### オーケストレーション

**オーケストレーション**は、[仮想化](#仮想化技術)された[システム](./system.md#システム)全体を必要に応じて自動的に制御する仕組み。

### SaaS

**SaaS**(Software as a Service)は、[サーバ](./system_processing_model.md#クライアントサーバシステム)上の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)にインストールされた[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)を、[インターネット](../../../network/_/chapters/network_architecture.md#インターネット)経由でサービスとしてユーザに提供する形態。ユーザはアカウントさえ持っていれば、[インターネット](../../../network/_/chapters/network_architecture.md#インターネット)を通じてどこからでも[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)を利用することができる。

### PaaS

**PaaS**(Platform as a Service)は、[サーバ](./system_processing_model.md#クライアントサーバシステム)上の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)にインストールされた[ミドルウェア](../../../computer/_/chapters/computer.md#ミドルウェア)などの開発[プラットフォーム](../../../computer/software/_/chapters/software.md#プラットフォーム)を、[インターネット](../../../network/_/chapters/network_architecture.md#インターネット)経由でサービスとしてユーザに提供する形態。開発者は[データベース](../../../development/database/_/chapters/database.md#データベース)やオブジェクトストレージといった[ミドルウェア](../../../computer/_/chapters/computer.md#ミドルウェア)を利用することができる。

### IaaS

**IaaS**(Infrastructure as a Service)は、[ネットワーク](../../../network/_/chapters/network.md#ネットワーク)や[サーバ](./system_processing_model.md#クライアントサーバシステム)（[CPU](../../../computer/hardware/_/chapters/processor.md#cpu)、[メモリ](../../../computer/hardware/_/chapters/memory.md#メモリ)、[ストレージ](../../../computer/hardware/_/chapters/hardware.md#記憶装置)）などのコンピューティングリソースを提供するサービス形態。従来は自社の[オンプレミス](#オンプレミス)環境を利用してITシステムを運用する必要があったが、IaaSを利用することで必要な時に必要なだけコンピューティングリソースを借りることができる。

### FaaS

**FaaS**(Function as a Service)は、サーバレスの状態で[アプリケーション](../../../computer/software/_/chapters/software.md#応用ソフトウェア)開発を可能とするようなサービスの提供形態。サーバレスは、サービス提供者がコンピューティングリソースの管理を行うことで、開発者の負担を減らすことを目的としている。コストの削減やスケーラビリティの確保といった面でも有利。

### DaaS

**DaaS**(Desktop as a Service)は、[VDI](#vdi)を利用することでデスクトップ環境を提供するサービス形態。


## グリッドコンピューティング

**グリッドコンピューティング**は、[インターネット](../../../network/_/chapters/network.md#インターネット)などを介して[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)を結び付け、高い処理能力を得る構成。


## エッジコンピューティング

**エッジコンピューティング**は、サービスを提供する[サーバ](./system_processing_model.md#クライアントサーバシステム)とサービスの利用者の距離が物理的に近くなるような構成。ユーザの使用するIoTデバイスやモバイル端末といった**エッジ**に処理を集中させることで、[ネットワーク](../../../network/_/chapters/network.md#ネットワーク)の[オーバヘッド](./system_performance_evaluation.md#オーバヘッド)を減らすことができる。

### CDN

**CDN**(Content Delivery Network)は、[クラウド](#クラウドコンピューティング)における[エッジコンピューティング](#エッジコンピューティング)の手法のひとつで、[サーバ](./system_processing_model.md#クライアントサーバシステム)を各所に分散して配置することで、[クライアント](./system_processing_model.md#クライアントサーバシステム)に近い[サーバ](./system_processing_model.md#クライアントサーバシステム)からコンテンツを配信する仕組み。[インターネット](../../../network/_/chapters/network.md#インターネット)回線の負荷を軽減するために、動画や音声などの大容量データをCDNによって配信することが多い。


## 仮想化技術

**仮想化技術**は、仮想[OS](../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)を用いて1台の物理[サーバ](./system_processing_model.md#クライアントサーバシステム)上で複数の[仮想マシン](#仮想マシン)を走らせ、それを1台の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)として利用したり、[クラスタリング](#クラスタコンピューティング)により複数台のマシンをひとつにまとめたりする技術。[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)を使って仮想的に必要な資源を割り当てる。

[システム](./system.md#システム)によっては、平日は利用者数が少ないが休日は利用者数が増える、一日の中でアクセスが多い時間帯と少ない時間帯がある、といった負荷のムラがある。仮想化技術は、こういった[ネットワーク](../../../network/_/chapters/network.md#ネットワーク)資源の無駄を減らすために生まれた。

### 仮想マシン

**仮想マシン**(**VM**: Virtual Machine)は、ある[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)環境において、[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)などによって作り出した別の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)環境のこと。別の[OS](../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)向けの[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)を動かしたり、1台の[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)で[冗長化](#冗長化)を実現したりすることができるようになる。エミュレータや[Java仮想マシン](../../../programming/_/chapters/programming_language.md#java)、[IaaS](#iaas)なども仮想マシンのひとつ。

### ホストOS型

**ホストOS型**の[仮想マシン](#仮想マシン)は、[OS](../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)上に[仮想化](#仮想化技術)[アプリケーション](../../../computer/software/_/chapters/software.md#応用ソフトウェア)をインストールし、その上に**ゲストOS**を導入する。ひとつの[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)上に複数の[OS](../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)が存在する状態となるので[オーバヘッド](./system_performance_evaluation.md#オーバヘッド)は大きいが、ゲストOSのカスタマイズが自由にできる利点がある。**VMWare**や**VirtualBox**といった[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)が有名。

### ハイパーバイザ型

**ハイパーバイザ型**の[仮想マシン](#仮想マシン)は、[サーバ](./system_processing_model.md#クライアントサーバシステム)などに直接ハイパーバイザと呼ばれる管理用の[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)をインストールし、その上でゲストOSを動作させる。ホストOSが不要なため処理速度の低下を最低限に抑えられ、複数の[仮想マシン](#仮想マシン)をリソースの無駄なく効率的に動かすことができる。

### コンテナ型

**コンテナ型**の[仮想マシン](#仮想マシン)は、ホストOS上で**コンテナエンジン**を起動しておき、**コンテナイメージ**を作成して独立したコンテナ環境で[アプリケーション](../../../computer/software/_/chapters/software.md#応用ソフトウェア)を動作させる。コンテナ型はゲストOSを必要とせず、それぞれの[アプリケーション](../../../computer/software/_/chapters/software.md#応用ソフトウェア)に対して個別のリソース領域を割り当てる。[ホストOS型](#ホストos型)や[ハイパーバイザ型](#ハイパーバイザ型)に比べてリソースの消費が抑えられ、高速に動作するというメリットがある。[Docker](../../../development/docker/_/chapters/docker.md#docker)や**Podman**といった[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)が有名。

### シンプロビジョニング

**シンプロビジョニング**は、[サーバ](./system_processing_model.md#クライアントサーバシステム)ではなく[ハードディスク](../../../computer/hardware/_/chapters/auxiliary_memory_unit.md#ハードディスク)などの[ストレージ](../../../computer/hardware/_/chapters/hardware.md#記憶装置)を[仮想化](#仮想化技術)する方法。仮想的なディスクドライブを設定することで、[サーバ](./system_processing_model.md#クライアントサーバシステム)は実際の物理的な容量を意識せずに大容量が割り当てられているものとして運用することができる。

### ライブマイグレーション

**ライブマイグレーション**は、仮想サーバで稼働している[OS](../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)や[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)を停止することなく、他の物理[サーバ](./system_processing_model.md#クライアントサーバシステム)へ差し替える技術。[サーバ](./system_processing_model.md#クライアントサーバシステム)障害時に切り替えることで処理を継続することができる。


## VDI

**VDI**（**デスクトップ仮想化**: Virtual Desktop Infrastructure）は、[アプリケーション](../../../computer/software/_/chapters/software.md#応用ソフトウェア)やデータをVDI[サーバ](./system_processing_model.md#クライアントサーバシステム)で管理しておき、[クライアント](./system_processing_model.md#クライアントサーバシステム)から接続して利用する方式。


## サーバコンソリデーション

**サーバコンソリデーション**は、[サーバ](./system_processing_model.md#クライアントサーバシステム)の[仮想化](#仮想化技術)を行うことで物理[サーバ](./system_processing_model.md#クライアントサーバシステム)を統合する方法。[仮想化](#仮想化技術)[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)を利用して、複数の物理[サーバ](./system_processing_model.md#クライアントサーバシステム)を[仮想化](#仮想化技術)し、マイグレーションなどによって1台の物理[サーバ](./system_processing_model.md#クライアントサーバシステム)に統合する。


## ハイパフォーマンスコンピューティング

**ハイパフォーマンスコンピューティング**(**HPC**: High Performance Computing)は、高性能で高速な[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)によって膨大な計算処理を行うことを目的としており、それを実現するための大規模な[ハードウェア](../../../computer/hardware/_/chapters/hardware.md#ハードウェア)や[ソフトウェア](../../../computer/software/_/chapters/software.md#ソフトウェア)によって構成されている。[スーパコンピュータ](../../../computer/_/chapters/computer.md#スーパーコンピュータ)により並列処理を実行することで大幅な高速化を可能としている。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)
