FROM debian:buster-slim AS runtime
WORKDIR /app
COPY first_aid_bot /usr/local/bin
# RUN apt-get update \
#   && apt-get install -y --no-install-recommends ca-certificates
# RUN update-ca-certificates
ENTRYPOINT first_aid_bot
