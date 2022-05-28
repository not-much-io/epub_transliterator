FROM ubuntu:jammy

###
### Base editor setup
###
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
    && apt-get -y install --no-install-recommends apt-utils dialog 2>&1 \
    #
    # Verify git, needed tools installed
    && apt-get -y install git iproute2 procps lsb-release lldb wget curl build-essential \
    #
    # Create a non-root user to use if preferred - see https://aka.ms/vscode-remote/containers/non-root-user.
    && groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    #
    # [Optional] Add sudo support for the non-root user
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME \
    #
    # Install pandoc
    && wget https://github.com/jgm/pandoc/releases/download/2.16.2/pandoc-2.16.2-1-amd64.deb \
    && dpkg -i pandoc-2.16.2-1-amd64.deb 
ENV DEBIAN_FRONTEND=dialog

# The rest as specific user - rust toolchain install and container run
USER ${USERNAME}

###
### Install rust toolchain
###
ENV RUST_VERSION="nightly-2021-10-24"
RUN curl -sSf https://sh.rustup.rs | \
    sh -s -- --default-toolchain ${RUST_VERSION} -y
ENV PATH=/home/vscode/.cargo/bin:${PATH}
RUN rustup component add rustfmt clippy
