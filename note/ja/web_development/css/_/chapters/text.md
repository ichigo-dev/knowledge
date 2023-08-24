# 『テキスト』ノート

（最終更新： 2023-08-24）


## 目次

1. [color](#color)
1. [font-size](#font-size)
1. [font-weight](#font-weight)
1. [line-height](#line-height)
1. [text-align](#text-align)
1. [letter-spacing](#letter-spacing)
1. [text-decoration](#text-decoration)
1. [text-overflow](#text-overflow)
1. [white-space](#white-space)


## color

**color**は、[CSS](./css.md#css)において、テキスト色を指定するための[プロパティ](./css.md#プロパティ)。色の指定方法としては、HEXカラー、RGB(RGBA)、HSL(HSLA)、カラーキーワードが使用できる。


## font-size

**font-size**は、[CSS](./css.md#css)において、フォントの大きさを指定するための[プロパティ](./css.md#プロパティ)。デフォルト[CSS](./css.md#css)においてはこの値は `16px` となっており、見出しなど目立たせたい部分は `32px` や `24px` 、やや小さく表示したい部分は `10px` や `12px` などを用いることが多い。


## font-weight

**font-size**は、[CSS](./css.md#css)において、フォントの太さを指定するための[プロパティ](./css.md#プロパティ)。 `100 ~ 900` （ `400` が通常の太さ）という数値で指定したり、 `normal` や `bold` などのキーワードで指定したりする。


## line-height

**line-height**は、[CSS](./css.md#css)において、行の高さを指定するための[プロパティ](./css.md#プロパティ)。 `font-size` を `1` としたときの倍率で指定する場合が多く（ `px` など他の単位も使用可能）、通常は `1.5 ~ 1.8` 程度とするのが一般的。


## text-align

**text-align**は、[CSS](./css.md#css)において、文字の整列方向を指定するための[プロパティ](./css.md#プロパティ)。 `left` （左揃え）、 `center` （中央揃え）、 `right` （右揃え）、 `justify` （両端揃え）というキーワードで指定する。


## letter-spacing

**letter-spacing**は、[CSS](./css.md#css)において、文字と文字の間隔を指定するための[プロパティ](./css.md#プロパティ)。見出しを目立たせたい場合などに使用されることが多い。


## text-decoration

**text-decoration**は、[CSS](./css.md#css)において、文字の装飾を指定するための[プロパティ](./css.md#プロパティ)。 `overline` （テキストの上線）、 `underline` （テキストの下線）、 `line-through` （テキストの字消し線）、 `none` （装飾なし）というキーワードで指定する。


## text-overflow

**text-overflow**は、[CSS](./css.md#css)において、[親要素](../../../html/_/chapters/html.md#要素)からテキストがはみ出た場合の表示方法を指定するための[プロパティ](./css.md#プロパティ)。 `ellipsis` （...に置き換える）、 `clip` （切り取る）などのキーワードを指定したり、値として任意の文字列を指定してはみ出た部分をそれに置き換えたりすることができる。


## white-space

**white-space**は、[CSS](./css.md#css)において、[親要素](../../../html/_/chapters/html.md#要素)からテキストがはみ出た場合の改行制御を指定するための[プロパティ](./css.md#プロパティ)。 `normal` （改行する）、 `nowrap` （改行しない）などのキーワードを指定することができる。
