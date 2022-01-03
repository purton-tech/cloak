## Folder Structure

* `/app` - Web and gRPC Server. Written in Rust using Axum and Tonic
* `/app/asset-pipeline` - Parcel builder for Typescript and SCSS assets.
* `/cli` - Command Line Interface. Rust.
* `/infra` - Infrastructure as Code. Pulumi.
* `/protos` - Protobuf definitions for the API.
* `/migrations` - Database migrations for building the Postgres Database.
* `/www` - The website. Built with Zola static site generator.
* `Earthfile` - Build for CICD.

## Alternatives

| Name  | URL | SaaS? | E2E?| CLI? | Notes |
| ---- | ---- | ---- | ---- | ---- | --- |
| 1Password  | https://1password.com/secrets/  | yes | ? | ? | |
| SecretHub  | https://secrethub.io/ | Yes | Yes | Yes | Now part of 1Password | 
| Akeyless  | https://www.akeyless.io/ | Yes | No | Yes |  | 
| Hashicorp Vault  | https://www.vaultproject.io/ | No | No | Yes | Lots of Features | 
| Conjur  | https://www.conjur.org/ | No | No | Yes |  | 
