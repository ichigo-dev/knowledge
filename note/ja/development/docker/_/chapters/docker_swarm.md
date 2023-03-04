# 『Docker Swarm』ノート

（最終更新： 2023-03-04）


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
	1. [Docker Swarmサービスの更新](#docker-swarmサービスの更新)
	1. [Docker Swarmサービスの削除](#docker-swarmサービスの削除)
	1. [Docker Swarmサービスのスケール](#docker-swarmサービスのスケール)
1. [Docker Swarmスタック](#docker-swarmスタック)
	1. [Docker Swarmスタックのデプロイ](#docker-swarmスタックのデプロイ)
	1. [Docker Swarmスタックの一覧](#docker-swarmスタックの一覧)
	1. [Docker Swarmスタックの削除](#docker-swarmスタックの削除)


## Docker Swarm

**Docker Swarm**は、Dockerが開発しているオープンソースのコンテナオーケストレーションツール。Docker SwarmがインストールされたホストOSは、Dockerクラスタ構成のマスタノード（**Swarm Manager**）となり、他のDockerがインストールされたワーカノードをまとめる役割を持つ。Docker Swarmのインタフェースは通常のDockerコマンドとほぼ同じで、APIを通じて各ノードのDockerを操作することができる。


## Docker Swarmの起動

`docker swarm init` は、Docker Swarmのマスタノードを起動するコマンド。Swarm Managerのホストで実行する。

```sh
$ docker swarm init
```


## Docker Swarmノードの追加

`docker swarm join` は、Docker Swarmのクラスタに参加するためのコマンド。Swarm Managerが管理する各ワーカノードとなるホストで実行する。トークンには、Docker Swarmの起動時に生成されたものを指定する。

```sh
$ docker swarm join <master node ip>:<master node port> --token <token>
```


## Docker Swarmのトークンの確認

`docker swarm join-token` は、ノードを追加するためのトークンを再度表示するためのコマンド。Swarm Managerのホストで実行することができる。

```sh
$ docker swarm join-token worker
```


## Docker Swarmノードの確認

`docker swarm node ls` は、Swarm Managerが管理するノードを確認するコマンド。

```sh
$ docker swarm node ls
```


## Docker Swarmネットワークの構築

Docker Swarmのマスタノードとワーカノードを接続するには、overlayネットワークを構築する必要がある。

```sh
$ docker network create -d overlay <network>
```


## Docker Swarmサービス

**サービス**は、Docker Swarmが管理するコンテナの単位。

### Docker Swarmサービスの作成

`docker service create` は、Docker Swarmが扱うサービスを作成するコマンド。 `--name` オプションにはサービス名を、 `--hostname` にはコンテナのホスト名を、 `--network` には使用するオーバレイネットワークを、 `--replicas` には起動時のコンテナの複製の数を指定する。また、 `--update-delay` オプションを指定すると、ローリングアップデート時にすべてのレプリカが同時に停止してまわないように遅延時間を設けることができる。

```sh
$ docker service create <image>:<tag> \
> --name <service> \
> --hostname <hostname> \
> --network <network> \
> --replicas <replica num>
```

### Docker Swarmサービスの一覧

`docker service ps` は、Docker Swarmが扱うサービスの一覧を確認するコマンド。

```sh
$ docker service ps <service>
```

### Docker Swarmサービスの更新

`docker service update` は、Docker Swarmのサービスをローリングアップデートするコマンド。

```sh
$ docker service update --image <image>:<tag> <service>
```

### Docker Swarmサービスの削除

`docker service rm` は、Docker Swarmのサービスを破棄するコマンド。

```sh
$ docker service rm <service>
```

### Docker Swarmサービスのスケール

`docker service sclae` は、Docker Swarmのサービスをスケールするコマンド。

```sh
$ docker service sclae <service>=<replica num>
```


## Docker Swarmスタック

**スタック**は、Docker Swarmのサービスをまとめて管理するための単位。スタックの設定には、Docker Composeと同様のYAML形式のファイルを用いる。

### Docker Swarmスタックのデプロイ

`docker stack deploy` は、Docker Swarmのスタックをデプロイするコマンド。スタックの定義ファイルから、サービスを生成する。

```sh
$ docker stack deploy <stack> --compose-file <filename>
```

### Docker Swarmスタックの一覧

`docker stack ls` は、Docker Swarmのスタックを一覧で確認するコマンド。

```sh
$ docker stack ls
```

### Docker Swarmスタックの削除

`docker stack remove` は、Docker Swarmのスタックを削除するコマンド。

```sh
$ docker stack remove <stack>
```
