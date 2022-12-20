# Arch Linuxのインストール


## 目次

1. [インストールメディアの準備](#インストールメディアの準備)
1. [インストールメディアの起動](#インストールメディアの起動)
1. [インターネットへの接続](#インターネットへの接続)
1. [システム時刻の調整](#システム時刻の調整)
1. [ディスクのフォーマット](#ディスクのフォーマット)
1. [パーティションのマウント](#パーティションのマウント)
1. [基本的なパッケージのインストール](#基本的なパッケージのインストール)
1. [システム設定](#システム設定)
1. [ロケール設定](#ロケール設定)
1. [コンピュータ名の設定](#コンピュータ名の設定)
1. [rootパスワードの変更](#rootパスワードの変更)
1. [ブートローダの設定](#ブートローダの設定)
1. [Network Managerのインストール](#network-managerのインストール)
1. [コンピュータの再起動](#コンピュータの再起動)
1. [ネットワーク設定](#ネットワーク設定)
1. [ユーザアカウントの作成](#ユーザアカウントの作成)
1. [AURヘルパのインストール](#aurヘルパのインストール)
1. [Pacmanの設定](#pacmanの設定)
1. [セキュリティ](#セキュリティ)
1. [参考リンク](#参考リンク)


## インストールメディアの準備

1. [Arch Linux download page](#https://archlinux.org/download/)からArch Linuxのミラーサイトを探す。

1. `archlinux-xxxx.xx.xx-x86_64.iso`のような形式のISOイメージをダウンロードする。

1. ダウンロードしたファイルが不正なものでないか、ISOイメージのチェックサムを計算することで確認することができる。

    1. まず、以下のコマンドでチェックサムを計算する

        ```
        > sha1sum archlinux-xxxx.xx.xx-x86_64.iso
        ```

    1. この結果を、イメージをダウンロードしてきたページのものと比較する。

1. Windowsユーザであれば、[Rufus](https://rufus.ie/ja/)をダウンロードする。

1. Rufusを用いてUSBにISOイメージを書き込む。


## インストールメディアの起動

1. Arch LinuxをインストールしたいコンピュータにUSBを挿し込み、UEFIモードで起動する。

    1. コンピュータをUEFI(BIOS)モードで起動するには、`F2`キーか`Delete`キーを連打する（UEFIモードを起動に使用するキーはメーカーによって異なる）。
    1. セキュアブートの設定をオフにする。
    1. スタートアップの優先順位の設定を開き、USBの優先度を最優先にする。

1. 表示されたメニューパネルから、`Arch Linux install media (x86_64, UEFI)`を選択する。

1. シェルが起動され、黒い画面が現れる。

    1. もし使用しているキーボードレイアウトがUS配列のものでなければ、配列を変更することができる。以下は、日本語配列のキーボードレイアウトに変更するための例である。

        ```
        # loadkeys jp106
        ```

    1. もし文字が小さくて見づらい場合は、次のコマンドでフォントを変更することができる。

        ```
        # setfont ter-128n
        ```


## インターネットへの接続

1. 有線LANの場合は接続するだけでコンピュータに認識される。Wi-Fi（無線LAN）を使用する場合は、次の手順でコンピュータをインターネットに接続する。

    1. `iw`コンソールを開く。

        ```
        # iwctl
        ```

    1. デバイス名を確認する。

        ```
        [iwc]# device list
        ```

    1. 確認したデバイス（ここでは`wlan0`）が接続できるWi-Fiを確認する。

        ```
        [iwc]# station wlan0 scan
        [iwc]# station wlan0 get-networks
        ```

    1. 接続したいWi-FiのSSIDを指定してインターネットに接続する。

        ```
        [iwc]# station wlan0 connect {SSID}
        ```

    1. エラーが表示されなければ接続が完了しているので、コンソールを抜ける。

        ```
        [iwc]# exit
        ```

1. コンピュータがインターネットに接続されいてることを確認する。

    ```
    # ping archlinux.us
    ```


## システム時刻の調整

1. システム時刻を合わせる。

    ```
    # timedatectl set-ntp true
    ```


## ディスクのフォーマット

1. コンピュータに接続されているディスクを確認する。

    ```
    # lsblk
    ```

1. Arch Linuxをインストールするディスク（ここでは`sda`）に対して、以下のようにパーティションを切る。（デュアルブートなどの場合に、Windowsがインストールされているディスクを上書きしてしまうなどしないように注意）

    ```
    # gdisk /dev/sda
    
    //	EFI
    First sector    : (return)
    Last sector     : 512M
    Type            : ef00
    
    //	archroot
    First sector    : (return)
    Last sector     : -3G
    Type            : 8300
    
    //	swap
    First sector    : (return)
    Last sector     : (return)
    Type            : 8200
    ```

1. それぞれのパーティションを以下のようにフォーマットする。

    ```
    # mkfs.fat -F32 /dev/sda1
    # mkfs.ext4 /dev/sda2
    # mkswap /dev/sda3
    ```


## パーティションのマウント

1. ルートパーティションをマウントする。

    ```
    # mount /dev/sda2 /mnt
    ```

1. `/mnt/boot`というディレクトリを作成し、そこにEFIパーティションをマウントする。

    ```
    # mkdir /mnt/boot
    # mount /dev/sda1 /mnt/boot
    ```

1. swapを起動する。

    ```
    # swapon /dev/sda3
    ```


## 基本的なパッケージのインストール

1, `reflector`を用いて、パッケージを取得するミラーサーバを自動的に選択する。
	```
	# reflector --country 'Japan' --age 12 --protocol https --sort rate --save /etc/pacman.d/mirrorlist
	```

1. 必須のパッケージをインストールする。

    ```
    # pacstrap /mnt linux linux-firmware base
    ```

1. `base-devel`と`vim`をインストールする。

    ```
    # pacstrap /mnt base-devel vim
    ```


## システム設定

1. パーティションのマウント状態を記録するために、`fstab`を生成する。

    ```
    # genfstab -U /mnt >> /mnt/etc/fstab
    ```

1. Arch Linuxがインストールされた環境の`/mnt`を`/`として扱う。

    ```
    # arch-chroot /mnt
    ```

1. タイムゾーンを変更し、ハードウェアクロックを合わせる。

    ```
    # ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
    # hwclock --systohc
    ```


## ロケール設定

1. `/etc/locale.gen`の以下の行のコメントを外す。

    ```
    # vim /etc/locale.gen
    ```

    ```
    en_US.UTF-8 UTF-8
    
    ja_JP.UTF-8 UTF-8
    ```

1. ロケール設定を生成する。

    ```
    # locale-gen
    ```

1. デフォルトのロケールを設定する。

    ```
    # echo LANG=en_US.UTF-8 > /etc/locale.conf
    ```

1. デフォルトのキーボードレイアウトを設定する。

    ```
    # echo KEYMAP=jp106 > /etc/vconsole.conf
    ```

1. コンソールのデフォルトのフォントを変更する。

    ```
    # pacman -S terminus-font
    # echo FONT=ter-128n >> /etc/vconsole.conf
    ```


## コンピュータ名の設定

1. 任意のホスト名（ここでは`myhostname`）を`/etc/hostname`に書き込む。

    ```
    # echo myhostname > /etc/hostname
    ```

1. `/etc/hosts`を以下のように編集する。

    ```
    # vim /etc/hosts
    ```

    ```
    127.0.0.1     localhost
    ::1           localhost
    127.0.0.1     myhostname.localdomain myhostname
    ```


## rootパスワードの変更

1. rootユーザのパスワードを変更する。

    ```
    # passwd
    ```


## ブートローダの設定

1. ブートローダの設定を行う方法はいくつかあるが、ここではUEFIモードで簡単に使用することができる`bootctl`を用いる。

    ```
    # bootctl --path=/boot install
    ```

1. `arch`というエントリーをデフォルトで起動するように設定する。

    ```
    # vim /boot/loader/loader.conf
    ```

    ```
    default arch
    timeout F32
    editor  no
    ```

    1, ルートパーティションの`PARTUUID`を確認し、メモをとっておく。

    ```
    # blkid -s PARTUUID /dev/sda2
    ```

1. `arch`エントリーを作成する。`********-****-****-************`の部分は、先程確認したPARTUUIDに置き換える。もし使用しているコンピュータがCPUとしてAMDを用いいているなら、`/intel-ucode.img`の部分を`/amd-ucode.img`に置き換える。

    ```
    # vim /boot/loader/entries/arch.conf
    ```

    ```
    title    Arch Linux
    linux    /vmlinuz-linux
    initrd   /intel-ucode.img
    initrd   /initramfs-linux.img
    options  root=PARTUUID=********-****-****-************
    ```

1. CPUマイクロコードとして、CPUがIntelであれば`intel-ucode`を、AMDであれば`amd-ucode`をインストールする。

    ```
    # pacman -S intel-ucode
    ```


## Network Managerのインストール

1. 先程ライブ環境におけるインターネット接続を行ったが、インストール環境においてもネットワーク設定が必要となる。ここでは`NetworkManager`を用いる。まずは必須パッケージをインストールする。

    ```
    # pacman -S networkmanager wpa_supplicant dialog
    ```

1. `NetworkManager`が自動で起動するように、`systemctl`を用いてシステムに登録する。

    ```
    # systemctl enable NetworkManager
    ```


## コンピュータの再起動

1. chroot環境から抜け、コンピュータを再起動する。この時点でArch Linuxのインストールは完了しているので、USBは抜いてしまっても構わない。

    ```
    # exit
    # reboot
    ```


## ネットワーク設定

1. `NetworkManager`を開き、設定メニューからネットワーク設定を行う。一度設定してしまえば、それ以降はコンピュータを起動したときに自動で接続されるようになる。

    ```
    # nmtui
    ```


## ユーザアカウントの作成

1. 常に`root`ユーザを使うのはセキュリティ上好ましくないので、一般ユーザを新しく作成し、`wheel`グループに追加する。

    ```
    # useradd -m -G wheel {username}
    # passwd {username}
    ```

1. `sudoers`ファイルを編集し、`wheel`グループのユーザに一時的にroot権限でコマンドを実行することを許可させる。ファイルを編集するエディタを`EDITOR`環境変数に設定し、`visudo`コマンドにより編集を開始する。`%wheel ALL=(ALL) NOPASSWD: ALL`の行のコメントを外せば`sudo`によりコマンドが実行できるようになる。

    ```
    # EDITOR=vim visudo
    ```

    ```
    %wheel ALL=(ALL) NOPASSWD: ALL
    ```

1. ログアウトして、作成したユーザアカウントで再度ログインし直す。


## AURヘルパのインストール

1. Arch Linuxでは、パッケージマネージャとして`Pacman`を用いる。また、コミュニティが開発するパッケージとしてAURがある。ここでは、`paru`というAURヘルパをインストールし、AURパッケージのインストールを容易に行えるようにする。

    ```
    $ sudo pacman -S git
    $ git clone https://aur.archlinux.org/paru.git
    ```

1. `paru`ディレクトリに入り、`PKGBUILD`によりビルド情報を確認する。コミュニティのパッケージには悪意のあるものも含まれている可能性があるので、パッケージが安全であることをよく確認する。

    ```
    $ cd paru
    $ less PKGBUILD
    ```

1. `PKGBUILD`に問題がなければ、パッケージをビルドする。`paru`はRustで書かれているため、Rustのパッケージマネージャであるcargoが必要となる。依存パッケージとして、`rust`か`rustup`のどちらかを選択し、ビルドを進める。

    ```
    $ makepkg -si
    ```

1. インストールが完了したら、ビルドファイルを削除し、不要であればRustパッケージもアンインストールする。

    ```
    $ cd ../
    $ rm -rf paru
    $ sudo pacman -Rs rust
    ```

1. AURパッケージをビルドする際のマルチスレッド設定を変更する。

    ```
    $ EDITOR=vim sudoedit /etc/makepkg.conf
    $ COMPRESSZST=(zstd -T0 -c -z -q -)
    ```


## Pacmanの設定

1. Pacmanの出力に色をつけたければ、`/etc/pacman.conf`の`Color`の行のコメントを外せばよい。

    ```
    $ sudo vim /etc/pacman.conf
    ```

    ```
    Color
    ```

1. `reflector`をインストールし、パッケージを取得するミラーサーバを更新する。

    ```
    $ sudo pacman -S reflector
    $ sudo cp /etc/pacman.d/mirrorlist /etc/pacman.d/mirrorlist.bak
    $ sudo reflector --country 'Japan' --age 24 --protocol https --sort rate --save /etc/pacman.d/mirrorlist
    ```


## セキュリティ

※ nftablesを導入したところ、Dockerが動作しなかったので注意

1. `iptables`はもともとパケットフィルタリングツールとして広く使用されていたが、近年では`nftables`に置き換わりつつある。そのため、ここでは`iptables-nft`をインストールし、デフォルトの`iptables`を置き換える。

    ```
    $ sudo pacman -S iptables-nft
    ```

1. `nftables`をインストールする。

    ```
    $ sudo pacman -S nftables
    ```

1. `nftables`が自動で起動するようにシステムに登録する。

    ```
    $ sudo systemctl enable nftables.service
    $ sudo systemctl start nftables.service
    ```


## 参考リンク

- [https://blog.livewing.net/install-arch-linux-2021](#https://blog.livewing.net/install-arch-linux-2021)
- [https://neko-mac.blogspot.com/2021/O5/arch-linux.html](#https://neko-mac.blogspot.com/2021/O5/arch-linux.html)
