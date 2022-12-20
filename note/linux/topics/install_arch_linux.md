# Install of Arch Linux


## Table of contents

1. [Preparation of installation media](#preparation-of-installation-media)
1. [Boot the installation media](#boot-the-installation-media)
1. [Connect to the Internet](#connect-to-the-internet)
1. [Set system time](#set-system-time)
1. [Format the disk](#format-the-disk)
1. [Mount partition](#mount-partition)
1. [Install basic packages](#install-basic-package)
1. [System settings](#system-settings)
1. [Locale settings](#locale-settings)
1. [Set computer name](#set-computer-name)
1. [Change root password](#change-root-password)
1. [Boot loader settings](#boot-loader-settings)
1. [Install network manager](#install-network-manager)
1. [Reboot computer](#reboot-computer)
1. [Network settings](#network-settings)
1. [Create a user account](#create-a-user-account)
1. [Install an AUR helper](#install-an-aur-helper)
1. [Pacman settings](#pacman-settings)
1. [Security](#security)
1. [Reference](#reference)


## Preparation of installation media

1. Find a mirror from the [Arch Linux download page](#https://archlinux.org/download/).

1. Download ISO image like `archlinux-xxxx.xx.xx-x86_64.iso.

1. You can check if the downloaded file is corrupted by calculating the checksum of the ISO image.

	1. Calculating the checksum with the following command.

	```
	> sha1sum archlinux-xxxx.xx.xx-x86_64.iso
	```

	1. Compare the result with the checksum listed on the download page.

1. Install [Rufus](https://rufus.ie/ja/) on Windows.

1. Write ISO image to flush drive(USB) using Rufus.


## Boot the installation media

1. Plug the flush drive into the computer you want to install and boot.

	1. You can boot your computer in UEFI(BIOS) mode by hitting F2, delete, or any other key.
	1. Open secure boot settings and turn it off.
	1. Open the item related to the startup order and set the priority of flush drive to the first.

1. Select `Arch Linux install media (x86_64, UEFI)` from the menu panel.

1. Then the shell will stand up.

	1. If you are using a keyboard with layout other than US array, you can convert the layout. The following is an example of a Japanese layout.

	```
	# loadkeys jp106
	```

	1. If the characters are small and hard to see, you can change the font with the following command.

	```
	# setfont ter-128n
	```


## Connect to the Internet

1. Connect to a wired LAN or connect to Wi-Fi(wireless LAN) by following the steps below.

	1. Open `iw` console.

	```
	# iwctl
	```

	1. Check the device name.

	```
	[iwc]# device list
	```

	1. Check the connection point of the confirmed device name(`wlan0` in this case).

	```
	[iwc]# station wlan0 scan
	[iwc]# station wlan0 get-networks
	```

	1. Connect to Wi-Fi using SSID and password.

	```
	[iwc]# station wlan0 connect {SSID}
	```

	1. If there are no errors, exit the console.

	```
	[iwc]# exit
	```

1. Make sure your computer is connected to the network.

```
# ping archlinux.us
```


## Set system time

1. Set system time.

```
# timedatectl set-ntp true
```


## Format the disk

1. Check the disl attached to your computer.

```
# lsblk
```

1. Create a disk partition table(`sda` in this case).

```
# gdisk /dev/sda

//	EFI
First sector	: (return)
Last sector	 : 512M
Type			: ef00

//	archroot
First sector	: (return)
Last sector	 : -3G
Type			: 8300

//	swap
First sector	: (return)
Last sector	 : (return)
Type			: 8200
```

1. Format each partition with the following command.

```
# mkfs.fat -F32 /dev/sda1
# mkfs.ext4 /dev/sda2
# mkswap /dev/sda3
```


## Mount partition

1. Mount the root partition.

```
# mount /dev/sda2 /mnt
```

1. Create `/mnt/boot` directory and mount the EFI partition.

```
# mkdir /mnt/boot
# mount /dev/sda1 /mnt/boot
```

1. Start swap.

```
# swapon /dev/sda3
```


## Install basic packages

1. Automatically select the mirror server to get packages.

```
# reflector --country 'Japan' --age 12 --protocol https --sort rate --save /etc/pacman.d/mirrorlist
```

1. Install required packages.

```
# pacstrap /mnt linux linux-firmware base
```

1. Install /mnt base-devel vim

```
# pacstrap /mnt base-devel vim
```


## System settings

1. Generate `fstab` to record partition mount status.

```
# genfstab -U /mnt >> /mnt/etc/fstab
```

1. Execute chroot to treat `/mnt` in the installed environment as `/`.

```
# arch-chroot /mnt
```

1. Set the time zone and acquire a hardware clock.

```
# ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
# hwclock --systohc
```


## Locale settings

1. Uncomment the following line from `/etc/locale.gen`.

```
# vim /etc/locale.gen
```

```
en_US.UTF-8 UTF-8

ja_JP.UTF-8 UTF-8
```

1. Generate locale.

```
# locale-gen
```

1. Set default locale.

```
# echo LANG=en_US.UTF-8 > /etc/locale.conf
```

1. Set default keymap.

```
# echo KEYMAP=jp106 > /etc/vconsole.conf
```

1. If you want to change the console font, install and set any font.

```
# pacman -S terminus-font
# echo FONT=ter-128n >> /etc/vconsole.conf
```


## Set computer name

1. Arbitrarily decide the host name and write it to `/etc/hostname`. Change the part of `myhostname` as you like.

```
# echo myhostname > /etc/hostname
```

1. Edit `/etc/hosts/` as below.

```
# vim /etc/hosts
```

```
127.0.0.1	 localhost
::1		   localhost
127.0.0.1	 myhostname.localdomain myhostname
```


## Change root password

1. Change root password.

```
# passwd
```


## Boot loader settings

1. There are several ways to configure the boot loader, but here we will use `bootctl`, which is easy to use in UEFI mode.

```
# bootctl --path=/boot install
```

1. Set to launch an entry called `arch` by default.

```
# vim /boot/loader/loader.conf
```

```
default arch
timeout F32
editor  no
```

1. Check `PARTUUID` of the root partition and make a note.

```
# blkid -s PARTUUID /dev/sda2
```

1. Then create an arch entry. For the `********-****-****-************` part, enter the PARTUUID you checked just before. If your computer uses AMD as the CPU, replace `/intel-ucode.img` with `/amd-ucode.img`.

```
# vim /boot/loader/entries/arch.conf
```

```
title     Arch Linux
linux     /vmlinuz-linux
initrd    /intel-ucode.img
initrd    /initramfs-linux.img
options   root=PARTUUID=********-****-****-************
```

1. Install `intel-ucode` for Intel and `amd-ucode` for AMD as the CPU microcode.

```
# pacman -S intel-ucode
```


## Install network manager

1. You connected to the internet in a live environment, but you also need to configure the network settings in the installation environment. Here we use `networkmanager`. Install required packages.

```
# pacman -S networkmanager wpa_supplicant dialog
```

1. Set `systemctl` to start `NetworkManager` automatically.

```
# systemctl enable NetworkManager
```


## Reboot computer

1. Exit the chroot environment and reboot your computer. You can unplug the flush drive for live enviroment after rebooting.

```
# exit
# reboot
```


## Network settings

1. Open the NetworkManager settings menu and configure network settings. Once set, it will connect automatically the next time you start your computer.

```
# nmtui
```


## Create a user account

1. You don't want to log in directly to `root` to continue working, so create a regular user. Also, make the created user belong to the `wheel` group.

```
# useradd -m -G wheel {username}
# passwd {username}
```

1. Edit the `sudoers` file to allow users in the `wheel` group to temporarily gain privilledges using the `sudo` command. Set the editor to the envirnment variable `EDITOR` and start editing with the `visudo` command and uncomment the line `%wheel ALL=(ALL) NOPASSWD: ALL`.

```
# EDITOR=vim visudo
```

```
%wheel ALL=(ALL) NOPASSWD: ALL
```

1. Log out and log back in with the user account you created.


## Install an AUR helper

1. On Arch Linux, command package are cloned from AUR, build and Install. Here we use `paru` as an AUR helper.

```
$ sudo pacman -S git
$ git clone https://aur.archlinux.org/paru.git
```

1. Go into the `paru` directory and check the `PKGBUILD` that contains the build information. AUR allows users to post packages freely, it is necessary to check if it is a malicious package.

```
$ cd paru
$ less PKGBUILD
```

1. If there is no problem with contents of `PKGBUILD`, build the package. Since `paru` is written in Rust, you will be required to install Rust's package manager `cargo`. Select `rust` from `rust` and `rustup` to continue the build.

```
$ makepkg -si
```

1. After the build and installation is complete, remove the build files and Rust.

```
$ cd ../
$ rm -rf paru
$ sudo pacman -Rs rust
```

1. Set to compress in multiple threads when compressing AUR packages.

```
$ EDITOR=vim sudoedit /etc/makepkg.conf
$ COMPRESSZST=(zstd -T0 -c -z -q -)
```


## Pacman settings

1. Colorize pacman output to make it easier to see.

```
$ sudo vim /etc/pacman.conf
```

```
Color
```

1. Install `reflector` and update pacman's mirrorlist.

```
$ sudo pacman -S reflector
$ sudo cp /etc/pacman.d/mirrorlist /etc/pacman.d/mirrorlist.bak
$ sudo reflector --country 'Japan' --age 24 --protocol https --sort rate --save /etc/pacman.d/mirrorlist
```


## Security

1. `iptables` was a widely used packet filtering tool, but in recent years it has been replaced by `nftables`. Install `iptables-nft` to replace the default `iptables`.

```
$ sudo pacman -S iptables-nft
```

1. Install `nftables`.

```
$ sudo pacman -S nftables
```

1. Register `nftables` as a service to system.

```
$ sudo systemctl enable nftables.service
$ sudo systemctl start nftables.service
```


## Reference

- [https://blog.livewing.net/install-arch-linux-2021](#https://blog.livewing.net/install-arch-linux-2021)
- [https://neko-mac.blogspot.com/2021/O5/arch-linux.html](#https://neko-mac.blogspot.com/2021/O5/arch-linux.html)
