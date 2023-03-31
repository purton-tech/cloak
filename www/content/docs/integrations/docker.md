+++
title = "Docker"
date = 2022-03-01
+++

The cloak binary can be used to inject secrets into a docker container at run time.

```sh
RUN wget https://github.com/purton-tech/cloak/releases/latest/download/cloak-linux \
    -O /usr/local/bin/cloak \
    && chmod +x /usr/local/bin/cloak
```

## Modify the start command in your Dockerfile

```
CMD ["cloak", "[your service start command]"]
```

## Generate a Service Account Private Key

When you create a service account you can download the service account private key in a `pem` file. The contents of this file is what will need to be passed into the docker container as an environment variable.

```
docker run --env ECDH_PRIVATE_KEY=[token] [DOCKER-IMAGE]...
```
