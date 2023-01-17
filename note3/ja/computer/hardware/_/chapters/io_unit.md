# 『入出力装置』

（最終更新： 2023-01-17）


## 目次

1. [デバイスドライバ](#デバイスドライバ)
1. [入力装置](#入力装置)
	1. [キーボード](#キーボード)
	1. [ポインティングデバイス](#ポインティングデバイス)
	1. [図形入力装置](#図形入力装置)
	1. [そのほかの入力装置](#そのほかの入力装置)
1. [出力装置](出力装置)
	1. [ディスプレイ装置](#ディスプレイ装置)
	1. [プリンタ](#プリンタ)
	1. [画像データの容量](#画像データの容量)


## デバイスドライバ

**デバイスドライバ**とは、接続された周辺装置を制御するためのソフトウェア。OSに組み込まれる形で、周辺装置ごとに専用のデバイスドライバがある。


## 入力装置

**入力装置**は、コンピュータにデータを取り込むための装置。

### キーボード

**キーボード**は、コンピュータに文字を入力するための入力装置。

### ポインティングデバイス

**ポインティングデバイス**は、装置を操作することで画面に表示されたカーソルを動かし、画面上の特定の位置を指し示す入力装置。

- **マウス** : 一般的な光学式マウスでは、マウスから出る光の反射を読み取って座標一の検知を行う
- **トラックボール** : ボールを回転させることで位置情報を入力する装置
- **タッチパネル**（**タッチスクリーン**） : 表示されている画面に触れることで位置情報を入力する装置
- **デジタイザ**（**ペンタブレット**） : パネルの上で専用のペンやマウス上のカーソルを移動させることで座標情報を入力する

### 図形入力装置

- **スキャナ** : 絵や写真などに光を当て、その反射の強弱を電気信号に変えてイメージ画像として読み取る装置
- **OCR**（光学式文字読取装置）、**OMR**（光学式マーク読取装置） : OCRは手書きの文字を文字データとして読み取る装置で、OMRはマークシートを読み取る装置
- **バーコードリーダ** : 光学的にバーコードを読み取る装置で、バーコードの規格としては日本ではJANコードが使われている（そのほか、ITFコードやISBNコード、2次元バーコード（QRコード）などがある）

### そのほかの入力装置

- **ICカード読取装置** : ICチップの情報を読み取る装置で、接触型のICカードとして銀行やクレジットカード、非接触型のICカードとして交通機関のICタグなどがある
- **磁気カード読取装置** : 銀行やクレジットカードなどで利用されている**磁気ストライプカード**を読み取る装置
- **生体認証装置** : 指紋や虹彩、声紋など、生体認証技術に基づいた入力装置


## 出力装置

**入力装置**は、コンピュータからデータを取り出すための装置。

### ディスプレイ装置

- **液晶ディスプレイ**(**LCD**: Liquid Crystal Display) : 電圧をかけて液晶分子の並ぶ向きを変えることで、光の通過・遮断をコントロールする方式
- **有機ELディスプレイ**(Electro Luminescence Display) : 傾向性化学物に電圧をかけて発行させる方式で、大型化は難しいが薄型化が可能

### プリンタ

- **ドットインパクトプリンタ** : 字形を構成する細いピンの集合でインクリボンをたたいて印字する方法
- **インクジェットプリンタ** : プリンタヘッドのノズルからインクを吹き付けて印字する方法
- **ページプリンタ**（**レーザプリンタ**） : 1ページ分のイメージに従い、ドラム上にトナーを付着させて印字する方法
- **プロッタ** : CADなどで図形出力に使用される
- **3Dプリンタ** : 3次元のオブジェクトを出力するプリンタで、順に積層させることで造形を行う

### 画像データの容量

ディスプレイへの情報の表示に必要なデータの処理は、CPU負荷を軽減するために**ビデオボード**（**ビデオカード**）が行う。また、画面表示の品質（解像度、色数）は、画面に表示するデータを一時的に蓄えておく**VRAM**(Video RAM)の容量によって決まる。

```math
画像データの容量 = 画像の総ドット数 \times 色情報
```


## 3D映像の立体化

### アクティブシャッタ方式

**アクティブシャッタ方式**では、利用者が専用を眼鏡を利用することで、遠近感を伴う映像を表現する。左右の目用に映像をそれぞれ用意し、交互に映像を透過・遮断することで立体視を可能としている。

### アナグリフ方式

**アナグリフ方式**では、片目に赤色、もう一方の目に青色のフィルタをつけた眼鏡を利用する方式。左右の目用の映像を重ねて描画することで立体視を可能としている。

### パララックスバリア方式

**パララックスバリア方式**では、専用のメガネは利用せず、専用の特殊なディスプレイに左右の目用の映像を同時に描画し、網目状のフィルタを用いて立体氏を可能としている。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)