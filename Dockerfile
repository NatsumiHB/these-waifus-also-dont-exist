FROM rust:1.45.2

WORKDIR /srv/these-waifus-also-dont-exist
COPY . .

RUN cargo install --path .

CMD ["these-waifus-also-dont-exist"]