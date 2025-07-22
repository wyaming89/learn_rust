# OKX API 历史持仓查询工具

## 项目概述

这是一个基于 Rust 开发的 OKX API 客户端工具，专门用于查询历史持仓信息。工具支持多种查询参数，包括产品类型、交易对、时间范围等，并实现了完整的 API 认证和限速机制。

## 主要功能

### ✅ 已实现功能

1. **基本查询功能**
   - 查询最近的历史持仓记录
   - 支持分页查询（最大100条记录）
   - 按持仓更新时间倒序排列

2. **高级查询参数**
   - 产品类型过滤（MARGIN, SWAP, FUTURES, OPTION）
   - 交易产品ID过滤（如：ETH-USDT-SWAP）
   - 保证金模式过滤（cross, isolated）
   - 时间范围查询（before, after）
   - 持仓ID查询
   - 平仓类型过滤

3. **API 认证与安全**
   - HMAC-SHA256 签名生成
   - ISO 8601 格式时间戳
   - 完整的请求头设置
   - 支持沙盒环境

4. **限速机制**
   - 滑动窗口限速器
   - 10请求/2秒的API限制
   - 按用户ID限速

5. **错误处理**
   - 详细的错误信息
   - HTTP状态码处理
   - API错误码解析

### 🔧 技术特性

- **语言**: Rust 1.74.0+
- **异步支持**: Tokio 运行时
- **HTTP客户端**: reqwest
- **序列化**: serde_json
- **命令行**: clap
- **错误处理**: anyhow
- **加密**: hmac, sha2, base64
- **时间处理**: chrono

## 项目结构

```
client_api/
├── src/
│   ├── main.rs              # 主程序入口
│   ├── config.rs            # 配置管理
│   ├── types.rs             # 数据类型定义
│   ├── rate_limiter.rs      # 限速器实现
│   └── positions_history.rs # API客户端核心
├── Cargo.toml               # 项目依赖
├── .env                     # 环境变量配置
├── demo.sh                  # 演示脚本
├── USAGE.md                 # 使用说明
└── PROJECT_SUMMARY.md       # 项目总结
```

## 核心模块说明

### 1. 配置管理 (`config.rs`)
- 从环境变量加载API密钥
- 支持沙盒和生产环境
- 统一的配置结构

### 2. 数据类型 (`types.rs`)
- API响应结构定义
- 历史持仓数据结构
- 完整的字段映射

### 3. 限速器 (`rate_limiter.rs`)
- 线程安全的滑动窗口限速
- 支持多用户并发
- 自动等待机制

### 4. API客户端 (`positions_history.rs`)
- 完整的API调用实现
- HMAC-SHA256签名生成
- 查询参数构建
- 响应解析

## 使用方法

### 1. 环境配置

创建 `.env` 文件：
```bash
OKX_API_KEY=your_api_key
OKX_SECRET_KEY=your_secret_key
OKX_PASSPHRASE=your_passphrase
OKX_BASE_URL=https://www.okx.com
```

### 2. 编译项目

```bash
cargo build --release
```

### 3. 基本使用

```bash
# 查询最近10条记录
./target/release/okx-api-client -l 10

# 查询SWAP类型持仓
./target/release/okx-api-client -t SWAP -l 5

# 查询特定交易对
./target/release/okx-api-client -i "ETH-USDT-SWAP" -l 3

# 时间范围查询
./target/release/okx-api-client -b 1752800000000 -l 5
```

### 4. 运行演示

```bash
./demo.sh
```

## API 响应示例

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "instType": "SWAP",
      "instId": "ETH-USDT-SWAP",
      "mgnMode": "isolated",
      "posId": "2606171386593583104",
      "posSide": "long",
      "openAvgPx": "3234.45",
      "closeAvgPx": "3285",
      "realizedPnl": "4.7951354",
      "type": "2",
      "cTime": "1752676985653",
      "uTime": "1752683977587",
      "lever": "10.0",
      "fee": "-0.2274225",
      "fundingFee": "-0.0324421",
      "direction": "long",
      "ccy": "USDT",
      "closeTotalPos": "1",
      "openMaxPos": "1",
      "pnl": "5.055",
      "pnlRatio": "0.1482519562831393",
      "liqPenalty": "0",
      "nonSettleAvgPx": "",
      "settledPnl": "",
      "triggerPx": "",
      "uly": "ETH-USDT"
    }
  ]
}
```

## 技术亮点

### 1. 时间戳处理
- 使用ISO 8601格式时间戳
- 正确处理UTC时区
- 避免时间戳过期问题

### 2. 签名算法
- 完整的HMAC-SHA256实现
- 正确的参数排序
- Base64编码输出

### 3. 限速机制
- 线程安全的滑动窗口
- 精确的时间控制
- 自动等待和重试

### 4. 错误处理
- 详细的错误信息
- 优雅的错误恢复
- 用户友好的提示

## 性能特点

- **编译优化**: Release模式编译
- **异步处理**: 非阻塞I/O操作
- **内存安全**: Rust的所有权系统
- **并发安全**: 线程安全的限速器

## 兼容性

- **Rust版本**: 1.74.0+
- **操作系统**: Linux, macOS, Windows
- **网络**: 支持代理设置
- **API版本**: OKX API v5

## 安全考虑

- API密钥通过环境变量管理
- 不在代码中硬编码敏感信息
- 支持沙盒环境测试
- 完整的请求签名验证

## 未来改进

1. **功能扩展**
   - 支持更多API端点
   - 添加WebSocket支持
   - 实现数据导出功能

2. **性能优化**
   - 连接池优化
   - 缓存机制
   - 批量查询支持

3. **用户体验**
   - 交互式命令行界面
   - 配置文件支持
   - 日志系统

4. **监控和调试**
   - 详细的日志记录
   - 性能监控
   - 错误追踪

## 总结

这个OKX API客户端工具成功实现了历史持仓查询的核心功能，具备完整的API认证、限速控制和错误处理机制。工具采用Rust语言开发，保证了性能和安全性，同时提供了友好的命令行界面和详细的文档说明。

项目代码结构清晰，模块化程度高，易于维护和扩展。通过演示脚本和文档，用户可以快速上手使用各种查询功能。 