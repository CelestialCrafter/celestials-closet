FROM rust:alpine as builder

RUN apk add alpine-sdk

WORKDIR /app
COPY . .
RUN cargo install --path .
CMD ["celestials-closet"]
