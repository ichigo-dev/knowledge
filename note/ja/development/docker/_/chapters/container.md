# 『コンテナ』ノート

（最終更新： 2023-08-20）


## 目次

1. [コンテナ](#コンテナ)
	1. [コンテナエンジン](#コンテナエンジン)
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
1. [コンテナIDの管理](#コンテナidの管理)
1. [データコンテナ](#データコンテナ)


## コンテナ

**コンテナ**は、ホストOS上で独立した[プロセス](../../../../computer/linux/_/chapters/process_and_job.md#プロセス)として実行される[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)環境。[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)の基本[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)や[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の実行[バイナリ](../../../../computer/software/_/chapters/file_system.md#バイナリファイル)、[ライブラリ](../../../../computer/software/_/chapters/package.md#ライブラリ)などの実行環境全体を[パッケージ](../../../../computer/software/_/chapters/package.md#パッケージ)化し、それらをホストOS上の分離された空間で実行できる技術。

### コンテナエンジン

**コンテナエンジン**は、[コンテナ](#コンテナ)を稼働させ、管理するための[ソフトウェア](../../../../computer/software/_/chapters/software.md#ソフトウェア)。中でも、実際に[コンテナ](#コンテナ)を実行する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)のことを**コンテナランタイム**という。


## コンテナの起動

**docker run**は、[イメージ](./image.md#イメージ)を元にして[コンテナ](#コンテナ)を起動する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# コンテナの起動
$ docker run <image>

# コンテナの起動の例
$ docker run ubuntu:22.10
```

`-i` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与することで、[コンテナ](#コンテナ)起動時に[標準入力](../../../../computer/linux/_/chapters/process_and_job.md#標準入力)を受け付けるようになる。 `-t` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与することで、[仮想端末](../../../../computer/linux/_/chapters/shell_and_terminal.md#ターミナル)(pseudo-TTY)を[コンテナ](#コンテナ)に割り当てる。 `--name` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与することで、起動する[コンテナ](#コンテナ)の名前を指定することができる。

また、 `docker run` の末尾に[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を指定することで、[コンテナ](#コンテナ)起動直後にその[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行することができる。 `/bin/bash` などを指定して、[シェル](../../../../computer/linux/_/chapters/shell_and_terminal.md#シェル)を起動しておくという使い方が多い。

```sh
# オプションを付与したコンテナの起動
# 実行すると、コンテナ内のシェルに接続される
$ docker run -i -t --name <name> <image> <command>
root@xxxxxxxxxxxx:/#

# よく用いられるオプションを付与した例
$ docker run -i -t --name ubuntu_linux_01 ubuntu:22.10 /bin/bash
root@c2d63875bf6a:/#
```

ここで、[シェル](../../../../computer/linux/_/chapters/shell_and_terminal.md#シェル)の `root@` 以降に並ぶ文字列は、[Docker](./docker.md#docker)が自動的に割り当てたコンテナID。

### コンテナのOSバージョン

[コンテナの起動](#コンテナの起動)の例で起動した[コンテナ](#コンテナ)はUbuntuのため、 `etc/issue` を見れば[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)の[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)を確認できる。[イメージ](./image.md#イメージ)のタグに指定された[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)と[OS](../../../../computer/software/_/chapters/operating_system.md#オペレーティングシステム)の[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)が一致していることを確認する。

```sh
root@c2d63875bf6a:/# cat /etc/issue
Ubuntu 22.10 \n \l
```

### コンテナのホスト名

[コンテナ](#コンテナ)の[ホスト名](../../../../network/_/chapters/internet_layer.md#ホスト名)は、初期状態で自動的にコンテナIDと同じ文字列が指定される。

```sh
root@c2d63875bf6a:/# hostname
c2d63875bf6a
```

### コンテナのIPアドレス

[コンテナ](#コンテナ)の[IPアドレス](../../../../network/_/chapters/address_on_network.md#ipアドレス)は、 `ip` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)によって確認する。[Docker](./docker.md#docker)のUbuntu[イメージ](./image.md#イメージ)には `ip` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)は入っていないので、以下の[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)でインストールする。

```sh
root@c2d63875bf6a:/# apt-get upgrade
root@c2d63875bf6a:/# apt-get update
root@c2d63875bf6a:/# apt-get install iproute2 -y
```

`ip` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)により[IPアドレス](../../../../network/_/chapters/address_on_network.md#ipアドレス)を確認する。

```sh
root@c2d63875bf6a:/# ip addr show dev eth0 | grep inet
    inet 172.17.0.2/16 brd 172.17.255.255 scope global eth0
```

`ping` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)によりホストOSの[IPアドレス](../../../../network/_/chapters/address_on_network.md#ipアドレス)と通信ができるかを確認する。

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

続いて、 `nslookup` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いて、[コンテナ](#コンテナ)から外部の[DNS](../../../../network/_/chapters/internet_layer.md#dns)[サーバ](../../../../computer/_/chapters/computer.md#サーバ)を使って外部[ホスト](../../../../network/_/chapters/network.md#ホスト)の名前解決ができるかを確認する。

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

[コンテナ](#コンテナ)の[ホスト名](../../../../network/_/chapters/internet_layer.md#ホスト名)には、自動的にコンテナIDが割り当てられるが、[コンテナ](#コンテナ)起動時に `--hostname` または `-h` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与することで任意のものを指定することもできる。

### バックグラウンドで起動

[コンテナ](#コンテナ)をバックグラウンドで起動するには、[コンテナ](#コンテナ)起動時に `--detach` または `-d` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与する。

### 一時的な起動

[コンテナ](#コンテナ)を一時的に利用し、停止後はすぐに[コンテナ](#コンテナ)を削除したい場合は、[コンテナ](#コンテナ)の起動時に `--rm` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与する。


## コンテナの一覧

**docker ps**は、起動中の[コンテナ](#コンテナ)の一覧を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。また、 `-a` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)をつけることで、停止中の[コンテナ](#コンテナ)も表示することができる。

起動中の[コンテナ](#コンテナ)は `STATUS` に `Up` と表示され、停止中の[コンテナ](#コンテナ)は `Exit` と表示される。

```sh
$ docker ps -a
CONTAINER ID   IMAGE          COMMAND       CREATED        STATUS                       PORTS     NAMES
c2d63875bf6a   ubuntu:22.10   "/bin/bash"   2 hours ago    Exited (137) 6 seconds ago             ubuntu_linux_01
```

### コンテナの情報

**docker inspect**は、[コンテナ](#コンテナ)の情報を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# コンテナの情報確認
$ docker inspect <container>

# コンテナの情報確認の例
$ docker inspect ubuntu_linux_01
```

### コンテナのリソース使用状況

**docker stats**は、[コンテナ](#コンテナ)のリソース使用状況を追跡する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。 `-a` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定すると、停止している[コンテナ](#コンテナ)を含めてすべてのリソース使用状況を表示する。 `--no-stream` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定すると、リソースの使用状況を追跡するのではなく、一度だけ表示する。 `--no-trunc` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定すると完全なコンテナIDを表示。

```sh
$ docker stats
CONTAINER ID   NAME      CPU %     MEM USAGE / LIMIT   MEM %     NET I/O   BLOCK I/O   PIDS
```

### コンテナのログ

**docker logs**は、[コンテナ](#コンテナ)内で作業した命令をログとして確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。 `-t` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定すると、時刻付きでログを出力できる。

```sh
# コンテナのログの確認
$ docker logs <container>

# コンテナのログの確認の例
$ docker logs ubuntu_linux_02
```


## コンテナのコミット

**docker commit**は、[コンテナ](#コンテナ)を[イメージ](./image.md#イメージ)として保存する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。コンテナIDと[イメージ](./image.md#イメージ)名、タグを指定する。

```sh
# コンテナのコミット
$ docker commit <container id> <image>:<tag>

# コンテナのコミットの例
$ docker commit c2d63875bf6a my_ubuntu:01
```

[コンテナ](#コンテナ)をコミットすると、次のように新しい[イメージ](./image.md#イメージ)としてローカルに保存される。この[イメージ](./image.md#イメージ)を元に新しい[コンテナ](#コンテナ)を起動することもできる。

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

**docker rm**は、停止中の[コンテナ](#コンテナ)を削除する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[コンテナ](#コンテナ)のIDあるいは[コンテナ](#コンテナ)名を指定することで削除することができる。起動中の[コンテナ](#コンテナ)を停止したい場合は、 `-f` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与する。

```sh
# コンテナの削除
$ docker rm <container>

# コンテナの削除の例
$ docker rm ubuntu_linux_01
```


## コンテナの起動

**docker start**は、停止中の[コンテナ](#コンテナ)を起動する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# コンテナの起動
$ docker start <container>

# コンテナの起動の例
$ docker start ubuntu_linux_02
```


## コンテナの停止

**docker stop**は、[コンテナ](#コンテナ)を停止する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# コンテナの停止
$ docker stop <container>

# コンテナの停止の例
$ docker stop ubuntu_linux_02
```


## コンテナのアタッチ

**docker attach**は、バックグラウンドで起動中の[コンテナ](#コンテナ)の[シェル](../../../../computer/linux/_/chapters/shell_and_terminal.md#シェル)に接続する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[コンテナ](#コンテナ)内で[シェル](../../../../computer/linux/_/chapters/shell_and_terminal.md#シェル)が動作していない場合は接続することはできず、 `exit` [コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を用いると[コンテナ](#コンテナ)は停止する。

```sh
# コンテナのアタッチ
$ docker attach <container>

# コンテナのアタッチの例
$ docker attach ubuntu_linux_02
```


## コンテナでのコマンド実行

**docker exec**は、[コンテナ](#コンテナ)内で[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)を実行するための[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[シェル](../../../../computer/linux/_/chapters/shell_and_terminal.md#シェル)を起動して[コンテナ](#コンテナ)に接続する際に用いる場合が多い。

```sh
# コンテナでのコマンド実行
$ docker exec <container> <command>

# コンテナでシェルを起動する例
$ docker exec ubuntu_linux_02 /bin/bash
```


## コンテナのエクスポート

**docker export**は、[コンテナ](#コンテナ)を `tar` 形式の[アーカイブ](../../../../computer/software/_/chapters/multimedia.md#アーカイブ)に変換する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# コンテナのエクスポート
$ docker export <container> > <filename>

# コンテナのエクスポートの例
$ docker export ubuntu_linux_02 > ubuntu_linux_02.tar
```


## コンテナのインポート

**docker import**は、エクスポートした[コンテナ](#コンテナ)の[アーカイブ](../../../../computer/software/_/chapters/multimedia.md#アーカイブ)を元に[イメージ](./image.md#イメージ)を取り込む[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# コンテナのインポート
$ docker import <filename> - <image>:<tag>

# コンテナのインポートの例
$ docker import ubuntu_linux_02.tar - my_ubuntu:02
```


## コンテナIDの管理

コンテナIDを[シェル変数](../../../../computer/linux/_/chapters/shell_and_terminal.md#シェル変数)に格納しておくことで、[コンテナ](#コンテナ)の管理が容易になる。[コンテナ](#コンテナ)起動時に、 `docker run` の結果をコマンドラインの[シェル変数](../../../../computer/linux/_/chapters/shell_and_terminal.md#シェル変数)に代入するとよい。

```sh
$ ID_UBUNTU_03=$(docker run -it -d --name ubuntu_linux_03 my_ubuntu:01 /bin/bash)
$ echo $ID_UBUNTU_03
a739176f4844ea9ecb013406915cc68dfb070c0b129fa164d2665a53fdb2d6ed
```


## データコンテナ

**データコンテナ**は、データ専用の[コンテナ](#コンテナ)。[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の[コンテナ](#コンテナ)とデータの[コンテナ](#コンテナ)を分離して、データの再利用性を高めたり、バックアップやリストアに利用したりする。データコンテナには、**busybox**[イメージ](./image.md#イメージ)がよく用いられる。
