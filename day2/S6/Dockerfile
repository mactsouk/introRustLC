FROM rust:latest
WORKDIR /usr/src/app

COPY hw.rs .
RUN rustc hw.rs

CMD ["/usr/src/app/hw"]
