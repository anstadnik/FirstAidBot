FROM rust:1.59
ARG PROFILE
ENV PROFILE $PROFILE
COPY ./ ./
RUN cargo build
CMD ["./target/release/first_aid_bot_rust"]
