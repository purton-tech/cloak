#!/bin/bash 
sed -i "s/keyvault:version.*$/keyvault:version: $1/" Pulumi.prod.yaml 