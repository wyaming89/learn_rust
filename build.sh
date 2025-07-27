#!/bin/bash

echo "=== OKX API Client 构建脚本 ==="
echo ""

# 检测操作系统
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "检测到Linux系统，检查OpenSSL依赖..."
    
    # 检查是否安装了OpenSSL开发库
    if ! pkg-config --exists openssl; then
        echo "❌ 未找到OpenSSL开发库"
        echo ""
        echo "请安装OpenSSL开发库:"
        echo ""
        echo "Ubuntu/Debian:"
        echo "  sudo apt update && sudo apt install pkg-config libssl-dev"
        echo ""
        echo "CentOS/RHEL:"
        echo "  sudo yum install pkg-config openssl-devel"
        echo ""
        echo "Fedora:"
        echo "  sudo dnf install pkg-config openssl-devel"
        echo ""
        echo "Arch Linux:"
        echo "  sudo pacman -S pkg-config openssl"
        echo ""
        exit 1
    else
        echo "✅ OpenSSL开发库已安装"
    fi
fi

# 检查Rust是否安装
if ! command -v cargo &> /dev/null; then
    echo "❌ 未找到Rust/Cargo"
    echo "请先安装Rust: https://rustup.rs/"
    exit 1
fi

echo "✅ Rust已安装"
echo ""

# 构建项目
echo "🔨 开始构建项目..."
if cargo build --release; then
    echo "✅ 构建成功！"
    echo ""
    echo "二进制文件位置: target/release/okx-api-client"
    echo ""
    echo "使用方法:"
    echo "  ./target/release/okx-api-client --help"
    echo "  ./target/release/okx-api-client positions --simple"
else
    echo "❌ 构建失败"
    exit 1
fi 