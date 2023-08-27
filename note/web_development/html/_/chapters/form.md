# 『フォーム』ノート

（最終更新： 2023-08-26）


## 目次

1. [form](#form)
	1. [fieldset](#fieldset)
	1. [legend](#legend)
1. [input](#input)
1. [textarea](#textarea)
1. [select](#select)
	1. [option](#option)
	1. [optgroup](#optgroup)
	1. [datalist](#datalist)
1. [label](#label)
1. [button](#button)


## form

**form**は、[HTML](./html.md#html)文書において、フォームを[マークアップ](./html.md#マークアップ)する目的で使用される[タグ](./html.md#タグ)。フォームとは、ユーザが入力した内容を[Webサーバ](../../../../network/_/chapters/web.md#webサーバ)に送信するための機能であり、 `form` タグはフォーム全体を包む親要素となる。

**action**[属性](./html.md#属性)に対して、フォームのデータをどの[URL](../../../../network/_/chapters/web.md#url)に送信するかを指定する。また、**method**[属性](./html.md#属性)に対して、フォームの送信時の[HTTPメソッド](../../../../network/_/chapters/web.md#httpメソッド)を指定する（[GET](../../../../network/_/chapters/web.md#httpメソッド)か[POST](../../../../network/_/chapters/web.md#httpメソッド)のどちらか）。

### fieldset

**fieldset**は、[HTML](./html.md#html)文書において、複数のフォーム部品をまとめてグループ化する目的で使用される[タグ](./html.md#タグ)。

### legend

**legend**は、[HTML](./html.md#html)文書において、グループ化されたフォーム部品にキャプションをつける目的で使用される[タグ](./html.md#タグ)。 `fieldset` の直後に記述する必要がある。


## input

**input**は、[HTML](./html.md#html)文書において、フォーム内の入力項目を[マークアップ](./html.md#マークアップ)する目的で使用される[タグ](./html.md#タグ)。

**type**[属性](./html.md#属性)のキーワードに応じて様々な種類のフォーム部品を表示することができる。フォーム送信時はそのデータを特定するために**name**[属性](./html.md#属性)の値がキーとして使用され、送信される値は**value**[属性](./html.md#属性)となる。 `type` に指定できるキーワードには次のようなものがある。

- **text**: テキスト入力のためのフィールド（改行は除去される）。
- **password**: 主にパスワード入力に用いられる、入力値を隠すテキストフィールド。
- **email**: 電子メールアドレスを入力するための入力欄。
- **tel**: 電話番号を入力するための入力欄。
- **url**: URLを入力するための入力欄。
- **number**: 数値を入力するためのコントロールで、スピナーを表示する。
- **range**: 厳密な値であることが重要ではない数値を入力するためのコントロールで、範囲のウィジェットを表示する。
- **checkbox**: 選択または未選択のうちどちらかの値を取ることができるチェックボックス。
- **radio**: 複数の選択肢から1つの値を選択することができるラジオボタン。
- **date**: 日付指定のための日付ピッカーのコントロール。
- **time**: 時間指定のためのコントロール。
- **file**: [ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)選択のためのコントロール。
- **color**: 色指定のためのカラーピッカーのコントロール。
- **button**: 規定の動作は持たず、 `value` 属性の値を表示するボタン。
- **submit**: フォームの入力内容を[サーバ](../../../../network/_/chapters/web.md#webサーバ)に送信するボタン。
- **hidden**: 画面に表示されないコントロールで、隠し値を[サーバ](../../../../network/_/chapters/web.md#webサーバ)に送信する目的で使用される。

**placeholder**[属性](./html.md#属性)に値を指定することで、フィールドがどのような情報を期待しているかについてユーザにヒントを示すことができる。また、**required**[属性](./html.md#属性)を付与することで入力項目を必須に、**readonly**[属性](./html.md#属性)を付与することで入力項目を読み取り専用に、**autofocus**[属性](./html.md#属性)を付与することでページ表示時に最初に入力項目にフォーカスするようにできる。


## textarea

**textarea**は、[HTML](./html.md#html)文書において、改行を含むことのできる長文のテキスト入力フォーム部品を[マークアップ](./html.md#マークアップ)する目的で使用される[タグ](./html.md#タグ)。SNSの投稿やお問い合わせフォームなどを作成する際に用いられることが多い。


## select

**select**は、[HTML](./html.md#html)文書において、プルダウンメニュー（ドロップダウンメニュー）の選択肢から値を選ぶフォーム部品を[マークアップ](./html.md#マークアップ)する目的で使用される[タグ](./html.md#タグ)。選択肢は `option` [タグ](./html.md#タグ)で作成する。[サーバ](../../../../network/_/chapters/web.md#webサーバ)に送信されるキーは `name` の値となり、その値は `option` に指定された `value` となる。

また、**multiple**[属性](./html.md#属性)を付与することで選択肢の中から複数の値を選択することができるようになる。

### option

**option**は、[HTML](./html.md#html)文書において、プルダウンメニュー（ドロップダウンメニュー）の選択肢を[マークアップ](./html.md#マークアップ)する目的で使用される[タグ](./html.md#タグ)。

**selected**[属性](./html.md#属性)を付与することで、ページ表示時に最初に選択されている選択肢を明示的に指定することができる。

### optgroup

**optgroup**(option group)は、[HTML](./html.md#html)文書において、プルダウンメニュー（ドロップダウンメニュー）の選択肢をグループ化する目的で使用される[タグ](./html.md#タグ)。 `select` の `option` の数が多すぎる場合に、それらをカテゴリごとに分類するために使用される。

**label**[属性](./html.md#属性)に指定した値がグループ名となる。

### datalist

**datalist**は、[HTML](./html.md#html)文書において、検索機能を持つプルダウンメニュー（ドロップダウンメニュー）を[マークアップ](./html.md#マークアップ)する目的で使用される[タグ](./html.md#タグ)。 `input` と紐付けてユーザの入力を補助するために用いられる場合が多い。補助される選択肢は `option` [タグ](./html.md#タグ)で列挙することができる。

`datalist` を使用する場合は、この[タグ](./html.md#タグ)にID[属性](./html.md#属性)を設定しておき、 `input` の**list**[属性](./html.md#属性)にそのID名を指定する。


## label

**label**は、[HTML](./html.md#html)文書において、フォーム部品にラベルをつける目的で使用される[タグ](./html.md#タグ)。 `input` [タグ](./html.md#タグ)などと隣接するように設置される場合が多く、明示的にフォーム部品と紐付けることで、ラベルクリック時にフォーム部品をフォーカスすることができるようになる。

`label` をフォーム部品に明示的に結びつけるには、フォーム部品を `label` で囲むか、 `label` の**for**[属性](./html.md#属性)に対してフォーム部品のID名を指定すればよい。


## button

**button**は、[HTML](./html.md#html)文書において、汎用的なボタンを[マークアップ](./html.md#マークアップ)する目的で使用される[タグ](./html.md#タグ)。フォームとは別に、主に[JavaScript](../../../javascript/_/chapters/javascript.md#javascript)によって何かしらの処理を実行するための[ユーザインタフェース](../../../../computer/software/_/chapters/software.md#ui)となる。
