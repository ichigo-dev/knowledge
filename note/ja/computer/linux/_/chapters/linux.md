# 『Linux』ノート

（最終更新： 2023-03-23）


## 目次

1. [Linux](#linux)
	1. [狭義のLinux](#狭義のlinux)
	1. [広義のLinux](#広義のlinux)
1. [ディストリビューション](#ディストリビューション)
	1. [ディストリビュータ](#ディストリビュータ)
	1. [Red Hat系](#red-hat系)
	1. [Debian系](#debian系)
	1. [Slackware系](#slackware系)
	1. [独立系](#独立系)
1. [パッケージ](#パッケージ)
	1. [パッケージマネージャ](#パッケージマネージャ)
	1. [RPM](#rpm)
	1. [yum](#yum)
	1. [dnf](#dnf)
	1. [dpkg](#dpkg)
	1. [apt](#apt)


## Linux

**Linux**は、Linus Torvaldsによって実装された[OS](../../../software/_/chapters/operating_system.md#オペレーティングシステム)であり、[サーバ](../../../_/chapters/computer.md#サーバ)用から組み込み機器用、学習用、デスクトップ用など、幅広い用途で利用されている。設計は[UNIX](../../../software/_/chapters/operating_system.md#unix)に非常に似ているものの、[System V](../../../software/_/chapters/operating_system.md#unix)の流れも[BSD](../../../software/_/chapters/operating_system.md#unix)の流れも汲まない独自の[UNIX互換OS](../../../software/_/chapters/operating_system.md#unix)として開発された。

[UNIX](../../../software/_/chapters/operating_system.md#unix)は高いライセンス使用料が必要であり、個人用として利用するには敷居が高いが、Linuxは[GPLライセンス](../../../software/_/chapters/open_source_software.md#gpl)であり誰でも無償で利用することができる。さらに、[ソースコード](../../../../programming/_/chapters/programming.md#ソースコード)の改変や再配布も自由に行えたため多くの[ディストリビュータ](#ディストリビュータ)により改良され、一般に広まることとなった。

また、Linuxには高品質の[ソフトウェア](../../../software/_/chapters/software.md#ソフトウェア)が多く揃っているため、大変使いやすい[OS](../../../software/_/chapters/operating_system.md#オペレーティングシステム)となっている。

### 狭義のLinux

**狭義のLinux**は、[Linux](#linux)[カーネル](../../../software/_/chapters/operating_system.md#カーネル)を指す言葉。[OS](../../../software/_/chapters/operating_system.md#オペレーティングシステム)の中核となる、[ハードウェア](../../../hardware/_/chapters/hardware.md#ハードウェア)制御を行う[ソフトウェア](../../../software/_/chapters/software.md#ソフトウェア)のみを指し、実際にユーザが用いるツールや[アプリケーション](../../../software/_/chapters/#応用ソフトウェア)は含まれない。

### 広義のLinux

**広義のLinux**は、[Linux](#linux)[カーネル](../../../software/_/chapters/operating_system.md#カーネル)に加えて、基本的な[コマンド](./basic_command.md#コマンド)群や[アプリケーション](../../../software/_/chapters/#応用ソフトウェア)などを含めて、ユーザがそのまま利用できるようにパッケージングして提供されたものを指す言葉。一般的には、単に[Linux](#linux)というときには、広義のLinuxのことを指す場合が多い。


## ディストリビューション

**ディストリビューション**（**Linuxディストリビューション**）は、[Linux](#linux)[カーネル](../../../software/_/chapters/operating_system.md#カーネル)と様々なツールや[アプリケーション](../../../software/_/chapters/software.md#応用ソフトウェア)群を組み合わせて[パッケージ](../../../software/_/chapters/package.md#パッケージ)化したもの。[Linux](#linux)は[OSS](../../../software/_/chapters/open_source_software.md#オープンソースソフトウェア)である性質上、様々な開発者によってメンテナンスされる、多くの種類のディストリビューションが存在する。

### ディストリビュータ

**ディストリビュータ**は、[Linuxディストリビューション](#ディストリビューション)の開発やメンテナンスを行う人、あるいは団体を指す言葉。[Linux](#linux)に限らず、[OSS](../../../software/_/chapters/open_source_software.md#オープンソースソフトウェア)のメンテナンスを行う人のことを指す場合にも用いられる。

### Red Hat系

**Red Hat系**の[ディストリビューション](#ディストリビューション)は、Red Hat社によって開発されたRed Hat Linuxをベースに開発された[ディストリビューション](#ディストリビューション)。バランスに特化した[ディストリビューション](#ディストリビューション)で、[サーバ](../../../_/chapters/computer.md#サーバ)などの用途で利用されることが多い。

有償の**Red Hat Enterprise Linux**(**RHEL**)や、RHELとの互換性を目指したフリーのOSである**CentOS**、REHLの後継として開発が進められた**Fedora**などが有名。

### Debian系

**Debian系**の[ディストリビューション](#ディストリビューション)は、ユーザフレンドリな考えのもと開発されているDebian Linuxをベースに開発された[ディストリビューション](#ディストリビューション)。見た目の美しさや、[Linux](#linux)に不慣れなユーザにも使いやすいことから人気が高い。

**Debian GNU/Linux**や**Ubuntu**、**Linux Mint**などが有名で、デスクトップ用途での人気が高い。

### Slackware系

**Slackware系**の[ディストリビューション](#ディスとリビューション)は、最古の[ディストリビューション](#ディストリビューション)であるSlackwareをベースに開発された[ディストリビューション](#ディストリビューション)。シンプルゆえに安全性が高く、セキュリティが強固であるという特徴がある。

**Slackware**や**openSUSE**などが有名。

### 独立系

**独立系**の[ディストリビューション](#ディストリビューション)は、独自の進歩を遂げた[ディストリビューション](#ディストリビューション)の総称で、個性的なものも多い。

**Arch Linux**は、シンプリシティ、ミニマリズム、エレガンスに焦点を当てて開発された[Linux](#linux)。


## パッケージ

**パッケージ**は、[Linux](#linux)で利用可能な[アプリケーション](../../../software/_/chapters/software.md#応用ソフトウェア)を、ユーザが容易に利用できるようにディストリビュータによって提供されたもの。パッケージを利用しない場合は、ユーザ自身でソースコードをダウンロードしてそれをビルドする必要がある。

### パッケージマネージャ

**パッケージマネージャ**は、パッケージの導入や管理を容易にするためのツール。パッケージが依存しているライブラリの解決を行ったり、パッケージマネージャを通して導入したパッケージの更新などを補助したりする。Linuxディストリビューションにはパッケージマネージャが備えられている。

### RPM

**RPM**(RPM Package Manager)は、Red Hat系のディストリビューションで利用されるパッケージマネージャ。

### yum

**yum**(Yellowdog Updater Modified)は、Red Hat系のディストリビューションで利用されるパッケージマネージャ。RPMの機能に加えて、パッケージ間の依存関係を管理する機能を持っている。

### dnf

**dnf**(Dandified yum)は、Red Hat系のディストリビューションで利用されるパッケージマネージャ。yumの後継ツールで、yumの欠点を改善していたり、パフォーマンス面で優れている。

### dpkg

**dpkg**は、Debian系のディストリビューションで利用されているパッケージマネージャ。

### apt

**apt**は、Debian系のディストリビューションで利用されているパッケージマネージャ。dpkgの機能に加えて、パッケージ間の依存関係を管理する機能を持っている。
