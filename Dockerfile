# Build Stage
FROM --platform="${BUILDPLATFORM}" rust:1.92.0-slim-bookworm AS builder
USER 0:0
WORKDIR /home/rust/src

ARG TARGETARCH

# Install build requirements
RUN dpkg --add-architecture "${TARGETARCH}"
RUN apt-get update && \
    apt-get install -y \
    make \
    pkg-config \
    libssl-dev:"${TARGETARCH}"

# Копируем ВООБЩЕ ВСЁ (и конфиги, и весь код сразу)
COPY . .

# Собираем всё одной честной командой (БЕЗ ВСЯКИХ STUB)
RUN cargo build --release --locked