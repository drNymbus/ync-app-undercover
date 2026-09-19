# --- Build stage: compile the Iced UI to WebAssembly ---
FROM docker.io/library/rust:1-bookworm AS builder

RUN rustup target add wasm32-unknown-unknown \
    && cargo install --locked trunk

WORKDIR /app

# Copy manifests first so dependency/tooling layers cache across source-only edits
COPY Cargo.toml Cargo.lock ./
COPY index.html ./index.html
COPY src ./src

RUN trunk build --release

# --- Runtime stage: serve the static build ---
FROM docker.io/library/httpd:2.4

RUN sed -i 's/^Listen 80$/Listen 8080/' /usr/local/apache2/conf/httpd.conf \
    && echo 'AddType application/wasm .wasm' >> /usr/local/apache2/conf/httpd.conf

COPY --from=builder /app/dist/ /usr/local/apache2/htdocs/
