# ------------------------------
# Stage 1. Build an app
# ------------------------------
FROM rust:1.96.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# ------------------------------
# Stage 2. Build for runtime
# ------------------------------
FROM dhi.io/debian-base:trixie

ARG GIT_REVISION
ARG BUILD_DATE
ARG VERSION

LABEL org.opencontainers.image.title="squirrelt" \
      org.opencontainers.image.description="lsの再開発,拡張子の種類によって色を変え，ソート順をサイズ，最終更新日，ファイル名などで選択できる" \
      org.opencontainers.image.url="https://nomurakids.github.io/squirrelt" \
      org.opencontainers.image.source="https://github.com/nomuraKIDS/squirrelt" \
      org.opencontainers.image.version=${VERSION} \
      org.opencontainers.image.revision=${GIT_REVISION} \
      org.opencontainers.image.created=${BUILD_DATE} \
      org.opencontainers.image.licenses="CC0-1.0"

COPY --from=builder /app/target/release/squirrelt /opt/squirrelt
WORKDIR /opt
ENTRYPOINT [ "/opt/squirrelt" ]
