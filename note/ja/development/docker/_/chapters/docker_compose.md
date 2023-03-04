# 『Docker Compose』ノート

（最終更新： 2023-03-04）


## 目次

1. [Docker Compose](#docker-compose)
	1. [Docker Composeの設定](#docker-composeの設定)
1. [Docker Composeによるビルド](#docker-composeによるビルド)
1. [Docker Composeによる起動](#docker-composeによる起動)
1. [Docker Composeによる停止](#docker-composeによる停止)
1. [Docker Composeによるコマンド実行](#docker-composeによるコマンド実行)
1. [Docker Composeによる削除](#docker-composeによる削除)


## Docker Compose

**Docker Compose**は、複数のコンテナを一括で管理、構築、連携することができるツール。複数のコンテナが連携して動作するようなアプリケーションにおいて、それぞれのコンテナのDockerfileやVolume、ネットワークといったものの管理をまとめて行いたい場合などに有用。Docker Composeを利用するには、コンテナの設定を記述したYAMLファイル（ `docker-compose.yml` という名前にするのが一般的）を用意する必要がある。

```yaml
version: '3'

services:

    # nginxコンテナ
    nginx:
        image: 'nginx:latest'
        ports:
            - 80:80
        volumes:
            - ./src:/var/www
            - ./nginx/app.conf:/etc/nginx/conf.d/app.conf

    # phpコンテナ
    php:
        image: 'php-fpm:latest'
        ports:
            - 9000:9000
        volumes:
            - ./src:/var/www
            - ./php/php.ini:/etc/php/php.ini
        depends_on: mysql

    # mysqlコンテナ
    mysq:
        image: 'mysql:latest'
        ports:
            - 3306:3306
        environment:
            MYSQL_ROOT_PASSWORD: 'password'
            MYSQL_DATABASE: 'my_app'
            MYSQL_USER: 'my_app'
            MYSQL_PASSWORD: 'password'
            TZ: 'Asia/Tokyo'
        command:
            mysqld --default-authentication-plugin=mysql_native_password
        volumes:
            - ./mysql/my.cnf:/etc/mysql/conf.d/my.cnf
            - my_app_data:/var/lib/mysql
```

### Docker Composeの設定

Docker Composeにより操作するコンテナやボリューム、ネットワークなどの定義はYAML形式で記述する。ファイル名は `docker-compose.yml` とするのが一般的。

| セクション、設定 | 概要                                                                          |
| ---------------- | ----------------------------------------------------------------------------- |
| `version`        | YAMLファイルに記述されているComposeファイルフォーマットのバージョンを指定する |
| `services`       | サービスコンテナの定義を記述するセクション                                    |
| `build`          | サービスコンテナのビルドコンテキストパスを指定する                            |
| `image`          | サービスコンテナの元となるイメージを指定する                                  |
| `container_name` | サービスコンテナの名前を指定する                                              |
| `command`        | サービスコンテナを起動したときに実行するコマンドを指定する                    |
| `environment`    | サービスコンテナ内で利用する環境変数を指定する                                |
| `volumes`        | サービスコンテナが使用するボリュームやbind mountの設定を記述する              |
| `netowrks`       | サービスコンテナが所属するネットワークを指定する                              |
| `ports`          | ホスト側でリッスンするポートとサービスコンテナ側のポートマッピングを指定する  |
| `tty`            | 疑似TTYの割り当てを指定する（ `true` 指定で起動時に自動で割り当てられる）     |
| `depends_on`     | サービスコンテナ間の依存関係を指定する（依存先のコンテナが先に起動される）    |


## Docker Composeによるビルド

`docker compose build` は、YAMLファイルに記述された定義を元に、Dockerファイルからイメージをビルドするコマンド。

```sh
$ docker compose build
```


## Docker Composeによる起動

`docker compose up` は、YAMLファイルに記述された定義を元に、コンテナを起動するコマンド。 `-d` オプションを付与すると、バックグラウンドで起動する。 `--build` オプションを付与すると、ビルド後に起動する。

```sh
$ docker compose up
$ docker compose up -d
$ docker compose up --build
```

`docker compose start` は、すでに作成されているコンテナを再起動するコマンド。


## Docker Composeによる停止

`docker compose stop` は、YAMLファイルに記述された定義を元に、コンテナを停止するコマンド。

```sh
$ docker compose stop
```


## Docker Composeによるコマンド実行

`docker compose exec` は、起動中のコンテナのサービス名を指定してコマンドを実行するコマンド。

```sh
# Docker Composeによるコマンド実行
$ docker compose exec <service> <command>

# Docker Composeによるコマンド実行の例
$ docker compose exec nginx /bin/bash
```


## Docker Composeによる削除

`docker compose rm` は、停止されているコンテナを削除するコマンド。

```sh
$ docker compose rm
```

`docker compose down` は、起動中のコンテナを停止して削除するコマンド。

```sh
$ docker compose down
```
