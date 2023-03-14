#!/bin/bash

set -xeuf -o pipefail

if [[ -v CI ]]; then
    SUDO=""
    apt-get update
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends tzdata
    echo "en_US UTF-8" > /etc/locale.gen
else
    SUDO="sudo"
fi

# Install system deps
${SUDO} apt update
${SUDO} apt-get install -y build-essential curl git libpq-dev lzip openssl pkg-config python3 python3-pip python3-venv \
                           tar protobuf-compiler libprotobuf-dev
python3 -m pip install --upgrade pip

curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"

rustup self update
rustup default nightly
cargo install sqlx-cli --version 0.5.13 --no-default-features --features native-tls,postgres

# Install docker
if ! type "docker" > /dev/null; then
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | ${SUDO} gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | ${SUDO} tee /etc/apt/sources.list.d/docker.list > /dev/null

    ${SUDO} apt update

    apt-cache policy docker-ce

    if [[ -v CI ]]; then
        echo "Skipping creation of docker group"
    else
        ${SUDO} apt install -y docker-ce
        ${SUDO} usermod -aG docker ${USER}
    fi
else
    echo 'docker already installed'
fi

# Install docker-compose
if ! type "docker-compose" > /dev/null; then
    ${SUDO} curl -SL https://github.com/docker/compose/releases/download/v2.3.3/docker-compose-linux-x86_64 -o /usr/local/bin/docker-compose
    ${SUDO} chmod +x /usr/local/bin/docker-compose
else
    echo 'docker-compose already installed'
fi
