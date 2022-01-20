## Deployment

1. `git pull` make sure we have the latest Pulumi.prod.yaml
1. `az login --use-device-code`
1. `pulumi login`
1. `az aks get-credentials --name tebi-prod-cluster --resource-group tebi-production`
1. `cd infra`
1. `pulumi up`

## Build containers and push locally

1. docker login
1. earthly -P --push +all