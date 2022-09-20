# FROM rust:1.63-slim-buster

# RUN apt-get update \
#  && apt-get install -y \
#       apt-transport-https \
#       build-essential \
#       ca-certificates libssl-dev pkg-config \
#       make

# # Empty shell project
# RUN USER=root cargo new --bin backend
# WORKDIR /backend

# # Copy manifests
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./backend/Cargo.toml ./Cargo.toml

# # Build only the dependencies to cache them
# RUN mkdir -p src/bin
# RUN mv src/main.rs src/bin/main.rs

# RUN cargo build --release
# RUN rm -rf src/*.rs
# RUN rm -rf src/bin/main.rs

# # Copy the source code
# COPY ./backend/src ./src

# # Build for release
# RUN rm -rf ./target/release/deps/backend_lib*
# RUN rm -rf ./target/release/deps/backend_bin*
# RUN cargo install --path .

# Load the frontend code
FROM node:16
WORKDIR /frontend

# We need to compile our code from TypeScript to JavaScript
COPY ./frontend .

# Install app dependencies
RUN npm ci --force

# Build the entire project
RUN npm run build

FROM debian:buster-slim
WORKDIR /backend

RUN apt-get update && apt-get install -y gnupg wget glibc

# Download my GPG keys from the server
RUN gpg --keyserver keyserver.ubuntu.com --recv-keys 80dcc4468de6f8c9

# Download binary releases from GitHub Releases
RUN wget "https://github.com/memothelemo/web-app/releases/download/v0.1.0/backend-bin"
RUN wget "https://github.com/memothelemo/web-app/releases/download/v0.1.0/backend-bin.gpg"

# Verification
RUN gpg --verify backend-bin.gpg
RUN chmod 577 ./backend-bin
EXPOSE 8000

# To avoid problems when loading static files later on
CMD ["./backend-bin"]
