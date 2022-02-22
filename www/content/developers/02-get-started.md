+++
title = "Your First Secret"
date = 2019-11-27
+++

### Step 1: Install Cloak

```sh
$ sudo /bin/sh -c 'wget https://github.com/purton-tech/cloak/releases/latest/download/cli -O /usr/local/bin/cloak && chmod +x /usr/local/bin/cloak'
```

Install a particular version 

```sh
$ export CLOAK_VERSION=1.1.0
$ sudo /bin/sh -c "wget https://github.com/purton-tech/cloak/releases/download/v$CLOAK_VERSION/cli -O /usr/local/bin/cloak && chmod +x /usr/local/bin/cloak"
```

### Step 2: Create a  Secret

### Step 3: Encrypt your cloak key

Encrypt the the `cloak.pem`

```sh
$ openssl pkcs8 -topk8 -in cloak.pem -out cloak.enc.pem
$ rm cloak.pem
```

Decrypt it.

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




