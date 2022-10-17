<h1 align="center">CLOAK</h1>
<div align="center">
 <strong>
   Your team's central source of truth for encrypted secrets and application configuration
 </strong>
</div>

<br />


<div align="center">
  <!-- License -->
  <a href="https://github.com/purton-tech/cloak#License">
    <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License">
  </a>
  <a href="https://hub.docker.com/repository/docker/purtontech/cloak-server">
    <img src="https://img.shields.io/docker/image-size/purtontech/cloak-server" alt="Docker Image Size">
  </a>
  <a href="https://hub.docker.com/repository/docker/purtontech/cloak-server">
    <img src="https://img.shields.io/docker/v/purtontech/cloak-server" alt="Docker Image Version">
  </a>
</div>

<br />

<div align="center">
  <h4>
    <a href="https://cloak.software">
      Homepage
    </a>
    <span> | </span>
    <a href="https://cloak.software/developers/">
      Docs
    </a>
  </h4>
</div>

<br />

![Alt text](www/static/secrets-screenshot.png "Secrets Screenshot")

## Features and Benefits

* End-to-End encryption - Every secret is encrypted before a single byte ever leaves your device. Only you and your team control the encryption keys and who is able decrypt your secrets.
* Eliminate secrets sprawl. When people leave or an incident happens, update secrets in one place and push it out everywhere.
* Role-Based Access Control (RBAC) means users only see the secrets they need to see to do their jobs.
* Built in multi tenancy. Teams can manage their own secrets and no one person has access to all the secrets unless you give them the permission.
* Uses well known and well tested Elliptic Curve cryptography. No proprietary encryption mechanisms.
* Our web interface manages encrypted key creation and distribution to make security the default for your projects.
* The CLI tool brings secrets from the vault to you local environment, CICD pipelines, servers and more.
* Hosted by us so you don't need to manage infrastructure, upgrades or support.
* You have the option to deploy on your own servers using our docker images. Our containers are built in public from our Github CI/CD pipeline.
* Permissive MIT licence and fully open client and server source code. This not only means that you can inspect our code, but that others have done so too.
* Extensive audit trail that monitors access and usage.
* Scales from Startup to Big Corp.

## documentation

Cloak uses the [Rust on Nails](https://rust-on-nails.com/) architecture for secure full stack web applications.
