#!/bin/bash

echo "=== OKX API 持仓查询工具演示 ==="
echo ""

# 检查环境变量
if [ ! -f ".env" ]; then
    echo "❌ 错误: 未找到 .env 文件"
    echo "请先复制 env.example 为 .env 并配置你的API密钥"
    exit 1
fi

echo "✅ 环境配置检查通过"
echo ""

# 编译项目
echo "🔨 编译项目..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ 编译失败"
    exit 1
fi
echo "✅ 编译成功"
echo ""

# 显示帮助信息
echo "📖 显示帮助信息:"
echo "----------------------------------------"
cargo run -- --help
echo ""

# 显示positions子命令帮助
echo "📖 显示当前持仓查询帮助:"
echo "----------------------------------------"
cargo run -- positions --help
echo ""

# 显示history子命令帮助
echo "📖 显示历史持仓查询帮助:"
echo "----------------------------------------"
cargo run -- history --help
echo ""

echo "=== 演示完成 ==="
echo ""
echo "使用示例:"
echo "  查询所有当前持仓: cargo run -- positions"
echo "  查询SWAP产品当前持仓: cargo run -- positions --inst-type SWAP"
echo "  查询所有历史持仓: cargo run -- history"
echo "  查询SWAP产品历史持仓: cargo run -- history --inst-type SWAP"
echo ""
echo "注意: 请确保你的API密钥已正确配置在 .env 文件中" 