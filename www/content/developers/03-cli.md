+++
title = "Accessing secrets from the command line"
date = 2022-02-22
+++

### Step 1: Create a Service Account

Service accounts allow developers or systems to access secrets. Create a service account and give it a name.

![Creating a service account](/developers/create-service-account.png)

### Step 2: Attach the service account to a vault

![Creating a service account](/developers/attach-service-account.png)

### Step 3: Access ther service accounts private key

Cut and paste the service account private key into a file called cloak.pem in the folder from which you will use the cloak cli.

![Creating a vault](/developers/show-private-key.png)

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