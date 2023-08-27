# 『レイアウト』ノート

（最終更新： 2023-08-26）


## 目次

1. [display](#display)
1. [フレックスボックス](#フレックスボックス)
	1. [flex](#flex)
	1. [flex-direction](#flex-direction)
	1. [flex-wrap](#flex-wrap)
	1. [justify-content](#justify-content)
	1. [align-items](#align-items)
	1. [align-self](#align-self)
	1. [gap](#gap)
1. [グリッドレイアウト](#グリッドレイアウト)
	1. [grid-template-rows](#grid-template-rows)
	1. [grid-template-columns](#grid-template-columns)
	1. [grid-row](#grid-row)
	1. [grid-column](#grid-column)
1. [position](#position)
	1. [z-index](#z-index)
1. [メディアクエリ](#メディアクエリ)


## display

**display**は、[CSS](./css.md#css)において、[ボックス](./css.md#ボックスモデル)の表示方法を指定するための[プロパティ](./css.md#プロパティ)。[ブロックボックス](./css.md#ブロックボックス)を[インラインボックス](./css.md#インラインボックス)にしたり、[フレックスボックス](#フレックスボックス)や[グリッドレイアウト](#グリッドレイアウト)を適用するために用いられる。


## フレックスボックス

**フレックスボックス**は、[子要素](../../../html/_/chapters/html.md#要素)を列方向や行方向に1次元にレイアウトすることを容易にするための[ボックス](./css.md#ボックスモデル)。 `display` [プロパティ](./css.md#プロパティ)に `flex` という値を指定することで[要素](../../../html/_/chapters/html.md#要素)を**フレックスコンテナ**にし、その[子要素](../../../html/_/chapters/html.md#要素)を**フレックスアイテム**にすることができる。

[子要素](../../../html/_/chapters/html.md#要素)の並び方向や整列位置、[要素](../../../html/_/chapters/html.md#要素)の伸縮などを指定できる。

### flex

**flex**は、[CSS](./css.md#css)において、[フレックスアイテム](#フレックスボックス)の伸縮を指定するための[プロパティ](./css.md#プロパティ)。1つ目の値として伸長係数、2つ目の値として縮小係数、3つ目の値として初期の寸法を一括で設定できる[ショートハンドプロパティ](./css.md#ショートハンドプロパティ)。

各スタイルを個別に指定する[プロパティ](./css.md#プロパティ)も存在し、伸長係数を指定する**flex-grow**、縮小係数を指定する**flex-shrink**、初期の寸法を指定する**flex-basis**が使用できる。

### flex-direction

**flex-direction**は、[CSS](./css.md#css)において、[フレックスボックス](#フレックスボックス)の[要素](../../../html/_/chapters/html.md#要素)の並び方向を指定するための[プロパティ](./css.md#プロパティ)。 `row` （行方向）、 `row-reverse` （行方向逆順）、 `column` （列方向）、 `column-reverse` （列方向逆順）といったキーワードを指定できる。

### flex-wrap

**flex-wrap**は、[CSS](./css.md#css)において、[フレックスボックス](#フレックスボックス)の[要素](../../../html/_/chapters/html.md#要素)の折返しを指定するための[プロパティ](./css.md#プロパティ)。 `nowrap` （折返しなし）、 `wrap` （折返しあり）、 `wrap-reverse` （折返し逆順）といったキーワードを指定できる。

### justify-content

**justify-content**は、[CSS](./css.md#css)において、[フレックスボックス](#フレックスボックス)の[要素](../../../html/_/chapters/html.md#要素)の並び方向に対する配置を指定するための[プロパティ](./css.md#プロパティ)。 `flex-start` （前揃え）、 `flex-end` （後揃え）、 `center` （中央揃え）、 `space-between` （アイテム間のスペースを均等に割当）、 `space-around` （アイテムの両端にスペースを均等に割当）といったキーワードを指定できる。

### align-items

**align-items**は、[CSS](./css.md#css)において、[フレックスボックス](#フレックスボックス)の[要素](../../../html/_/chapters/html.md#要素)の並び方向に直行する方向に対する配置を指定するための[プロパティ](./css.md#プロパティ)。 `stretch` （伸縮）、 `flex-start` （始端揃え）、 `flex-end` （終端揃え）、 `center` （中央揃え）といったキーワードを指定できる。

### align-self

**align-self**は、[CSS](./css.md#css)において、[フレックスアイテム](#フレックスボックス)が自身の配置位置を指定するための[プロパティ](./css.md#プロパティ)。使用できるキーワードは `align-items` と同様。

### gap

**gap**は、[CSS](./css.md#css)において、[フレックスボックス](#フレックスボックス)の[要素](../../../html/_/chapters/html.md#要素)同士の間隔（余白）を指定するための[プロパティ](./css.md#プロパティ)。


## グリッドレイアウト

**グリッドレイアウト**は、[子要素](../../../html/_/chapters/html.md#要素)を2次元にレイアウトすることを容易にするための[ボックス](./css.md#ボックスモデル)。 `display` [プロパティ](./css.md#プロパティ)に `grid` という値を指定することで[要素](../../../html/_/chapters/html.md#要素)を**グリッドコンテナ**にし、その[子要素](../../../html/_/chapters/html.md#要素)を**グリッドアイテム**にすることができる。

[子要素](../../../html/_/chapters/html.md#要素)を行と列の方向にレイアウトすることができ、複雑なレイアウトをより簡単に構築できるようになる。**fr**(fraction)という単位を用いて、残りの幅や高さを自動で埋めることもでき、画面サイズに合わせたレイアウトが容易となる。

### grid-template-rows

**grid-template-rows**は、[CSS](./css.md#css)において、[グリッドレイアウト](#グリッドレイアウト)の行のレイアウトを指定するための[プロパティ](./css.md#プロパティ)。複数の値を指定することで、行方向に要素をどのように配置するのかを指定できる。

### grid-template-columns

**grid-template-columns**は、[CSS](./css.md#css)において、[グリッドレイアウト](#グリッドレイアウト)の列のレイアウトを指定するための[プロパティ](./css.md#プロパティ)。複数の値を指定することで、列方向に要素をどのように配置するのかを指定できる。

### grid-row

**grid-row**は、[CSS](./css.md#css)において、[グリッドアイテム](#グリッドレイアウト)の行方向の位置（サイズ）を指定するための[プロパティ](./css.md#プロパティ)。値として、始点と終点となる番号をスラッシュ区切りで指定する。

### grid-column

**grid-column**は、[CSS](./css.md#css)において、[グリッドアイテム](#グリッドレイアウト)の列方向の位置（サイズ）を指定するための[プロパティ](./css.md#プロパティ)。値として、始点と終点となる番号をスラッシュ区切りで指定する。


## position

**position**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)同士の位置関係を指定するための[プロパティ](./css.md#プロパティ)。この[プロパティ](./css.md#プロパティ)の値に応じて**top**、**bottom**、**left**、**right**といった[プロパティ](./css.md#プロパティ)の位置指定が適用される。 `position` には以下のような値が適用できる。

- **static**: デフォルトの位置指定で、 `top` や `left` といった位置指定はできない。
- **relative**: 本来[要素](../../../html/_/chapters/html.md#要素)が配置されるべき位置を基準として、 `top` や `left` といった位置指定を適用する。また、 `absolute` に対する基準位置としても使用される。
- **absolute**: `relative` が指定されている[親要素](../../../html/_/chapters/html.md#要素)の位置を基準として、 `top` や `left` といった位置指定を適用する。
- **fixed**: [ブラウザ](../../../../network/_/chapters/web.md#webブラウザ)のviewportを基準として、 `top` や `left` といった位置指定を適用する。ページをスクロールした場合も常に固定位置に表示される。
- **sticky**: 最初は `relative` と同様のフローに従って配置され、[親要素](../../../html/_/chapters/html.md#要素)が特定のスクロール位置に達したときに要素の位置が固定されるようになる。 `top` や `left` で位置指定を適用することもできる。テーブルのヘッダーなどを `sticky` にするといった使い方が多い。

### z-index

**z-index**は、[CSS](./css.md#css)において、同じ位置にある[要素](../../../html/_/chapters/html.md#要素)同士の重なり順を指定するための[プロパティ](./css.md#プロパティ)。より大きな値を指定したものが上に重なるようになる。


## メディアクエリ

**メディアクエリ**は、[CSS](./css.md#css)において、特定の特性（viewportのサイズなど）に応じて適用するスタイルを切り替えるための機能。メディアクエリの使用例は以下の通り。

```css
@media screen and (min-width: 768px)
{
    /* PC用スタイル */
}

@media screen and (max-width: 768px)
{
    /* モバイル用（タブレット含む）スタイル */
}
```
