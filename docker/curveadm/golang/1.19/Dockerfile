# Dockerfile - Debian 10 Buster - DEB version


FROM golang:1.19-buster

LABEL maintainer="Wine93 <wine93.info@gmail.com>"

ENV TZ=Asia/Shanghai \
    LANG=C.UTF-8     \
    LC_ALL=C.UTF-8

ARG DEBIAN_FRONTEND=noninteractive
COPY setup.sh .setup.sh
RUN bash .setup.sh
