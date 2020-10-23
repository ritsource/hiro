FROM rust:1.43.1 as base
WORKDIR /usr/src/hiro
COPY . .
RUN cargo build

FROM base as master
RUN chmod +x ./target/debug/hiro
CMD [ "./target/debug/hiro", "--master" ]

FROM base as worker
RUN chmod +x ./target/debug/hiro
CMD [ "./target/debug/hiro", "--worker" ]
