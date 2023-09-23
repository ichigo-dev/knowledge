# 『基本HTML』ノート

（最終更新： 2023-09-23）


## 目次

1. [DOCTYPE宣言](#doctype宣言)
1. [html](#html)
1. [head](#head)
	1. [title](#title)
	1. [meta](#meta)
    1. [link](#link)
1. [script](#script)
    1. [noscript](#noscript)


## DOCTYPE宣言

**DOCTYPE宣言**は、[HTML](./html.md#html)文書の先頭に配置される特殊な[タグ](./html.md#タグ)で、[ブラウザ](../../../../network/_/chapters/web.md#webブラウザ)に対してどの[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)の[HTML](./html.md#html)を使用しているかを伝える役割を持つ。文書がどの[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)の[HTML](./html.md#html)仕様に準拠しているかを[ブラウザ](../../../../network/_/chapters/web.md#webブラウザ)に伝えることで、正しく内容を解釈してもらうために重要となる。

最新のHTML5の標準モードで文書を解釈するように[ブラウザ](../../../../network/_/chapters/web.md#webブラウザ)に指示をする場合は、 `<!DOCTYPE html>` と記述し、基本的に新しいサイトを作成する際はこれを覚えておくだけで十分。

過去[バージョン](../../../../computer/software/_/chapters/package.md#バージョン)の[HTML](./html.md#html)仕様を使用する場合、次のように記述する。

```html
<!-- HTML 4.01 Strict -->
<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">

<!-- XHTML 1.0 Strict -->
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
```


## html

**html**は、[HTML](./html.md#html)文書のルート[要素](./html.md#要素)であり、[HTML](./html.md#html)文書の開始と終了を示すために使用される[タグ](./html.md#タグ)。すべての[HTML](./html.md#html)[要素](./html.md#要素)はこの[タグ](./html.md#タグ)の内部に配置される。

```html
<!DOCTYPE html>
<html>
 <!-- HTMLコンテンツ -->
</html>
```

また、[HTML](./html.md#html)文書内で使われている主要な言語を設定したい場合は、 `html` [要素](./html.md#要素)に `lang` [属性](./html.md#属性)を設定する。これは、[ブラウザ](../../../../network/_/chapters/web.md#webブラウザ)の翻訳ツールやアクセス解析のデータに影響するため、設定が推奨される。

```html
<!-- 主要な言語を日本語に設定 -->
<html lang="ja">
```


## head

**head**は、[HTML](./html.md#html)文書のヘッド部を記述するための[タグ](./html.md#タグ)。**ヘッド部***は、[HTML](#html)文書のうち、ページが読み込まれても[Webブラウザ](../../../../network/_/chapters/web.md#webブラウザ)に表示されない部分。 `title` や[CSS](../../../css/_/chapters/css.md#css)および[JavaScript](../../../javascript/_/chapters/javascript.md#javascript)へのリンク、独自のファビコンへのリンク、その他のメタデータの情報などを含む。

### title

**title**は、[HTML](./html.md#html)文書全体のタイトルを表すメタデータとなる[タグ](./html.md#タグ)で、 `head` [タグ](./html.md#タグ)内に記述される。この[要素](./html.md#要素)の内容は、SEOに大きく影響しており、ユーザが検索結果ページで目にすることになる重要なものとなる。また、 `title` はページのブックマークやSNSシェア時の表示などにも利用される。

```html
<!DOCTYPE html>
<html>
 <head>
  <meta charset="utf-8">

  <!-- ページタイトルの設定 -->
  <title>Example page</title>
 </head>
 <body></body>
</html>
```

### meta

**meta**は、[HTML](./html.md#html)文書のメタデータを記述するための[タグ](./html.md#タグ)で、 `head` [タグ](./html.md#タグ)内に複数記述することができる。[HTML](./html.md#html)文書の文字コードの指定や、作成者やページの説明の追加、SNSシェア時のOGP画像の設定などに使用する。

```html
<!DOCTYPE html>
<html>
 <head>

  <!-- ページの文字コードの設定 -->
  <meta charset="utf-8">

  <title>Example page</title>

  <!-- ページ概要の設定 -->
  <meta name="description" content="ページ概要">

  <!-- キーワードの設定 -->
  <meta name="keywords" content="キーワード1, キーワード2, キーワード3">
 </head>
 <body></body>
</html>
```

### link

**link**は、[HTML](./html.md#html)文書内で外部リソースとの関連付けやスタイルシートの適用などを行うために使用される[タグ](./html.md#タグ)。 `rel` [属性](./html.md#属性)にはリンクの関係性、 `type` [属性](./html.md#属性)にはリンクされている[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の種類、 `href` [属性](./html.md#属性)にはリンク先の[URL](../../../../network/_/chapters/web.md#url)を指定する。

linkは主に[CSS](../../../css/_/chapters/css.md#css)やファビコンの設定などに使用される。

```html
<!DOCTYPE html>
<html>
 <head>
  <meta charset="utf-8">
  <title>Example page</title>

  <!-- スタイルシートの関連付け -->
  <link rel="stylesheet" type="text/css" href="style.css">

  <!-- ファビコンの設定 -->
  <link rel="icon" type="image/png" href="favicon.png">
 </head>
 <body></body>
</html>
```


## script

**script**は、[HTML](./html.md#html)文書内にスクリプトコード（[JavaScript](../../../javascript/_/chapters/javascript.md#javascript)）を埋め込むために使用される[タグ](./html.md#タグ)。直接スクリプトコードを記述したり、外部の[JavaScript](../../../javascript/_/chapters/javascript.md#javascript)[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)を読み込んだりすることができる。

```html
<script>
// 直接JavaScriptコードを記述
alert("Hello, world!");
</script>

<!-- 外部のJavaScriptファイルを読み込み -->
<script src="script.js"></script>
```

### noscript

**noscript**は、[ブラウザ](../../../../network/_/chapters/web.md#webブラウザ)で[JavaScript](../../../javascript/_/chapters/javascript.md#javascript)が動作しない場合に変わりのコンテンツを表示するために使用される[タグ](./html.md#タグ)。

```html
<noscript>
JavaScriptが無効になっています。ブラウザの設定からJavaScriptの動作を有効化してください。
</noscript>
```
