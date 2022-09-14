# Dockerfile - Debian 10 Buster - DEB version

FROM golang:1.19-buster

LABEL maintainer="Wine93 <wine93.info@gmail.com>"

RUN DEBIAN_FRONTEND=noninteractive apt-get clean \
    && DEBIAN_FRONTEND=noninteractive apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        musl \
        musl-dev \
        musl-tools \
    && DEBIAN_FRONTEND=noninteractive apt-get autoremove -y
