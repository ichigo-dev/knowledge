# 『テーブル』ノート

（最終更新： 2023-08-25）


## 目次

1. [table](#table)
	1. [tr](#tr)
	1. [td](#td)
	1. [th](#th)
	1. [thead](#thead)
	1. [tbody](#tbody)
	1. [tfoot](#tfoot)
	1. [caption](#caption)


## table

**table**は、[HTML](./html.md#html)文書において、テーブル（表）を[マークアップ](./html.md#マークアップ)する目的で使用される[タグ](./html.md#タグ)。この[タグ](./html.md#タグ)でテーブル全体を囲み、 `tr` や `td` によって行やセルを表現する。原則として、テーブルの各行には同じ数のセルを作る（セルを結合する場合は例外）。

```html
<!-- シンプルなテーブル -->
<table>
 <tr>
  <td>ケーキ</td>
  <td>モンブラン</td>
  <td>400円</td>
 </tr>
 <tr>
  <td>ケーキ</td>
  <td>ショートケーキ</td>
  <td>350円</td>
 </tr>
</table>

<!-- 複雑なテーブル -->
<table>
 <thead>
  <tr>
   <th colspan="2">品名</th>
   <th>金額</th>
  </tr>
 </thead>

 <tbody>
  <tr>
   <td rowspan="2">ケーキ</td>
   <td>モンブラン</td>
   <td>400円</td>
  </tr>
  <tr>
   <td>ショートケーキ</td>
   <td>350円</td>
  </tr>
 </tbody>

 <tfoot>
  <tr>
   <th colspan="2">合計金額</th>
   <td>750円</td>
  </tr>
 </tfoot>
</table>
```

### tr

**tr**(table row)は、[HTML](./html.md#html)文書において、テーブル（表）の行を[マークアップ](./html.md#マークアップ)する目的で、 `table` [タグ](./html.md#タグ)の中で使用される[タグ](./html.md#タグ)。行の中には `td` （セル）や `th` （ヘッダーセル）が含まれる。

### td

**td**(table data)は、[HTML](./html.md#html)文書において、テーブル（表）のデータ（セル）を[マークアップ](./html.md#マークアップ)する目的で、 `table` [タグ](./html.md#タグ)内の `tr` [タグ](./html.md#タグ)の中で使用される[タグ](./html.md#タグ)。

**rowspan**[属性](./html.md#属性)を指定することでセルを列方向に、**colspan**[属性](./html.md#属性)を指定することでセルを行方向に結合することができる。値としては結合する行数や列数を指定する。また、結合した分はセルの数を減らさなければテーブルが崩れてしまうので注意。

### th

**th**(table header)は、[HTML](./html.md#html)文書において、テーブル（表）のヘッダーセルを[マークアップ](./html.md#マークアップ)する目的で、 `table` [タグ](./html.md#タグ)内の `tr` [タグ](./html.md#タグ)の中で使用される[タグ](./html.md#タグ)。

### thead

**thead**(table head)は、[HTML](./html.md#html)文書において、テーブル（表）のヘッダー行を[マークアップ](./html.md#マークアップ)する目的で、 `table` [タグ](./html.md#タグ)の中で使用される[タグ](./html.md#タグ)。

### tbody

**tbody**(table body)は、[HTML](./html.md#html)文書において、テーブル（表）のボディ行を[マークアップ](./html.md#マークアップ)する目的で、 `table` [タグ](./html.md#タグ)の中で使用される[タグ](./html.md#タグ)。

### tfoot

**tfoot**(table foot)は、[HTML](./html.md#html)文書において、テーブル（表）のフッター行を[マークアップ](./html.md#マークアップ)する目的で、 `table` [タグ](./html.md#タグ)の中で使用される[タグ](./html.md#タグ)。

### caption

**caption**は、[HTML](./html.md#html)文書において、テーブル（表）にキャプションをつける目的で、 `table` [タグ](./html.md#タグ)の直後に記述される[タグ](./html.md#タグ)。
