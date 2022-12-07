FROM rust:1.65.0-slim-bullseye

RUN mkdir /root/project
WORKDIR /opt/

COPY openssl-1.0.2u-dist.tar /opt/
RUN tar -xvf /opt/openssl-1.0.2u-dist.tar && \
    rm -rf /opt/openssl-1.0.2u-dist.tar && \
    cp /opt/openssl-1.0.2u-dist/lib/libssl.so.1.0.0 /usr/lib/ && \
    cp /opt/openssl-1.0.2u-dist/lib/libcrypto.so.1.0.0 /usr/lib/


ENV OPENSSL_INCLUDE_DIR="/opt/openssl-1.0.2u-dist/include"
ENV OPENSSL_LIB_DIR="/opt/openssl-1.0.2u-dist/lib"
# /usr/local/cargo/registry
WORKDIR /root/project