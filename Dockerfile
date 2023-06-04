FROM debian:bullseye-slim
WORKDIR /app
COPY bin/bot /usr/local/bin
# https://stackoverflow.com/a/70096420
COPY LICENSE.md tabl[e].csv .
RUN chmod u+x /usr/local/bin/bot
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates
ENTRYPOINT bot
