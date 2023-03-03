# 『Dockerの基礎知識』ノート

（最終更新： 2023-03-03）


## 目次

1. [Docker](#docker)
	1. [コンテナ](#コンテナ)
	1. [コンテナエンジン](#コンテナエンジン)
	1. [Dockerイメージ](#dockerイメージ)
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

**Docker**は、2013年にリリースされたOSSで、アプリケーションやOSの開発・配信を行うためのコンテナエンジン。Dockerを利用することで、ソフトウェア開発者はハードウェア資源を意識せずに、容易にアプリケーションの開発・実行環境を構築することができる。ハイパーバイザ型の仮想化ソフトウェアに比べて、ハードウェア資源の消費が少なく、アプリケーションに必要な環境のみを集約して再現することができる。

ソフトウェア開発においては、アプリケーション単位での環境の分離や、開発環境の作成と廃棄を容易に行えるというメリットがある。また、運用管理の面で見ても、ITインフラでDockerを運用することにより、開発したアプリケーションを素早く展開することができるDevOps環境やイミュータブルインフラストラクチャが実現しやすくなる。

### コンテナ

**コンテナ**は、ホストOS上で独立したプロセスとして実行されるアプリケーション環境。OSの基本コマンドやアプリケーションの実行バイナリ、ライブラリなどの実行環境全体をパッケージ化し、それらをホストOS上の分離された空間で実行できる技術。

### コンテナエンジン

**コンテナエンジン**は、コンテナを稼働させ、管理するためのソフトウェア。中でも、実際にコンテナを実行するプログラムのことを**コンテナランタイム**という。

### Dockerイメージ

**Dckerイメージ**は、コンテナの生成時のテンプレートとなるもので、コンテナ内で使用する実行ファイルやライブラリ、コンテナ起動時に実行させたいコマンドなどが含まれる。Dockerイメージにより、アプリケーションの実行環境を容易に入手したり、開発環境と本番環境のランタイムシステムの違いを吸収したりすることができる。また、1つのDockerイメージから複数のコンテナを素早く起動することができるため、アプリケーションのスケールを容易に行うことができるというメリットもある。

### Dockerエンジン

**Dockerエンジン**は、Dockerのコンテナエンジン。デーモンとしてホストOSに常駐し、コンテナの管理や実行を担う。

### Dockerクライアント

**Dockerクライアント**は、Dockerエンジンを操作するためのインタフェース。ホストOS上でデーモンとして起動しているDockerエンジンに対して命令を与える。遠隔でDockerデーモンと通信することも可能。

### Docker Hub

**Docker Hub**は、Dockerイメージのレジストリサービス。Dockerイメージがリポジトリとして管理されており、Dockerイメージの検索や変更履歴の管理、取得、アップロードといったことが可能。

公開されているDockerイメージにマルウェアが含まれていたという事例もあるため、慎重なセキュリティ対策が必要となる側面もある。

### Rootfulモード

**Rootfulモード**は、Dockerのデフォルトのエンジンモード。

### Rootlessモード

**Rootlessモード**は、Dockerをroot権限を必要とせずに一般ユーザとして利用できるモード。


## 名前空間

**名前空間**（**ネームスペース**）は、Linuxが持つ機能で、コンテナをホストOS上で分離（隔離）するために利用されている。プロセスやネットワーク、ディレクトリ、ユーザなどを分離する機能となっている。

| 名前空間 | 概要                                                                                            |
| -------- | ----------------------------------------------------------------------------------------------- |
| UTS      | ホスト名、ドメイン名などを分離                                                            |
| PID      | 実行するプロセスIDを分離し、異なる名前空間のプロセス同士は互いにアクセスできない          |
| マウント | ファイルシステムのマウント                                                                |
| ユーザ   | ユーザやグループの管理を分離                                                              |
| IPC      | 共有メモリやセマフォ、メッセージキューなどを分離                                          |
| Network  | ネットワークデバイスやIPアドレス、ルーティングテーブル、フィルタリング、ポート番号を分離 |


## コンテナ専用OS

**コンテナ専用OS**は、Dockerなどのコンテナ利用に特化して開発された専用OS。コンテナを稼働させることに目的を絞った、アプライアンスOSのため、管理方法も一般的なLinuxとは異なる。アプリケーションや稼働するデーモン、パッケージ管理マネージャなどが削られており、コンテナの稼働に必要最低限のコンポーネントで構成されている。一般のサーバOSに比べて強固なセキュリティ、性能面での優位性、高い保守性を確保できるとされている。

コンテナ専用OSとしては、Red Hat Enterprise Linux CoreOS(RHCOS)、Fedora CoreOS、FlatcarContainer Linux、k3OSといったものが有名。


## Dockerのエディション

### Docker CE

**Docker CE**(Docker Community Edition)は、無償版のDockerエンジンで、コミュニティによってメンテナンスされているため、Docker社やDocker純正製品の提供を行うベンダの保守サポートは得られない。

### MCR

**MCR**(Mirantis Container Runtime)は、Docker EE(Docker Enterprise Edition)をベースとしたエンタープライズ向けのDockerエンジン。Mirantis社が製品提供を行っている。

### Docker Desktop

**Docker Desktop**は、Linux、Windows、macOSに対応したGUI画面付属のデスクトップ版アプリケーション。


## Dockerのインストール

Dockerが既に自身の環境にインストールされているかを確認するには、ターミナルソフトウェア（Macのターミナル、Windowsのコマンドプロンプト等）を起動し、次のコマンドを実行する。

```sh
$ docker --version
Docker version 23.0.1, build a5ee5b1dfc
```

上の例のように、Dockerのバージョン情報が表示されれば、Dockerは既に使用できる状態となっている。コマンドが存在しない旨のエラーが表示される場合は、Dockerをインストールする必要がある。

- [Dockerのインストール](https://docs.docker.com/engine/install)

上記サイトより、自身の環境のOSに応じたインストール手順を実施し、改めて `docker --version` を実行してバージョン情報が出力されることを確認する。

### Dockerエンジンの起動

Dockerエンジンを `systemd` 経由で起動する。

```sh
$ systemctl start docker
$ systemctl enable docker
$ systemctl is-active docker
active
```

### Docker Hubへのログイン

下記URLよりDocker Hubアカウントを作成する。

- [Docker Hub](https://hub.docker.com)

上記URLのサイトで作成したユーザ名とパスワードを用いて、CLIコマンドよりDocker Hubにログインする。

```sh
$ docker login
Login with your Docker ID to push and pull images from Docker Hub. If you don't have a Docker ID, head over to https://hub.docker.com to create one.
Username:
Password:

Login Succeeded
```
