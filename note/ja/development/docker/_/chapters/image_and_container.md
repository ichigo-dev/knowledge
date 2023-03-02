# 『イメージとコンテナ』

（最終更新： 2023-03-02）


## 目次


## イメージの入手

`docker pull` は、Docker HubからDockerイメージを入手するコマンド。インターネット環境に繋がった状態で実行すると、イメージのダウンロードが始まる。

```sh
# Docker Hubからイメージを入手
$ docker pull <image>

# イメージ入手の例
$ docker pull ubuntu:22.10
```

## イメージの確認

`docker images` は、入手したイメージ一覧を確認するコマンド。

```sh
# イメージの一覧を確認
$ docker images
REPOSITORY   TAG       IMAGE ID       CREATED       SIZE
ubuntu       22.10     10c2f4041af1   4 weeks ago   70.2MB
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
