# 『基本コマンド』ノート

（最終更新： 2023-03-05）


## 目次

1. [ファイル操作](#ファイル操作)
	1. [ls](#ls)
	1. [cp](#cp)


## ファイル操作

### ls

`ls` は、指定したディレクトリの中にあるファイルやディレクトリの一覧を取得するコマンド。ディレクトリを指定しなかった場合はカレントディレクトリ以下の一覧を取得する。

```sh
$ ls
repo/  scripts/  user/  workspace/

$ ls user
Desktop/  Documents/  Downloads/  Music/  Pictures/  Public/  Templates/  Videos/
```

`-a` オプションを付与すると、ドットファイル（ `.` から始まるファイル、隠しファイル）なども含めてすべて取得できる。 `-l` オプションを付与すると、詳細情報も含めた長い形式で結果を出力する。 `-t` オプションを付与すると、最終更新日時でソートして結果を出力する。 `-r` オプションを付与すると、逆順でソートして結果を出力する。

複数のオプションを組み合わせて指定することもできる。

```sh
$ ls user -lat
drwxr-xr-x 20 user user 4096 Mar  5 16:39 ../
drwxr-xr-x  2 user user 4096 Mar  5 16:19 Downloads/
drwxr-xr-x  2 user user 4096 Dec 17 16:39 Pictures/
drwxr-xr-x 10 user user 4096 Dec 11 14:29 ./
drwxr-xr-x  2 user user 4096 Aug 18  2022 Desktop/
drwxr-xr-x  2 user user 4096 Aug  7  2022 Documents/
drwxr-xr-x  2 user user 4096 Aug  7  2022 Music/
drwxr-xr-x  2 user user 4096 Aug  7  2022 Public/
drwxr-xr-x  2 user user 4096 Aug  7  2022 Templates/
drwxr-xr-x  2 user user 4096 Aug  7  2022 Videos/
```

`ls` コマンドでは、ワイルドカードとして `*` や `?` を用いることができる。 `*` は任意の文字列、 `?` は任意の位置文字を表すワイルドカード。

```sh
$ ls user/Pictures/*.png
sample_01.png  sample_02.png  sample_03.png

$ ls user/Pictures/sample_??.png
sample_01.png  sample_02.png  sample_03.png
```

### cp

`cp` は、ファイルを複製するコマンド。
