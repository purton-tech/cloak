+++
title = "How we secure your secrets"
date = 2022-01-01
+++

> ## TL;DR 
> Out most notable features are
> * End to End encryption of your secrets. 
> * We handle encryption key creation and management in the client using native web cryptography.
> * The server never sees any plain text secrets.
> * All code is available for both client and server on github.
> * The software is build with Rust for performance and safety.

## Concepts

A **Vault** is a placeholder for secrets.

## Secrets and Service Accounts

In the simple case where you have one vault with several secrets and one service account, the following procedures are followed.

Note, all these actions happen client side.

* When you add a secret to a vault it is immediately encrypted with the vault symmetric key.
* When you add a service account to a vault all the secrets in the vault are copied and encrypted with the service accounts asymmetric public key.
* Any subsequent secrets added to a vault will be encrypted for each service account that has access to that vault.
* The command line tool has the public and private asymmetric encryption keys.
* When the command line toll requests the secrets it decrypts them in memory and injects them into the process.

## Managing web user keys in the browser

Many web applications with client-side encryption that use passwords derive both encryption and server authentication keys from them. Examples are:

- Bitwarden (https://www.bitwarden.com/)
- Lastpass (https://www.lastpass.com/de)
- Blockchain Crypto Currency (https://www.blockchain.com)

### How does this in the browser?

In the **web browser** a master key is first derived from the given user **password**. The user email is used as cryptographic salt, i.e. used to increase the input entropy. PBDKF2 is a popular choice for this step.

User data encryption takes place using a randomly generated 64 byte symmetric key. After generation, this key is encrypted using the master key derived from the user password. The outcome of this process is called a "Wrapped key" (asymmetrically encrypted symmetric key). Finally, the master key is put into PBKFD2 with one iteration to generate a master password hash which is used to authenticate with the server.

### How does this work on the server?

On the **server side** the master password hash and the wrapped key are received. The master password hash is again passed through a hashing algorithm on the server (using Argon2id) in a similar way a normal password would be stored to perform user authentication. The protected wrapped keys are also encrypted once more with a second Argon2id hash to ensure that the only way to recover the keys is to brute force the password. 

The server wrapped key is then stored in the database along with any public keys and the Argon2id output of the master password hash. 

### Key Generation Process

![Browser based key management based on passwords](/reference/client-side-encryption.png)

## Algorithms

## Defending the supply chain