ARG PROFILE=debug

FROM lukemathwalker/cargo-chef:latest-rust-1.60.0 AS chef
WORKDIR /app

#############
#  Prepare  #
#############

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

###########
#  Build  #
###########

FROM chef AS builder 
ARG PROFILE
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN if [ "$PROFILE" = "release" ]; then \
	cargo chef cook --release --recipe-path recipe.json; \
	else \
	cargo chef cook --recipe-path recipe.json; \
	fi

# Build application
COPY . .
# Build dependencies - this is the caching Docker layer!
RUN if [ "$PROFILE" = "release" ]; then \
	cargo build --release;  \
	else \
	cargo build; \
	fi

# We do not need the Rust toolchain to run the binary!
# FROM debian:buster-slim AS runtime
# WORKDIR /app
# COPY --from=builder /app/target/release/first_aid_bot_rust /usr/local/bin
ENTRYPOINT if [ "$PROFILE" = "release" ]; then \
	cargo run --release;  \
	else \
	cargo run; \
	fi
