# 『Dockerfile』ノート

（最終更新： 2023-08-20）


## 目次

1. [Dockerfile](#dockerfile)
	1. [ビルドコンテキスト](#ビルドコンテキスト)
	1. [Dockerfileのビルド](#dockerfileのビルド)
	1. [Dockerfileの命令](#dockerfileの命令)
1. [マルチステージビルド](#マルチステージビルド)


## Dockerfile

**Dockerfile**は、[イメージ](./image.md#イメージ)を作成するための手順を示した定義書。[イメージ](./image.md#イメージ)がどのような[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)で構成されているのか、どのような手順で構築されるのかといった情報を含んでおり、[イメージ](./image.md#イメージ)のメンテナンス性を向上させる。

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

**ビルドコンテキスト**は、[イメージ](./image.md#イメージ)の[ビルド](#dockerfileのビルド)時に与える付加情報。[Dockerfile](#dockerfile)内でホストOSを参照する命令において、参照先となる[パス](../../../../computer/software/_/chapters/file_system.md#パス)を指定する。

### Dockerfileのビルド

**docker build**は、[Dockerfile](#dockerfile)から[イメージ](./image.md#イメージ)を生成する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。[コンテキスト](#ビルドコンテキスト)には、[Dockerfile](#dockerfile)が参照するホストOSの[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)[パス](../../../../computer/software/_/chapters/file_system.md#パス)を指定する。デフォルトでは[カレントディレクトリ](../../../../computer/software/_/chapters/file_system.md#カレントディレクトリ)にある `Dockerfile` という名前の[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を元にビルドを行うが、 `-f` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)で任意の[Dockerfile](#dockerfile)を使用できる。また、 `-t` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与して、[イメージ](./image.md#イメージ)名やタグを指定できる。

```sh
# イメージのビルド
$ docker build <context>
$ docker build <context> -t <image>:<tag> -f <dockerfile>

# イメージのビルドの例
$ docker build . -t my_ubuntu:01 -f ./Dockerfile
```

### Dockerfileの命令

[Dockerfile](#dockerfile)内で使用できる命令は以下の通り。

| 命令          | 機能                                                                                                |
| ------------- | --------------------------------------------------------------------------------------------------- |
| `ADD`         | ホストOS上のファイルやディレクトリをコンテナにコピーし、アーカイブファイルを指定した場合は展開する  |
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

**マルチステージビルド**は、[Dockerfile](#dockerfile)内に、処理対象とする[イメージ](./image.md#イメージ)を複数記述して、それらから生成された[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)や[バイナリ](../../../../computer/software/_/chapters/file_system.md#バイナリファイル)を最終的な[Dockerイメージ](./image.md#イメージ)に埋め込む方法。マルチステージビルドを利用することにより、開発用の[Dockerイメージ](./image.md#イメージ)と本番環境用の[Dockerイメージ](./image.md#イメージ)を1つの[Dockerfile](#dockerfile)で記述できたり、[イメージ](./image.md#イメージ)のファイルサイズを減らすことができる。

以下は、[Rust](../../../../programming/rust/_/chapters/rust.md)によって開発環境で[ビルド](#dockerfileのビルド)したリリースバイナリを、リリース用の[イメージ](./image.md#イメージ)にコピーするマルチステージビルド[Dockerfile](#dockerfile)の例。

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

特定の構築ステージ用の[イメージ](./image.md#イメージ)を作成したい場合は、 `docker build` に `--target` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を指定し、[Dockerfile](#dockerfile)に `AS` で指定した名前を渡す。

```sh
$ docker build --target develop .
```
