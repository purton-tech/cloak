+++
title = "Encrypting your private key"
date = 2022-02-20
sort_by = "weight" 
weight = 50
+++

The cloak.pem file is full compatible with OpenSSL. You can import and export the pem file with encryption.

### Encrypt the `cloak.pem`

```sh
$ openssl pkcs8 -topk8 -in cloak.pem -out cloak.enc.pem
$ rm cloak.pem
```

### Decrypt it.

```sh
$ openssl pkcs8 -topk8 -nocrypt -in cloak.enc.pem -out cloak.pem
```

Decrypt and store as env var.

```sh
export ECDH_PRIVATE_KEY=$(openssl pkcs8 -topk8 -nocrypt -in cloak.enc.pem)
```

And now we can see the private key picked up by cloak

```sh
$ cloak
Secrets automation

USAGE:
    cloak [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -a, --api-host-url <API_HOST_URL>
            [env: API_HOST_URL=] [default: https://cloak.software]

    -e, --ecdh-private-key <ECDH_PRIVATE_KEY>
            [env: ECDH_PRIVATE_KEY=-----BEGIN PRIVATE KEY-----
            MIGHnma2HHoGRic2wNgIGKQ+B1pULy2kFDMvQ/AwvYS13uH2Trfja9M9wRqYjM2M
            KXeBAMZaHaF4NxZYjc4ri0rujiKPGJY4qQ8iY3M3M0tJ3LZqbcjtKLDNSVlijnXX
            nma2HHoGRic2wNgIGKQ+B1pULy2kFDMvQ/AwvYS13uH2Trfja9M9wRqY
            -----END PRIVATE KEY-----]

        --ecdh-private-key-file <ECDH_PRIVATE_KEY_FILE>
            [default: ./cloak.pem]

    -h, --help
            Print help information

SUBCOMMANDS:
    help       Print this message or the help of the given subcommand(s)
    info       
    secrets    
/vscode/infra (main) 
$ 
```