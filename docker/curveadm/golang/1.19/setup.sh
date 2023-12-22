#!/usr/bin/env bash

apt-get clean \
&& apt-get update \
&& apt-get install -y --no-install-recommends \
    musl \
    musl-dev \
    musl-tools \
&& apt-get autoremove -y
