containers=( "ianpurton/vault:init" "ianpurton/vault:app" "ianpurton/vault:envoy" "ianpurton/vault:www")

for i in "${containers[@]}"
do
   docker pull $i \
    && CONFIG_NAME=$(echo $i | cut -c 11-) \
    && HASH=$(docker inspect --format='{{index .RepoDigests 0}}' $i | cut -c 17-) \
    && pulumi config set key$CONFIG_NAME $HASH
done