FROM rust:1.90.0-bookworm

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN rustup component add rustfmt clippy
RUN cargo install wasm-pack
RUN cargo install cargo-lambda

# Install AWS CLI and utilities
RUN apt-get update && \
    apt-get install -y unzip curl less && \
    cd /tmp && \
    curl "https://awscli.amazonaws.com/awscli-exe-linux-aarch64.zip" -o "awscliv2.zip" && \
    unzip -q awscliv2.zip && \
    ./aws/install && \
    rm -rf aws awscliv2.zip && \
    rm -rf /var/lib/apt/lists/*

# Install Zig for cargo-lambda
RUN cd /tmp && \
    wget -q https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz && \
    tar -xf zig-linux-x86_64-0.13.0.tar.xz && \
    mv zig-linux-x86_64-0.13.0 /usr/local/zig && \
    rm zig-linux-x86_64-0.13.0.tar.xz && \
    ln -s /usr/local/zig/zig /usr/local/bin/zig
