# Rust as the base image
FROM rust:1.66-slim-buster as builder

ENV project=gamejam

### SERVER STUFF ###

# Required for sqlx-cli
RUN apt update
RUN apt install -y pkg-config libssl-dev openssl

RUN cargo install sqlx-cli

WORKDIR /usr/src/${project}/ae-position

# Copy Cargo files
COPY ./ae-position/Cargo.toml .
COPY ./ae-position/Cargo.lock .

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs

WORKDIR /usr/src/${project}/ae-direction

# Copy Cargo files
COPY ./ae-direction/Cargo.toml .
COPY ./ae-direction/Cargo.lock .

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs

# FROM rust as rust-builder
WORKDIR /usr/src/${project}/

# Copy Cargo files
COPY ./Cargo.toml .
COPY ./Cargo.lock .

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs

# Create a dummy release build that builds all the app's real dependencies
RUN cargo build --release

# Remove dummy files
RUN rm ./**/*.rs

# Copy actual source code
COPY ./src ./src
COPY ./ae-direction/src ./ae-direction/src
COPY ./ae-position/src ./ae-position/src

# Copy additional relevant fies
COPY ./.env ./.env
COPY ./migrations ./migrations

# Create initial database
RUN sqlx database create
RUN sqlx migrate run

# Remove the dummy build
RUN rm ./target/release/deps/${project}*

# Build the real app without recompiling its dependencies
RUN cargo build --release

### CLIENT STUFF ###

SHELL [ "/bin/bash", "-l", "-c" ]

# Install NVM
RUN apt install -y curl
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash

# Install Node (to build the Vite app)
RUN nvm install 18
RUN nvm use 18

# Copy package files to build and cache client dependencies
COPY ./client/package.json ./client/package.json
COPY ./client/package-lock.json ./client/package-lock.json

WORKDIR /usr/src/${project}/client

# Install client dependencies
RUN npm install

COPY ./client .

# Do a production build of the Vite client
RUN npm run build

WORKDIR /usr/src/${project}/

# Start a fresh image that will only contain the compiled code and
# leave behind all the intermediary stuff we added that we don't need anymore
FROM rust:1.66-slim-buster

ENV project=gamejam

WORKDIR /usr/src/${project}/

# Copy the server build artifact from the build stage
COPY --from=builder /usr/src/${project}/target/release/${project} .

# Copy the production build of the Vite app
COPY --from=builder /usr/src/${project}/client/dist ./client/dist

EXPOSE 8080

# Run the game server (which also serves the client app in a static directory)
CMD ["./gamejam"]