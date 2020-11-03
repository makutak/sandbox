FROM rust:1.43

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

RUN cargo install --path .
CMD ["todo"]
