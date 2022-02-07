FROM ianpurton/rust-fullstack-devcontainer:latest AS development

COPY *.bash ./

# Add our aliases and ps1
RUN cat aliases.bash >> ~/.bashrc && cat ps1.bash >> ~/.bashrc && sudo rm *.bash


# Enable our git hooks and set the permisisons on docker sock.
RUN echo 'git config core.hooksPath /vscode/.devcontainer/.githooks' >> ~/.bashrc \
    && echo 'sudo chmod 777 /var/run/docker.sock' >> ~/.bashrc

# Zola
#ARG ZOLA_VERSION=0.12.2
ARG ZOLA_VERSION=0.15.3
RUN sudo curl -OL https://github.com/getzola/zola/releases/download/v$ZOLA_VERSION/zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    && sudo tar -xvf zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    && sudo mv zola /usr/bin/zola \
    && sudo chmod +x /usr/bin/zola

ARG USERNAME=vscode

# Pulumi
RUN curl -fsSL https://get.pulumi.com | sudo -E bash -
RUN sudo chown -R $USERNAME:$USERNAME /home/$USERNAME/.pulumi

# Install az
RUN sudo apt-get update --allow-releaseinfo-change && curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash

# k9s
RUN curl -L -s https://github.com/derailed/k9s/releases/download/v0.24.15/k9s_Linux_x86_64.tar.gz | tar xvz -C /tmp && sudo mv /tmp/k9s /usr/bin

# Proto compiler and web grpc.
RUN sudo apt update \
    && sudo apt install -y protobuf-compiler \
    && sudo curl -OL https://github.com/grpc/grpc-web/releases/download/1.3.0/protoc-gen-grpc-web-1.3.0-linux-x86_64  \
    && sudo mv protoc-gen-grpc-web* /usr/local/bin/protoc-gen-grpc-web \
    && sudo chmod +x /usr/local/bin/protoc-gen-grpc-web

# Install secrets management
RUN sudo /bin/sh -c 'wget https://github.com/ianpurton/keyvault/releases/latest/download/cli -O /usr/local/bin/cloak && chmod +x /usr/local/bin/cloak'

# Install Doppler
RUN sudo apt-get update && sudo apt-get install -y apt-transport-https ca-certificates curl gnupg \
    && curl -sLf --retry 3 --tlsv1.2 --proto "=https" 'https://packages.doppler.com/public/cli/gpg.DE2A7741A397C129.key' | sudo apt-key add - \
    && echo "deb https://packages.doppler.com/public/cli/deb/debian any-version main" | sudo tee /etc/apt/sources.list.d/doppler-cli.list \
    && sudo apt-get update && sudo apt-get install doppler
    
# all the volumes configured in the docker-compose.yml
RUN sudo mkdir -p /vscode/target && sudo chown $USERNAME:$USERNAME /vscode/target
RUN sudo mkdir -p /vscode/app/asset-pipeline/node_modules && sudo chown $USERNAME:$USERNAME /vscode/app/asset-pipeline/node_modules
RUN sudo mkdir -p /vscode/infra/node_modules && sudo chown $USERNAME:$USERNAME /vscode/infra/node_modules
