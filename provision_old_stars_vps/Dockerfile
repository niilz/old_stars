FROM ubuntu:22.04

RUN apt-get update && apt-get -y install sudo

COPY . /home/provision_old_stars

WORKDIR /home/provision_old_stars

RUN chmod 744 ./provision-old-star-vps.sh
