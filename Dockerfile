FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/shlonk /usr/local/bin/shlonk
EXPOSE 8080
CMD ["shlonk"]
