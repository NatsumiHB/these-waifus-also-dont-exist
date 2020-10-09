FROM rust:1.47-alpine AS build

WORKDIR /srv/these-waifus-also-dont-exist
COPY . .

RUN apk add --no-cache musl-dev

RUN cargo install --path .

FROM alpine:latest

WORKDIR /srv/these-waifus-also-dont-exist
COPY --from=build /usr/local/cargo/bin/these-waifus-also-dont-exist .

EXPOSE 5001
CMD ./these-waifus-also-dont-exist