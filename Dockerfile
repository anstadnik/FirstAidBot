FROM debian:bullseye-slim
WORKDIR /app
COPY first_aid_bot /usr/local/bin
# https://stackoverflow.com/a/70096420
COPY tabl[e].csv .
RUN chmod u+x /usr/local/bin/first_aid_bot
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates
ENTRYPOINT first_aid_bot
