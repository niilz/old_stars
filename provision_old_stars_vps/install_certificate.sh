#! /usr/bin/bash

echo "Requesting certificate"

sudo certbot certonly --standalone --non-interactive --agree-tos --domains niilz.de niilz@outlook.de

echo "Creating certs directoy"

CERTS_DIR=/home/oldstars/certs

mkdir -p $CERTS_DIR

echo "Copying certs to $CERTS_DIR"
cp /etc/letsencrypt/live/niilz.de/*.pem $CERTS_DIR

echo "Change dir into $CERTS_DIR"
cd $CERTS_DIR

echo "Change owner of certs"
sudo chown oldstars:oldstars *.pem
