FROM rust:latest AS planner
WORKDIR backend
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:latest AS cacher
WORKDIR backend
RUN cargo install cargo-chef
COPY --from=planner /backend/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:latest AS builder
WORKDIR backend
COPY . .
COPY --from=cacher /backend/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin old_stars_api
RUN cargo build --release --bin configurator

FROM rust:latest AS runtime
WORKDIR /backend
COPY ./.env /usr/local/bin
COPY ./wait_for_it.sh .
COPY --from=builder /backend/target/release/old_stars_api /usr/local/bin
COPY --from=builder /backend/target/release/configurator /usr/local/bin
#ENTRYPOINT ["/usr/local/bin/backend"]
#CMD ["bash"]

