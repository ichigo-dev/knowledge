# 『JavaScript』ノート

（最終更新： 2023-08-27）


## 目次

1. [JavaScript](#javascript)
1. [DOM](#dom)


## JavaScript

**JavaScript**(JS)は、主に[Webページ](../../../../network/_/chapters/web.md#web)のインタラクティブな機能を実装するために使用される[プログラミング言語](../../../../programming/_/chapters/programming.md#プログラミング言語)。[Webブラウザ](../../../../network/_/chapters/web.md#webブラウザ)上で動作させる場合が多いが、[サーバ](../../../../network/_/chapters/web.md#webサーバ)でJavaScript実行するためのNode.jsといった環境が存在したり、動画編集[ソフトウェア](../../../../computer/software/_/chapters/software.md#応用ソフトウェア)で使用されるケースもあるなど、[プログラミング言語](../../../../programming/_/chapters/programming.md#プログラミング言語)としてのわかりやすさから幅広く普及している。

JavaScriptを利用することで、ユーザのアクション（フォーム送信やボタンのクリックなど）に反応する機能を作ったり、[DOM](#dom)を介して[HTML](../../../html/_/chapters/html.md#html)[要素](../../../html/_/chapters/html.md#要素)を操作したり、[ブラウザ](../../../../network/_/chapters/web.md#webブラウザ)の[ストレージ](../../../../computer/hardware/_/chapters/hardware.md#記憶装置)を利用してデータを保存したりできる。


## DOM

**DOM**(Document Object Model)は、[Webページ](../../../../network/_/chapters/web.md#web)の構造を表現し、[HTML](../../../html/_/chapters/html.md#html)[要素](../../../html/_/chapters/html.md#要素)やコンテンツにアクセスするためのインタフェース。[HTML](../../../html/_/chapters/html.md#html)が読み込まれると、[ブラウザ](../../../../network/_/chapters/web.md#webブラウザ)はそれを解析して、階層構造を持つツリー形式で[Webページ](../../../../network/_/chapters/web.md#web)を表現するDOMを構築する。DOMは[JavaScript](#javascript)などの[プログラミング言語](../../../../programming/_/chapters/programming.md#プログラミング言語)によって操作することが可能で、[要素](../../../html/_/chapters/html.md#要素)の追加や削除、変更、イベント処理、スタイルの変更などができる。DOMにおいて各[要素](../../../html/_/chapters/html.md#要素)は**ノード**と呼ばれる。
