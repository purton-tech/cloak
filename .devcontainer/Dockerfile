FROM ianpurton/rust-fullstack-devcontainer:latest AS development

COPY *.bash .

# Add our aliases and ps1
RUN cat aliases.bash >> ~/.bashrc && cat ps1.bash >> ~/.bashrc && sudo rm *.bash


# Enable our git hooks and set the permisisons on docker sock.
RUN echo 'git config core.hooksPath /vscode/.devcontainer/.githooks' >> ~/.bashrc \
    && echo 'sudo chmod 777 /var/run/docker.sock' >> ~/.bashrc

# Zola
ARG ZOLA_VERSION=0.12.2
RUN sudo curl -OL https://github.com/getzola/zola/releases/download/v$ZOLA_VERSION/zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    && sudo tar -xvf zola-v$ZOLA_VERSION-x86_64-unknown-linux-gnu.tar.gz \
    && sudo mv zola /usr/bin/zola \
    && sudo chmod +x /usr/bin/zola

# all the volumes configured in the docker-compose.yml
ARG USERNAME=vscode
RUN sudo mkdir -p /vscode/target && sudo chown $USERNAME:$USERNAME /vscode/target
RUN sudo mkdir -p /vscode/app/node_modules && sudo chown $USERNAME:$USERNAME /vscode/app/node_modules
