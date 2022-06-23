FROM rustlang/rust:stable AS planner
WORKDIR backend
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rustlang/rust:stable AS cacher
WORKDIR backend
RUN cargo install cargo-chef
COPY --from=planner /backend/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rustlang/rust:stable AS builder
WORKDIR backend
COPY . .
COPY --from=cacher /backend/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin backend

FROM rustlang/rust:stable AS runtime
WORKDIR /backend
COPY ./.env /usr/local/bin
COPY ./wait_for_it.sh .
COPY --from=builder /backend/target/release/backend /usr/local/bin
#ENTRYPOINT ["/usr/local/bin/backend"]
#CMD ["bash"]

