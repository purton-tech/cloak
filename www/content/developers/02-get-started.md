+++
title = "Your First Secret"
date = 2019-11-27
+++

If you haven’t done so already, first make sure you’ve set up Keyvault by **signing up** and **installing the CLI** on your workstation.

### Step 1: Login to your account from the CLI

```bash
$ keyvault login you@youremail.com
Greetings from KeyVault.
```

### Step 2: Your first secret

Before you begin, check to verify that no secrets exists at `secret/hello`.

```bash
$ keyvault kv get /secret/hello
No value found at secret/data/hello
```

You can write a new version of the secret with:

```bash$ 
keyvault kv put secret/hello foo=world

Key              Value
---              -----
created_time     2020-09-02T21:40:01.635656Z
deletion_time    n/a
destroyed        false
version          1
```



