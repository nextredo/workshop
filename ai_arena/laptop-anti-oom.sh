#! /usr/bin/env sh
#
# A script designed to recover as much memory as possible
# Buyer beware

# killers
systemctl --user --runtime mask gnome-software
systemctl --user stop gnome-software

systemctl --user --runtime mask localsearch-3
systemctl --user stop localsearch-3

sudo systemctl --runtime mask packagekit
sudo systemctl stop packagekit

sudo systemctl --runtime mask fwupd --runtime
sudo systemctl stop fwupd

sudo systemctl --runtime mask tailscaled --runtime
sudo systemctl stop tailscaled

sudo systemctl --runtime mask keyd --runtime
sudo systemctl stop keyd

pkill copyq

# HARDCORE
read -p "Run fireworks? (kill GNOME, use ctrl+alt+F3 for a virtual terminal or smth)" -n 1 -r
echo "AAAAAAAAAAAAAAH"
if [[$REPLY =~ ^[Yy}$ ]]
then
    # Danger goes here
    sudo systemctl stop gdm
fi
