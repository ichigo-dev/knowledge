# 『ニューラルネットワークアルゴリズム』ノート

（最終更新： 2023-05-11）


## 目次

1. [全結合層](#全結合層)
1. [CNN](#cnn)
	1. [畳み込み層](#畳み込み層)
	1. [プーリング層](#プーリング層)
1. [RNN](#rnn)
	1. [再帰セル](#再帰セル)
	1. [内部状態](#内部状態)
	1. [LSTM](#lstm)
	1. [GRU](#gru)
	1. [双方向RNN](#双方向rnn)
	1. [Seq2Seq](#seq2seq)
	1. [ELMo](#elmo)
1. [Attention](#attention)
1. [Transformer](#transformer)
1. [BERT](#bert)
1. [オートエンコーダ](#オートエンコーダ)
	1. [VAE](#vae)
1. [GAN](#gan)
1. [強化学習](#強化学習)
	1. [Q学習](#Q学習)
	1. [DQN](#dqn)
	1. [Experience Reply](#experience-reply)
	1. [greedy方策](#greedy方策)
	1. [方策勾配法](#方策勾配法)
	1. [REINFORCE](#reinforce)
	1. [Actor-Critic](#actor-critic)
1. [物体検出](#物体検出)
	1. [sliding window method](#sliding-window-method)
	1. [region proposal method](#region-proposal-method)
	1. [end-to-end](#end-to-end)


## 全結合層

**全結合層**は、[ニューラルネットワーク](./neural_network.md#ニューラルネットワーク)に用いられるレイヤのひとつで、前の層の出力全てと結合するような[ノード](./neural_network.md#ノード)からなる。


## CNN

**CNN**（**畳み込みニューラルネットワーク**）は、平面・空間上で隣り合う[特徴量](./machine_learning.md#特徴量)を考慮して学習を行う機構を持った[ニューラルネットワーク](./neural_network.md#ニューラルネットワーク)。

### 畳み込み層

**畳み込み層**は、畳み込みフィルタによって平面・空間上のある範囲に反応するフィルタを用いて新しい**特徴マップ**を作るレイヤ。データの[特徴量](./machine_learning.md#特徴量)の端の部分からフィルタを適用し、フィルタをずらしていくことで特徴マップを作成する。

### プーリング層

**プーリング層**は、平面・空間上のある範囲（ウィンドウ）のうち値を1つだけ抽出することで[特徴量](./machine_learning.md#特徴量)を削減するレイヤ。**マックスプーリング**では、ウィンドウ上で最大の値だけを抽出する。


## RNN

**RNN**（**再帰型ニューラルネットワーク**）は、[時系列データ](./machine_learning_algorithm.md#時系列データ)を再帰的に処理する機構を持った[ニューラルネットワーク](./neural_network.md#ニューラルネットワーク)。[ニューラルネットワーク](./neural_network.md#ニューラルネットワーク)がループ状に接続されるような構造となっており、古いデータを処理したときの情報を次のデータを処理する際に引き継ぐ。

### 再帰セル

**再帰セル**は、[RNN](#rnn)の[モデル](./machine_learning.md#学習モデル)中でループで繋がれているレイヤ。

### 内部状態

**内部状態**（隠れ状態）は、[再帰セル](#再帰セル)が保持する情報。

### LSTM

**LSTM**(Long Short Term Memory)は、[再帰セル](#再帰セル)の構造のひとつで、古い情報の記憶が薄れていく（内部情報には直近のデータの状態が反映されやすく、最初のデータの情報は徐々に消えていく）という[再帰セル](#再帰セル)の弱点を軽減している。また、時間方向の[勾配消失問題](./neural_network.md#勾配消失問題)を軽減し、学習が効率よく進む。LSTMには情報の伝わり方を調整するための3つの**ゲート**が設けられている。

**忘却ゲート**では前の情報をどれだけ切り捨てるかを調整し、**入力ゲート**では新しい情報をどれだけ取り込むかを調整し、**出力ゲート**では情報をどれだけ出力するかを調整する。

### GRU

**GRU**(Gated Recurrent Unit)は、[再帰セル](#再帰セル)の構造のひとつで、[LSTM](#lstm)の構造を単純化している。GRUには情報の伝わり方を調整するための2つの**ゲート**が設けられている。

**リセットゲート**では情報をどれだけ切り捨てるかを調整し、**更新ゲート**では情報をどれだけ取り込むかを調整する。

### 双方向RNN

**双方向RNN**は、前のデータの情報だけでなく、後ろのデータの情報も用いることで予測精度を向上させた[RNN](#rnn)。

### Seq2Seq

**Seq2Seq**(sequence-to-sequence)は、[自然言語処理](./neural_network.md#自然言語処理)に特化した双方向[RNN](#rnn)[モデル](./machine_learning.md#学習モデル)。

ある単語系列をRNN(**Encoder**)に入力し、Encoderの最終的な内部状態を別のRNN(**Decoder**)に入力として渡すことで、新しい単語系列を出力させる。これにより機械翻訳などを実現している。

### ELMo

**ELMo**(Embeddings from Language Models)は、双方向[LSTM](#lstm)を用いた[自然言語処理](./neural_network.md#自然言語処理)の[モデル](./machine_learning.md#学習モデル)で、ある単語をその前後の文脈を考慮して[ベクトル](../../../basics/applied_mathematics/_/chapters/numerical_calculation.md#ベクトル)に変換する。純粋な[分散表現](./neural_network.md#分散表現)では、ひとつの単語をそのまま[ベクトル](../../../basics/applied_mathematics/_/chapters/numerical_calculation.md#ベクトル)として表現するため、文脈の違いや多義語に対する認識性能が低いという欠点があった。


## Attention

**Attention**（**注意機構**）は、入力された[時系列データ](./machine_learning_algorithm.md#時系列データ)のすべての[内部状態](#内部状態)を参照し、それらに[重み](./neural_network.md#重み)をつけて着目すべき部分を変化させる機構。[Seq2Seq](#seq2seq)では、最終的な[内部状態](#内部状態)だけを[Encoder](#encoder)から[Decoder](#decoder)に渡していたため、情報のボトルネックが生じてしまっていた。Attentionはこのような弱点を克服し、翻訳精度を上げることに成功している。

また、[RNN](#rnn)のような前の出力を[モデル](./machine_learning.md#学習モデル)に再帰的に入力するような方法では、学習を並列化させることができないため最適化に時間がかかるという欠点があったが、Attentionを用いることでこれも解決できる。


## Transformer

**Transformer**は、[Attention](#attention)を応用した**Self-Attention**機構を持つ[ニューラルネットワーク](./neural_network.md#ニューラルネットワーク)の[モデル](./machine_learning.md#学習モデル)。Self-Attentionは[自然言語処理](./neural_network.md#自然言語処理)のタスクにおいて、ある単語がその文章中のどの単語と結びつきが強いのか、という情報を明らかにするレイヤ。基本形は[Seq2Seq](#seq2seq)と同様Encoder-Decoderの[モデル](./machine_learning.md#学習モデル)であるが、Encoderのみを取り出したものもある。


## BERT

**BERT**(Bidirectional Encoder Representations from Transformers)は、Googleが開発した[自然言語処理](./neural_network.md#自然言語処理)の[モデル](./machine_learning.md#学習モデル)で、[Transformer](#transformer)を用いて双方向に単語をエンコードする。双方向[RNN](#rnn)や[ELMo](#elmo)と同様、前後の文脈から単語の意味を推測することができる。


## オートエンコーダ

**オートエンコーダ**（**自己符号化器**）は、[教師なし学習](./machine_learning.md#教師なし学習)の手法のひとつで、[ニューラルネットワーク](./neural_network.md#ニューラルネットワーク)の入力と同じ出力を行うように[モデル](./machine_learning.md#学習モデル)を学習する方法。[中間層](./neural_network.md#中間層)のサイズを[入力層](./neural_network.md#入力層)よりも小さくしておくことで、学習後の[モデル](./machine_learning.md#学習モデル)の[中間層](./neural_network.md#中間層)には入力された[特徴量](./machine_learning.md#特徴量)を圧縮した情報が存在する状態となる。最終的に[出力層](./neural_network.md#出力層)（あるいは後段の層）を取り除くことで、[ニューラルネットワーク](./neural_network.md#ニューラルネットワーク)を[特徴抽出](./neural_network.md#特徴抽出)器として利用することができる。

### VAE

**VAE**（**変分自己符号化器**: Variational Autoencoder）は、[オートエンコーダ](#オートエンコーダ)において[中間層](./neural_network.md#中間層)の潜在変数に[確率分布](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#確率分布)を用いることで、未知のデータに対しても[確率](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#確率)的に応用できる。[オートエンコーダ](#オートエンコーダ)は入力データと同じデータを出力することしかできないが、VAEは[確率](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#確率)的に様々なデータを生成することができる。


## GAN

**GAN**（**敵対的生成ネットワーク**: Generative Adversarial Network）は、データから特徴を学ぶことで実在しない偽造データを生成するGeneratorと、データが本物であるか偽造データであるかを識別するDiscriminatorからなるネットワーク。

学習の方向性によっては、生成されるデータに偏りができる**モード崩壊**を起こす可能性がある。そういった場合は、[ハイパーパラメータ](./machine_learning.md#ハイパーパラメータ)の[チューニング](./machine_learning.md#チューニング)などを見直す必要がある。


## 強化学習

強化学習の[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)は大きく**モデルベース**と**モデルフリー**に大別できる。さらにモデルベースの学習は、**方策ベース**と**価値ベース**の[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)に分類できる。

### Q学習

**Q学習**は、[価値ベース](#強化学習)の[強化学習](./machine_learning.md#強化学習)手法で、対応表（Qテーブル）を用いてある状態においてある行動をとることがどれほど価値があるかを学習する。状態と行動の選択肢が増えると、Qテーブルが膨大になってしまうという欠点がある。

### DQN

**DQN**(Deep Q-Network)は、[Q学習](#q学習)のテーブルを[ニューラルネットワーク](./neural_network.md#ニューラルネットワーク)に置き替えたもの。状態を入力すると、それに対して各行動をとったときの価値が出力される。

### Experience Reply

**Experience Reply**は、[DQN](#dqn)において、行動や行動前後の状態、報酬を記録しておき、その記録を何度も学習に生かせる機能。

### greedy方策

**greedy方策**は、[価値ベース](#強化学習)の[強化学習](./machine_learning.md#強化学習)における[エージェント](./machine_learning.md#エージェント)の行動選択の[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)のひとつで、最適行動価値関数がわかっていると仮定したときに、現在の状態に対して最適行動価値関数が最大となるような行動を選択する方法。

学習中は最適行動価値関数が確定していないため、 $\epsilon$ の[確率](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#確率)でランダムな行動を選択し、 $1 - \epsilon$ の[確率](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#確率)で最適な行動を選択する**ε-greedy方策**が用いられる。

### 方策勾配法

**方策勾配法**は、[方策ベース](#強化学習)の[強化学習](./machine_learning.md#強化学習)における[エージェント](./machine_learning.md#エージェント)の行動選択の[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)のひとつで、方策をパラメータで表された関数として、パラメータを最適化することで方策を学習する方法。

### REINFORCE

**REINFORCE**は、[方策勾配法](#方策勾配法)の代表的な[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)で、最初に行動を繰り返して状態・行動・報酬のデータを収集し、高い報酬を得ることのつながった行動の確率を高くする手法。

### Actor-Critic

**Actor-Critic**は、[方策ベース](#強化学習)の[モデル](./machine_learning.md#学習モデル)であるActor（状態を入力として各行動の[確率](../../../basics/applied_mathematics/_/chapters/probability_and_statistics.md#確率)を出力する）と、[価値ベース](#強化学習)の[モデル](./machine_learning.md#学習モデル)であるCritic（状態を入力として状態価値を出力する）を組み合わせて学習を進める。


## 物体検出

**物体検出**は、画像の中に何が写っている物体を認識する技術。**バウンディングボックス**という矩形の範囲を生成し、その中に含まれる物体の[ラベル](./machine_learning.md#ラベル)を出力する、というタスクを行う。

物体検出は、注目領域の決定と物体の[ラベル](./machine_learning.md#ラベル)推測という2つのタスクを解決する必要がある。[sliding window method](#sliding-window-method)や[region proposal method](#region-proposal-method)は注目領域決定のための[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)で、[end-to-end](#end-to-end)は注目領域決定と物体の[ラベル](./machine_learning.md#ラベル)推測をまとめて行う[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)。

### sliding window method

**sliding window method**は、いくつかの大きさのウィンドウをスライドさせながら、画像の全領域を網羅するように切り抜き、それらすべての[ラベル](./machine_learning.md#ラベル)を推測する手法。物体の切り抜き漏れはないが、膨大な[計算量](../../../basics/information_theory/_/chapters/computational_complexity.md#計算量)がかかる。

### region proposal method

**region proposal method**は、画像中で物体がありそうな領域を提案する[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)を用いて、その部分について[ラベル](./machine_learning.md#ラベル)を推測する手法。[sliding window method](#sliding-window-method)に比べて大幅に[計算量](../../../basics/information_theory/_/chapters/computational_complexity.md#計算量)を削減できる。

### end-to-end

**end-to-end**は、注目領域決定と[ラベル](./machine_learning.md#ラベル)推測のタスクをひとつの[ニューラルネットワーク](./neural_network.md#ニューラルネットワーク)で行う手法。代表的な[アルゴリズム](../../../programming/_/chapters/algorithm.md#アルゴリズム)として、**Faster R-CNN**、**SSD**、**Yolo**といったものがある。

| アルゴリズム | 識別精度 | 処理速度 |
|--------------|----------|----------|
| Faster R-CNN | 高       | 低       |
| SSD          | 中       | 中       |
| Yolo         | 低       | 高       |
