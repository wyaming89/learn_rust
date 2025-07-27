# learn_rust - OKX API 历史持仓查询工具

这个仓库包含了学习Rust编程的项目，其中主要项目是一个用Rust编写的OKX API调用工具，用于查询OKX交易所的历史持仓信息。

## 项目内容

- **OKX API客户端**: 一个完整的Rust项目，用于查询OKX交易所的历史持仓信息和当前持仓信息
- **其他Rust学习项目**: 包含基础的Rust学习示例

---

# OKX API 持仓查询工具

这是一个用Rust编写的OKX API调用工具，用于查询历史持仓信息和当前持仓信息。

## 功能特性

- 查询最近3个月有更新的历史仓位信息
- 查询当前持仓信息
- 按照仓位更新时间倒序排列
- 支持组合保证金账户模式下的持仓查询
- 内置限速功能（10次/2秒）
- 支持多种查询参数过滤
- 支持简化输出格式

## 安装和配置

### 1. 克隆项目
```bash
git clone <repository-url>
cd okx-api-client
```

### 2. 系统依赖 (Linux用户)

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install pkg-config libssl-dev
```

**CentOS/RHEL:**
```bash
sudo yum install pkg-config openssl-devel
```

**Fedora:**
```bash
sudo dnf install pkg-config openssl-devel
```

**Arch Linux:**
```bash
sudo pacman -S pkg-config openssl
```

### 3. 配置环境变量
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

### 4. 编译项目

**方法1: 直接编译**
```bash
cargo build --release
```

**方法2: 使用构建脚本**
```bash
chmod +x build.sh
./build.sh
```

**方法3: 使用Docker**
```bash
# 构建Docker镜像
docker build -t okx-api-client .

# 运行容器
docker run -v $(pwd)/.env:/app/.env:ro okx-api-client positions --simple
```

## 使用方法

### 基本查询
```bash
# 查询所有当前持仓 (简化格式)
cargo run -- positions --simple

# 查询所有当前持仓 (表格格式)
cargo run -- positions --simple --format table

# 查询所有当前持仓 (完整格式)
cargo run -- positions

# 查询指定产品类型的当前持仓
cargo run -- positions --simple --inst-type SWAP

# 查询指定交易对的当前持仓
cargo run -- positions --simple --inst-id BTC-USD-SWAP
```

### 历史持仓查询
```bash
# 查询所有历史持仓
cargo run -- history

# 查询指定产品类型的历史持仓
cargo run -- history --inst-type SWAP

# 查询指定交易对的历史持仓
cargo run -- history --inst-id BTC-USD-SWAP

# 查询指定保证金模式的历史持仓
cargo run -- history --mgn-mode cross
```

### 账户信息查询
```bash
# 查询账户余额
cargo run -- account balance

# 查询指定币种余额
cargo run -- account balance --ccy BTC

# 查询账户配置
cargo run -- account config
```

### 高级查询
```bash
# 查询最近3个月的历史持仓，限制返回50条
cargo run -- history --limit 50

# 查询指定时间范围的历史持仓
cargo run -- history --after 1640995200000 --before 1643673600000

# 查询指定平仓类型的历史持仓
cargo run -- history --close-type 2  # 2表示完全平仓

# 查询指定持仓ID的历史记录
cargo run -- history --pos-id 123456789
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
| `--simple` | Flag | 使用简化输出格式 |
| `--format` | String | 输出格式 (json, table) |

### 简化输出字段说明

- **Pair**: 交易对名称 (如 SUI-USDT-SWAP)
- **Side**: 持仓方向 (long/short)
- **availPos**: 可用持仓数量
- **avgPx**: 开仓平均价格
- **markPx**: 当前标记价格
- **upl**: 未实现盈亏 (USDT)
- **uplRatio**: 未实现盈亏比例

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

查询成功后会返回JSON格式的持仓数据，包含以下主要字段：

### 当前持仓数据
- `inst_type`: 产品类型
- `inst_id`: 交易产品ID
- `pos_id`: 持仓ID
- `pos_side`: 持仓方向
- `pos`: 持仓数量
- `avg_px`: 开仓平均价
- `upl`: 未实现收益
- `upl_ratio`: 未实现收益率
- `mark_px`: 标记价格
- `u_time`: 仓位更新时间

### 历史持仓数据
- `inst_type`: 产品类型
- `inst_id`: 交易产品ID
- `pos_id`: 持仓ID
- `pos_side`: 持仓方向
- `open_avg_px`: 开仓平均价
- `close_avg_px`: 平仓平均价
- `realized_pnl`: 已实现收益
- `close_type`: 平仓类型
- `open_time`: 开仓时间
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
├── client.rs            # 通用API客户端
├── rate_limiter.rs      # 限速器
├── positions.rs         # 当前持仓API
├── positions_history.rs # 历史持仓API
└── account.rs           # 账户API
```

### 添加新功能
1. 在 `src/` 目录下创建新的模块文件
2. 在 `src/lib.rs` 中导出新模块
3. 在 `src/main.rs` 中添加相应的命令行参数

### 测试
```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_positions_params_serialization
```

### Docker开发
```bash
# 开发环境
docker-compose up okx-api-client-dev

# 生产环境
docker-compose up okx-api-client
```

## 故障排除

### OpenSSL错误
如果在Linux系统上遇到OpenSSL相关错误：
```bash
# Ubuntu/Debian
sudo apt install pkg-config libssl-dev

# CentOS/RHEL
sudo yum install pkg-config openssl-devel

# 或者使用Docker
docker build -t okx-api-client .
```

### 网络问题
如果遇到网络连接问题，可以设置代理：
```bash
export https_proxy=http://127.0.0.1:7890
export http_proxy=http://127.0.0.1:7890
export all_proxy=socks5://127.0.0.1:7890
```

## 注意事项

1. 请妥善保管你的API密钥，不要提交到版本控制系统
2. 建议先在沙盒环境测试
3. 注意API调用频率限制
4. 持仓ID有30天有效期，过期后需要使用新的posId

## 许可证

MIT License
