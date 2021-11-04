+++
title = "Introduction"
date = 2019-11-27
+++

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

Because we know you love to script, write also accepts input on stdin:

echo "Hello World" | secrethub write ianpurton/start/hello
Step 2: Consume secrets in your application
There are many way to provision an application with the secrets it needs. One common way is through environment variables, which you’ll see below. For a full list of integrations, see integrations.

Pass secrets as environment variables
Many applications that follow the popular 12-Factor App guidelines source their secrets from the environment and those secrets need to be managed too.

To see the mechanism in action, the SecretHub CLI comes packed with a demo application. This application serves a web page and tries to connect to https://demo.secrethub.io/api/v1/basic-auth using credentials provided in the environment (DEMO_USERNAME and DEMO_PASSWORD).

First, try to run the app locally without setting the username and password:

secrethub demo serve
A web page will now be served at http://localhost:8080, but if you visit it, you’ll see that it shows an error because it’s missing the username and password.

To get the demo application to work correctly, you’ll need to provide a username and password. You wouldn’t want to have those scattered around in plaintext, so let’s store those on SecretHub instead and use secrethub run to inject them at runtime.

Here’s a nice shortcut to auto-generate the values for you at ianpurton/demo:

secrethub demo init
Next, instead of populating environment variables with plaintext secrets, use secret references:

export DEMO_USERNAME=secrethub://ianpurton/demo/username
export DEMO_PASSWORD=secrethub://ianpurton/demo/password
Then, wrap the app start command in secrethub run:

secrethub run -- secrethub demo serve
The referenced secrets will now automatically get fetched, decrypted and injected as environment variables to the app.

If you visit http://localhost:8080 again, you’ll see that the red cross got replaced by a green checkmark. The wisdom that was hidden in the Demo API has now been revealed!

Step 3: Check audit logs
By now, you’ve touched your secrets a few times already.

When working in teams, it’s important to be able to track down who accessed which secrets at which point in time. That’s what the audit command is for.

For instance, use the following command to track down how the hello secret has been used (and abused) over time:

secrethub audit ianpurton/start/hello
As you can see, it prints out an audit log for the hello secret.