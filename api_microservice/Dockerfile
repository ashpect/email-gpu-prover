FROM ubuntu:22.04

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    curl \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /root

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH "/root/.cargo/bin:${PATH}"

WORKDIR /api_microservice

# Copy Rust source code and build the project
COPY Cargo.* ./
COPY src ./src
RUN cargo build

# Set the default command to open a bash shell
CMD ["cargo", "run"]
