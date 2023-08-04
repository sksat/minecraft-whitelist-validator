FROM gcr.io/distroless/cc
LABEL maintainer "sksat <sksat@sksat.net>"

FROM rust:1.71.1 as chef
RUN cargo install --version 0.1.33 cargo-chef
WORKDIR /build

# get package name
FROM chef as metadata
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
RUN apt-get update -y && apt-get install -y jq
ADD . .
RUN cargo metadata --format-version=1 | jq --raw-output '.workspace_members[0]' | cut -d' ' -f 1 > app_name

FROM chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

# build
FROM chef as builder
COPY --from=planner /build/recipe.json recipe.json
# build deps(cached)
RUN cargo chef cook --release --recipe-path recipe.json
# build bin
COPY . .
RUN cargo build --release

# change binary name to /app/bin
FROM alpine as tmp
WORKDIR /app
COPY --from=metadata /build/app_name /tmp
COPY --from=builder /build/target/release /build
RUN cp /build/$(cat /tmp/app_name) bin

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=tmp /app/bin .
CMD ["/app/bin"]
