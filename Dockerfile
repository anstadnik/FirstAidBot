FROM debian:buster-slim AS runtime
WORKDIR /app
COPY first_aid_bot /usr/local/bin
COPY table.csv .
RUN chmod u+x /usr/local/bin/first_aid_bot
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates
ENTRYPOINT first_aid_bot
