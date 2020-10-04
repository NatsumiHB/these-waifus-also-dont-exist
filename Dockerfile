FROM rust:1.46-slim

WORKDIR /srv/these-waifus-also-dont-exist
COPY . .

RUN cargo install --path .

EXPOSE 5002
CMD ["these-waifus-also-dont-exist"]