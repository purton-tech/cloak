+++
title = "Securing your development environment against supply chain attacks"
date = 2022-09-08
draft = true
+++

As a developer you install executables almost daily. Sometimes directly sometimes indirectly.

You're also often the weakest link in the supply chain. Most likely your production server is hardened but your development environment is not.

Here are some of the attacks you need to adequately defend yourself against.

1. Typosquatting. Attackers add libraries to repositories with names similar to popular libraries but with malware installed.
1. Directly. Build tools, testing tools, bash scripts.

All of which may have full access to your system.

## Protecting your development machine

## Locking Egress