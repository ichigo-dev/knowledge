# 『ネットワーク』ノート

（最終更新： 2023-06-27）


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

[Docker](./docker.md#docker)では、各[コンテナ](./container.md#コンテナ)に固有の仮想ネットワークインタフェース（veth）が割り当てられており、それぞれが隔離されたような状態となる。[コンテナ](./container.md#コンテナ)間で通信を行いたい場合や、[コンテナ](./container.md#コンテナ)を外部[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)に接続するためには、[Docker](./docker.md#docker)が提供する仮想ネットワークを利用する必要がある。

vethを作成すると、2つの仮想[NIC](../../../../network/_/chapters/network.md#nic)ペアができ、この2ポイント間での通信が可能となる。デフォルトでは、vethペアの一方が[コンテナ](./container.md#コンテナ)の仮想ネットワークインタフェースとして利用され、もう一方が[Docker](./docker.md#docker)の[ブリッジネットワーク](#bridge)を介してホストOSのネットワークインタフェースに接続される。


## ネットワークドライバ

[Docker](./docker.md#docker)の[コンテナ](./container.md#コンテナ)同士を接続したり、[コンテナ](./container.md#コンテナ)を外部[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)に接続するためのネットワークドライバとして、[bridge](#bridge)や[host](#host)などがある。[コンテナ](./container.md#コンテナ)同士を接続したり、[コンテナ](./container.md#コンテナ)間で[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)を分離したりするには、ネットワークドライバを適切に選択する必要がある。

### bridge

**bridge**は、デフォルトのネットワークドライバで、[コンテナ](./container.md#コンテナ)はブリッジを介してホストOSの[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)空間に接続される。vethペアの一方が[コンテナ](./container.md#コンテナ)の仮想ネットワークインタフェースとして割り当てられ、もう一方がブリッジネットワークを介してホストOSのネットワークインタフェースに接続される。

### host

**host**は、[Docker](./docker.md#docker)のネットワークドライバのひとつで、[コンテナ](./container.md#コンテナ)が[ホスト](../../../../network/_/chapters/network.md#ホスト)側の[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)名前空間を共有する方法。[Docker](./docker.md#docker)の仮想ネットワークを介することがないため、本番環境における性能の最適化などに利用される。ただし、[ホスト](../../../../network/_/chapters/network.md#ホスト)側との[ポート](../../../../network/_/chapters/address_on_network.md#ポート番号)競合が発生する可能性があるので注意が必要。

### overlay

**overlay**は、[Docker](./docker.md#docker)のネットワークドライバのひとつで、マルチ[ホスト](../../../../network/_/chapters/network.md#ホスト)環境において複数の[Dockerデーモン](./docker.md#dockerデーモン)上の[コンテナ](./container.md#コンテナ)を接続する方法。ネットワークトンネルを使用して[ホスト](../../../../network/_/chapters/network.md#ホスト)間通信を行うことで、[コンテナ](./container.md#コンテナ)が同じ[ホスト](../../../../network/_/chapters/network.md#ホスト)上に存在するかのように動作させることができる。

### ipvlan

**ipvlan**は、[Docker](./docker.md#docker)のネットワークドライバのひとつで、[IPv4](../../../../network/_/chapters/internet_layer.md#ipv4)と[IPv6](../../../../network/_/chapters/internet_layer.md#ipv6)の両方の[IPアドレス](../../../../network/_/chapters/address_on_network.md#ipアドレス)の割り当てをユーザがコントロールできる方法。

### macvlan

**macvlan**は、[Docker](./docker.md#docker)のネットワークドライバのひとつで、[コンテナ](./container.md#コンテナ)に対して[MACアドレス](../../../../network/_/chapters/address_on_network.md#macアドレス)を割り当て、[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)上の物理デバイスをシミュレートする方法。[コンテナ](./container.md#コンテナ)を、ホストOSと同じ[LAN](../../../../network/_/chapters/network.md#lan)[セグメント](../../../../network/_/chapters/datalink_layer.md#セグメント)に所属した、ひとつの物理機器として扱うことができる。

### none

**none**は、[Docker](./docker.md#docker)のネットワークドライバのひとつで、[コンテナ](./container.md#コンテナ)の[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)機能を無効化する方法。noneネットワークを割り当てた[コンテナ](./container.md#コンテナ)は、別の[コンテナ](./container.md#コンテナ)やホストOS、外部[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)には接続できなくなる。

### デフォルトネットワーク

**デフォルトネットワーク**は、[Docker](./docker.md#docker)をインストールした際に自動的に作成される[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)。[bridge](#bridge)と[host](#host)、[none](#none)が作成される。デフォルトの[bridge](#bridge)は、**docker0**という名前になっており、[コンテナ](./container.md#コンテナ)に対して[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)を指定しなかった場合はこれが利用される。


## ネットワークの割り当て

[コンテナ](./container.md#コンテナ)に[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)を割り当てるには、[コンテナ](./container.md#コンテナ)起動時に `--network` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定する。

```sh
# ネットワークの割り当て
$ docker run <image> --network <network>

# ネットワークの割り当ての例
$ docker run ubuntu:22.10 --network docker01
```


## ネットワークの作成

`docker network create` は、新しい[Docker](./docker.md#docker)[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)を作成する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。 `--attachable` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与すると、手動による[コンテナ](./container.md#コンテナ)のアタッチが可能となる。また、 `--subnet` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与することで、[コンテナ](./container.md#コンテナ)が所属する[サブネット](../../../../network/_/chapters/address_on_network.md#サブネットワーク)を指定することができる。

```sh
# ネットワークの作成
$ docker network create <network>
$ docker network create <network> --attachable --subnet <subnet>

# ネットワークの作成の例
$ docker network create my_network
$ docker network create my_network --attachable --subnet 172.21.0.0/16
```


## ネットワークの一覧

`docker network ls` は、[Docker](./docker.md#docker)上に存在する[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)を一覧で表示する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ docker network ls
NETWORK ID     NAME                DRIVER    SCOPE
e7d006713a1a   bridge              bridge    local
4d634efa7d98   host                host      local
58494b028369   none                null      local
```


## ネットワークの削除

`docker network rm` は、[Docker](./docker.md#docker)上に存在する[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)を削除する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# ネットワークの削除
$ docker network rm <network>

# ネットワークの削除の例
$ docker network rm my_network
```


## ネットワークの詳細

`docker network inspect` は、[Docker](./docker.md#docker)上に存在する[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)の詳細を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

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

`docker network connect` は、[コンテナ](./container.md#コンテナ)を[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)に接続する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# ネットワークの接続
$ docker network connect <network> <container>

# ネットワークの接続の例
$ docker network connect my_network ubuntu_linux_01
```


## ネットワークの切断

`docker network disconnect` は、[コンテナ](./container.md#コンテナ)を[ネットワーク](../../../../network/_/chapters/network.md#ネットワーク)から切断する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# ネットワークの切断
$ docker network disconnect <network> <container>

# ネットワークの切断の例
$ docker network disconnect my_network ubuntu_linux_01
```
