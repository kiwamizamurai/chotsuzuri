FROM --platform=$BUILDPLATFORM rust:1.76

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# bug:  https://github.com/yewstack/yew/issues/2716
RUN cargo install --locked trunk@0.15.0 && \
    cargo install --locked wasm-bindgen-cli && \
    rustup target add wasm32-unknown-unknown

ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

EXPOSE 3000

WORKDIR /app

CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "3000"]