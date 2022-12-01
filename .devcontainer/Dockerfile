# syntax=docker/dockerfile:1
FROM rust:1.62.1
ARG UIDGID
ARG USER

# fail build if args are missing
RUN if [ -z "$USER" ]; then echo '\nERROR: USER not set. Run \n\n \texport USER=$(whoami) \n\n on host before building Dockerfile.\n'; exit 1; fi
RUN if [ -z "$UID" ]; then echo '\nERROR: UID not set. Run \n\n \texport UID=$(id -u) \n\n on host before building Dockerfile.\n'; exit 1; fi
RUN if [ -z "$GID" ]; then echo '\nERROR: GID not set. Run \n\n \texport GID=$(id -g) \n\n on host before building Dockerfile.\n'; exit 1; fi

# Prevent the interactive wizards from stopping the build
ARG DEBIAN_FRONTEND=noninteractive

# Get the basics
RUN apt update -y &&       \
    apt install -y         \
        git                \
        sudo               \
        vim                \
        wget

RUN wget -O /tmp/nvim-linux64.deb https://github.com/neovim/neovim/releases/download/v0.7.0/nvim-linux64.deb && \
    apt install -y /tmp/nvim-linux64.deb

RUN git clone https://github.com/AstroNvim/AstroNvim /home/${USER}/.config/nvim

# Get rust tools
RUN rustup component add rustfmt clippy

# Get dependencies and download code
COPY . /tmp/ws
RUN cd /tmp/ws &&  \
    cargo build && \
    rm -rf /tmp/ws

# add the user so conan installs in the correct home directory, even though
# when we log into the container parts of this will be mapped over
RUN groupadd --gid ${GID} ${USER} \
    && useradd --no-log-init --uid ${UID} --gid ${GID} -m ${USER} --groups sudo
# grab the conanfile for installation
WORKDIR /home/${USER}/ws
# chown everything before we change users and run conan otherwise there
# will be permission errors
RUN chown -R ${UID}:${GID} /home/${USER}
USER ${USER}
