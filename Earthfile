FROM ianpurton/rust-fullstack-devcontainer:latest

ARG APP_EXE_NAME=app
ARG APP_FOLDER=app
ARG CLI_FOLDER=cli
ARG CLI_EXE_NAME=cli

# Base images
ARG ENVOY_PROXY=envoyproxy/envoy:v1.17-latest
ARG NGINX=nginx:latest

# This file builds the following containers
ARG APP_IMAGE_NAME=ianpurton/vault:app
ARG INIT_IMAGE_NAME=ianpurton/vault:init
ARG ENVOY_IMAGE_NAME=ianpurton/vault:envoy
ARG WWW_IMAGE_NAME=ianpurton/vault:www


WORKDIR /build

USER root

# Set up for docker in docker https://github.com/earthly/earthly/issues/1225
DO github.com/earthly/lib+INSTALL_DIND

USER vscode

all:
    BUILD +init-container
    BUILD +app-container
    BUILD +envoy-container
    BUILD +www-container
    #BUILD +integration-test

npm-deps:
    COPY $APP_FOLDER/package.json package.json
    COPY $APP_FOLDER/package-lock.json package-lock.json
    RUN npm install
    SAVE ARTIFACT node_modules

npm-build:
    FROM +npm-deps
    COPY $APP_FOLDER/asset-pipeline asset-pipeline
    COPY $APP_FOLDER/src src
    COPY +npm-deps/node_modules node_modules
    RUN npm run release
    SAVE ARTIFACT dist

prepare-cache:
    COPY --dir $APP_FOLDER/src $APP_FOLDER/Cargo.toml $APP_FOLDER/build.rs $APP_FOLDER/asset-pipeline $APP_FOLDER
    COPY --dir $CLI_FOLDER/src $CLI_FOLDER/Cargo.toml $CLI_FOLDER
    COPY Cargo.lock Cargo.toml .
    RUN cargo chef prepare --recipe-path recipe.json
    SAVE ARTIFACT recipe.json

build-cache:
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook --release --target x86_64-unknown-linux-musl 
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home

build:
    COPY --dir $APP_FOLDER/src $APP_FOLDER/Cargo.toml $APP_FOLDER/build.rs $APP_FOLDER/asset-pipeline $APP_FOLDER
    COPY --dir $CLI_FOLDER/src $CLI_FOLDER/Cargo.toml $CLI_FOLDER
    COPY --dir migrations Cargo.lock Cargo.toml protos .
    COPY +build-cache/cargo_home $CARGO_HOME
    COPY +build-cache/target target
    RUN mkdir asset-pipeline
    COPY --dir +npm-build/dist $APP_FOLDER/
    COPY --dir $APP_FOLDER/asset-pipeline/images $APP_FOLDER/asset-pipeline
    # We need to run inside docker as we need postgres running for SQLX
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432
    USER root
    WITH DOCKER \
        --pull postgres:alpine
        RUN docker run -d --rm --network=host -e POSTGRES_PASSWORD=testpassword postgres:alpine \
            && while ! pg_isready --host=localhost --port=5432 --username=postgres; do sleep 1; done ;\
                diesel migration run \
            && cargo build --release --target x86_64-unknown-linux-musl
    END
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$APP_EXE_NAME $APP_EXE_NAME
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$CLI_EXE_NAME $CLI_EXE_NAME

init-container:
    FROM ianpurton/rust-diesel:latest
    COPY --dir migrations .
    CMD diesel migration run
    SAVE IMAGE --push $INIT_IMAGE_NAME

app-container:
    FROM scratch
    COPY +build/$APP_EXE_NAME rust-exe
    COPY --dir +npm-build/dist dist
    COPY --dir $APP_FOLDER/asset-pipeline/images asset-pipeline/images
    ENTRYPOINT ["./rust-exe"]
    SAVE IMAGE --push $APP_IMAGE_NAME

envoy-container:
    FROM $ENVOY_PROXY
    COPY .devcontainer/envoy.yaml /etc/envoy/envoy.yaml
    # Update the first entry in our config to point at the marketing pages
    RUN sed -i '0,/development/{s/development/www/}' /etc/envoy/envoy.yaml
    # The second development entry in our cluster list is the app
    RUN sed -i '0,/development/{s/development/app/}' /etc/envoy/envoy.yaml
    SAVE IMAGE --push $ENVOY_IMAGE_NAME

zola-generate:
    ARG ZOLA_VERSION=0.12.2
    RUN sudo curl -OL https://github.com/getzola/zola/releases/download/v$ZOLA_VERSION/zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
        && sudo tar -xvf zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
        && sudo mv zola /usr/bin/zola \
        && sudo chmod +x /usr/bin/zola
    COPY --dir www www
    RUN cd www && zola build
    SAVE ARTIFACT www/public public

# Test this with docker run --rm -p7180:80 ianpurton/vault:www
www-container:
    FROM $NGINX
    COPY +zola-generate/public /usr/share/nginx/html/
    SAVE IMAGE --push $WWW_IMAGE_NAME

# Run the full stack and test it with selenium.
# To reproduce the tests locally run the following from the terminal
# docker-compose -f docker-compose.earthly.yml -f docker-compose.yml -p earthly up db auth selenium
# and the following in a devcontainer
# WEB_DRIVER_URL=http://host.docker.internal:4444/wd/hub WEB_DRIVER_DESTINATION_HOST=http://auth:9090 cargo test signup
integration-test:
    FROM +build
    COPY --dir $WEBAPP_FOLDER/tests $WEBAPP_FOLDER/
    COPY --dir migrations .
    COPY --dir docs/ccsds-examples ./ccsds-examples
    COPY .devcontainer/docker-compose.yml ./ 
    COPY .devcontainer/docker-compose.earthly.yml ./ 
    COPY .devcontainer/Dockerfile.postgres ./ 
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432/cream
    ARG WEB_APP_DATABASE_URL=postgresql://web_application:testpassword@host.docker.internal:5432/cream
    ARG WEB_DRIVER_URL='http://localhost:4444/wd/hub' 
    ARG WEB_DRIVER_DESTINATION_HOST='http://auth:9090' 
    USER root
    WITH DOCKER \
        --compose docker-compose.earthly.yml \
        --compose docker-compose.yml \
        --service db \
        --service auth \
        --service selenium \
        --load webui:latest=+web-app-container

        RUN docker run -d --rm --network=host -e WEB_APP_DATABASE_URL=$WEB_APP_DATABASE_URL webui:latest \
            && diesel migration run \
            && sleep 5 \
            && docker ps \
            && curl localhost:9091 \
            # Make a directory for the screenshots or the build fails
            && mkdir tmp \
            && cargo test --release --target x86_64-unknown-linux-musl -- --nocapture
    END
    SAVE ARTIFACT tmp AS LOCAL ./tmp/earthly