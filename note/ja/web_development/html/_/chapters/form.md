# 『フォーム』ノート

（最終更新： 2023-08-25）


## 目次

1. [form](#form)
1. [input](#input)


## form

**form**は、[HTML](./html.md#html)文書において、フォームを[マークアップ](./html.md#マークアップ)する目的で使用される[タグ](./html.md#タグ)。フォームとは、ユーザが入力した内容を[Webサーバ](../../../../network/_/chapters/web.md#webサーバ)に送信するための機能であり、 `form` タグはフォーム全体を包む親要素となる。

**action**[属性](./html.md#属性)に対して、フォームのデータをどの[URL](../../../../network/_/chapters/web.md#url)に送信するかを指定する。また、**method**[属性](./html.md#属性)に対して、フォームの送信時の[HTTPメソッド](../../../../network/_/chapters/web.md#httpメソッド)を指定する（[GET](../../../../network/_/chapters/web.md#httpメソッド)か[POST](../../../../network/_/chapters/web.md#httpメソッド)のどちらか）。


## input

**input**は、[HTML](./html.md#html)文書において、フォーム内の入力項目を[マークアップ](./html.md#マークアップ)する目的で使用される[タグ](./html.md#タグ)。

**type**[属性](./html.md#属性)のキーワードに応じて様々な種類のフォーム部品を表示することができる。また、フォーム送信時はそのデータを特定するために**name**[属性](./html.md#属性)の値がキーとして使用される。
