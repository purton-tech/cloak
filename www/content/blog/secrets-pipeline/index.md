+++
title = "What are your options for a Kubernetes Secrets Pipeline"
date = 2023-02-08
draft = true

[extra]
main_image = "blog/secrets-pipeline/secrets-pipeline-art.png"
listing_image = "blog/secrets-pipeline/secrets-pipeline-art.png"
+++

I looked at some of the threads about secrets management in Kubernetes and was able to come up with the following diagram.

![Kubernetes Secrets Pipeline](./pipeline.svg)

A couple of the concepts.

## First, I'm calling it a pipeline because secrets can flow left and right.

Maturity, the more 

The bridge. To get secrets into the cluster.

## Picking a learning curve that maximises your return on investemnt (ROI)

## A simple secrets pipeline

![Sealed Secrets Pipeline](./sealed-secrets.svg)

Creating a sealed secret in a Kubernetes cluster involves the following steps:

1. Install the Sealed Secrets operator: You can install the Sealed Secrets operator using the Kubernetes manifests or by using a package manager such as Helm.

1. 1. Create a public key: The Sealed Secrets operator uses public-key cryptography to encrypt secrets. You will need to create a public key and make it available to the Sealed Secrets operator.

1. Encrypt a secret: To create a sealed secret, you can use the `kubeseal` CLI tool provided by the Sealed Secrets operator to encrypt a Kubernetes secret. The `kubeseal` tool takes a Kubernetes secret as input, encrypts its values, and outputs a sealed secret file.

1. Store the sealed secret in the cluster: The sealed secret file can be stored in the cluster as a custom resource definition (CRD). You can use the kubectl command-line tool to create the sealed secret in the cluster.

1. Access the sealed secret: To access the encrypted values of a sealed secret, you will need to have the necessary permissions and the Sealed Secrets operator installed in your cluster. The Sealed Secrets operator uses role-based access control (RBAC) to manage access to sealed secrets.

Note: The details of the above steps may vary depending on the specific Sealed Secrets operator you are using, so it is important to consult the documentation for your particular setup. 

## A pipeline where we maximise the re-use of our knowledge

Vendor -> External Secrets -> K8

## A pipeline with full key rotation (Secrets flowing left)

Vendor <-> External Secrets <- Hashicorp Vault / Azure etc.

I'd be happy to recommend that stack. 

## How to choose a vendor on the left

The criteria

Defence in Depth
