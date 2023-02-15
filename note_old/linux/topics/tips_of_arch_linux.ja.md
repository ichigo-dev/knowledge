# Arch LinuxのTips


## システムメンテナンス

- systemdサービスで失敗しているものがないかを確認する。

```
$ sudo systemctl --failed
$ sudo journalctl -p 3 -xb
```

- システムのアップデート。長期間アップデートしていないと致命的な不具合が生じる可能性があるので、注意して実行すること。

```
$ sudo pacman -Syu
```

- 孤児パッケージの削除。

```
$ sudo pacman -Rns $(pacman -Qtdq)
```


## デュアルブートにおいて時間がずれた場合

以下のコマンドを実行すれば正しい時刻に修正できる。

```
$ sudo hwclock --verbose --systohc --localtime
```
