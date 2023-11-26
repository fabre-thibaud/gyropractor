# Fat image with Rust to build project
FROM rust:1.74-alpine as builder

WORKDIR /usr/src/
COPY . .

RUN apk add musl-dev
RUN cargo install --profile release --locked --path .

# Final slim image
FROM alpine:3.18.4

COPY ./res/ /usr/local/bin/res
COPY --from=builder /usr/local/cargo/bin/gyropractor /usr/local/bin/gyropractor

WORKDIR /usr/local/bin/

EXPOSE 8000

CMD ["gyropractor"]
