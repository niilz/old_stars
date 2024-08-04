#! /usr/bin/bash

chmod 744 ./create_user.sh
chmod 744 ./install_docker.sh
chmod 744 ./install_certbot.sh
chmod 744 ./install_certificate.sh

./install_docker.sh
./create_user.sh
./install_certbot.sh
./install_certificate.sh

