# Build all apps
COPY crates ./crates

# Создаем файл с уникальным ID коммита, чтобы Docker ВСЕГДА пересобирал этот слой
ARG GITHUB_SHA=unknown
RUN echo "Building commit: $GITHUB_SHA" > /build_id.txt

# Твоя кувалда (оставляем для верности)
RUN find crates -type f -name "*.rs" -exec touch {} +

RUN sh /tmp/build-image-layer.sh apps