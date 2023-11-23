# Use the official Rust image as the base image
FROM ubuntu:latest

# install basic tooling
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    curl \
    python3

# install asciidoctor tooling
RUN apt-get update && apt-get install -y \
    asciidoctor \
    ruby-rouge

# Disable host key checking for SSH to avoid known_hosts issues
RUN echo "StrictHostKeyChecking no" >> /etc/ssh/ssh_config

# create user 'docker' with sudo privileges
RUN useradd -ms /bin/bash docker \
    && apt-get update \
    && apt-get install -y sudo \
    && echo 'docker ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER docker

# create sym link ~/.vscode-server
RUN sudo mkdir /opt/vscode-server \
    && sudo chmod 777 /opt/vscode-server \
    && ln -s /opt/vscode-server /home/docker/.vscode-server

# install rust using rustup
ENV RUSTUP_HOME /opt/rustup
ENV CARGO_HOME /opt/cargo
RUN sudo mkdir /opt/rustup \
    && sudo chmod 777 /opt/rustup \
    && sudo mkdir /opt/cargo \
    && sudo chmod 777 opt/cargo \
    && curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y \
    && curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
ENV PATH="$CARGO_HOME/bin:$PATH"
RUN echo "source $CARGO_HOME/env" >> $HOME/.bashrc

ARG REPO_BASE_DIR
ARG REPO_MAIN_BRANCH
ARG AOC_YEAR
ENV REPO_BASE_DIR ${REPO_BASE_DIR}
ENV REPO_MAIN_BRANCH ${REPO_MAIN_BRANCH}
ENV AOC_YEAR ${AOC_YEAR}
