# 『ボリュームとマウント』ノート

（最終更新： 2023-08-20）


## 目次

1. [Volume](#volume)
	1. [Volumeの作成](#volumeの作成)
	1. [Volumeの一覧](#volumeの一覧)
	1. [Volumeの共有](#volumeの共有)
1. [bind mount](#bind-mount)
1. [tmpfs mount](#tmpfs-mount)


## Volume

**Volume**は、ホストOSが管理するデータを[コンテナ](./container.md#コンテナ)で利用できるようにする機能。Volumeを利用するには、[コンテナ](./container.md#コンテナ)起動時に `--mount` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与し、 `type=volume` を指定する。また、 `readonly` を指定することで、読み取り専用で[マウント](../../../../computer/software/_/chapters/file_system.md#マウント)することができる。 `--mount` の代わりに `-v` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を利用することもできる。

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

**docker volume create**は、[ボリューム](#volume)を作成する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# Volumeの作成
$ docker volume create <volume>

# Volumeの作成の例
$ docker volume create vol_fuga
```

### Volumeの一覧

**docker volume ls**は、[ボリューム](#volume)の一覧を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ dokcer volume ls
DRIVER    VOLUME NAME
local     vol_hoge
local     vol_fuga
```

### Volumeの共有

既存の[ボリューム](#volume)を他の[コンテナ](./container.md#コンテナ)からも参照できるようにするには、[コンテナ](./container.md#コンテナ)起動時に `--volumes-from` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与する。指定する[Volume](#volume)名の末尾に `:ro` を指定することで、読み取り専用で共有することも可能。

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

**bind mount**は、ホストOS上の[デバイスファイル](../../../../computer/linux/_/chapters/file.md#デバイスファイル)や[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)などを[コンテナ](./container.md#コンテナ)から参照するための機能。bind mountを行うには、[コンテナ](./container.md#コンテナ)起動時に `--mount` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与し、 `type=bind` を指定する。また、 `readonly` を指定することで、読み取り専用で[マウント](../../../../computer/software/_/chapters/file_system.md#マウント)することができる。 `--mount` の代わりに `-v` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を利用することもできる。

bind mountを使用すると、ホストOS上の[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)に[コンテナ](./container.md#コンテナ)が干渉できるようになるので、操作には注意が必要。

```sh
# bind mount
$ docker run --mount type=bind,src=<host path>,dst=<container path> <container>
$ docker run -v <host path>:<container path> <container>

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

**tmpfs mount** は、ホストOSの[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)の一部を[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)として[コンテナ](./container.md#コンテナ)から利用する機能。オンメモリの[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)を提供できるため、[コンテナ](./container.md#コンテナ)内で非常に高速な読み書きができるが、[メモリ](../../../../computer/hardware/_/chapters/memory.md#メモリ)上のデータは永続的に利用することはできない。tmpfs mountを行うには、[コンテナ](./container.md#コンテナ)起動時に `--mount` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与し、 `type=tmpfs` を指定する。また、オンメモリの[ファイルシステム](../../../../computer/software/_/chapters/file_system.md#ファイルシステム)の[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)や[権限](../../../../computer/linux/_/chapters/user_and_permission.md#権限)、サイズなどを指定できる。

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
