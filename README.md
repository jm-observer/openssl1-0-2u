# openssl1-0-2u

1. build image
> docker build -t openssl1-0-2u:1.65.0 .
2. install cargo-make
> cargo install cargo-make
3. run ntex-connector
> cargo make build-window-openssl
4. run ntex-connector
> cargo make build-window-ntex-tls