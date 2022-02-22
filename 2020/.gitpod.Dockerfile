FROM gitpod/workspace-full

USER gitpod

RUN sudo apt-get -q update \
    && sudo apt-get install -yq \
        libpython3.6 \
        rust-lldb \
    && sudo rm -rf /var/lib/apt/lists/*
ENV RUST_LLDB=/usr/bin/lldb-8

# Project specifics
# Setup diesel_cli
ENV PATH="$HOME/.cargo/bin:$PATH"

RUN rustup install nightly
RUN rustup default nightly
