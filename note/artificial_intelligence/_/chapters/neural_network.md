# 『ニューラルネットワーク』ノート

（最終更新： 2023-05-10）


## 目次

1. [パーセプトロン](#パーセプトロン)
	1. [ニューロン](#ニューロン)
1. [ニューラルネットワーク](#ニューラルネットワーク)
	1. [ノード](#ノード)
	1. [エッジ](#エッジ)
	1. [重み](#重み)
	1. [バイアス](#バイアス)
	1. [入力層](#入力層)
	1. [中間層](#中間層)
	1. [出力層](#出力層)
	1. [3層ニューラルネットワーク](#3層ニューラルネットワーク)
	1. [ディープラーニング](#ディープラーニング)
1. [順伝搬](#順伝搬)
1. [誤差逆伝搬法](#誤差逆伝搬法)
1. [勾配降下法](#勾配降下法)
	1. [大域最適解](#大域最適解)
	1. [局所最適解](#局所最適解)
	1. [SGD](#sgd)
	1. [モーメンタム](#モーメンタム)
	1. [Adagrad](#adagrad)
	1. [RMSprop](#rmsprop)
	1. [Adam](#adam)
1. [勾配消失問題](#勾配消失問題)
1. [勾配爆発問題](#勾配爆発問題)
1. [活性化関数](#活性化関数)
	1. [ステップ関数](#ステップ関数)
	1. [シグモイド関数](#シグモイド関数)
	1. [tanh関数](#tanh関数)
	1. [ReLU関数](#relu関数)
1. [ドロップアウト](#ドロップアウト)
1. [転移学習](#転移学習)
	1. [ファインチューニング](#ファインチューニング)
	1. [負の転移](#負の転移)
	1. [ドメイン混合](#ドメイン混合)
	1. [マルチタスク学習](#マルチタスク学習)
	1. [One-shot学習](#one-shot学習)
	1. [Zero-shot学習](#zero-shot学習)
1. [パターン認識](#パターン認識)
1. [自然言語処理](#自然言語処理)
	1. [分散表現](#分散表現)


## パーセプトロン

**パーセプトロン**は、単一の[ニューロン](#ニューロン)を[モデル](./machine_learning.md#学習モデル)化したもの。パーセプトロンでは、複数の入力に対してそれぞれ[重み](#重み)をかけたものの和をとる。また、この時[バイアス](#バイアス)を足し合わせる場合もある。こうして求められた和に対して、[活性化関数](#活性化関数)と呼ばれる非線形関数を適用し、最終的な出力を求める。

### ニューロン

**ニューロン**は、人間の神経回路。


## ニューラルネットワーク

**ニューラルネットワーク**(**NN**: Neural Network)は、人間の脳の神経回路を模した[学習モデル](./machine_learning.md#学習モデル)で、[パーセプトロン](#パーセプトロン)を連結した構造となっている。

### ノード

**ノード**は、[ニューラルネットワーク](#ニューラルネットワーク)における、単体の[パーセプトロン](#パーセプトロン)。

### エッジ

**エッジ**は、[ニューラルネットワーク](#ニューラルネットワーク)における、[ノード](#ノード)と[ノード](#ノード)を接続している部分。各エッジはそれぞれ[重み](#重み)を持っている。

### 重み

**重み**は、[ニューラルネットワーク](#ニューラルネットワーク)における[パラメータ](./machine_learning.md#パラメータ)で、[ノード](#ノード)と[ノード](#ノード)の結びつきの強さ。重みが大きいほど後段の[ノード](#ノード)への影響が大きくなる。

### バイアス

**バイアス**は、[ニューラルネットワーク](#ニューラルネットワーク)における[パラメータ](./machine_learning.md#パラメータ)で、各[ノード](#ノード)で入力に関係なく加えられる項。

### 入力層

**入力層**は、[ニューラルネットワーク](#ニューラルネットワーク)に入力される[特徴量](./machine_learning.md#特徴量)を受け付けるレイヤ。

### 中間層

**中間層**（**隠れ層**）は、[ニューラルネットワーク](#ニューラルネットワーク)において、[入力層](#入力層)と[出力層](#出力層)の間にあるレイヤで、データを分類したり分析したりといった様々な操作を行う。ネットワークごとに様々な種類の中間層のレイヤが用意される。

### 出力層

**出力層**は、[ニューラルネットワーク](#ニューラルネットワーク)において、最終的なデータの分類・分析結果を出力するレイヤ。

### 3層ニューラルネットワーク

**3層ニューラルネットワーク**は、[中間層](#中間層)を1層持つ、最も基本的な[ニューラルネットワーク](#ニューラルネットワーク)。

### ディープラーニング

**ディープラーニング**(**DL**: Deep Learning)は、[中間層](#中間層)の階層が深い（入力層と出力層を含めて3階層よりレイヤが多い）[ニューラルネットワーク](#ニューラルネットワーク)を用いた学習方法。[隠れ層](#中間層)や[ノード](#ノード)の数が増えたことにより、複雑な出力を行うことができるようになる。


## 順伝搬

**順伝搬**は、[ニューラルネットワーク](#ニューラルネットワーク)に入力されたデータが、[重み](#重み)を掛けられたり、[活性化関数](#活性化関数)を適用されたりすることで、最終的な結果が出力されるという流れ。[ニューラルネットワーク](#ニューラルネットワーク)により予測や分類を行うには順伝搬を用いる。


## 誤差逆伝搬法

**誤差逆伝搬法**（**バックプロバゲーション**）は、[ニューラルネットワーク](#ニューラルネットワーク)の出力と正解データとの誤差を、[出力層](#出力層)から[入力層](#入力層)に向かって伝搬（**逆伝搬**）しながら[パラメータ](./machine_learning.md#パラメータ)を調整する方法。[出力層](#出力層)の[損失関数](./machine_learning.md#損失関数)の値が小さくなるように、各[ノード](#ノード)での局所誤差を小さくしながら[モデル](./machine_learning.md#学習モデル)を最適化していく。


## 勾配降下法

**勾配降下法**は、[最適化アルゴリズム](./machine_learning.md#最適化問題)のひとつで、ある[説明変数](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#回帰分析)が入力された時の[目的関数](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#回帰分析)の値とその傾きを用いて、[目的関数](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#回帰分析)の値が最小化されるように傾きの下降方向に[パラメータ](./machine_learning.md#パラメータ)を調整する方法。**勾配**（傾き）は、ある[説明変数](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#回帰分析)が入力されたときの目的関数の[微分係数](../../../basics/applied_mathematics/_/chapters/formal_processing.md#微分係数)で求められる。

勾配降下法は、初期値によって、必ずしも[大域最適解](#大域最適解)に向かって[パラメータ](./machine_learning.md#パラメータ)を調整するとは限らず、[局所最適解](#局所最適解)に陥ってしまうこともある。

勾配降下法においては、[学習率](./machine_learning.md#学習率)の設定が非常に重要となる。[学習率](./machine_learning.md#学習率)が小さすぎると[パラメータ](./machine_learning.md#パラメータ)が収束するまでに時間がかかってしまい、[学習率](./machine_learning.md#学習率)が大きすぎると[パラメータ](./machine_learning.md#パラメータ)が発散して正しい値に収束しない。

### 大域最適解

**大域最適解**は、[説明変数](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#回帰分析)がとり得る範囲全体で、目的関数が最小化（または最大化）されるような[説明変数](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#回帰分析)の組み合わせ。

### 局所最適解

**局所最適解**は、ある限られた範囲において、目的関数が最小化（または最大化）されるような[説明変数](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#回帰分析)の組み合わせ。

### SGD

**SGD**（**確率的勾配降下法**）は、[誤差関数](./machine_learning.md#損失関数)に用いるデータをランダムに選択することで目的関数の形を微妙に変化させ、[局所最適解](#局所最適解)に陥ることを防ぐ手法。[学習率](./machine_learning.md#学習率)の設定が難しく、収束が遅い。

### モーメンタム

**モーメンタム**は、[SGD](#sgd)に慣性の概念を加えた[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)で、直前の勾配を考慮した項を加えることで収束までの時間を短縮することができる。

### Adagrad

**Adagrad**は、[SGD](#sgd)において学習が進むにつれて[学習率](./machine_learning.md#学習率)を自動的に小さくしていく[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)で、[パラメータ](./machine_learning.md#パラメータ)が発散することを防ぐ。

### RMSprop

**RMSprop**は、[Adagrad](#adagrad)を改良した[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)で、勾配の合計を指数移動平均することで、より最近の勾配を重視することができる。

### Adam

**Adam**は、[RMSprop](#rmsprop)と[モーメンタム](#モーメンタム)を組み合わせた[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)で、それぞれの恩恵を受けることができる。


## 勾配消失問題

**勾配消失問題**は、[誤差逆伝搬法](#誤差逆伝搬法)によって[パラメータ](./machine_learning.md#パラメータ)を更新する際に、レイヤを通過するたびに勾配が小さくなっていき、前段の層の[パラメータ](./machine_learning.md#パラメータ)の収束に時間がかかる問題。勾配は各[ノード](#ノード)における[活性化関数](#活性化関数)の[微分](../../../basics/applied_mathematics/_/chapters/formal_processing.md#微分)により算出できるが、この値が $1$ よりも小さい場合は徐々に勾配が小さくなっていく。

[シグモイド関数](#シグモイド関数)を[微分](../../../basics/applied_mathematics/_/chapters/formal_processing.md#微分)した関数は最大値が $0.25$ と小さく、勾配消失問題が発生しやすいため、[隠れ層](#中間層)の[活性化関数](#活性化関数)としては[ReLU関数](#relu関数)がよく用いられている。


## 勾配爆発問題

**勾配爆発問題**は、[誤差逆伝搬法](#誤差逆伝搬法)によって[パラメータ](./machine_learning.md#パラメータ)を更新する際に、レイヤを通過するたびに勾配が大きくなっていき、前段の層の[パラメータ](./machine_learning.md#パラメータ)の更新時に発散してしまう問題。


## 活性化関数

**活性化関数**は、入力を重み付けした和を別の値に変形させる関数のことで、非線形関数が用いられる。活性化関数を用いない[パーセプトロン](#パーセプトロン)や[ニューラルネットワーク](#ニューラルネットワーク)は、出力が各層の重みづけを行った入力の線形関数で表現されるため、線形分離可能な問題にしか対応できない。

### ステップ関数

**ステップ関数**は、入力が $0$ より大きければ $1$ 、 $0$ 以下であれば $0$ を出力するような関数。初期の[パーセプトロン](#パーセプトロン)の[活性化関数](#活性化関数)として用いられていた。

### シグモイド関数

**シグモイド関数**は、最小値が $0$ で最大値が $1$ となるようなS字曲線の関数で、最終的な[確率](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#確率)を出力するような場合に有用。ただし、勾配が $0$ に近い値となることが多く、学習がうまく進まないという問題（[勾配消失問題](#勾配消失問題)）がある。

### tanh関数

**tanh関数**（ハイパボリックタンジェント関数）は、最小値が $-1$ で最大値が $1$ となるようなS字曲線の関数。[シグモイド関数](#シグモイド関数)に比べて[勾配消失問題](#勾配消失問題)が軽減される。

### ReLU関数

**ReLU関数**(Rectified Linear Unit)は、負の入力は $0$ として、それ以外の入力はそのまま出力する関数。[勾配消失問題](#勾配消失問題)が起きにくいため、よく用いられている。


## ドロップアウト

**ドロップアウト**は、一定の[確率](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#確率)で[中間層](#中間層)の一部を無視して学習を行うことで、[過学習](./machine_learning.md#過学習)を防止する手法。


## 転移学習

**転移学習**は、あるタスクに最適化されている[ニューラルネットワーク](#ニューラルネットワーク)[モデル](./machine_learning.md#学習モデル)を、類似した別のタスクに応用するために調整する学習方法。転移学習がうまくいけば、学習に必要な時間が大幅に削減できる。

### ファインチューニング

**ファインチューニング**は、既に学習済みの[ニューラルネットワーク](#ニューラルネットワーク)[モデル](./machine_learning.md#学習モデル)の後段の層の[パラメータ](./machine_learning.md#パラメータ)のみを学習し直し、タスクに最適化させる手法。

### 負の転移

**負の転移**は、[転移学習](#転移学習)において、元の[モデル](./machine_learning.md#学習モデル)が解決しようとしていたタスクと転移先のタスクがそれほど似ていなかった場合に、通常より性能の悪い[モデル](./machine_learning.md#学習モデル)が完成してしまう現象。

### ドメイン混合

**ドメイン混合**は、[ニューラルネットワーク](#ニューラルネットワーク)において本来特化させたいタスクとは別の出力を設けることで、[モデル](./machine_learning.md#学習モデル)があるタスクにフィッティングしすぎることを防ぐ手法。これにより、[モデル](./machine_learning.md#学習モデル)を[転移学習](#転移学習)に利用しやすくする。

### マルチタスク学習

**マルチタスク学習**は、複数のタスクに関する学習を同時に行う手法。

### One-shot学習

**One-shot学習**は、[分類](./machine_learning.md#分類)問題において、ある[ラベル](./machine_learning.md#ラベル)の[訓練データ](./machine_learning.md#訓練データ)がひとつ（あるいは小数）しかなくても正しく[分類](./machine_learning.md#分類)ができるような学習方法。

### Zero-shot学習

**Zero-shot学習**は、[分類](./machine_learning.md#分類)問題において、ある[ラベル](./machine_learning.md#ラベル)の[訓練データ](./machine_learning.md#訓練データ)がひとつもなくても正しく[分類](./machine_learning.md#分類)ができるような学習方法。


## 特徴抽出

**特徴抽出**は、あるデータの[特徴量](./machine_learning.md#特徴量)から、情報量の少ない[特徴量](./machine_learning.md#特徴量)を削除したり、いくつかの[特徴量](./machine_learning.md#特徴量)からそれらを複合した新しい[特徴量](./machine_learning.md#特徴量)を合成したりする方法。[ニューラルネットワーク](#ニューラルネットワーク)の[出力層](#出力層)をそのほかの[モデル](./machine_learning.md#学習モデル)（[SVM](./machine_learning_algorithm.md#svm)など）につなげることで、特徴抽出器としても用いることができる。


## パターン認識

**パターン認識**は、画像認識や音声認識といった、学習データから一定のパターンを見つけ出すことによって未知データを解析する方法。


## 自然言語処理

**自然言語処理**は、人が用いる言語をテキストとして[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)に処理させるタスク。

### 分散表現

**分散表現**は、ひとつの単語を低次元の[ベクトル](../../../basics/applied_mathematics/_/chapters/numerical_calculation.md#ベクトル)として表現することで、[コンピュータ](../../../computer/_/chapters/computer.md#コンピュータ)で処理できるようにする手法。