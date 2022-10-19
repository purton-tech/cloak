+++
title = "Accessing secrets from the command line"
date = 2022-02-22
sort_by = "weight" 
weight = 30
+++

### Step 1: Create a Service Account

Service accounts allow documentation or systems to access secrets. Create a service account and give it a name.

![Creating a service account](/docs/getting-started/create-service-account.png)

### Step 2: Attach the service account to a vault

![Creating a service account](/docs/getting-started/attach-service-account.png)

### Step 3: Access their service accounts private key

Cut and paste the service account private key into a file called cloak.pem in the folder from which you will use the cloak cli.

![Creating a vault](/docs/getting-started/show-private-key.png)

### Step 4: View your secrets with Cloak cli.

```sh
$ cloak secrets
+--------------+---------------------------------+
| Name         | Value                           |
+--------------+---------------------------------+
| PORT         | 8080                            |
+--------------+---------------------------------+
| DATABASE_URL | psql://localhost/payments       |
+--------------+---------------------------------+
| API_KEY      | sk_test_mKciizZfpXhQgXoNZmzECVN |
+--------------+---------------------------------+
```