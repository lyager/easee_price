FROM rust:slim-buster as builder
WORKDIR /usr/src/easee_cost_post_rs
RUN apt-get update && apt-get install -y libssl-dev pkg-config
COPY . .
RUN cargo install --path .
FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/easee_cost_post_rs /usr/local/bin/easee_cost_post_rs
CMD ["easee_cost_post_rs"]
