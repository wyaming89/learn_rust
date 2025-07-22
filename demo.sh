#!/bin/bash

# OKX API 客户端演示脚本
echo "=== OKX API 历史持仓查询工具演示 ==="
echo

# 确保环境变量已加载
if [ ! -f .env ]; then
    echo "错误: 找不到 .env 文件，请先配置API密钥"
    exit 1
fi

source .env

# 检查API密钥是否配置
if [ -z "$OKX_API_KEY" ] || [ -z "$OKX_SECRET_KEY" ] || [ -z "$OKX_PASSPHRASE" ]; then
    echo "错误: API密钥未正确配置，请检查 .env 文件"
    exit 1
fi

echo "✅ API密钥已配置"
echo

# 演示1: 基本查询（最近5条记录）
echo "📊 演示1: 查询最近5条历史持仓记录"
./target/release/okx-api-client -l 5
echo

# 演示2: 按产品类型查询
echo "📊 演示2: 查询SWAP类型的历史持仓"
./target/release/okx-api-client -t SWAP -l 3
echo

# 演示3: 按具体产品查询
echo "📊 演示3: 查询ETH-USDT-SWAP的历史持仓"
./target/release/okx-api-client -i "ETH-USDT-SWAP" -l 2
echo

# 演示4: 按时间范围查询
echo "📊 演示4: 查询指定时间之前的历史持仓"
./target/release/okx-api-client -b 1752800000000 -l 2
echo

# 演示5: 按保证金模式查询
echo "📊 演示5: 查询隔离保证金模式的历史持仓"
./target/release/okx-api-client -m isolated -l 2
echo

# 演示6: 组合查询
echo "📊 演示6: 组合查询 - SWAP类型，隔离保证金，限制2条"
./target/release/okx-api-client -t SWAP -m isolated -l 2
echo

echo "=== 演示完成 ==="
echo "更多用法请运行: ./target/release/okx-api-client --help" 