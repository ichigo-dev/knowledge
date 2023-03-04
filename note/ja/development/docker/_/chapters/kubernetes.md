# 『Kubernetes』ノートkubernetes

（最終更新： 2023-03-04）


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

**コンテナオーケストレーション**は、複数のコンテナやマルチサーバに導入されているDockerの管理、コンテナのデプロイなど、コンテナをより便利に扱うための機能。


## Kubernetes

**Kubernetes**(**K8s**)は、Googleが開発したコンテナオーケストレーションツール。複数の物理サーバからなるマルチホストのコンテナ環境を統合的に管理するためのフレームワーク。Kubernetesは、マルチホストのコンテナ環境全体を管理する**Kubernetes管理ノード**（マスタノード）と、コンテナが稼働する**管理対象ノード**（ワーカノード）からなる。

Kubernetesは、Dockerだけではなくcontainerdやcri-oといったコンテナエンジンもサポートしている。

### Pod

**Pod**は、管理対象ノード上で稼働する複数のアプリケーションコンテナをひとまとめにしたものの単位。KubernetesではPodを単位としてアプリケーションを管理する。

### ReplicaSet

**ReplicaSet**は、Kubernetesにおいて複数のPodのレプリカをセットで作成する機能。

### Podの起動

`kubectl run` は、Pod単位でコンテナを起動するコマンド。

```sh
$ kubectl run <pod> --image=<image>:<tag>
```

### Podの状態確認

`kubectl get pods` は、Podの状態を一覧で確認するコマンド。

```sh
$ kubectl get pods
```

`kubectl describe pods` は、Podの詳細を確認するコマンド。

```sh
$ kubectl describe pods <pod>
```


## 複数のPodの管理

Kubernetesでは、複数のPodをYAMLファイルを用いて管理することができる。

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

YAMLファイルを元にPodを生成するには、 `kubectl create` に `-f` オプションを付与してYAMLファイルのパスを指定する。

```sh
$ kubectl create -f <filename>
```

### ReplicaSetの利用

ReplicaSetを利用するには、Podの定義を書いたYAMLファイルを次のように記述する。

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

`kubectl get rs` は、ReplicaSetの状態を一覧で確認するコマンド。

```sh
$ kubectl get rs
```

### ReplicaSetのスケール

`kubectl scale` は、ReplicaSetのスケールを行うコマンド。

```sh
$ kubectl scale --replicas <replica num> -f <filename>
```
