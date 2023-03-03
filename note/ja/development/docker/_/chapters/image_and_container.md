# 『イメージとコンテナ』ノート

（最終更新： 2023-03-03）


## 目次

1. [イメージの入手](#イメージの入手)
1. [イメージの一覧](#イメージの一覧)
1. [イメージの削除](#イメージの削除)
1. [イメージのセーブ](#イメージのセーブ)
1. [イメージのロード](#イメージのロード)
1. [コンテナの起動](#コンテナの起動)
	1. [コンテナのOSバージョン](#コンテナのosバージョン)
	1. [コンテナのホスト名](#コンテナのホスト名)
	1. [コンテナのIPアドレス](#コンテナのipアドレス)
	1. [ホスト名指定での起動](#ホスト名指定での起動)
	1. [バックグランドで起動](#バックグラウンドで起動)
	1. [一時的な起動](#一時的な起動)
1. [コンテナの一覧](#コンテナの一覧)
	1. [コンテナの情報](#コンテナの情報)
	1. [コンテナのリソース使用状況](#コンテナのリソース使用状況)
	1. [コンテナのログ](#コンテナのログ)
1. [コンテナのコミット](#コンテナのコミット)
1. [コンテナの削除](#コンテナの削除)
1. [コンテナの起動](#コンテナの起動)
1. [コンテナのアタッチ](#コンテナのアタッチ)
1. [コンテナでのコマンド実行](#コンテナでのコマンド実行)
1. [コンテナのエクスポート](#コンテナのエクスポート)
1. [コンテナのインポート](#コンテナのインポート)
1. [コンテナIDの管理](#コンテナIDの管理)
1. [Volume](#volume)
	1. [Volumeの作成](#volumeの作成)
	1. [Volumeの一覧](#volumeの一覧)
	1. [Volumeの共有](#volumeの共有)
1. [bind mount](#bind-mount)
1. [tmpfs mount](#tmpfs-mount)
1. [データコンテナ](#データコンテナ)


## イメージの入手

`docker pull` は、Docker HubからDockerイメージを入手するコマンド。インターネット環境に繋がった状態で実行すると、イメージのダウンロードが始まる。

```sh
# Docker Hubからイメージを入手
$ docker pull <image>

# イメージ入手の例
$ docker pull ubuntu:22.10
$ docker pull python:latest
```


## イメージの一覧

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


## イメージのセーブ

`docker save` は、イメージを `tar` 形式のアーカイブに変換するコマンド。

```sh
# イメージのセーブ
$ docker save <image> -o <filename>

# イメージのセーブの例
$ docker save ubuntu:22.10 -o ubuntu-22.10.tar
```


## イメージのロード

`docker load` は、セーブしたイメージのアーカイブからイメージを取り込むコマンド。

```sh
# イメージのロード
$ docker load -i <filename>

# イメージのロードの例
$ docker load -i ubuntu-22.10.tar
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


## コンテナの一覧

`docker ps` は、起動中のコンテナの一覧を確認するコマンド。また、 `-a` オプションをつけることで、停止中のコンテナも表示することができる。

起動中のコンテナは `STATUS` に `Up` と表示され、停止中のコンテナは `Exit` と表示される。

```sh
$ docker ps -a
CONTAINER ID   IMAGE          COMMAND       CREATED        STATUS                       PORTS     NAMES
c2d63875bf6a   ubuntu:22.10   "/bin/bash"   2 hours ago    Exited (137) 6 seconds ago             ubuntu_linux_01
```

### コンテナの情報

`docker inspect` は、コンテナの情報を確認するコマンド。

```sh
# コンテナの情報確認
$ docker inspect <container>

# コンテナの情報確認の例
$ docker inspect ubuntu_linux_01
```

### コンテナのリソース使用状況

`docker stats` は、コンテナのリソース使用状況を追跡するコマンド。 `-a` オプションを指定すると、停止しているコンテナを含めてすべてのリソース使用状況を表示する。 `--no-stream` オプションを指定すると、リソースの使用状況を追跡するのではなく、一度だけ表示する。 `--no-trunc` オプションを指定すると完全なコンテナIDを表示。

```sh
$ docker stats
CONTAINER ID   NAME      CPU %     MEM USAGE / LIMIT   MEM %     NET I/O   BLOCK I/O   PIDS
```

### コンテナのログ

`docker logs` は、コンテナ内で作業した命令をログとして確認するコマンド。

```sh
# コンテナのログの確認
$ docker logs <container>

# コンテナのログの確認の例
$ docker logs ubuntu_linux_02
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


## コンテナのエクスポート

`docker export` は、コンテナを `tar` 形式のアーカイブに変換するコマンド。

```sh
# コンテナのエクスポート
$ docker export <container> > <filename>

# コンテナのエクスポートの例
$ docker export ubuntu_linux_02 > ubuntu_linux_02.tar
```


## コンテナのインポート

`docker import` は、エクスポートしたコンテナのアーカイブを元にイメージを取り込むコマンド。

```sh
# コンテナのインポート
$ docker import <filename> - <image>:<tag>

# コンテナのインポートの例
$ docker import ubuntu_linux_02.tar - my_ubuntu:02
```


## コンテナIDの管理

コンテナIDをシェル変数に格納しておくことで、コンテナの管理が容易になる。コンテナ起動時に、 `docker run` の結果尾をコマンドラインのシェル変数に代入するとよい。

```sh
$ ID_UBUNTU_03=$(docker run -it -d --name ubuntu_linux_03 my_ubuntu:01 /bin/bash)
$ echo $ID_UBUNTU_03
a739176f4844ea9ecb013406915cc68dfb070c0b129fa164d2665a53fdb2d6ed
```


## Volume

**Volume**は、ホストOSが管理するボリュームをコンテナで利用できるようにする機能。Volumeを利用するには、コンテナ起動時に `--mount` オプションを付与し、 `type=volume` を指定する。また、 `readonly` を指定することで、読み取り専用でマウントすることができる。 `--mount` の代わりに `-v` オプションを利用することもできる。

Volumeは基本的に、ホストOS上の `/var/lib/docker/volumes` 以下に作成される。

```sh
# Volumeの利用
$ docker run --mount type=volume,src=<volume>,dst=<container path> <container>
$ docker run -v <volume>:<container path> <container>

# Volume名未指定だと、自動でハッシュが割り当てられる
$ docker run -v <container path> <container>

# Volumeの利用の例
$ docker run -it --name ubuntu_linux_04 -h ubuntu_linux_04 \
> --mount type=volume,src=vol_hoge,dst=/root/hoge_dir \
> ubuntu:22.10 /bin/bash

# Volumeの利用の例（読み取り専用）
$ docker run -it --name ubuntu_linux_04 -h ubuntu_linux_04 \
> --mount type=volume,src=vol_hoge,dst=/root/hoge_dir,readonly \
> ubuntu:22.10 /bin/bash
```

### Volumeの作成

`docker volume create` は、ボリュームを作成するコマンド。

```sh
# Volumeの作成
$ docker volume create <volume>

# Volumeの作成の例
$ docker volume create vol_fuga
```

### Volumeの一覧

`docker volume ls` は、ボリュームの一覧を確認するコマンド。

```sh
$ dokcer volume ls
DRIVER    VOLUME NAME
local     vol_hoge
local     vol_fuga
```

### Volumeの共有

既存のVolumeを他のコンテンからも参照できるようにするには、コンテナ起動時に `--volumes-from` オプションを付与する。指定するVolume名の末尾に `:ro` を指定することで、読み取り専用で共有することも可能。

```sh
# Volumeの共有
$ docker run --volumes-from <volume> <container>

# Volumeの共有の例
$ docker run -it --name ubuntu_linux_04_shared -h ubuntu_linux_04_shared \
> --volumes-from vol_hoge \
> ubuntu:22.10 /bin/bash

# Volumeの共有の例（読み取り専用）
$ docker run -it --name ubuntu_linux_04_shared -h ubuntu_linux_04_shared \
> --volumes-from vol_hoge:ro \
> ubuntu:22.10 /bin/bash
```


## bind mount

**bind mount**は、ホストOS上のデバイスファイルやディレクトリなどをコンテナから参照するための機能。bind mountを行うには、コンテナ起動時に `--mount` オプションを付与し、 `type=bind` を指定する。また、 `readonly` を指定することで、読み取り専用でマウントすることができる。

bind mountを使用すると、ホストOS上のファイルシステムにコンテナが干渉できるようになるので、操作には注意が必要。

```sh
# bind mount
$ docker run --mount type=bind,src=<host path>,dst=<container path> <container>

# bind mountの例
$ docker run -it --name ubuntu_linux_05 -h ubuntu_linux_05 \
> --mount type=bind,src=$HOME/hoge_dir,dst=/root/hoge_dir \
> ubuntu:22.10 /bin/bash

# 読み取り専用でbind mountする例
$ docker run -it --name ubuntu_linux_05 -h ubuntu_linux_05 \
> --mount type=bind,src=$HOME/hoge_dir,dst=/root/hoge_dir,readonly \
> ubuntu:22.10 /bin/bash
```


## tmpfs mount

**tmpfs mount** は、ホストOSのメモリの一部をファイルシステムとしてコンテナから利用する機能。インメモリのファイルシステムを提供できるため、コンテナ内で非常に高速な読み書きができるが、メモリ上のデータは永続的に利用することはできない。tmpfs mountを行うには、コンテナ起動時に `--mount` オプションを付与し、 `type=tmpfs` を指定する。また、インメモリのファイルシステムのディレクトリや権限、サイズなどを指定できる。

```sh
# tmpfs mount
$ docker run --mount \
> type=tmpfs,\
> dst=<container path>,\   # コンテナ内のマウント先
> tmpfs-mode=<mode>,\      # dstで指定したディレクトリの権限をスティッキービット付きで設定
> tmpfs-size=<size>,\      # dstで指定したディレクトリのサイズをバイト単位で指定
> <container>

# tmpfs mountの例
$ docker run -it --name ubuntu_linux_06 -h ubuntu_linux_06 --mount \
> type=tmpfs,\
> dst=/root,\
> tmpfs-mode=1770,\
> tmpfs-size=42949672960,\
> ubuntu:22.10 /bin/bash
```


## データコンテナ

**データコンテナ**は、データ専用のコンテナ。アプリケーションのコンテナとデータのコンテナを分離して、データの再利用性を高めたり、バックアップやリストアに利用したりする。データコンテナには、 `busybox` イメージがよく用いられる。
