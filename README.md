## Tech Stack and Mono Repo Structure

| Folder | Description | Docker Container |
| ---- | ---- | ---- | 
| `.devcontainer` | Version controlled [Development Environment](https://code.visualstudio.com/docs/remote/containers)|purtontech/cloak-envoy|
| `.github` | Continuous integration pipeline calls out to [Earthly](https://earthly.dev)||
| `/app` | Web and gRPC Server. Written in Rust using [Axum](https://github.com/tokio-rs/axumtomni) and [Tonic](https://github.com/hyperium/tonic)|purtontech/cloak-server|
| `/app/asset-pipeline` | [Parcel](https://parceljs.org/) builder for [Typescript](https://www.typescriptlang.org/) and [SCSS](https://sass-lang.com/) assets.||
| `/cli` | Command Line Interface. Written in Rust.||
| `/infra` | Infrastructure as Code using [Pulumi](https://www.pulumi.com/).||
| `/protos` | [gRPC](https://grpc.io/) schema for the API.||
| `/db/migrations` | Database migrations for building the Postgres Database.|purtontech/cloak-db-migrations|
| `/www` | The website. Built with [Zola](https://www.getzola.org) static site generator.|purtontech/cloak-website|
| `Earthfile` |[Earthly](https://earthly.dev) Builds our containers and executables||
| `.releaserc.json`|[Semantic Releases](https://github.com/semantic-release/semantic-release)||

## Alternatives

| Name  | URL | SaaS? | E2E?| CLI? | Notes |
| ---- | ---- | ---- | ---- | ---- | --- |
| 1Password  | https://1password.com/secrets/  | yes | ? | ? | |
| SecretHub  | https://secrethub.io/ | Yes | Yes | Yes | Now part of 1Password | 
| Akeyless  | https://www.akeyless.io/ | Yes | No | Yes |  | 
| Hashicorp Vault  | https://www.vaultproject.io/ | No | No | Yes | Lots of Features | 
| Conjur  | https://www.conjur.org/ | No | No | Yes |  | 
| Doppler  | https://www.doppler.com/ | Yes | No | Yes |  | 

