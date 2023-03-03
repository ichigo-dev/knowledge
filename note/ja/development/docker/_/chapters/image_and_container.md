# 『イメージとコンテナ』ノート

（最終更新： 2023-03-03）


## 目次

1. [イメージの入手](#イメージの入手)
1. [イメージの確認](#イメージの確認)
1. [イメージの削除](#イメージの削除)
1. [コンテナの起動](#コンテナの起動)
	1. [コンテナのOSバージョン](#コンテナのosバージョン)
	1. [コンテナのホスト名](#コンテナのホスト名)
	1. [コンテナのIPアドレス](#コンテナのipアドレス)
	1. [ホスト名指定での起動](#ホスト名指定での起動)
	1. [バックグランドで起動](#バックグラウンドで起動)
	1. [一時的な起動](#一時的な起動)
1. [コンテナの確認](#コンテナの確認)
1. [コンテナのコミット](#コンテナのコミット)
1. [コンテナの削除](#コンテナの削除)
1. [コンテナの起動](#コンテナの起動)
1. [コンテナのアタッチ](#コンテナのアタッチ)
1. [コンテナでのコマンド実行](#コンテナでのコマンド実行)
1. [コンテナIDの管理](#コンテナIDの管理)
1. [コンテナでのsystemdの有効化](#コンテナでのsystemdの有効化)


## イメージの入手

`docker pull` は、Docker HubからDockerイメージを入手するコマンド。インターネット環境に繋がった状態で実行すると、イメージのダウンロードが始まる。

```sh
# Docker Hubからイメージを入手
$ docker pull <image>

# イメージ入手の例
$ docker pull ubuntu:22.10
$ docker pull python:latest
```


## イメージの確認

`docker images` は、入手したイメージ一覧を確認するコマンド。

```sh
# イメージの一覧を確認
$ docker images
REPOSITORY   TAG       IMAGE ID       CREATED          SIZE
ubuntu       22.10     10c2f4041af1   4 weeks ago      70.2MB
python       latest    33ce09363487   33 hours ago     925MB
```


## イメージの削除

`docker rmi` は、イメージを削除するコマンド。イメージを削除するためには、そのイメージを元にしたコンテナを全て停止するか削除する必要がある。ただし、そのイメージを元にした停止中のコンテナがある場合には、 `-f` オプションを付与する必要がある。

```sh
# イメージの削除
$ docker rmi <image>

# イメージの削除の例
$ docker rmi python:latest
```


## コンテナの起動

`docker run` は、イメージを元にしてコンテナを起動するコマンド。

```sh
# コンテナの起動
$ docker run <image>

# コンテナの起動の例
$ docker run ubuntu:22.10
```

`-i` オプションを付与することで、コンテナ起動時に標準入力を受け付けるようになる。 `-t` オプションを付与することで、仮想端末(pseudo-TTY)をコンテナに割り当てる。 `--name` オプションを付与することで、起動するコンテナの名前を指定することができる。

また、 `docker run` の末尾にコマンドを指定することで、コンテナ起動直後にそのコマンドを実行することができる。 `/bin/bash` などを指定して、シェルを起動しておくという使い方が多い。

```sh
# オプションを付与したコンテナの起動
# 実行すると、コンテナ内のシェルに接続される
$ docker run -i -t --name <name> <image> <command>
root@xxxxxxxxxxxx:/#

# よく用いられるオプションを付与した例
$ docker run -i -t --name ubuntu_linux_01 ubuntu:22.10 /bin/bash
root@c2d63875bf6a:/#
```

ここで、シェルの `root@` 以降に並ぶ文字列は、Dockerが自動的に割り当てたコンテナID。

### コンテナのOSバージョン

例で起動したコンテナはUbuntuのため、 `etc/issue` を見ればOSのバージョンを確認できる。イメージのタグに指定されたバージョンとOSのバージョンが一致していることを確認する。

```sh
root@c2d63875bf6a:/# cat /etc/issue
Ubuntu 22.10 \n \l
```

### コンテナのホスト名

コンテナのホスト名は、初期状態で自動的にコンテナIDと同じ文字列が指定される。

```sh
root@c2d63875bf6a:/# hostname
c2d63875bf6a
```

### コンテナのIPアドレス

コンテナのIPアドレスは、 `ip` コマンドによって確認する。DockerのUbuntuイメージには `ip` コマンドは入っていないので、以下のコマンドでインストールする。

```sh
root@c2d63875bf6a:/# apt-get upgrade
root@c2d63875bf6a:/# apt-get update
root@c2d63875bf6a:/# apt-get install iproute2 -y
```

`ip` コマンドによりIPアドレスを確認する。

```sh
root@c2d63875bf6a:/# ip addr show dev eth0 | grep inet
    inet 172.17.0.2/16 brd 172.17.255.255 scope global eth0
```

`ping` コマンドによりホストOSのIPアドレスと通信ができるかを確認する。

```sh
root@c2d63875bf6a:/# apt-get install iputils-ping net-tools
root@c2d63875bf6a:/# ping -c 3 xxx.xxx.xxx.xxx
PING xxx.xxx.xxx.xxx (xxx.xxx.xxx.xxx) 56(84) bytes of data.
64 bytes from xxx.xxx.xxx.xxx: icmp_seq=1 ttl=64 time=0.226 ms
64 bytes from xxx.xxx.xxx.xxx: icmp_seq=2 ttl=64 time=0.112 ms
64 bytes from xxx.xxx.xxx.xxx: icmp_seq=3 ttl=64 time=0.095 ms

--- xxx.xxx.xxx.xxx ping statistics ---
3 packets transmitted, 3 received, 0% packet loss, time 1999ms
rtt min/avg/max/mdev = 0.095/0.144/0.226/0.058 ms
```

続いて、 `nslookup` コマンドを用いて、コンテナから外部のDNSサーバを使って外部ホストの名前解決ができるかを確認する。

```sh
root@c2d63875bf6a:/# apt-get install dnsutils -y
root@c2d63875bf6a:/# nslookup google.com
Server:         172.22.224.1
Address:        172.22.224.1#53

Non-authoritative answer:
Name:   google.com
Address: 142.251.42.142
Name:   google.com
Address: 2404:6800:4004:822::200e
```

### ホスト名指定での起動

コンテナのホスト名には、自動的にコンテナIDが割り当てられるが、コンテナ起動時に `--hostname` または `-h` オプションを付与することで任意のものを指定することもできる。

### バックグラウンドで起動

コンテナをバックグラウンドで起動するには、コンテナ起動時に `--detach` または `-d` オプションを付与する。

### 一時的な起動

コンテナを一時的に利用し、停止後はすぐにコンテナを削除したい場合は、コンテナの起動時に `--rm` オプションを付与する。


## コンテナの確認

`docker ps` は、起動中のコンテナの一覧を確認するコマンド。また、 `-a` オプションをつけることで、停止中のコンテナも表示することができる。

起動中のコンテナは `STATUS` に `Up` と表示され、停止中のコンテナは `Exit` と表示される。

```sh
$ docker ps -a
CONTAINER ID   IMAGE          COMMAND       CREATED        STATUS                       PORTS     NAMES
c2d63875bf6a   ubuntu:22.10   "/bin/bash"   2 hours ago    Exited (137) 6 seconds ago             ubuntu_linux_01
```


## コンテナのコミット

`docker commit` は、コンテナをイメージとして保存するコマンド。コンテナIDとイメージ名、タグを指定する。

```sh
# コンテナのコミット
$ docker commit <container id> <image>:<tag>

# コンテナのコミットの例
$ docker commit c2d63875bf6a my_ubuntu:01
```

コンテナをコミットすると、次のように新しいイメージとしてローカルに保存される。このイメージを元に新しいコンテナを起動することもできる。

```sh
$ docker images
REPOSITORY   TAG       IMAGE ID       CREATED              SIZE
my_ubuntu    01        b6a541e83c8f   About a minute ago   168MB
ubuntu       22.10     10c2f4041af1   5 weeks ago          70.2MB

$ docker run -i -t --detach --name ubuntu_linux_02 my_ubuntu:01 /bin/bash
$ docker ps -a
CONTAINER ID   IMAGE          COMMAND       CREATED         STATUS                        PORTS     NAMES
ffe49c0476d9   my_ubuntu:01   "/bin/bash"   6 seconds ago   Up 5 seconds                            ubuntu_linux_02
c2d63875bf6a   ubuntu:22.10   "/bin/bash"   23 hours ago    Exited (137) 14 minutes ago             ubuntu_linux_01
```


## コンテナの削除

`docker rm` は、停止中のコンテナを削除するコマンド。コンテナのIDあるいはコンテナ名を指定することで削除することができる。起動中のコンテナを停止したい場合は、 `-f` オプションを付与する。

```sh
# コンテナの削除
$ docker rm <container>

# コンテナの削除の例
$ docker rm ubuntu_linux_01
```


## コンテナの起動

`docker start` は、停止中のコンテナを起動するコマンド。

```sh
# コンテナの起動
$ docker start <container>

# コンテナの起動の例
$ docker start ubuntu_linux_02
```


## コンテナの停止

`docker stop` は、コマンドを停止するコマンド。

```sh
# コンテナの停止
$ docker stop <container>

# コンテナの停止の例
$ docker stop ubuntu_linux_02
```


## コンテナのアタッチ

`docker attach` は、バックグラウンドで起動中のコンテナのシェルに接続するコマンド。コンテナ内でシェルが動作していない場合は接続することはできず、 `exit` コマンドを用いるとコンテナは停止する。

```sh
# コンテナのアタッチ
$ docker attach <container>

# コンテナのアタッチの例
$ docker attach ubuntu_linux_02
```


## コンテナでのコマンド実行

`docker exec` は、コンテナ内でコマンドを実行するためのコマンド。シェルを起動してコンテナに接続する際に用いる場合が多い。

```sh
# コンテナでのコマンド実行
$ docker exec <container> <command>

# コンテナでシェルを起動する例
$ docker exec ubuntu_linux_02 /bin/bash
```


## コンテナIDの管理

コンテナIDをシェル変数に格納しておくことで、コンテナの管理が容易になる。コンテナ起動時に、 `docker run` の結果尾をコマンドラインのシェル変数に代入するとよい。

```sh
$ ID_UBUNTU_03=$(docker run -it -d --name ubuntu_linux_03 my_ubuntu:01 /bin/bash)
$ echo $ID_UBUNTU_03
a739176f4844ea9ecb013406915cc68dfb070c0b129fa164d2665a53fdb2d6ed
```
