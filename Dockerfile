FROM rust:slim AS build

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM alpine:latest

COPY --from=build /usr/src/app/target/release/sum /usr/local/bin/sum

CMD ["sum"]