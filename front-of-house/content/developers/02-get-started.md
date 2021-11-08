+++
title = "Get Started"
date = 2019-11-27
+++

If you haven’t done so already, first make sure you’ve set up Keyvault by **signing up** and **installing the CLI** on your workstation.

### Step 1: Login to your account from the CLI



```bash
$ keyvault login you@youremail.com
Greetings from KeyVault.
```

### Step 2: Your first secret

When you signed up we created a sample secret using. To read a secret, run:

```bash
$ keyvault read /start/hello
Greetings from KeyVault.
```

You can write a new version of the secret with:

secrethub write ianpurton/start/hello
Secrets are automatically versioned so you’ll never accidentally overwrite a secret. You can access a specific version of a secret by appending the version number to the path, e.g. :1. When no version number is given, it defaults to :latest.