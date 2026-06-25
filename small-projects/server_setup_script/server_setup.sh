#!/bin/bash
set -euo pipefail
sudo apt update
sudo apt install -y git curl vim ufw
new_user=$1
pass=$2
port=$3
echo "creating user $new_user..."
sudo adduser "$new_user"
sudo passswd "$pass"
sudo usermod -aG sudo "$new_user"
echo "hardening ssh..."
sudo sed -i 's/#PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config
sudo sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
sudo sed -i "s/#Port 22/Port $port/" /etc/ssh/sshd_config
sudo systemctl restart sshd
echo "configuring firewall..."
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow "$port"
sudo ufw deny 22
sudo ufw enable
echo "setting up dotfiles..."
sudo cp ~/.gitconfig /home/"$new_user"/.gitconfig
sudo chown "$new_user":"$new_user" /home/"$new_user"/.gitconfig
echo "setup complete..."