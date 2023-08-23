# 『リンクと画像』ノート

（最終更新： 2023-08-23）


## 目次

1. [a](#a)
1. [img](#img)
1. [svg](#svg)


## a

**a**(anchor)は、[HTML](./html.md#html)文書において、[リンク](../../../../network/_/chapters/web.md#ハイパーリンク)を作成するために使用される[タグ](./html.md#タグ)。 `href` [属性](./html.md#属性)を用いて、別の[Web](../../../../network/_/chapters/web.md#web)ページや[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)、メールアドレス、同ページ内の場所などを紐付けることができる。

[リンク](../../../../network/_/chapters/web.md#ハイパーリンク)先の[URL](../../../../network/_/chapters/web.md#url)への遷移だけでなく、[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)をダウンロードさせるといった使い方もできる。

`target` [属性](./html.md#属性)に `_blank` という値を指定することで、[リンク](../../../../network/_/chapters/web.md#ハイパーリンク)先を別タブで開くことができる。


## img

**img**(image)は、[HTML](./html.md#html)文書において、画像を挿入するために使用される[タグ](./html.md#タグ)。 `src` [属性](./html.md#属性)に画像[URL](../../../../network/_/chapters/web.md#url)を指定することで、ページ内に画像を表示することができる。また、 `alt` [属性](./html.md#属性)にその画像を表現する文字列を含めておくことで、画像を表示できなかった場合に代替テキストを表示することができる（ `alt` の指定はアクセシビリティの観点から重要）。


## svg

**svg**は、、[HTML](./html.md#html)文書において、SVG画像を埋め込むために使用される[タグ](./html.md#タグ)。SVG画像は `img` タグでも表示することができるが、 `svg` [タグ](./html.md#タグ)で埋め込むことにより、[JavaScript](../../../javascript/_/chapters/javascript.md#javascript)と連動した動的な処理が容易となる。
