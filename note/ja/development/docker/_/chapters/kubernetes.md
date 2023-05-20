# 『Kubernetes』ノートkubernetes

（最終更新： 2023-05-20）


## 目次

1. [コンテナオーケストレーション](#コンテナオーケストレーション)
1. [Kubernetes](#kubernetes)
	1. [Pod](#pod)
	1. [Podの起動](#podの起動)
	1. [Podの状態確認](#podの状態確認)
	1. [Podの削除](#podの削除)
1. [複数のPodの管理](#複数のpodの管理)
	1. [ReplicaSetの利用](#replicasetの利用)
	1. [ReplicaSetの状態確認](#replicasetの状態確認)
	1. [ReplicaSetのスケール](#replicasetのスケール)


## コンテナオーケストレーション

**コンテナオーケストレーション**は、複数の[コンテナ](./container.md#コンテナ)やマルチサーバに導入されている[Docker](./docker.md#docker)の管理、[コンテナ](./container.md#コンテナ)のデプロイなど、[コンテナ](./container.md#コンテナ)をより便利に扱うための機能。


## Kubernetes

**Kubernetes**(**K8s**)は、Googleが開発した[コンテナオーケストレーション](#コンテナオーケストレーション)ツール。複数の物理[サーバ](../../../../computer/_/chapters/computer.md#サーバ)からなるマルチ[ホスト](../../../../network/_/chapters/network.md#ホスト)の[コンテナ](./container.md#コンテナ)環境を統合的に管理するためのフレームワーク。Kubernetesは、マルチ[ホスト](../../../../network/_/chapters/network.md#ホスト)の[コンテナ](./container.md#コンテナ)環境全体を管理する**Kubernetes管理ノード**（マスタノード）と、コンテナが稼働する**管理対象ノード**（ワーカノード）からなる。

Kubernetesは、[Docker](./docker.md#docker)だけではなくcontainerdやcri-oといった[コンテナエンジン](./container.md#コンテナエンジン)もサポートしている。

### Pod

**Pod**は、[管理対象ノード](#kubernetes)上で稼働する複数の[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)[コンテナ](./container.md#コンテナ)をひとまとめにしたものの単位。[Kubernetes](#kubernetes)ではPodを単位として[アプリケーション](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)を管理する。

### ReplicaSet

**ReplicaSet**は、[Kubernetes](#kubernetes)において複数の[Pod](#pod)のレプリカをセットで作成する機能。

### Podの起動

`kubectl run` は、[Pod](#pod)単位で[コンテナ](./container.md#コンテナ)を起動する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ kubectl run <pod> --image=<image>:<tag>
```

### Podの状態確認

`kubectl get pods` は、[Pod](#pod)の状態を一覧で確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ kubectl get pods
```

`kubectl describe pods` は、[Pod](#pod)の詳細を確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ kubectl describe pods <pod>
```


## 複数のPodの管理

[Kubernetes](#kubernetes)では、複数の[Pod](#pod)をYAMLファイルを用いて管理することができる。

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: my-pod0001
spec:
  containers:
  - name: ubuntu-container
    image: ubuntu:22.10
    ports:
    - containerPort: 80
```

YAMLファイルを元に[Pod](#pod)を生成するには、 `kubectl create` に `-f` [オプション](../../../../computer/linux/_/chapters/basic_command.md#オプション)を付与してYAMLファイルの[パス](../../../../computer/software/_/chapters/file_system.md#パス)を指定する。

```sh
$ kubectl create -f <filename>
```

### ReplicaSetの利用

[ReplicaSet](#replicaset)を利用するには、[Pod](#pod)の定義を書いたYAMLファイルを次のように記述する。

```yaml
apiVersion: apps/v1
kind: ReplicaSet
metadata:
  name: webfrontend
  labels:
    app: webfrontend
spec:
  replicas: 3
  selector:
    matchLabels:
      app: webfrontend
  template:
    metadata:
      labels:
        app: webfrontend
    spec:
      containers:
      - name: webfrontend
        image: httpd:latest
        ports:
        - containerPort: 80
```

### ReplicaSetの状態確認

`kubectl get rs` は、[ReplicaSet](#replicaset)の状態を一覧で確認する[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ kubectl get rs
```

### ReplicaSetのスケール

`kubectl scale` は、[ReplicaSet](#replicaset)のスケールを行う[コマンド](../../../../computer/linux/_/chapters/basic_command.md#コマンド)。

```sh
$ kubectl scale --replicas <replica num> -f <filename>
```
