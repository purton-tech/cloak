## Folder Structure

* `/app` - Web and gRPC Server. Rust with Typescript and SCSS.
* `/cli` - Command Line Interface. Rust.
* `/infra` - Infrastructure as Coe. Pulumi.
* `/protos` - Protobuf definitions for the API
* `/migrations` - Database migrations for building the Postgres Database
* `/www` The website. Built with Zola

## Alternatives

| Name  | URL | SaaS? | E2E?| CLI? | Notes |
| ---- | ---- | ---- | ---- | ---- | --- |
| 1Password  | https://1password.com/secrets/  | yes | ? | ? | |
| SecretHub  | https://secrethub.io/ | Yes | Yes | Yes | Now part of 1Password | 
| Akeyless  | https://www.akeyless.io/ | Yes | No | Yes |  | 
| Hashicorp Vault  | https://www.vaultproject.io/ | No | No | Yes | Lots of Features | 
| Conjur  | https://www.conjur.org/ | No | No | Yes |  | 
