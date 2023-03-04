# 『Dockerfile』ノート

（最終更新： 2023-03-04）


## 目次

1. [Dockerfile](#dockerfile)
	1. [ビルドコンテキスト](#ビルドコンテキスト)
	1. [Dockerfileのビルド](#dockerfileのビルド)
	1. [Dockerfileの命令](#dockerfileの命令)
1. [マルチステージビルド](#マルチステージビルド)


## Dockerfile

**Dockerfile**は、イメージを作成するための手順を示した定義書。イメージがどのようなアプリケーションで構成されているのか、どのような手順で構築されるのかといった情報を含んでおり、イメージのメンテナンス性を向上させる。

```dockerfile
FROM python:3.6

WORKDIR /app
ADD . /app
RUN pip install --trusted-host pypi.python.org -r requirements.txt
EXPOSE 80
ENV DISTRIBUTION dev
CMD ["python", "app.py"]
```

### ビルドコンテキスト

**ビルドコンテキスト**は、イメージのビルド時に与える付加情報。Dockerfile内でホストOSを参照する命令において、参照先となるパスを指定する。

### Dockerfileのビルド

`docker build` は、Dockerfileからイメージを生成するコマンド。コンテキストには、Dockerfileが参照するホストOSのディレクトリパスを指定する。デフォルトではカレントディレクトリにある `Dockerfile` という名前のファイルを元にビルドを行うが、 `-f` オプションで任意のDockerfileを使用できる。また、 `-t` オプションを付与して、イメージ名やタグを指定できる。

```sh
# イメージのビルド
$ docker build <context>
$ docker build <context> -t <image>:<tag> -f <dockerfile>

# イメージのビルドの例
$ docker build . -t my_ubuntu:01 -f ./Dockerfile
```

### Dockerfileの命令

Dockerfile内で使用できる命令は以下の通り。

| 命令          | 機能                                                                                                |
| ------------- | --------------------------------------------------------------------------------------------------- |
| `ADD`         | ホストOS上のファイルやディレクトリをコンテナにコピーし、アーカイブファイルを指定した場合は展開する |
| `ARG`         | `docker build` コマンドの引数を渡す                                                                 |
| `CMD`         | コンテナ内でコマンドを実行する（ `docker run` 時のコマンド指定で上書き）                            |
| `COPY`        | ホストOS上のファイルやディレクトリをコンテナにコピーする                                            |
| `ENTRYPOINT`  | コンテナ内でコマンドを実行する                                                                      |
| `ENV`         | コンテナ内の環境変数を設定する                                                                      |
| `EXPORT`      | コンテナ起動時に指定したポートでリッスンする                                                        |
| `FROM`        | ベースイメージを取得する                                                                            |
| `HEALTHCHECK` | コンテナのアプリケーションの死活監視を行う                                                          |
| `LABEL`       | イメージにメタデータ（ビルド日時、ライセンス、メンテナンス担当者情報など）を付加する                |
| `ONBUILD`     | 親となるDockerfile内の `ONBUILD` で指定したコマンドが、子のDockerfileのビルド時に実行される         |
| `RUN`         | イメージに含まれているコマンドを実行する                                                            |
| `STOPSIGNAL`  | コンテナ終了時に送信するシステムコールシグナルを設定する                                            |
| `USER`        | ユーザを切り替える                                                                                  |
| `VOLUME`      | 指定したディレクトリをVolumeとして扱う                                                              |
| `WORKDIR`     | 作業ディレクトリを変更する                                                                          |


## マルチステージビルド

**マルチステージビルド**は、Dockerfile内に、処理対象とするイメージを複数記述して、それらから生成されたファイルやバイナリを最終的なDockerイメージに埋め込む方法。マルチステージビルドを利用することにより、開発用のDockerイメージと本番環境用のDockerイメージを1つのDockerfileで記述できたり、イメージのファイルサイズを減らすことができる。

以下は、Rustによって開発環境でビルドしたリリースバイナリを、リリース用のイメージにコピーするマルチステージビルドDockerfileの例。

```dockerfile
# ステージ1: 開発環境用
FROM rust:latest AS develop
WORKDIR /app
COPY . .

# ステージ2: ビルド環境用
FROM develop AS build
RUN cargo build --release

# ステージ3: 本番環境用
FROM scratch AS release
COPY --from=build /app/target/release/my_app .
CMD ["./my_app"]
```

特定の構築ステージ用のイメージを作成したい場合は、 `docker build` に `--target` オプションを指定し、Dockerfileに `AS` で指定した名前を渡す。

```sh
$ docker build --target develop .
```
