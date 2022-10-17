+++
title = "Install the CLI"
date = 2019-11-27
sort_by = "weight" 
weight = 10
+++

Installing the precompiled binary is the easiest way to get started with Cloak. Our binaries are publicly compiled using our CI/CD pipeline. Checksums for binaries are available on github.

## Step 1. Install the precompiled binary (Linux)

```sh
$ sudo /bin/sh -c 'wget https://github.com/purton-tech/cloak/releases/latest/download/cloak-linux -O /usr/local/bin/cloak && chmod +x /usr/local/bin/cloak'
```

Install a particular version 

```sh
$ export CLOAK_VERSION=1.1.0
$ sudo /bin/sh -c "wget https://github.com/purton-tech/cloak/releases/download/v$CLOAK_VERSION/cli -O /usr/local/bin/cloak && chmod +x /usr/local/bin/cloak"
```

## Step 1. Install the precompiled binary (Windows)

Coming soon.

## Step 1. Install the precompiled binary (MacOS)

```sh
$ sudo /bin/sh -c 'curl https://github.com/purton-tech/cloak/releases/latest/download/cloak-macos -L -o /usr/local/bin/cloak && chmod +x /usr/local/bin/cloak'
```

## Step 2. Verify the checksum (Optional)

Coming soon.

## Step 3. Test the installation

```sh
$ cloak
cloak 
Secrets automation

USAGE:
    cloak [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -a, --api-host-url <API_HOST_URL>
            [env: API_HOST_URL=http://envoy:7100] [default: https://cloak.software]

    -e, --ecdh-private-key <ECDH_PRIVATE_KEY>
            [env: ECDH_PRIVATE_KEY=]

        --ecdh-private-key-file <ECDH_PRIVATE_KEY_FILE>
            [default: ./cloak.pem]

    -h, --help
            Print help information

SUBCOMMANDS:
    help       Print this message or the help of the given subcommand(s)
    info       
    secrets  
```