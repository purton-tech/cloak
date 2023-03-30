+++
title = "Kubernetes"
date = 2022-03-01
sort_by = "weight" 
weight = 20
+++

## Cloak

Sync secrets from the [Cloak Encrypted Secrets Platform](https:/cloak.software) to Kubernetes using the External Secrets Operator.

Cloak uses the webhook provider built into the External Secrets Operator but also required a proxy service to handle decrypting secrets when they arrive into your cluster.

## Key Setup

From the Cloak user interface [create a service account](https://cloak.software/docs/getting-started/03-cli/) and store the private key on your file system.

Now create a kubernetes secret in the same namespace as the External Secrets Operator.

```sh
HISTIGNORE='*kubectl*' kubectl --namespace=external-secrets \
    create secret generic cloak-key \
    --from-file=ecdh_private_key=$LOCATION_OF_YOUR_PEM_FILE
```

## Deploy the decryption proxy

```yaml
# The cloak external secrets proxy
apiVersion: apps/v1
kind: Deployment
metadata:
  name: cloak-external-secrets
  namespace: external-secrets
spec:
  selector:
    matchLabels:
      app: cloak-external-secrets
  replicas: 1
  template:
    metadata:
      labels:
        app: cloak-external-secrets
    spec:
      containers:
      - name: cloak-external-secrets
        image: purtontech/cloak-external-secrets:latest
        imagePullPolicy: IfNotPresent
        env: 
          - name: ECDH_PRIVATE_KEY 
            valueFrom: 
              secretKeyRef: 
                name: cloak-key 
                key: ecdh_private_key 
        ports:
        - containerPort: 7105
```

And a Kubernetes Service so External Secrets Operator can access the proxy.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: cloak-external-secrets-service
  namespace: external-secrets
spec:
  selector:
    app: cloak-external-secrets
  ports:
    - protocol: TCP
      port: 7105
      targetPort: 7105
```

## Create a secret store

You can now place the configuration in any Kubernetes Namespace.

```yaml
# An External secrets webhookl
apiVersion: external-secrets.io/v1beta1
kind: SecretStore
metadata:
  name: cloak-backend
spec:
  provider:
    webhook:
      url: "http://cloak-external-secrets-service:7105/{{ .remoteRef.key }}"
      result:
        jsonPath: "$.value"
      headers:
        Content-Type: application/json
```

## Connect a secret to the provider

Each `secretKey` reference in the yaml should point to the name of the secret as it is stored in Cloak.

```yaml

# Access a secret
apiVersion: external-secrets.io/v1beta1
kind: ExternalSecret
metadata:
  name: cloak-example
spec:
  refreshInterval: "15m"
  secretStoreRef:
    name: cloak-backend
    kind: SecretStore
  target:
    name: example-sync
  data:
  - secretKey: access-token
    remoteRef:
      key: PULUMI_ACCESS_TOKEN
  - secretKey: do-access-token
    remoteRef:
      key: DIGITALOCEAN_ACCESS_TOKEN
```