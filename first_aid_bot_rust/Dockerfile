ARG PROFILE

FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN if [ "$PROFILE" = "release" ]; then \
	cargo chef cook --release --recipe-path recipe.json; \
	else \
	cargo chef cook --recipe-path recipe.json; \
	fi

ENV PROFILE $PROFILE
# Build application
COPY . .
RUN cargo build

# We do not need the Rust toolchain to run the binary!
# FROM debian:buster-slim AS runtime
# WORKDIR /app
# COPY --from=builder /app/target/release/first_aid_bot_rust /usr/local/bin
ENTRYPOINT ["cargo run"]
