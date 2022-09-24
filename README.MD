# Setup

## EC2

- Login to EC2
  ssh -i ~/.ssh/aws/private-access-key.pem ubuntu@ec1-2-42-43-123.eu-central-1.compute.amazonaws.com
- Install docker (see: [How to install Docker on Ubuntu](https://docs.docker.com/engine/install/ubuntu/))

## SSL

To access backend-api from Android we also need the api-server to be accessable via https

- Install certbot (ACME-client see: [certbot instructions](https://certbot.eff.org/instructions))
- Note that ssl-certificates *cannot* be issued for AWS-Public-DNS-Addresses directly!
- Use **some-other-url.com** instead and configure *CNAME* at the Host-Provider to point to the aws-address
- Use issued certificate in the server
- Certificate (pk and chain) are placed in `/etc/letsencrypt/live/domain.com/`
