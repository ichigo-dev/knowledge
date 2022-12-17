# tmuxチートシート


## セッションを作成

```sh
$ tmux new-session -s [セッション名]
```


## セッションの一覧

```sh
$ tmux list-session
```


## セッションをデタッチ、アタッチ

```sh
$ tmux attach -t [セッション名]
$ tmux detach
```


## セッションをキル

```sh
$ tmux kill-session -t [セッション名]
```


## ウィンドウ作成

```sh
$ tmux new-window
```


## ウィンドウのリネーム

```sh
$ tmux renmae-window [ウィンドウ名]
```
