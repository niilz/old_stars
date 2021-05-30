FROM rustlang/rust:nightly AS planner
WORKDIR backend
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rustlang/rust:nightly AS cacher
WORKDIR backend
RUN cargo install cargo-chef
COPY --from=planner /backend/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rustlang/rust:nightly AS builder
WORKDIR backend
COPY . .
COPY --from=cacher /backend/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin backend

FROM rustlang/rust:nightly AS runtime
WORKDIR /backend
COPY --from=builder /backend/target/release/backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/backend"]
