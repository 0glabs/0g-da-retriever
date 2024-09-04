FROM rust:alpine3.20 as builder

WORKDIR /
COPY . 0g-da-retriever
WORKDIR /0g-da-retriever
RUN apk update && apk add --no-cache make protobuf-dev && apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine:3.20

COPY --from=builder /0g-da-retriever/target/release/retriever /usr/local/bin/retriever

CMD ["retriever"]
