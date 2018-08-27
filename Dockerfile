FROM rust:latest
MAINTAINER Julius de Bruijn <julius@nauk.io>

WORKDIR /usr/src
ENV USER root
ENV RUST_BACKTRACE 1

RUN apt-get -y update

RUN mkdir -p /usr/src/t1-dashboard
COPY Cargo.toml Cargo.lock /usr/src/t1-dashboard/
COPY src /usr/src/t1-dashboard/src

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/t1-dashboard
RUN cargo build --release
RUN mv target/release/t1-dashboard /bin
RUN chmod a+x /bin/t1-dashboard
run rm -rf /usr/src/t1-dashboard

WORKDIR /

CMD "/bin/t1-dashboard"
