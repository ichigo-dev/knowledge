# ハイパーメディアフォーマット


## 目次

- [HTML](#html)
	- [HTMLの仕様](#htmlの仕様)
- [メタデータの埋め込み](#メタデータの埋め込み)
	- [セマンティクス](#セマンティクス)
	- [RDF](#rdf)
	- [microformats](#microformats)
	- [RDFa](#rdfa)
- [Atom](#atom)
	- [エントリ](#エントリ)
- [AtomPub](#atompub)
- [JSON](#json)
	- [JSONP](#jsonp)


## HTML

**HTML**（HyperText Markup Language）は、**タグ**によって文章の構造を表現する**マークアップ言語**のひとつ。マークアップされた構造を持つ文書のことを**構造化文書**と呼ぶ。拡張子は `.html` もしくは `.htm` 。

HTMLはもともと、**SGML**をベースとしてBerners-Leeが開発したものだったが、のちに**XML**をベースにしたXHTMLが開発された。

### HTMLの仕様

HTMLでは**要素**（Element）を、**開始タグ**と**内容**と**終了タグ**で表現する。この要素を入れ子にすることで、文書を**木構造**として表現することができる。この時、要素同士の相対的階層関係から、ある要素よりも上位の要素を**親要素**、下位の要素を**子要素**と呼ぶ。また、内容を持たない要素を**空要素**と呼び、空要素では終了タグを省略できる。

要素は**属性**（Attribute）と呼ばれる、属性名と属性値からなる補助的な情報を持つことができる。


## メタデータの埋め込み

### セマンティクス

**セマンティクス**（意味論）とは、言語が持つ意味を扱う理論。

プログラミング言語が持つ意味を確定させるための理論のことを**プログラム意味論**という。**コンパイル**とは、コンパイラがプログラミング言語の持つ文法を解析し、マシン語に翻訳する作業のことである。コンパイラが文法を理解して翻訳するためには、プログラミング言語の仕様を形式的に記述する必要がある。

Webにおいて[リソース](/note/web/chapters/03_rest.ja.md#リソース)が持つ意味を確定させるために、Webページの意味をプログラムからも処理できるように形式的に意味を記述するための技術を**セマンティックWeb**という。

### RDF

**RDF**（Resource Description Framework）では、**トリプル**と呼ばれる主語、述語、目的語の3つの組を使って、Web上の[リソース](/note/web/chapters/03_rest.ja.md#リソース)にメタデータを与え、プログラムが[リソース](/note/web/chapters/03_rest.ja.md#リソース)の意味を処理できるようにする。

RDFは記述が冗長になるという欠点があり、統一的な記述がしづらいという欠点がある。

### microformats

**microformats**は、HTML文書に対して必要最低限の情報を追加してメタデータを埋め込む技術。

**elemental microformats**は、リンク関係（ `<a>` 要素や `<link>` 要素の `rel` 属性）を使ってメタデータを表現するフォーマット。**rel-license**（ライセンス情報）は `rel="license"` で指定でき、Webページのライセンスを記述するためのmicroformats。**rel-nofollow**（スパムリンク防止）は `rel="no-follow"` で指定でき、検索エンジンに対してリンクをランキングの重みづけに利用させないようにするためのmicroformats。

**compound microformats**は、主に `class` 属性を使って階層構造のあるメタデータを表現するフォーマット。**hCalendar**（イベント情報）は、カレンダー情報、イベント情報を記述するためのmicroformats。**hAtom**（更新情報）は、Atomが持つメタデータをHTMLに埋め込むmicroformats。

```html
<!-- hCalendar -->
<ul class="vevent">
 <li class="summary">
  <a class="url" href="https://...">
   サンプルイベント
  </a>
 </li>

 <li>
  日時： 2022年1月1日
  日時： 2022年1月1日
  <addr class="dtstart" title="2022-01-01T19:00:00+09:00">19:00</addr>
   ~
  <addr class="dtend" title="2022-01-01T21:00:00+09:00">21:00</addr>
 </li>

 <li>
  <span class="location">
   東京都〇〇区
  </span>
 </li>
</ul>

<!-- hAtom -->
<div class="hfeed">
 <div class="hentry">
  <h2 class="entry-title">
   <a href="https://..." rel="bookmark">サンプル</a>
  </h2>

  <div class="entry-content">
   <p>hAtomのサンプルです。</p>
  </div>

  <p>
   <addr class="updated" title="2022-01-01T09:00:00+09:00">
    2022年1月1日 9:00
   </addr>
  </p>
 </div>
</div>
```

### RDFa

[microformats](#microformats)では意味付けを意図していない `class` 属性や `rel` 属性を持ったWebページがあった場合にプログラムが誤判定を起こしてしまう。

この問題点を解決するために[W3C](/note/web/chapters/02_history_of_web.ja.md#webの標準化)が標準化を進めているのが**RDFa**（RDF - in - attributes）で、 `class` などの名前の衝突問題をXMLの名前空間で解決する。 `cc` の接頭辞で名前空間を定義しているため、衝突問題を回避できる。ただし、XHTMLでしか利用できないという短所もある。

```xml
<a xmlns:cc="https://..." rel="cc:license" href="https://...">RDFaの例</a>
```


## Atom

**Atom**（Atom Syndication Format）はブログなどの更新情報を配信するためのフィード（**RSS**）として用いられる。拡張子は `.atom` である。Atomは大きく分けて**メンバリソース**と**コレクションリソース**から構成されている。

メンバリソースは、Atomにおける最小の[リソース](/note/web/chapters/03_rest.ja.md#リソース)単位で、ブログであれば1つ1つの記事のことを指す。メンバリソースは、XMLで表現できる**エントリリソース**と、それ以外の**メディアリソース**に分かれる。エントリリソースは `<entry>` 要素で囲った**エントリ**として表現し、メディアリソースは**メディアリンクエントリ**という特別なエントリで表現する。

コレクションリソースは、複数のメンバリソースを含む[リソース](/note/web/chapters/03_rest.ja.md#リソース)で、`<feed>`要素で囲った**フィード**として表現する。

### エントリ

エントリは、ID（ `<id>` ）、タイトル（ `<title>` ）、著者（ `<author>` ）、更新日時（ `<updated>` ）といったメタデータを持つ。エントリの子要素である`<content>`には、HTMLやXML、プレーンテキストなど様々な情報を含めることができる。


## AtomPub

**Atom Publishing Protocol**（**AtomPub**）は、[Atom](#atom)を利用した[リソース](/note/web/chapters/03_rest.ja.md#リソース)編集を行うための[プロトコル](/note/internet/chapters/01_basic_knowledge_of_network.ja.md#プロトコル)である。AtomPubでは、[Atom](#atom)が規定しているリソースモデルをベースに、[エントリ](#エントリ)を操作する。AtomPubでは、コレクションのメタデータを表現する**サービス文書**と、[エントリ](#エントリ)のカテゴリに指定できる値を列挙する**カテゴリ文書**を追加している。


## JSON

**JSON**（JavaScript Object Notation）は**データ記述言語**で、JavaScriptの記法でデータを記述できるという特徴がある。拡張子は `.json` 。JSONのデータ型には、**オブジェクト**、**配列**、**文字列**、**数値**、**ブーリアン**、**null**がある。

### JSONP

[JSON](#json)で[リソース](/note/web/chapters/03_rest.ja.md#リソース)表現を提供する副次的効果として、**JSONP**（JSON with Padding）を利用できることが挙げられる。JSONPは、**クロスドメイン通信**（不特定多数の[ドメイン](/note/internet/chapters/07_internet_layer.ja.md#dnsの役割)に属するサーバにアクセスする通信）を実現するための機能である。
