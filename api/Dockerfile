FROM rust:latest

RUN apt-get update && apt-get install libpq-dev musl-tools -y

WORKDIR /usr/src/api

COPY . .

RUN cargo install --path .

EXPOSE 32112

CMD ["cargo", "run"]