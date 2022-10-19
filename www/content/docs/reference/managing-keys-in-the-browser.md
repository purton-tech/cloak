+++
title = "Managing security keys in the browser"
date = 2022-02-03
weight = 20
+++

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

![Browser based key management based on passwords](/docs/reference/client-side-encryption.png)

## Algorithms

## Defending the supply chain