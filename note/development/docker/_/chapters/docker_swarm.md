# 『Docker Swarm』ノート

（最終更新： 2023-08-20）


## 目次

1. [Docker Swarm](#docker-swarm)
1. [Docker Swarmの起動](#docker-swarmの起動)
1. [Docker Swarmノードの追加](#docker-swarmノードの追加)
1. [Docker Swarmのトークンの確認](#docker-swarmのトークンの確認)
1. [Docker Swarmノードの確認](#docker-swarmノードの確認)
1. [Docker Swarmネットワークの構築](#docker-swarmネットワークの構築)
1. [Docker Swarmサービス](#docker-swarmサービス)
	1. [Docker Swarmサービスの作成](#docker-swarmサービスの作成)
	1. [Docker Swarmサービスの一覧](#docker-swarmサービスの一覧)
	1. [Docker Swarmサービスの確認](#docker-swarmサービスの確認)
	1. [Docker Swarmサービスの更新](#docker-swarmサービスの更新)
	1. [Docker Swarmサービスの削除](#docker-swarmサービスの削除)
	1. [Docker Swarmサービスのスケール](#docker-swarmサービスのスケール)
1. [Docker Swarmスタック](#docker-swarmスタック)
	1. [Docker Swarmスタックのデプロイ](#docker-swarmスタックのデプロイ)
	1. [Docker Swarmスタックの一覧](#docker-swarmスタックの一覧)
	1. [Docker Swarmスタックの確認](#docker-swarmスタックの確認)
	1. [Docker Swarmスタックの削除](#docker-swarmスタックの削除)


## Docker Swarm

**Docker Swarm**は、[Docker](./docker.md#docker)が開発している[オープンソース](../../../../computer/software/_/chapters/open_source_software.md#オープンソースソフトウェア)の[コンテナオーケストレーション](./kubernetes.md#コンテナオーケストレーション)ツール。Docker SwarmがインストールされたホストOSは、[Docker](./docker.md#docker)[クラスタ](../../../../system/_/chapters/system_architecture.md#クラスタコンピューティング)構成のマスタノード（**Swarm Manager**）となり、他の[Docker](./docker.md#docker)がインストールされたワーカノードをまとめる役割を持つ。Docker Swarmのインタフェースは通常の[Docker](./docker.md#docker)[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)とほぼ同じで、[API](../../../../computer/software/_/chapters/operating_system.md#api)を通じて各[ノード](../../../../network/_/chapters/network.md#ノード)の[Docker](./docker.md#docker)を操作することができる。


## Docker Swarmの起動

**docker swarm init**は、Docker Swarmのマスタノードを起動する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[Swarm Manager](#docker-swarm)の[ホスト](../../../../network/_/chapters/network.md#ホスト)で実行する。

```sh
$ docker swarm init
```


## Docker Swarmノードの追加

**docker swarm join**は、[Docker Swarm](#docker-swarm)の[クラスタ](../../../../system/_/chapters/system_architecture.md#クラスタコンピューティング)に参加するための[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[Swarm Manager](#docker-swarm)が管理する各ワーカノードとなる[ホスト](../../../../network/_/chapters/network.md#ホスト)で実行する。トークンには、[Docker Swarm](#docker-swarm)の起動時に生成されたものを指定する。

```sh
$ docker swarm join <master node ip>:<master node port> --token <token>
```


## Docker Swarmのトークンの確認

**docker swarm join-token**は、[ノード](../../../../network/_/chapters/network.md#ノード)を追加するためのトークンを再度表示するための[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[Swarm Manager](#docker-swarm)の[ホスト](../../../../network/_/chapters/network.md#ホスト)で実行することができる。

```sh
$ docker swarm join-token worker
```


## Docker Swarmノードの確認

**docker swarm node ls**は、[Swarm Manager](#docker-swarm)が管理する[ノード](../../../../network/_/chapters/network.md#ノード)を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ docker swarm node ls
```


## Docker Swarmネットワークの構築

[Docker Swarm](#docker-swarm)のマスタノードとワーカノードを接続するには、[overlay](./network.md#overlay)[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)を構築する必要がある。

```sh
$ docker network create -d overlay <network>
```


## Docker Swarmサービス

**サービス**は、[Docker Swarm](#docker-swarm)が管理する[コンテナ](./container.md#コンテナ)の単位。

### Docker Swarmサービスの作成

**docker service create**は、[Docker Swarm](#docker-swarm)が扱う[サービス](#docker-swarmサービス)を作成する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。 `--name` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)にはサービス名を、 `--hostname` には[コンテナ](./container.md#コンテナ)の[ホスト](../../../../network/_/chapters/network.md#ホスト)名を、 `--network` には使用する[overlay](./network.md#overlay)[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)を、 `--replicas` には起動時の[コンテナ](./container.md#コンテナ)の複製の数を指定する。また、 `--update-delay` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定すると、ローリングアップデート時にすべてのレプリカが同時に停止してまわないように遅延時間を設けることができる。

```sh
$ docker service create <image>:<tag> \
> --name <service> \
> --hostname <hostname> \
> --network <network> \
> --replicas <replica num>
```

### Docker Swarmサービスの一覧

**docker service ls**は、 [Docker Swarm](#docker-swarm)が扱う[サービス](#docker-swarmサービス)の一覧を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ docker service ls
```

### Docker Swarmサービスの確認

**docker service ps**は、[Docker Swarm](#docker-swarm)が扱う[サービス](#docker-swarmサービス)の状態を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ docker service ps <service>
```

### Docker Swarmサービスの更新

**docker service update**は、[Docker Swarm](#docker-swarm)の[サービス](#docker-swarmサービス)をローリングアップデートする[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ docker service update --image <image>:<tag> <service>
```

### Docker Swarmサービスの削除

**docker service rm**は、[Docker Swarm](#docker-swarm)の[サービス](#docker-swarmサービス)を破棄する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ docker service rm <service>
```

### Docker Swarmサービスのスケール

**docker service scale**は、[Docker Swarm](#docker-swarm)の[サービス](#docker-swarmサービス)をスケールする[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ docker service scale <service>=<replica num>
```


## Docker Swarmスタック

**スタック**は、[Docker Swarm](#docker-swarm)の[サービス](#docker-swarmサービス)をまとめて管理するための単位。スタックの設定には、[Docker Compose](./docker_compose.md#docker-compose)と同様のYAML形式のファイルを用いる。

### Docker Swarmスタックのデプロイ

**docker stack deploy**は、[Docker Swarm](#docker-swarm)の[スタック](#docker-swarmスタック)をデプロイする[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[スタック](#docker-swarmスタック)の定義ファイルから、[サービス](#docker-swarmサービス)を生成する。

```sh
$ docker stack deploy <stack> --compose-file <filename>
```

### Docker Swarmスタックの一覧

**docker stack ls**は、[Docker Swarm](#docker-swarm)の[スタック](#docker-swarmスタック)を一覧で確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ docker stack ls
```

### Docker Swarmスタックの確認

**docker stack ps**は、[Docker Swarm](#docker-swarm)の[スタック](#docker-swarmスタック)の状態を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ docker stack ps <stack>
```

### Docker Swarmスタックの削除

**docker stack remove**は、[Docker Swarm](#docker-swarm)の[スタック](#docker-swarmスタック)を削除する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ docker stack remove <stack>
```
