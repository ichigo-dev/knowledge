# 『Proxy』ノート

（最終更新： 2023-08-05）


## 目次

1. [Proxyパターン](#proxyパターン)
	1. [Subject](#subject)
	1. [Proxy](#proxy)
	1. [RealSubject](#realsubject)


## Proxyパターン

**Proxyパターン**は、本人の代理人となる[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)が本人でもなくともできる処理を受け持ち、本人にしかできない場合にだけ処理を任せる[デザインパターン](./design_pattern.md#デザインパターン)。負荷の高い[オブジェクト](../../../../programming/_/chapters/object_oriented.md#オブジェクト)の代わりに一部の処理を代理人が行うことで、負荷を軽減することができる。

Proxyパターンは、[Subject](#subject)、[Proxy](#proxy)、[RealSubject](#realsubject)から構成される。

### Subject

**Subject**（主体）は、[Proxyパターン](#proxyパターン)において、[Proxy](#proxy)と[RealSubject](#realsubject)を同一視するための[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)。これにより、[Proxyパターン](#proxyパターン)を利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)は、[Proxy](#proxy)と[RealSubject](#realsubject)の使い分けを意識する必要がなくなる。

### Proxy

**Proxy**（代理人）は、[Proxyパターン](#proxyパターン)において、このパターンを利用する[プログラム](../../../../programming/_/chapters/programming.md#プログラム)からの要求をできる限り処理する[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。自身が処理できない処理に関しては、[RealSubject](#realsubject)に依頼する。Proxyは、[Subject](#subject)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を実装する。

### RealSubject

**RealSubject**（実際の主体）は、[Proxyパターン](#proxyパターン)において、[Proxy](#proxy)では手に負えない処理を要求された際に、[Proxy](#proxy)からの依頼を受けて処理を行う[クラス](../../../../programming/_/chapters/object_oriented.md#クラス)。[Proxy](#proxy)と同じく[Subject](#subject)が定義する[インタフェース](../../../../programming/_/chapters/object_oriented.md#インタフェース)を実装する。
