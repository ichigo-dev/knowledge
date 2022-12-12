# Neovim設定


## セットアップ

- [Neovimのダウンロード](https://neovim.io/)

### パッケージマネージャ

パッケージマネージャとして[tani/vim-jetpack](https://github.com/tani/vim-jetpack)を使用しており、初期化のために各環境に応じて以下のコマンドを実行する必要がある。

```bash
// Neovim(Linux or MacOS)
$ curl -fLo ~/.local/share/nvim/site/pack/jetpack/opt/vim-jetpack/plugin/jetpack.vim --create-dirs https://raw.githubusercontent.com/tani/vim-jetpack/master/plugin/jetpack.vim

// Neovim(Windows)
$ curl -fLo %USERPROFILE%\AppData\Local\nvim-data\site\pack\jetpack\opt\vim-jetpack\plugin\jetpack.vim --create-dirs https://raw.githubusercontent.com/tani/vim-jetpack/master/plugin/jetpack.vim
```

次にNeovimを開いて `:JetpackSync` を実行することで、プラグインをインストールすることができる。定期的にこのコマンドを実行してパッケージをアップデートすることができ、場合によっては設定ファイル（luaファイル）を調整する必要がある。


## ショートカット

独自定義のショートカットのうち、よく利用するものを挙げる。

- `<Space>t` : ターミナルコマンドの実行
- `<Space>y` : システムのクリップボードにヤンク
- `<Space>p` : システムのクリップボードからペースト

---

- `<Space>s` : ウィンドウ水平分割
- `<Space>v` : ウィンドウ垂直分割
- `<Space>j` : 下のウィンドウに移動
- `<Space>k` : 上のウィンドウに移動
- `<Space>h` : 左のウィンドウに移動
- `<Space>l` : 右のウィンドウに移動
- `<Space>J` : 今のウィンドウを下に移動
- `<Space>K` : 今のウィンドウを上に移動
- `<Space>H` : 今のウィンドウを左に移動
- `<Space>L` : 今のウィンドウを右に移動
- `<Space>w` : 次のウィンドウに移動
- `<Space>W` : 前のウィンドウに移動
- `<Space>r` : 今のウィンドウを次に移動
- `<Space>R` : 今のウィンドウを前に移動
- `<Space><` : 今のウィンドウサイズを水平方向に縮小
- `<Space>>` : 今のウィンドウサイズを水平方向に拡大
- `<Space>-` : 今のウィンドウサイズを垂直方向に縮小
- `<Space>+` : 今のウィンドウサイズを垂直方向に拡大

---

- `<C-n>` : Fernツリーを表示・非表示
- `<C-z>` : FZF検索


## LSP

LSP(Language Server Protocol)の管理用プラグインとして[williamboman/mason](https://github.com/williamboman/mason.nvim)を利用している。言語を追加したい場合は、 `:Mason` を実行してコントローラを開き任意のLSPを選択するか、 `:MasonInstall` から直接LSP指定でインストールを行う。


## プラグイン

### ファイラー

- [lambdalisue/fern.vim](https://github.com/lambdalisue/fern.vim)

### ステータスライン

- [nvim-lualine/lualine.nvim](https://github.com/nvim-lualine/lualine.nvim)

### ファジー検索

- [junegunn/fzf.vim](https://github.com/junegunn/fzf.vim)

### 補完

- [neovim/nvim-lspconfig](https://github.com/neovim/nvim-lspconfig)
- [hrsh7th/nvim-cmp](https://github.com/hrsh7th/nvim-cmp)

### Git

- [tpope/vim-fugitive](https://github.com/tpope/vim-fugitive)
- [lewis6991/gitsigns.nvim](https://github.com/lewis6991/gitsigns.nvim)
