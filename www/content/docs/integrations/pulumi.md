+++
title = "Pulumi"
date = 2022-03-01
+++

When you start to use infrastructure as code tools to manage your deployments you get the opportunity to greatly reduce the number of secrets you need to manage.

## Reducing the number of secrets

Pulumi can create secrets using `pulumi.secret` such as when generating a new random password. Pulumi takes care to encrypt these secrets per project.

So to reduce the number of secrets you need to manage, get your infrastructure as code tool to generate as many secrets as possible. i.e. database passwords etc.

## Defence in depth

Technically the Pulumi team would have access to your secrets. We can mitigate this by using the Pulumi `PULUMI_CONFIG_PASSPHRASE` which adds additional encryption to the secrets before they reach the Pulumi service.

We can use `openssl` to generate a cryptographically secure random passphrase.

```sh
$ openssl rand -hex 16
43f1bc2431521b8f80c768fd2efed483
```

And setup cloak with 2 environment variables, `PULUMI_CONFIG_PASSPHRASE` and `PULUMI_ACCESS_TOKEN`.

After which you should be able to run `cloak secrets` and get something like

```
+---------------------------+--------------------------------------+
| Name                      | Value                                |
+------------------------------------------------------------------+
| PULUMI_ACCESS_TOKEN       | pul-2f5b45e04b835004f84da381b7d48e52 |
+------------------------------------------------------------------+
| PULUMI_CONFIG_PASSPHRASE  | 40ebe4d72fa98da51814674f91d1733c     |
+------------------------------------------------------------------+
```

## Logging into Pulumi

Pulumi uses the environment variable `PULUMI_ACCESS_TOKEN` for authentication. After configuring this in Cloak we can authenticate with.

```sh
cloak pulumi login
```

## Running Pulumi Up (or Down)

You should now be able to run all Pulumi commands with cloak passing in the environment variables needed for authentication and encryption.

```sh
cloak pulumi up
```

Any issues, let us know.