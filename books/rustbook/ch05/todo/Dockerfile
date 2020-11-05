# ビルド用のイメージを用意
FROM rust:1.43 AS builder

WORKDIR /todo

COPY Cargo.toml Cargo.toml

# ビルドするために何もしないソースコードを入れておく。
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs

# 上記で作成したコードと依存クレートをビルドする。
RUN cargo build --release

# アプリケーションのコードをイメージにコピーする。
COPY ./src ./src
COPY ./templates ./templates

# 先程ビルドした生成物のうち、アプリケーションのもののみを削除する。
RUN rm -f target/release/deps/todo*

# 改めてアプリケーションをビルドする。
RUN cargo build --release

# 新しくリリース用のイメージを用意
FROM debian:10.4

# builder イメージから todo のみをコピーして /usr/local/bin に配置する
COPY --from=builder /todo/target/release/todo /usr/local/bin/todo
CMD ["todo"]
