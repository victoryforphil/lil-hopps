# Use the official Ubuntu 22.04 LTS image
FROM ubuntu:22.04

RUN apt-get -yqq update && apt-get -yqq install docker.io

# Install dependencies including ca-certificates
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        git \
        zsh \
        locales \
        crossbuild-essential-armhf \
        ansible \
        sshpass \
        valgrind \
        musl-tools \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set locale (optional, but recommended)
RUN locale-gen en_US.UTF-8
ENV LANG=en_US.UTF-8
ENV LANGUAGE=en_US:en
ENV LC_ALL=en_US.UTF-8

# Install rustup (Rust toolchain installer)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add Cargo's bin directory to PATH environment variable
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Oh My Zsh manually
RUN git clone https://github.com/ohmyzsh/ohmyzsh.git /root/.oh-my-zsh && \
    cp /root/.oh-my-zsh/templates/zshrc.zsh-template /root/.zshrc && \
    sed -i 's/ZSH_THEME="robbyrussell"/ZSH_THEME="agnoster"/' /root/.zshrc && \
    chsh -s $(which zsh)
ENV CROSS_CONTAINER_IN_CONTAINER=true

# Set working directory
WORKDIR /lil-dev

# Add Rust targets
RUN rustup target add arm-unknown-linux-gnueabihf
RUN rustup target add x86_64-unknown-linux-musl

# Create the target directory
RUN mkdir -p /lil-dev/target

# Copy a preconfigured .zshrc file (optional)
COPY zshrc /root/.zshrc

# Set the correct permissions
RUN chown root:root /root/.zshrc


# Start Zsh in interactive login mode
CMD ["zsh", "-l"]