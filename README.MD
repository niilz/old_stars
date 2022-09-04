# Setup EC2

- Install docker (see: [How to install Docker on Ubuntu](https://docs.docker.com/engine/install/ubuntu/))
- Transfer `docker-compose.yml` to server:  
  scp -i ~/.ssh/aws/private-access-key.pem docker-compose.yml ubuntu@ec1-2-42-43-123.eu-central-1.compute.amazonaws.com:~/docker-compose.yml
