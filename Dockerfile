FROM rust:1.74

WORKDIR /usr/src/
COPY . .

RUN cargo install --profile release --locked --path .
RUN ls -la

EXPOSE 8000

CMD ["gyropractor"]
