FROM python:3.10

ENV DEBIAN_FRONTEND=noninteractive
ENV LC_ALL=C.UTF-8
ENV LANG=C.UTF-8
ARG NIGHTLY_VERSION=nightly-2023-01-13

ENV PATH=${PATH}:$HOME/.cargo/bin

RUN apt-get update --yes \
 && apt-get install --yes --no-install-recommends \ 
    git curl unzip \
    libusb-1.0-0-dev libudev-dev usbutils \
    libssl-dev pkg-config libtinfo5 \
    qemu-system-arm \
 && apt-get clean -y \
 && rm -rf /var/lib/apt/lists/* /tmp/library-scripts

 # Install Chromium (headless) for mdbook-pdf
 # https://geekflare.com/install-chromium-ubuntu-centos/

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- --default-toolchain ${NIGHTLY_VERSION} -y --profile minimal \
 && $HOME/.cargo/bin/rustup component add rust-src --toolchain ${NIGHTLY_VERSION}

RUN ${HOME}/.cargo/bin/rustup component add llvm-tools-preview \
 && ${HOME}/.cargo/bin/cargo install 
    mdbook \
    # mdbook-pdf \
    cargo-binutils cargo-generate \
    probe-run \
    uf2conv cargo-hf2

RUN ${HOME}/.cargo/bin/rustup target add thumbv7em-none-eabihf \
 && ${HOME}/.cargo/bin/rustup target add wasm32-unknown-unknown \
 && ${HOME}/.cargo/bin/rustup toolchain install nightly