+++
title = "Get Started"
date = 2019-11-27
+++

Test your Installation
You can verify your installation by running the following command:

`keyvault --version`

Some content
Get Started with SecretHub
This guide will show you the very basics of working with the SecretHub CLI.

If you haven’t done so already, first make sure you’ve set up SecretHub by signing up and installing the CLI on your workstation.

To make the upcoming example code copy-pasteable for you to smoothly follow along with this guide, fill in your username below:

ianpurton
Step 1: Your first secret
Every account comes with a personal workspace. To help you find your way, we’ve already created a sample secret. To read a secret, run:

secrethub read ianpurton/start/hello
You can write a new version of the secret with:

secrethub write ianpurton/start/hello
Secrets are automatically versioned so you’ll never accidentally overwrite a secret. You can access a specific version of a secret by appending the version number to the path, e.g. :1. When no version number is given, it defaults to :latest.