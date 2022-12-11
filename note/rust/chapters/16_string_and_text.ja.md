# 文字列とテキスト


## 目次

1. [Unicode](#unicode)
	1. [ASCII](#ascii)
	1. [UTF-8](#utf-8)
1. [文字](#文字)
1. [Stringとstr](#文字)


## Unicode

**Unicode**は、世界で使われている文字を共通の文字集合として利用できるようにしようという考えで作られた文字コードの規格である。

### ASCII

Unicodeと**ASCII**は、すべてのASCIIのコードポイント（0x00から0x7f）において一致する。UnicodeはASCIIのコードポイントをISO/IEC 8859-1文字セットに割り当てており、このコードポイントは**Latin-1コードブロック**と呼ばれている。

### UTF-8

RustのString型やstr型は、テキストを**UTF-8**形式で表現する。UTF-8は、1文字を1バイトから4バイトの列にエンコードする。


## 文字

RustのcharはUnicodeのコードポイントを保持する32ビットの値である。
