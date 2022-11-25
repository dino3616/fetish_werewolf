FROM rust:bullseye

WORKDIR /fetish_werewolf

# hadolint ignore=DL3008
RUN apt-get update \
  && apt-get --no-install-recommends -y install git gnupg2 ca-certificates \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists \
  && cargo install cargo-edit cargo-watch cargo-build-deps \
  && rustup component add rust-analysis rust-src rustfmt clippy \
  && cargo init

COPY ./Cargo.lock ./Cargo.toml /fetish_werewolf/
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build-deps

COPY ./src /fetish_werewolf/src
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build
