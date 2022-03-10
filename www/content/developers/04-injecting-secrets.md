+++
title = "Injecting secrets into applications"
date = 2022-02-21
+++

The Cloak cli can run your application and inject environment variables into the process.

To show this we can run `printenv` which is a Linux tool to show the environment.

```sh
$ cloak printenv | grep DATABASE_URL 
DATABASE_URL=psql://localhost/payments
```

It's the same process for running web apps, infrastructure as code tools and so on.