# 『Docker』ノート

（最終更新： 2023-05-18）


## 目次

1. [Docker](#docker)
	1. [Dockerエンジン](#dockerエンジン)
	1. [Dockerクライアント](#dockerクライアント)
	1. [Docker Hub](#docker-hub)
	1. [Rootfulモード](#rootfulモード)
	1. [Rootlessモード](#rootlessモード)
1. [名前空間](#名前空間)
1. [コンテナ専用OS](#コンテナ専用os)
1. [Dockerのエディション](#dockerのエディション)
	1. [Docker CE](#docker-ce)
	1. [MCR](#mcr)
	1. [Docker Desktop](#docker-desktop)
1. [Dockerのインストール](#dockerのインストール)
	1. [Dockerエンジンの起動](#dockerエンジンの起動)
	1. [Docker Hubへのログイン](#docker-hubへのログイン)


## Docker

**Docker**は、2013年にリリースされた[OSS](../../../../computer/software/_/chapters/open_source_software.md#オープンソースソフトウェア)で、[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)や[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)の開発・配信を行うためのコンテナエンジン。Dockerを利用することで、[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)開発者は[ハードウェア](../../../../computer/hardware/_/chapters/hardware.md#ハードウェア)資源を意識せずに、容易に[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の開発・実行環境を構築することができる。[ハイパーバイザ型](../../../../system/_/chapters/system_architecture.md#ハイパーバイザ型)の[仮想化](../../../../system/_/chapters/system_architecture.md#仮想化技術)[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)に比べて、[ハードウェア](../../../../computer/hardware/_/chapters/hardware.md#ハードウェア)資源の消費が少なく、[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)に必要な環境のみを集約して再現することができる。

[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)開発においては、[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)単位での環境の分離や、開発環境の作成と廃棄を容易に行えるというメリットがある。また、運用管理の面で見ても、ITインフラでDockerを運用することにより、開発した[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)を素早く展開することができるDevOps環境やイミュータブルインフラストラクチャが実現しやすくなる。

### Dockerエンジン

**Dockerエンジン**は、[Docker](#docker)のコンテナエンジン。[デーモン](../../../../computer/linux/_/chapters/process_and_job.md#デーモン)としてホストOSに常駐し、[コンテナ](./container.md#コンテナ)の管理や実行を担う。

### Dockerクライアント

**Dockerクライアント**は、[Dockerエンジン](#dockerエンジン)を操作するためのインタフェース。ホストOS上で[デーモン](../../../../computer/linux/_/chapters/process_and_job.md#デーモン)として起動している[Dockerエンジン](#dockerエンジン)に対して命令を与える。遠隔で[Docker](#docker)[デーモン](../../../../computer/linux/_/chapters/process_and_job.md#デーモン)と通信することも可能。

### Docker Hub

**Docker Hub**は、[Dockerイメージ](./image.md#イメージ)のレジストリサービス。[Dockerイメージ](./image.md#イメージ)がリポジトリとして管理されており、[Dockerイメージ](./image.md#イメージ)の検索や変更履歴の管理、取得、アップロードといったことが可能。

公開されている[Dockerイメージ](./image.md#イメージ)にマルウェアが含まれていたという事例もあるため、慎重なセキュリティ対策が必要となる側面もある。

### Rootfulモード

**Rootfulモード**は、[Docker](#docker)のデフォルトのエンジンモード。

### Rootlessモード

**Rootlessモード**は、[Docker](#docker)を[root](../../../../computer/linux/_/chapters/user_and_permission.md#rootユーザ)権限を必要とせずに一般[ユーザ](../../../../computer/linux/_/chapters/user_and_permission.md#ユーザ)として利用できるモード。


## 名前空間

**名前空間**（**ネームスペース**）は、[Linux](../../../../computer/linux/_/chapters/linux.md#linux)が持つ機能で、[コンテナ](./container.md#コンテナ)をホストOS上で分離（隔離）するために利用されている。[プロセス](../../../../computer/linux/_/chapters/process_and_job.md#プロセス)や[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)、[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)、[ユーザ](../../../../computer/linux/_/chapters/user_and_permission.md#ユーザ)などを分離する機能となっている。

| 名前空間 | 概要                                                                                     |
| -------- | ---------------------------------------------------------------------------------------- |
| UTS      | ホスト名、ドメイン名などを分離                                                           |
| PID      | 実行するプロセスIDを分離し、異なる名前空間のプロセス同士は互いにアクセスできない         |
| マウント | ファイルシステムのマウント                                                               |
| ユーザ   | ユーザやグループの管理を分離                                                             |
| IPC      | 共有メモリやセマフォ、メッセージキューなどを分離                                         |
| Network  | ネットワークデバイスやIPアドレス、ルーティングテーブル、フィルタリング、ポート番号を分離 |


## コンテナ専用OS

**コンテナ専用OS**は、[Docker](#docker)などの[コンテナ](./container.md#コンテナ)利用に特化して開発された専用[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)。[コンテナ](./container.md#コンテナ)を稼働させることに目的を絞った、アプライアンスOSのため、管理方法も一般的な[Linux](../../../../computer/linux/_/chapters/linux.md#linux)とは異なる。[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)や稼働する[デーモン](../../../../computer/linux/_/chapters/process_and_job.md#デーモン)、[パッケージマネージャ](../../../../computer/software/_/chapters/package.md#パッケージマネージャ)などが削られており、[コンテナ](./container.md#コンテナ)の稼働に必要最低限のコンポーネントで構成されている。一般の[サーバ](../../../../computer/_/chapters/computer.md#サーバ)[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)に比べて強固なセキュリティ、性能面での優位性、高い保守性を確保できるとされている。

コンテナ専用OSとしては、**Red Hat Enterprise Linux CoreOS**(**RHCOS**)、**Fedora CoreOS**、**FlatcarContainer Linux**、**k3OS**といったものが有名。


## Dockerのエディション

### Docker CE

**Docker CE**(Docker Community Edition)は、無償版の[Dockerエンジン](#dockerエンジン)で、コミュニティによってメンテナンスされているため、Docker社や[Docker](#docker)純正製品の提供を行うベンダの保守サポートは得られない。

### MCR

**MCR**(Mirantis Container Runtime)は、Docker EE(Docker Enterprise Edition)をベースとしたエンタープライズ向けの[Dockerエンジン](#dockerエンジン)。Mirantis社が製品提供を行っている。

### Docker Desktop

**Docker Desktop**は、[Linux](../../../../computer/linux/_/chapters/linux.md#linux)、[Windows](../../../../computer/software/_/chapters/operating_system.md#windows)、[macOS](../../../../computer/software/_/chapters/operating_system.md#macos)に対応した[GUI](../../../../computer/software/_/chapters/software.md#gui)画面付属のデスクトップ版[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)。


## Dockerのインストール

[Docker](#docker)が既に自身の環境にインストールされているかを確認するには、[ターミナルソフトウェア](../../../../computer/linux/_/chapters/shell_and_terminal.md#ターミナル)（[Mac](../../../../computer/software/_/chapters/operating_system.md#macos)の[ターミナル](../../../../computer/linux/_/chapters/shell_and_terminal.md#ターミナル)、[Windows](../../../../computer/software/_/chapters/operating_system.md#windows)のコマンドプロンプト等）を起動し、次の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行する。

```sh
$ docker --version
Docker version 23.0.1, build a5ee5b1dfc
```

上の例のように、[Docker](#docker)の[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)情報が表示されれば、[Docker](#docker)は既に使用できる状態となっている。[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)が存在しない旨のエラーが表示される場合は、[Docker](#docker)をインストールする必要がある。

- [Dockerのインストール](https://docs.docker.com/engine/install)

上記サイトより、自身の環境の[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)に応じたインストール手順を実施し、改めて `docker --version` を実行して[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)情報が出力されることを確認する。

### Dockerエンジンの起動

[Dockerエンジン](#dockerエンジン)を起動するには、 `systemd` に[デーモン](../../../../computer/linux/_/chapters/process_and_job.md#デーモン)として登録する。

```sh
$ systemctl start docker
$ systemctl enable docker
$ systemctl is-active docker
active
```

### Docker Hubへのログイン

下記URLより[Docker Hub](#docker-hub)アカウントを作成する。

- [Docker Hub](https://hub.docker.com)

上記URLのサイトで作成したユーザ名とパスワードを用いて、[CLI](../../../../computer/software/_/chapters/software.md#cui)[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)より[Docker Hub](#docker-hub)にログインする。

```sh
$ docker login
Login with your Docker ID to push and pull images from Docker Hub. If you don't have a Docker ID, head over to https://hub.docker.com to create one.
Username:
Password:

Login Succeeded
```
