FROM ianpurton/rust-fullstack-devcontainer:latest

ARG WEBAPP_EXE_NAME=web-ui
ARG WEBAPP_FOLDER=web-ui
ARG GRPC_SERVER_EXE_NAME=grpc-server
ARG GRPC_SERVER_FOLDER=grpc-server

WORKDIR /build

USER root
# Protoc
RUN apt install -y protobuf-compiler

DO github.com/earthly/lib+INSTALL_DIND

USER vscode

all:
    #BUILD +init-container
    BUILD +web-app-container
    #BUILD +grpc-server-container
    #BUILD +api-docs-nginx-container
    #BUILD +integration-test

npm-deps:
    COPY $WEBAPP_FOLDER/package.json package.json
    COPY $WEBAPP_FOLDER/package-lock.json package-lock.json
    RUN npm install
    SAVE ARTIFACT node_modules

npm-build:
    FROM +npm-deps
    COPY $WEBAPP_FOLDER/asset-pipeline asset-pipeline
    COPY $WEBAPP_FOLDER/src src
    COPY protos protos
    # Update so we point at the correct location of the proto folder
    RUN sed -i 's/\.\/protos/\/protos/g' package.json
    COPY +npm-deps/node_modules node_modules
    RUN npm run release
    SAVE ARTIFACT asset-pipeline/dist

prepare-cache:
    COPY --dir $WEBAPP_FOLDER/src $WEBAPP_FOLDER/Cargo.toml $WEBAPP_FOLDER/build.rs $WEBAPP_FOLDER/asset-pipeline $WEBAPP_FOLDER
    COPY Cargo.lock Cargo.toml .
    COPY --dir $GRPC_SERVER_FOLDER/src $GRPC_SERVER_FOLDER/Cargo.toml $GRPC_SERVER_FOLDER/build.rs $GRPC_SERVER_FOLDER
    RUN cargo chef prepare --recipe-path recipe.json
    SAVE ARTIFACT recipe.json

build-cache:
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook --release --target x86_64-unknown-linux-musl 
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home

build:
    COPY --dir $WEBAPP_FOLDER/src $WEBAPP_FOLDER/Cargo.toml $WEBAPP_FOLDER/build.rs $WEBAPP_FOLDER/asset-pipeline $WEBAPP_FOLDER
    COPY --dir migrations Cargo.lock Cargo.toml protos .
    COPY .devcontainer/postgres_users.sql .
    COPY --dir $GRPC_SERVER_FOLDER/src $GRPC_SERVER_FOLDER/Cargo.toml $GRPC_SERVER_FOLDER/build.rs $GRPC_SERVER_FOLDER
    COPY +build-cache/cargo_home $CARGO_HOME
    COPY +build-cache/target target
    RUN mkdir asset-pipeline
    COPY --dir +npm-build/dist $WEBAPP_FOLDER/asset-pipeline
    COPY --dir $WEBAPP_FOLDER/asset-pipeline/images $WEBAPP_FOLDER/asset-pipeline
    # We need to run inside docker as we need postgres running for SQLX
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432
    USER root
    WITH DOCKER \
        --pull postgres:alpine
        RUN docker run -d --rm --network=host -e POSTGRES_PASSWORD=testpassword postgres:alpine \
            && while ! pg_isready --host=localhost --port=5432 --username=postgres; do sleep 1; done ;\
                psql $DATABASE_URL -f postgres_users.sql \
            && diesel migration run \
            && cargo build --release --target x86_64-unknown-linux-musl
    END
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$WEBAPP_EXE_NAME $WEBAPP_EXE_NAME
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$GRPC_SERVER_EXE_NAME $GRPC_SERVER_EXE_NAME

init-container:
    FROM ianpurton/rust-diesel:latest
    ARG INIT_IMAGE_NAME=ianpurton/cream:init
    COPY --dir migrations .
    CMD diesel migration run
    SAVE IMAGE --push $INIT_IMAGE_NAME

web-app-container:
    FROM scratch
    ARG WEBAPP_IMAGE_NAME=ianpurton/cream:webapp
    COPY +build/$WEBAPP_EXE_NAME rust-exe
    COPY --dir +npm-build/dist asset-pipeline/dist
    COPY --dir $WEBAPP_FOLDER/asset-pipeline/images asset-pipeline/images
    ENTRYPOINT ["./rust-exe"]
    SAVE IMAGE --push $WEBAPP_IMAGE_NAME

grpc-server-container:
    FROM scratch
    ARG GRPC_SERVER_IMAGE_NAME=ianpurton/cream:grpc
    COPY +build/$GRPC_SERVER_EXE_NAME rust-exe
    ENTRYPOINT ["./rust-exe"]
    SAVE IMAGE --push $GRPC_SERVER_IMAGE_NAME

api-docs-generate:
    ARG PROTO_NAME=cream_server.proto
    ARG SWAGGER_NAME=cream_server
    FROM namely/gen-grpc-gateway:1.30_0
    # Node needed for api2html
    RUN apk add --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/v3.7/main/ nodejs=8.9.3-r1
    # To handle 'not get uid/gid'
    # https://stackoverflow.com/questions/52196518/could-not-get-uid-gid-when-building-node-docker
    RUN npm config set unsafe-perm true
    RUN npm i api2html -g
    COPY ./protos /defs
    COPY ./.devcontainer/strip.sh /defs
    RUN chmod +x /defs/strip.sh
    RUN protoc --plugin==protoc-gen-grpc=protoc-gen-swagger --swagger_out=logtostderr=true,fqn_for_swagger_name=true,simple_operation_ids=true:. -I/opt/include -I./ $PROTO_NAME
    # Convert strings format uint64 to integer
    RUN ./strip.sh $SWAGGER_NAME.swagger.json > $SWAGGER_NAME.swagger.json.convert
    RUN mv $SWAGGER_NAME.swagger.json.convert $SWAGGER_NAME.swagger.json
    # Now generate API docs.
    RUN api2html $SWAGGER_NAME.swagger.json -o index.html
    SAVE ARTIFACT /defs

api-docs-nginx-container:
    ARG API_DOCS_IMAGE_NAME=ianpurton/cream:api
    FROM nginx
    COPY +api-docs-generate/defs /usr/share/nginx/html
    SAVE IMAGE --push $API_DOCS_IMAGE_NAME

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