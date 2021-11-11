## Start Zola

- `cd front-of-house`
- `zola serve`

## Alternatives

### 1Password

https://1password.com/secrets/ 

1Password Secrets Automation (The result of merging SecretHub into 1Pass)

- Appears not to be end 2 end encrypted (check this)
- Closed source
- Doesn't support secrets rotation

### SecretHub

https://secrethub.io/ 

Now merged into 1 pass

- Uses a CLI tool that can set the env vars i.e. secrehub -run
- Good K8 Integration, can restart pods on secret rotation

### Hashicorp Vault

https://www.vaultproject.io/

Probably the industry standard

- Needs to be provisioned on re-start with 3 of 5 shamir keys
- Can be auto provisioned with a KMS. Is this paid only?
- Fully open source
- Has a front end you can deploy. 
- Really great plugins like key rotation etc.
- Multiple auth methods
- RBAC
- Web UI only for configuration. i..e can't create secrets etc.

Features

- Just in time secrets
- Encryption as a service
- Secrets Rotation

### A keyless

https://www.akeyless.io/

- Has their own encryption. Not sure why or how it works
- Closed source
- 3 clients or 50 passwords for free.
- Starts at 2000 USD per month.
- Clunky on boarding, cli didn't work and web ui doesn't have instructions.

Features

- Just in time secrets
- Centralised UI
- Secrets Rotation - can be setup with a timner i.e. once a day.
- SSH or PKI short term certs.
- Multiple Auth methods i.e. kubernetes, Azure etc.
- RBAC
- Has the concept of gateways.