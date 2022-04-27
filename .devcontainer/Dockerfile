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

# all the volumes configured in the docker-compose.yml
RUN sudo mkdir -p /workspace/target && sudo chown $USERNAME:$USERNAME /workspace/target
RUN sudo mkdir -p /workspace/app/node_modules && sudo chown $USERNAME:$USERNAME /workspace/app/node_modules
RUN sudo mkdir -p /workspace/infra/node_modules && sudo chown $USERNAME:$USERNAME /workspace/infra/node_modules
