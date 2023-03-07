## Docker container to connect with the External Secrets Operator

The container is basically a proxy that intercepts requests from the [Webhook](https://external-secrets.io/v0.7.2/provider/webhook/) and pulls the secrets from Cloak and decrypts them.

## Install Kind

Kind allows us to run a kubernetes cluster in our devconatiner.

Install into our devcontainer

```sh
curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.17.0/kind-linux-amd64 && chmod +x ./kind && sudo mv ./kind /usr/local/bin/kind
```

Create a cluster

```sh
kind create cluster
```

Access the cluster

```sh
k9s --insecure-skip-tls-verify

or

kubectl get pods --insecure-skip-tls-verify
```

## Install Kubectl

```sh
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl" && sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl && rm kubectl
```

## Install Helm

```sh
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
```

## Install External Secrets Operator

```sh
helm repo add external-secrets https://charts.external-secrets.io
```

```sh
helm install external-secrets external-secrets/external-secrets  -n external-secrets --create-namespace --kube-insecure-skip-tls-verify
```

## Build our container

```sh
earthly -P +external-secrets-container
```

## Add a Private Key as a secret.

```sh
kubectl --insecure-skip-tls-verify create secret generic cloak-key --from-file=ecdh_private_key=/workspace/cloak.pem
```

```sh
kind load docker-image purtontech/cloak-external-secrets:latest
```

```sh
kubectl --insecure-skip-tls-verify apply -f crates/external-secrets/yaml
```
