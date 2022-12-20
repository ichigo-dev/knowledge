# My softwares list for Arch Linux


## Table of contents

1. [Development](#development)
1. [Display server](#display-server)
1. [Display manager](#display-manager)
1. [Window manager](#window-manager)
1. [File manager](#file-manager)
1. [Screen lock](#screen-lock)
1. [Screen shot](#screen-shot)
1. [Notification](#notification)
1. [Status bar](#status-bar)
1. [Application launcher](#application-launcher)
1. [Fonts and icons](#fonts-and-icons)
1. [Web browser](#web-browser)
1. [Text editor](#text-editor)
1. [Terminal emulator](#terminal-emulator)
1. [Shell](#shell)
1. [Utility](#utility)
	1. [Clipboard](#clipboard)
	1. [Compression](#compression)
	1. [Image viewer](#image-viewer)
	1. [Document viewer](#document-viewer)
1. [Device](#device)
	1. [Display](#display)
	1. [Audio](#audio)
	1. [Backlight](#backlight)
1. [Battery management](#battery-management)


## Development

- git
- php
- php-fpm
- phpmyadmin
- nginx
- mariadb

- wget


## Display server

- wayland


## Display manager

- greetd\* (`/etc/greetd/config.toml`, `/etc/greetd/sway-config`, `/etc/greetd/environments`)

	```
	$ sudo systemctl enable greetd
	```

- greetd-gtkgreet\* (`/etc/greetd/gtkgreet.css`)

- breeze-gtk


## Window manager

- sway (`$HOME/.config/sway/config`)

	1. Install sway.

	```
	$ sudo pacman -S sway polkit foot
	$ sudo pacman -S swaybg
	```

	1. Configuration

	```
	$ mkdir -p ~/.config/sway
	$ cp /etc/sway/config ~/.config/sway/
	```


## File manager

- thunar

- ranger (`$HOME/.config/ranger/rc.conf`)

	1. Initialize configuration.

	```
	$ ranger --copy-config all
	```


## Screen lock

- swayidle
- swaylock


## Screren shot

- swayshot\*


## Notification

- mako (`$HOME/.config/mako/config`)
- libnotify


## Status bar

- waybar (`$HOME/.config/waybar/config`, `$HOME/.config/waybar/style.css`)


## Application launcher

- wofi (`$HOME/.config/wofi/style.css`)


## Fonts and icons

- adobe-source-han-sans-jp-fonts
- noto-fonts
- ttf-font-awesome
- otf-font-awesome


## Web browser

- firefox
- firefox-i18n-ja
- chromium


## Text editor

- vim (`$HOME/.vimrc`)

	1. Add plugins.

	```
	$ mkdir -p $HOME/.vim/pack/vimpack/start
	$ cd $HOME/.vim/pack/vimpack/start
	$ git clone https://...
	```

- nvim (`$HOME/.config/nvim`)


## Terminal emulator

- alacritty (`$HOME/.config/alacritty/alacritty.yml`)
- foot


## Shell

- zsh (`$HOME/.zshrc`)

	1. Change default -S zsh

	```
	$ sudo pacman -S zsh
	$ chsh -s $(which zsh)
	```

- sheldon\* (`HOME/.sheldon/plugins.toml`)

	```
	$ sudo systemctl enable --now apparmor.service
	$ sudo systemctl enable --now snapd.apparmor.service
	```


## Utility

- mandoc


### Clipboard

- wl-clipboard


### Compression

- zip
- unzip


### Image viewer

- imv


### Document viewer

- zathura (PDF viewer)


## Device

### Display

- wdisplays\*


### Audio

- pulseaudio
- pavucontrol (GUI)
- wob\*


### Backlight

- light\*

	```
	$ sudo light -A 50
	$ sudo light -U 50
	```


## Battery management

- tlp (`/etc/default/tlp`)

- tlp-rdw

	```
	$ sudo systemctl enable tlp.service)
	```
