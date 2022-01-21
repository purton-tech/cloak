containers=( "purtontech/cloak-server:latest" "purtontech/cloak-db-migrations:latest" "purtontech/cloak-envoy:latest" "purtontech/cloak-website:latest")

for i in "${containers[@]}"
do
    docker pull $i 
    CONFIG_NAME=$(echo $i | cut -c 12-) \
    HASH=$(docker inspect --format='{{index .RepoDigests 0}}' $i )
    HASH=$(echo $HASH | sed 's/^.*@//' )
    echo "Name $CONFIG_NAME"
    echo "Hash $HASH"
    sed -i "s/keyvault:$CONFIG_NAME.*$/keyvault:$CONFIG_NAME: $HASH/" Pulumi.prod.yaml 
done