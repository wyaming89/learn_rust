# 使用官方Rust镜像作为基础镜像
FROM rust:1.75 as builder

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制Cargo.toml和Cargo.lock
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src

# 构建项目
RUN cargo build --release

# 创建运行时镜像
FROM debian:bullseye-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 从构建镜像复制二进制文件
COPY --from=builder /app/target/release/okx-api-client /usr/local/bin/

# 设置工作目录
WORKDIR /app

# 复制环境变量示例文件
COPY env.example ./

# 设置入口点
ENTRYPOINT ["okx-api-client"] 