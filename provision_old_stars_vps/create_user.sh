#! /usr/bin/bash

echo "Creating oldstars user"

sudo useradd -m oldstars

echo oldstars:$OLDSTARS_PWD | chpasswd

sudo usermod -aG docker oldstars
