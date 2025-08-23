# 1. 빌드 단계
FROM rust:latest as builder

# 작업 디렉토리
WORKDIR /usr/src/app

# Cargo.toml과 Cargo.lock만 복사 → 의존성 캐시 최적화
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# 실제 소스 복사 & 빌드
COPY . .
RUN cargo build --release

# 2. 실행 단계 (슬림 베이스)
FROM debian:bookworm-slim

# 실행에 필요한 라이브러리
RUN apt-get update && apt-get install -y \
    libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/pdm_back .

# 기본 실행 (바이너리 이름 맞게 수정)
CMD ["./pdm_back"]
