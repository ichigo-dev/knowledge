# 『ボックス』ノート

（最終更新： 2023-08-24）


## 目次

1. [width](#width)
1. [height](#height)
1. [margin](#margin)
1. [border](#border)
	1. [border-radius](#border-radius)
1. [padding](#padding)
1. [background](#background)
1. [overflow](#overflow)
1. [vertical-align](#vertical-align)
1. [opacity](#opacity)
1. [box-shadow](#box-shadow)


## width

**width**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)の[コンテンツ領域](./css.md#コンテンツ領域)の幅を指定するための[プロパティ](./css.md#プロパティ)。[要素](../../../html/_/chapters/html.md#要素)の最小幅を指定するための**min-width**や、最大幅を指定するための**max-width**もある。


## height

**height**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)の[コンテンツ領域](./css.md#コンテンツ領域)の高さを指定するための[プロパティ](./css.md#プロパティ)。[要素](../../../html/_/chapters/html.md#要素)の最小高さを指定するための**min-height**や、最大高さを指定するための**max-height**もある。


## margin

**margin**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)の外側余白（[マージン領域](./css.md#マージン領域)）の大きさを指定するための[プロパティ](./css.md#プロパティ)。上・右・下・左の順番の4つの値、上下・左右の順番の2つの値、全方向に共通の1つの値、といった様々な値の指定方法がある。 `margin` は上下の[要素](../../../html/_/chapters/html.md#要素)で**畳み込み**（両方の[マージン領域](./css.md#マージン領域)が重なり、大きほうが採用されたように見える）が発生するので注意が必要。

各方向の余白を設定する**margin-top**、**margin-right**、**margin-botom**、**margin-left**といった[プロパティ](./css.md#プロパティ)も存在する。


## border

**border**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)の枠線（[ボーダー領域](./css.md#ボーダー領域)）のスタイルを指定するための[プロパティ](./css.md#プロパティ)。太さ、線種、色の3つの値によってスタイルを決定する。

各方向の枠線を設定する**border-top**、**border-right**、**border-botom**、**border-left**といった[プロパティ](./css.md#プロパティ)や、太さ、線種、色を個別に設定する**border-width**、**border-style**、**border-color**といった[プロパティ](./css.md#プロパティ)も存在する。

### border-radius

**border-radius**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)の枠線（[ボーダー領域](./css.md#ボーダー領域)）の角丸の半径を指定するための[プロパティ](./css.md#プロパティ)。


## padding

**padding**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)の内側余白（[パディング領域](./css.md#パディング領域)）の大きさを指定するための[プロパティ](./css.md#プロパティ)。上・右・下・左の順番の4つの値、上下・左右の順番の2つの値、全方向に共通の1つの値、といった様々な値の指定方法がある。

各方向の余白を設定する**padding-top**、**padding-right**、**padding-botom**、**padding-left**といった[プロパティ](./css.md#プロパティ)も存在する。


## background

**background**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)の背景スタイルを指定するための[プロパティ](./css.md#プロパティ)。背景画像や色、位置、サイズ、繰り返しパターンなどの背景に関する様々なスタイルを一度に適用することができる。

各スタイルを個別に指定する[プロパティ](./css.md#プロパティ)も存在し、背景色を指定する**background-color**、背景画像やグラデーションを指定する**background-image**、背景画像の位置を指定する**background-position**、背景画像のサイズを指定する**background-size**、背景画像の繰り返しパターンを指定する**background-repeat**、背景画像のスクロールに関する挙動を指定する**background-attachment**が使用できる。


## overflow

**overflow**は、[CSS](./css.md#css)において、[子要素](../../../html/_/chapters/html.md#要素)のコンテンツが[親要素](../../../html/_/chapters/html.md#要素)に収まりきらなかった場合の表示方法を指定するための[プロパティ](./css.md#プロパティ)。 `hidden` （隠す）、 `scroll` （スクロール表示）、 `visible` （はみ出して表示）などのキーワードで指定する。

各方向の表示方法を個別に設定する**overflow-x**、**overflow-y**といった[プロパティ](./css.md#プロパティ)も存在する。


## vertical-align

**vertical-align**は、[CSS](./css.md#css)において、子となる[インラインボックス](./css.md#インラインボックス)の縦方向の位置を指定するための[プロパティ](./css.md#プロパティ)。


## opacity

**opacity**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)の透明度を指定するための[プロパティ](./css.md#プロパティ)。0を透明、1を不透明として、0〜1の数値で指定する。


## box-shadow

**box-shadow**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)の影（ドロップシャドウ）を指定するための[プロパティ](./css.md#プロパティ)。横方向のずれ、縦方向のずれ、ぼかし量、拡張量（スプレッド）、色の値を指定する。また、[要素](../../../html/_/chapters/html.md#要素)の内側に影をつけたい場合は、 `inset` キーワードを値の最初に指定する。
