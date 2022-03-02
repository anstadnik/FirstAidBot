FROM rust:1.59

ENV TELOXIDE_TOKEN=5102517984:AAEcP7Exnth-GvEfA2wDH7NEmrzaXCUQ3dU

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/first_aid_bot_rust"]
