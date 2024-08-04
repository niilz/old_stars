# Provision Virtual Private Server

## Dockerfile

This is only a playground to experiment that the scripts kinda do what we want, once we are in the reals VPS

## How To

- scp all files (apart from Dockerfile) into server (suggestion `/home/provisioning_old_stars`)
- login to server and cd into dir from previous step
- add execution privilege (`sudo chmod 744`) for root on `./provision-old-star-vps.sh`
- set password for _oldstars_ user (`export OLDSTARS_PWD=<the-password>`)
- execute the `provision-old-star-vps.sh`
