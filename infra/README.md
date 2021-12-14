## Deployment

1. `docker login`
1. `az login`
1. `pulumi login`
1. `az aks get-credentials --name tebi-prod-cluster --resource-group tebi-production`
1. `cd infra`
1. Make sure we have the correct hashes. We have to pull first or all the digests are empty.
1. `./update-config.sh` Get the hashes from the build
1. `pulumi up`

## Build containers and push locally

1. docker login
1. earthly -P --push +all