FROM rust:alpine as rust-build
WORKDIR /home/app

RUN apk add git curl &&\
      apk add pkgconfig openssl-dev gcc musl-dev rustup &&\
      rm -rf /var/cache/apk/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo 'fn main() {println!("hello world")}' > src/main.rs

RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release


RUN rm -rf src

COPY ./src /home/app/src

RUN rm -rf ./target/release/incremental/user* && \ 
      rm -rf ./target/release/deps/user*
      
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build --release

FROM alpine
WORKDIR /home/app


COPY --from=rust-build /home/app/target/release/user .
CMD ["./user"]
