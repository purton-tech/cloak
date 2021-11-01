FROM ianpurton/rust-fullstack-devcontainer:latest AS development

COPY *.bash .

RUN cat *.bash >> ~/.bashrc && sudo rm *.bash

# Zola
RUN sudo curl -OL https://github.com/getzola/zola/releases/download/v0.10.0/zola-v0.10.0-x86_64-unknown-linux-gnu.tar.gz \
    && sudo tar -xvf zola-v0.10.0-x86_64-unknown-linux-gnu.tar.gz \
    && sudo mv zola /usr/bin/zola \
    && sudo chmod +x /usr/bin/zola

# all the volumes configured in the docker-compose.yml
ARG USERNAME=vscode
RUN sudo mkdir -p /vscode/webui/target && sudo chown $USERNAME:$USERNAME /vscode/webui/target
RUN sudo mkdir -p /vscode/webui/node_modules && sudo chown $USERNAME:$USERNAME /vscode/webui/node_modules
