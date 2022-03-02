FROM rust:1.59

ENV TELOXIDE_TOKEN=Миші з'їли

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/first_aid_bot_rust"]
