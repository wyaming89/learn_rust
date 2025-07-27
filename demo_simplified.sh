#!/bin/bash

echo "=== OKX API 持仓查询工具 - 简化输出演示 ==="
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
echo "📖 显示简化输出帮助:"
echo "----------------------------------------"
cargo run -- positions --help
echo ""

echo "=== 演示简化输出功能 ==="
echo ""

# 1. JSON格式简化输出
echo "1️⃣ JSON格式简化输出:"
echo "----------------------------------------"
cargo run -- positions --simple
echo ""

# 2. 表格格式简化输出
echo "2️⃣ 表格格式简化输出:"
echo "----------------------------------------"
cargo run -- positions --simple --format table
echo ""

# 3. 完整输出对比
echo "3️⃣ 完整输出格式 (对比):"
echo "----------------------------------------"
cargo run -- positions | head -20
echo "..."

echo ""
echo "=== 演示完成 ==="
echo ""
echo "使用说明:"
echo "  JSON格式简化输出: cargo run -- positions --simple"
echo "  表格格式简化输出: cargo run -- positions --simple --format table"
echo "  完整输出格式:     cargo run -- positions"
echo ""
echo "简化输出字段说明:"
echo "  Pair      - 交易对名称"
echo "  Side      - 持仓方向 (long/short)"
echo "  Pos       - 持仓数量"
echo "  AvgPx     - 开仓平均价"
echo "  MarkPx    - 标记价格"
echo "  UPL       - 未实现盈亏"
echo "  UPLRatio  - 未实现盈亏比例"
echo ""
echo "注意: 请确保你的API密钥已正确配置在 .env 文件中" 