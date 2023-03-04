# 『ネットワーク』ノート

（最終更新： 2023-03-04）


## 目次

1. [Dockerのネットワーク](#dockerのネットワーク)
1. [ネットワークドライバ](#ネットワークドライバ)
	1. [bridge](#bridge)
	1. [overlay](#overlay)
	1. [ipvlan](#ipvlan)
	1. [macvlan](#macvlan)
	1. [none](#none)
	1. [デフォルトネットワーク](#デフォルトネットワーク)
1. [ネットワークの割り当て](#ネットワークの割り当て)
1. [ネットワークの作成](#ネットワークの作成)
1. [ネットワークの一覧](#ネットワークの一覧)
1. [ネットワークの削除](#ネットワークの削除)
1. [ネットワークの詳細](#ネットワークの詳細)
1. [ネットワークの接続](#ネットワークの接続)
1. [ネットワークの切断](#ネットワークの切断)


## Dockerのネットワーク

Dockerでは、各コンテナに固有の仮想ネットワークインタフェース（veth）が割り当てられており、それぞれが隔離されたような状態となる。コンテナ間で通信を行いたい場合や、コンテナを外部ネットワークに接続するためには、Dockerが提供する仮想ネットワークを利用する必要がある。

vethを作成すると、2つの仮想NICペアができ、この2ポイント間での通信が可能となる。デフォルトでは、vethペアの一方がコンテナの仮想ネットワークインタフェースとして利用され、もう一方がDockerのブリッジネットワークを介してホストOSのネットワークインタフェースに接続される。


## ネットワークドライバ

**ネットワークドライバ**は、Dockerのコンテナ同士を接続したり、コンテナを外部ネットワークに接続するための接続形式。コンテナ同士を接続したり、コンテナ間でネットワークを分離したりするには、ネットワークドライバを適切に選択する必要がある。

### bridge

**bridge**は、デフォルトのネットワークドライバで、コンテナはブリッジを介してホストOSのネットワーク空間に接続される。vethペアの一方がコンテナの仮想ネットワークインタフェースとして割り当てられ、もう一方がブリッジネットワークを介してホストOSのネットワークインタフェースに接続される。

### host

**host**は、Dockerのネットワークドライバのひとつで、コンテナがホスト側のネットワーク名前空間を共有する方法。Dockerの仮想ネットワークを介することがないため、本番環境における性能の最適化などに利用される。ただし、ホスト側とのポート競合が発生する可能性があるので注意が必要。

### overlay

**overlay**は、Dockerのネットワークドライバのひとつで、マルチホスト環境において複数のDockerデーモン上のコンテナを接続する方法。ネットワークトンネルを使用してホスト間通信を行うことで、コンテナが同じホスト上に存在するかのように動作させることができる。

### ipvlan

**ipvlan**は、Dockerのネットワークドライバのひとつで、IPv4とIPV6の両方のIPアドレスの割り当てをユーザがコントロールできる方法。

### macvlan

**macvlan**は、Dockerのネットワークドライバのひとつで、コンテナに対してMACアドレスを割り当て、ネットワーク上の物理デバイスをシミュレートする方法。コンテナを、ホストOSと同じLANセグメントに所属した、ひとつの物理機器として扱うことができる。

### none

**none**は、Dockerのネットワークドライバのひとつで、コンテナのネットワーク機能を無効化する方法。noneネットワークを割り当てたコンテナは、別のコンテナやホストOS、外部ネットワークには接続できなくなる。

### デフォルトネットワーク

**デフォルトネットワーク**は、Dockerをインストールした際に自動的に作成されるネットワーク。bridgeとhost、noneが作成される。デフォルトのbridgeは、**docker0**という名前になっており、コンテナに対してネットワークを指定しなかった場合はこれが利用される。


## ネットワークの割り当て

コンテナにネットワークを割り当てるには、コンテナ起動時に `--network` オプションを指定する。

```sh
# ネットワークの割り当て
$ docker run <image> --network <network>

# ネットワークの割り当ての例
$ docker run ubuntu:22.10 --network docker01
```


## ネットワークの作成

`docker network create` は、新しいDockerネットワークを作成するコマンド。 `--attachable` オプションを付与すると、手動によるコンテナのアタッチが可能となる。また、 `--subnet` オプションを付与することで、コンテナが所属するサブネットを指定することができる。

```sh
# ネットワークの作成
$ docker network create <network>
$ docker network create <network> --attachable --subnet <subnet>

# ネットワークの作成の例
$ docker network create my_network
$ docker network create my_network --attachable --subnet 172.21.0.0/16
```


## ネットワークの一覧

`docker network ls` は、Docker上に存在するネットワークを一覧で表示するコマンド。

```sh
$ docker network ls
NETWORK ID     NAME                DRIVER    SCOPE
e7d006713a1a   bridge              bridge    local
4d634efa7d98   host                host      local
58494b028369   none                null      local
```


## ネットワークの削除

`docker network rm` は、Docker上に存在するネットワークを削除するコマンド。

```sh
# ネットワークの削除
$ docker network rm <network>

# ネットワークの削除の例
$ docker network rm my_network
```


## ネットワークの詳細

`docker network inspect` は、Docker上に存在するネットワークの詳細を確認するコマンド。

```sh
# ネットワークの詳細確認
$ docker network inspect <network>

# ネットワークの詳細確認の例
$ docker network inspect bridge
[
    {
        "Name": "bridge",
...
        "Labels": {}
    }
]
```


## ネットワークの接続

`docker network connect` は、コンテナをネットワークに接続するコマンド。

```sh
# ネットワークの接続
$ docker network connect <network> <container>

# ネットワークの接続の例
$ docker network connect my_network ubuntu_linux_01
```


## ネットワークの切断

`docker network disconnect` は、コンテナをネットワークから切断するコマンド。

```sh
# ネットワークの切断
$ docker network disconnect <network> <container>

# ネットワークの切断の例
$ docker network disconnect my_network ubuntu_linux_01
```
