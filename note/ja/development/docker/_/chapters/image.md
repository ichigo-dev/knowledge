# 『イメージ』ノート

（最終更新： 2023-05-18）


## 目次

1. [イメージ](#イメージ)
1. [イメージの入手](#イメージの入手)
1. [イメージの一覧](#イメージの一覧)
1. [イメージの削除](#イメージの削除)
1. [イメージのセーブ](#イメージのセーブ)
1. [イメージのロード](#イメージのロード)


## イメージ

**イメージ**は、[コンテナ](./container.md#コンテナ)の生成時のテンプレートとなるもので、[コンテナ](./container.md#コンテナ)内で使用する[実行ファイル](../../../../computer/software/_/chapters/file_system.md#実行ファイル)や[ライブラリ](../../../../computer/software/_/chapters/package.md#ライブラリ)、[コンテナ](./container.md#コンテナ)起動時に実行させたい[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)などが含まれる。Dockerイメージにより、[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)の実行環境を容易に入手したり、開発環境と本番環境のランタイムシステムの違いを吸収したりすることができる。また、1つのDockerイメージから複数の[コンテナ](./container.md#コンテナ)を素早く起動することができるため、[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)のスケールを容易に行うことができるというメリットもある。


## イメージの入手

`docker pull` は、[Docker Hub](./docker.md#docker-hub)から[Dockerイメージ](#イメージ)を入手する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[インターネット](../../../../network/_/chapters/network.md#インターネット)環境に繋がった状態で実行すると、[イメージ](#イメージ)のダウンロードが始まる。

```sh
# Docker Hubからイメージを入手
$ docker pull <image>

# イメージ入手の例
$ docker pull ubuntu:22.10
$ docker pull python:latest
```


## イメージの一覧

`docker images` は、入手した[イメージ](#イメージ)一覧を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# イメージの一覧を確認
$ docker images
REPOSITORY   TAG       IMAGE ID       CREATED          SIZE
ubuntu       22.10     10c2f4041af1   4 weeks ago      70.2MB
python       latest    33ce09363487   33 hours ago     925MB
```


## イメージの削除

`docker rmi` は、[イメージ](#イメージ)を削除する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[イメージ](#イメージ)を削除するためには、その[イメージ](#イメージ)を元にした[コンテナ](./container.md#コンテナ)を全て停止するか削除する必要がある。ただし、その[イメージ](#イメージ)を元にした停止中の[コンテナ](./container.md#コンテナ)がある場合には、 `-f` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与する必要がある。

```sh
# イメージの削除
$ docker rmi <image>

# イメージの削除の例
$ docker rmi python:latest
```


## イメージのセーブ

`docker save` は、[イメージ](#イメージ)を `tar` 形式の[アーカイブ](../../../../computer/software/_/chapters/multimedia.md#アーカイブ)に変換する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# イメージのセーブ
$ docker save <image> -o <filename>

# イメージのセーブの例
$ docker save ubuntu:22.10 -o ubuntu-22.10.tar
```


## イメージのロード

`docker load` は、[セーブ](#イメージのセーブ)した[イメージ](#イメージ)の[アーカイブ](../../../../computer/software/_/chapters/multimedia.md#アーカイブ)から[イメージ](#イメージ)を取り込む[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
# イメージのロード
$ docker load -i <filename>

# イメージのロードの例
$ docker load -i ubuntu-22.10.tar
```
