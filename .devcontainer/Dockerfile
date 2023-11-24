# Use the latest Ubuntu image as starting point
FROM ubuntu:latest

# Install basic tooling
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    curl \
    python3

# Install asciidoctor tooling
RUN apt-get update && apt-get install -y \
    asciidoctor \
    ruby-rouge

# Install web server
RUN apt-get update && apt-get install -y nginx
EXPOSE 8080
COPY nginx.conf /etc/nginx/nginx.conf

# Disable host key checking for SSH to avoid known_hosts issues
RUN echo "StrictHostKeyChecking no" >> /etc/ssh/ssh_config

# Install Java JRE AND SDK
RUN apt-get update && apt-get install -y default-jre default-jdk

# Create user 'docker' with sudo privileges
RUN useradd -ms /bin/bash docker \
    && apt-get update \
    && apt-get install -y sudo \
    && echo 'docker ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER docker

# Install rust (nightly and stable) using rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y \
    && curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

# Install kotlin using sdkman
RUN curl -s "https://get.sdkman.io" | bash \
    && chmod 755 $HOME/.sdkman/bin/sdkman-init.sh \
    && bash -c "source $HOME/.sdkman/bin/sdkman-init.sh && sdk install kotlin"

# Setup git environment and checker
COPY check-git.sh /home/docker/check-git.sh
RUN echo "$HOME/check-git.sh" >> /home/docker/.bashrc \
    && git config --global core.editor "code --wait" \
    && git config --global --add safe.directory "/workspace"

# Set environment variables
ARG REPO_BASE_DIR
ARG REPO_MAIN_BRANCH
ARG AOC_YEAR
ENV REPO_BASE_DIR ${REPO_BASE_DIR}
ENV REPO_MAIN_BRANCH ${REPO_MAIN_BRANCH}
ENV AOC_YEAR ${AOC_YEAR}
