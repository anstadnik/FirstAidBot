# FROM lukemathwalker/cargo-chef:latest-rust-1.63.0 AS chef
# WORKDIR /app
#
# #############
# #  Prepare  #
# #############
#
# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json
#
# ###########
# #  Build  #
# ###########
#
# FROM chef AS builder 
# ARG RELEASE
# COPY --from=planner /app/recipe.json recipe.json
#
# # Build dependencies - this is the caching Docker layer!
# RUN \
#   cargo chef cook --release --recipe-path recipe.json; \
#   cargo chef cook --recipe-path recipe.json; 
#
# # Build application
# COPY . .
# RUN if [[ -n "$RELEASE" ]] ; then \
#   cargo build --release; \
#   else \
#   cargo build; \
#   fi
#
# # We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR /app
COPY first_aid_bot /usr/local/bin
# RUN apt-get update \
#   && apt-get install -y --no-install-recommends ca-certificates
# RUN update-ca-certificates
ENTRYPOINT first_aid_bot_rust
