# OKX API 历史持仓查询工具

这是一个用Rust编写的OKX API调用工具，用于查询历史持仓信息。

## 功能特性

- 查询最近3个月有更新的仓位信息
- 按照仓位更新时间倒序排列
- 支持组合保证金账户模式下的历史持仓查询
- 内置限速功能（10次/2秒）
- 支持多种查询参数过滤

## 安装和配置

### 1. 克隆项目
```bash
git clone <repository-url>
cd okx-api-client
```

### 2. 配置环境变量
复制环境变量示例文件并填写你的API密钥：
```bash
cp env.example .env
```

编辑 `.env` 文件，填入你的OKX API配置：
```
OKX_API_KEY=your_api_key_here
OKX_SECRET_KEY=your_secret_key_here
OKX_PASSPHRASE=your_passphrase_here
OKX_SANDBOX=false
RUST_LOG=info
```

### 3. 编译项目
```bash
cargo build --release
```

## 使用方法

### 基本查询
```bash
# 查询所有历史持仓
cargo run

# 查询指定产品类型的历史持仓
cargo run -- --inst-type SWAP

# 查询指定交易对的历史持仓
cargo run -- --inst-id BTC-USD-SWAP

# 查询指定保证金模式的历史持仓
cargo run -- --mgn-mode cross
```

### 高级查询
```bash
# 查询最近3个月的历史持仓，限制返回50条
cargo run -- --limit 50

# 查询指定时间范围的历史持仓
cargo run -- --after 1640995200000 --before 1643673600000

# 查询指定平仓类型的历史持仓
cargo run -- --close-type 2  # 2表示完全平仓

# 查询指定持仓ID的历史记录
cargo run -- --pos-id 123456789
```

### 参数说明

| 参数 | 类型 | 描述 |
|------|------|------|
| `--inst-type` | String | 产品类型 (MARGIN, SWAP, FUTURES, OPTION) |
| `--inst-id` | String | 交易产品ID，如：BTC-USD-SWAP |
| `--mgn-mode` | String | 保证金模式 (cross, isolated) |
| `--close-type` | String | 最近一次平仓的类型 (1-5) |
| `--pos-id` | String | 持仓ID |
| `--before` | String | 查询仓位更新之前的时间戳 (毫秒) |
| `--after` | String | 查询仓位更新之后的时间戳 (毫秒) |
| `--limit` | String | 分页返回结果的数量，最大100，默认100 |

### 平仓类型说明

- `1`: 部分平仓
- `2`: 完全平仓
- `3`: 强平
- `4`: 强减
- `5`: ADL自动减仓

## API限速

工具内置了限速功能，按照OKX API的要求：
- 限制：10次/2秒
- 基于User ID进行限速
- 自动等待以避免超出限速

## 响应数据

查询成功后会返回JSON格式的历史持仓数据，包含以下主要字段：

- `inst_type`: 产品类型
- `inst_id`: 交易产品ID
- `pos_id`: 持仓ID
- `pos_side`: 持仓方向
- `pos`: 持仓数量
- `avg_px`: 开仓平均价
- `realized_pnl`: 已实现收益
- `close_type`: 平仓类型
- `open_time`: 开仓时间
- `close_time`: 平仓时间
- `u_time`: 仓位更新时间

## 错误处理

工具会处理以下类型的错误：
- API认证错误
- 网络连接错误
- 限速错误
- 参数验证错误

## 开发

### 项目结构
```
src/
├── main.rs              # 主程序入口
├── lib.rs               # 库模块导出
├── config.rs            # 配置管理
├── types.rs             # 数据类型定义
├── rate_limiter.rs      # 限速器
└── positions_history.rs # 历史持仓API
```

### 添加新功能
1. 在 `src/` 目录下创建新的模块文件
2. 在 `src/lib.rs` 中导出新模块
3. 在 `src/main.rs` 中添加相应的命令行参数

## 注意事项

1. 请妥善保管你的API密钥，不要提交到版本控制系统
2. 建议先在沙盒环境测试
3. 注意API调用频率限制
4. 持仓ID有30天有效期，过期后需要使用新的posId

## 许可证

MIT License 