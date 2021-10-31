FROM ianpurton/rust-fullstack-devcontainer:latest AS development

COPY *.bash .

RUN cat *.bash >> ~/.bashrc && sudo rm *.bash

# all the volumes configured in the docker-compose.yml
ARG USERNAME=vscode
RUN sudo mkdir -p /vscode/webui/target && sudo chown $USERNAME:$USERNAME /vscode/webui/target
RUN sudo mkdir -p /vscode/webui/node_modules && sudo chown $USERNAME:$USERNAME /vscode/webui/node_modules
