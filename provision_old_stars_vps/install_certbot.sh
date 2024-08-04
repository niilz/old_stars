#! /usr/bin/bash

echo "installing snapd"

sudo apt-get update
sudo apt-get install -y snapd

echo "installing certbot"

sudo snap install --classic certbot

echo "simlinking to bin"

sudo ln -s /snap/bin/certbot /usr/bin/certbot
