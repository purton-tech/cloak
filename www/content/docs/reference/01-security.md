+++
title = "How we secure your secrets"
date = 2022-01-01
weight = 10
+++

> ## TL;DR 
> Out most notable features are
> * End to End encryption of your secrets. 
> * We handle encryption key creation and management in the client using native web cryptography.
> * The server never sees any plain text secrets.
> * All code is available for both client and server on github.
> * The software is build with Rust for performance and safety.

## Concepts

- A **Vault** is a placeholder for secrets.
- A **Service Account** gives access to the secrets in a vault for the CLI tool or for CI/CD pipelines etc.

## Vault Creation

Alice wants to create a vault that Bob can access later. Bob has not yet registered so he will be added later. When a user registers with Cloak, we create an ECDH key pair encrypted with a key derived from their password.

* An ECDH keypair is generated. (ecdh_keypair)
* Alice creates a new AES key for the vault which will be used to encrypt secrets. (aes_key)
* Alice already has an ECDH key pair, which she got during registration. (alice_ecdh_keypair)
* Alice encrypts the (aes_key) key, with an ECDH agreement between (alice_ecdh_keypair) and (ecdh_keypair).
* We store the wrapped (aes_key) in the database as well as the (ecdh_keypair) public key.

Alice wishes to add a secret to the vault.

* Alice retrieves her wrapped (aes_key) for the vault.
* Alice decrypts the AES Vault key, with an ECDH agreement between (alice_ecdh_keypair) and the (ecdh_keypair) public key.
* She encrypts the secret with the now unwrapped (aes_key)
* The secret is stored in the database

Later on Bob has registered and Alice wishes to give him access to the Vault.

* Bob already has an ECDH key pair, which she got during registration. (bob_ecdh_keypair)
* Alice retrieves her wrapped (aes_key) for the vault.
* Alice decrypts the AES Vault key, with an ECDH agreement between (alice_ecdh_keypair) and the (ecdh_keypair) public key.
* An ECDH keypair is generated. (ecdh_keypair)
* She creates a key agreement between (ecdh_keypair) and (bob_ecdh_keypair) with which she encrypts (aes_key).
* We store the wrapped (aes_key) in the database as well as the (ecdh_keypair) public key.

Bob wants to see the secret

* Bob retrieves the ECDH key pair, which he got during registration. (bob_ecdh_keypair)
* Bob retrieves the wrapped (aes_key) and the public key of the (ecdh_keypair)
* Bob uses a key agreement between (bob_ecdh_keypair) and (ecdh_keypair) to decrypt the AES key.
* He can now use the unwrapped (aes_key) to decrypt the secret.

## Secrets and Service Accounts

Alice wants to create a service account so she can access the secrets in a vault from her development machine.

* Alice retrieves her wrapped (aes_key) for the vault.
* Alice decrypts the AES Vault key, with an ECDH agreement between (alice_ecdh_keypair) and the (ecdh_keypair) public key.
* A service account ECDH key is created (service_account_ecdh_keypair)
* A throw away ECDH key is created. (temporary_ecdh_keypair)
* For every secret re-encrypt the secret with the agreement between (temporary_ecdh_keypair) and (service_account_ecdh_keypair)
* Wrap the (service_account_ecdh_keypair) private key with Alice's master AES key.
* Alice takes the (service_account_ecdh_keypair) and copies it to her machine.
* The newly encrypted secrets and the corresponding (temporary_ecdh_keypair) public key is stored in the database.

Alice uses the CLI tool to access secrets

* The tool downloads all the encrypted secrets with the corresponding (temporary_ecdh_keypair) public key
* The secret are decrypted using an agreement between the (temporary_ecdh_keypair) public key and the (service_account_ecdh_keypair) private key which Alice copied to her machine.


