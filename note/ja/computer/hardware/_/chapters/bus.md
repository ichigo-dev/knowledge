# 『バス』

（最終更新： 2023-01-17）


## 目次

1. [バス](#バス)
	1. [アドレスバス](#アドレスバス)
	1. [データバス](#データバス)
	1. [コントロールバス](#コントロールバス)
1. [内部バス](#内部バス)
1. [外部バス](#外部バス)
	1. [システムバス](#システムバス)
	1. [入出力バス](#入出力バス)
1. [拡張バス](#拡張バス)
1. [ハブ](#ハブ)
1. [バスの種類](#バスの種類)
	1. [シリアルバス](#シリアルバス)
	1. [パラレルバス](#パラレルバス)
1. [バスの規格](#バスの規格)
	1. [バスパワー方式](#バスパワー方式)
	1. [プラグアンドプレイ](#プラグアンドプレイ)
	1. [ホットプラグ機能](#ホットプラグ機能)
	1. [USB](#usb)
	1. [シリアルATA](#シリアルata)
	1. [IEEE1394](#ieee1394)
	1. [HDMI](#hdmi)
	1. [DisplayPort](#displayport)
	1. [SCSI](#scsi)
	1. [IrDA](#irda)
	1. [Bluetooth](#bluetooth)
	1. [Zigbee](#zigbee)
	1. [NFC](#nfc)
1. [バスのアーキテクチャ](#バスのアーキテクチャ)
	1. [ハーバードアーキテクチャ](#ハーバードアーキテクチャ)
	1. [ノイマンアーキテクチャ](#ノイマンアーキテクチャ)
1. [PIO方式](#pio方式)
1. [DMA方式](#dma方式)
1. [チャネル制御方式](#チャネル制御方式)


## バス

**バス**とは、コンピュータ内部の各装置を接続するための共通の伝送路のこと。データや信号はバスを介して伝送される。バスはクロックにより同期をとりながらデータをやり取りしている。

### アドレスバス

**アドレスバス**は、メモリアドレスや入出力装置のアドレスをやり取りするためのバス。

### データバス

**データバス**は、CPUのレジスタと各装置間でデータやプログラムのやり取りを行うためのバス。

### コントロールバス

**コントロールバス**（**制御バス**）は、各バスを制御するためのバス。アドレスバスやデータバスで実際に入出力を行うタイミングや、CPUと外部との間での必要な制御情報のやり取りなどを行う。

アドレスバスで指定されたアドレスに対して、読み込みを行うのか書き込みを行うのかをコントロールバスで指定する。書き込みが指定されていれば、データバスで送られてきたデータを受け取る。読み込みが指定されていれば、データバスに指定アドレスのデータを渡す。


## 内部バス

**内部バス**は、CPU内部の装置（制御装置、演算装置、キャッシュメモリなど）を接続するためのバス。


## 外部バス

**外部バス**は、CPUと外部にある主記憶装置や周辺機器などを接続するためのバス。

### システムバス

**システムバス**は、CPUや主記憶を接続する高速なバス。

- **フロントサイドバス** : CPUとチップセットを接続する伝送路
- **メモリバス** : チップセットとメモリを接続する伝送路

### 入出力バス

**入出力バス**（**I/Oバス**）は、入出力制御装置と入出力装置を接続するバス。


## 拡張バス

**拡張バス**は、コンピュータに機能を追加するために接続するバス。PCIなどの拡張カードを直接接続するバスなどがある。


## ハブ

**ハブ**とは集線装置のことで、ハブを介することで接続ポート数を増やしたり、信号を増幅して伝送距離を延ばしたりすることができる。USBハブやLAN用のネットワークハブなどがある。


## バスの種類

### シリアルバス

**シリアルバス**とは、データを1本の伝送路で1ビットずつ転送するバス。高周波信号で高速にデータを送るにはパラレルバスよりもシリアルバスが適しており、パラレルバスに比べて高速でデータが伝送できる。

### パラレルバス

**パラレルバス**とは、データを複数ビットひとかたまりにして複数本の伝送路で送るバス。一時期は伝送の高速化のためにパラレルバスが用いられていたが、複数の伝送路でデータを送ると**タイミングスキュー**や干渉（**クロストーク**）といった問題が発生するため、高速のデータ伝送には向いていない。


## バスの規格

### バスパワー方式

**バスパワー方式**とは、接続ケーブルによってコンピュータ本体から供給される電力を用いる方式。ケーブルで供給できる電源容量には制限があるため、周辺装置の消費電力が大きい場合には電力不足となる場合もある。

### プラグアンドプレイ

**プラグアンドプレイ**とは、装置を接続してコンピュータを起動すると、自動的に認識されてデバイスドライバなどのインストールが行われる機能。

### ホットプラグ機能

**ホットプラグ機能**とは、コンピュータの電源を入れたままコネクタの抜き差しが可能で、接続すれば直ちに利用可能となる機能。

### USB

**USB**(Universal Serial Bus)は、キーボードやマウス、プリンタなどの接続に用いられる。[バスパワー方式](#バスパワー方式)や[ホットプラグ機能](#ホットプラグ機能)にも対応している。

転送速度は、USB1.0では12Mbps、USB2.0では480Mbpsとなっている。USB3.0は、全二重通信を行う高速シリアルインタフェース規格で、転送速度は5Gbpsとなっている。さらにUSB3.1では転送速度が10Gbpsとなっている。

USBには**Type-A**、**Type-B**、**Type-C**、**Mini-B**、**Micro-B**など様々な形状がある。

USBの転送モードは以下の4種類。

- **アイソクロナス転送** : 動画や音声などのリアルタイム性が必要となるデータの転送に適した転送方式で、エラーが発生しても再送が行われない
- **インタラプト転送** : 少量のデータの転送に用いられる転送方式で、キーボードやマウスなどの入力データの転送に利用される
- **コントロール転送** : 機器の初期化や設定用に用いられる転送方式で、標準リクエストやクラスリクエスト、ベンだリクエストといった様々なコマンドのやり取りに利用される
- **バルク転送** : 磁気ディスクやスキャナなどの大容量のデータ転送に用いられる転送方式で、エラー検知や再送が行われる

USBの接続形態にはスター接続とツリー接続がある。**スター接続**は、1台のUSBハブを使って最大127台の装置を接続可能。**ツリー接続**は、複数のハブを階層的に配置することで最大6階層（ハブ5台まで）まで接続可能。このような階層的な接続をカスケード接続という。

### シリアルATA

**シリアルATA**(**SATA**)とは、従来からハードディスクの接続用に用いられていたパラレル伝送のATAを、より高速なシリアル伝送に置き換えたもの。転送速度は6Gbpsとなっている。**eSATA**(External Serial ATA)は、シリアルATAの外付け用規格。

シリアルATAの接続形態は**ポイントツーポイント接続**で、コントローラと装置を1対1で接続する。装置の数だけポートが必要になる。

### IEEE1394

**IEEE1394**とは、高速シリアルインタフェース規格で、外付けハードディスク装置との接続や、デジタルビデオと接続してビデオ画像の取り込みなどに利用されている。

IEEE1394の接続形態は**デイジーチェーン接続**で、装置同士を芋づる式に最大17台まで接続できる。また、ハブを用いたツリー接続も可能で、これを加えると最大63台まで接続可能となる。

### HDMI

**HDMI**(High-Defenition Multimedia Interface)は、パソコンとディスプレイの接続や、オーディオ、カメラ、AV機器などに使われるデジタル接続規格。1本のケーブルで映像と音声、制御用の信号を受信でき、著作権保護技術にも対応している。

### DisplayPort

**DisplayPort**とは、パソコンやAV機器とディスプレイ装置を接続するためのデジタル接続規格で、映像と音声をパケット化し、シリアル伝送を行う。HDMIと同様、1本のケーブルで映像と音声の入出力が可能なほか、著作権保護技術にも対応している。

### SCSI

**SCSI**（スカジー、Small Computer System Interface）は、パラレルインタフェースの規格で、最大16台までの機器をデイジーチェーン接続できる。接続機器の両端には**ターミネータ**と呼ばれる終端抵抗が必要。

### IrDA

**IrDA**とは、近距離赤外線通信規格で、パソコンの周辺機器や携帯端末との無線接続が可能。

### Bluetooth

**Bluetooth**とは、2.4GHz帯の電波を利用した近距離無線通信の接続規格。10m～数十mの接続範囲でのデータや音声の送受信に使用され、最大7台まで接続が可能。

大幅な省電力化が行われた新しい規格を**BLE**(Bluetooth Low Energy)といい、これと区別するために古い規格のことを**Bluetooth Classic**と呼ぶ。

### Zigbee

**Zigbee**（ジグビー）とは、多数の機器との接続を行うセンサーネットワークを前提とした2.4GHz帯の電波による近距離無線通信規格。Bluetoothに比べて伝送距離が短く、速度も遅いものの、低消費電力・低コストで中継機能を持つという特徴がある。

### NFC

**NFC**(Near field radio communication)とは、電波によって近距離での無線通信を行う接続規格で、通信機能と識別・認証機能を持っている。もともとはICカードの接続規格として作られ、**RFID**技術（無線通信によってICタグとの情報交換を行う技術）を利用している。日本では、NFC規格のひとつである**Felica**が普及しており、交通系ICや電子マネーカードの通信に利用されている。

**アクティブ方式**のICタグは、電源を持っており通信可能距離が長い。一方、**パッシブ方式**のICタグは、電源を持たずに読み取り装置からの電磁波（または磁界）による電力で稼働する。


## バスのアーキテクチャ

### ハーバードアーキテクチャ

**ハーバードアーキテクチャ**は、命令とデータを分離してバスで転送するアーキテクチャ。命令の実行が完了すると同時に次の命令を読み込むことができるため高速化が可能であるが、より多くの電気回路を必要とする。

### ノイマンアーキテクチャ

**ノイマンアーキテクチャ**は、命令とデータを分離せずに同一のバスで転送するアーキテクチャ。命令とデータを同時に転送することはできない。


## バス幅

**バス幅**とは、バスが一回のクロック信号で同時に伝送することができるデータ量のこと。バス幅はデータ伝送のための信号線の数によって左右され、単位はビットとなる。内部バスのクロック周波数を**コアクロック周波数**、外部バスのクロック周波数を**外部クロック周波数**という。


## アクセスモード

**アクセスモード**は、データを伝送するときのバス幅や方式を制御するモード。


## PIO方式

**PIO方式**(Programmed I/O)は、CPUからの命令により主記憶装置間や主記憶と入出力装置の間でのやり取りを行う方法。


## DMA方式

**DMA方式**(Direct Memory Access)は、DMA転送を行うアクセスモードで、CPUからの命令を必要とせず主記憶装置間や主記憶と入出力装置の間でデータのやり取りを行うことができる。CPUの待ちや付加を軽減し、データの高速転送が可能となる。CPUやDMAコントローラに対する指示のみを行う。


## チャネル制御方式

**チャネル制御方式**は、CPUの支持を受けて独立して入出力の制御を行う**チャネル**という装置を用いるデータ転送の制御方式。この方式では、記憶装置と入出力装置の間にチャネルを設置してデータ転送を制御する。1つの装置の制御を行う**セレクタチャネル**と、低速な入出力装置の制御を時分割により並行的に行う**マルチプレクサチャネル**がある。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)