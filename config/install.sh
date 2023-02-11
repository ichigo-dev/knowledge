#!/bin/bash

echo "================================================================================"
echo " 1. Update all packages"
echo "================================================================================"
echo -e "\n\n"
sudo pacman -Syu

echo -e "\n\n"
echo "================================================================================"
echo " 2. Install paru as needed"
echo "================================================================================"
echo -e "\n\n"
if [ ! `which paru` ]; then
	git clone https://aur.archlinux.org/paru.git
	cd paru
	makepkg -si
	cd ..
	rm paru -rf
fi

echo -e "\n\n"
echo "================================================================================"
echo " 3. Install all packages"
echo "================================================================================"
echo -e "\n\n"
sudo pacman -S --needed - < pkglist.txt

echo -e "\n\n"
echo "================================================================================"
echo " 4. Install paru packages"
echo "================================================================================"
echo -e "\n\n"
paru -S --needed \
	neomutt \
	trash-cli

sudo systemctl enable cronie
sudo systemctl enable docker
