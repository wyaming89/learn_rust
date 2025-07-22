# OKX API 历史持仓查询工具使用说明

## 快速开始

### 1. 配置环境变量

首先复制环境变量示例文件：
```bash
cp env.example .env
```

然后编辑 `.env` 文件，填入你的OKX API配置：
```
OKX_API_KEY=your_api_key_here
OKX_SECRET_KEY=your_secret_key_here
OKX_PASSPHRASE=your_passphrase_here
OKX_SANDBOX=false
```

### 2. 编译项目
```bash
cargo build --release
```

### 3. 运行工具
```bash
./target/release/okx-api-client
```

## 命令行参数

### 基本查询
```bash
# 查询所有历史持仓（默认返回100条）
./target/release/okx-api-client

# 查询指定产品类型的历史持仓
./target/release/okx-api-client -t SWAP

# 查询指定交易对的历史持仓
./target/release/okx-api-client -i BTC-USD-SWAP

# 查询指定保证金模式的历史持仓
./target/release/okx-api-client -m cross
```

### 高级查询
```bash
# 限制返回结果数量
./target/release/okx-api-client -l 50

# 查询指定时间范围的历史持仓
./target/release/okx-api-client -a 1640995200000 -b 1643673600000

# 查询指定平仓类型的历史持仓
./target/release/okx-api-client -c 2  # 2表示完全平仓

# 查询指定持仓ID的历史记录
./target/release/okx-api-client -p 123456789
```

### 组合查询
```bash
# 查询BTC永续合约的全仓模式历史持仓
./target/release/okx-api-client -t SWAP -i BTC-USD-SWAP -m cross

# 查询最近10条完全平仓的历史记录
./target/release/okx-api-client -c 2 -l 10
```

## 参数说明

| 参数 | 短选项 | 长选项 | 描述 | 示例 |
|------|--------|--------|------|------|
| 产品类型 | `-t` | `--inst-type` | 产品类型 | SWAP, FUTURES, OPTION, MARGIN |
| 交易产品ID | `-i` | `--inst-id` | 交易产品ID | BTC-USD-SWAP |
| 保证金模式 | `-m` | `--mgn-mode` | 保证金模式 | cross, isolated |
| 平仓类型 | `-c` | `--close-type` | 平仓类型 | 1-5 |
| 持仓ID | `-p` | `--pos-id` | 持仓ID | 123456789 |
| 时间之前 | `-b` | `--before` | 查询时间之前 | 1643673600000 |
| 时间之后 | `-a` | `--after` | 查询时间之后 | 1640995200000 |
| 返回数量 | `-l` | `--limit` | 返回结果数量 | 100 |

## 平仓类型说明

- `1`: 部分平仓
- `2`: 完全平仓
- `3`: 强平
- `4`: 强减
- `5`: ADL自动减仓

## 产品类型说明

- `MARGIN`: 币币杠杆
- `SWAP`: 永续合约
- `FUTURES`: 交割合约
- `OPTION`: 期权

## 保证金模式说明

- `cross`: 全仓
- `isolated`: 逐仓

## 时间戳格式

时间戳使用Unix时间戳（毫秒），例如：
- `1640995200000`: 2022-01-01 00:00:00 UTC
- `1643673600000`: 2022-02-01 00:00:00 UTC

## 输出格式

工具会输出JSON格式的响应数据，包含以下主要字段：

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "inst_type": "SWAP",
      "inst_id": "BTC-USD-SWAP",
      "pos_id": "123456789",
      "pos_side": "long",
      "pos": "1",
      "avg_px": "50000",
      "realized_pnl": "1000",
      "close_type": "2",
      "open_time": "1640995200000",
      "close_time": "1643673600000",
      "u_time": "1643673600000"
    }
  ]
}
```

## 错误处理

### 常见错误

1. **API认证错误**
   ```
   查询失败: API错误: 50001 - 无效的API密钥
   ```
   解决方案：检查API密钥、密钥和密码是否正确

2. **限速错误**
   ```
   查询失败: API错误: 50002 - 请求过于频繁
   ```
   解决方案：工具会自动处理限速，等待一段时间后重试

3. **参数错误**
   ```
   查询失败: API错误: 50004 - 参数错误
   ```
   解决方案：检查命令行参数是否正确

### 调试模式

设置环境变量来启用调试信息：
```bash
export RUST_LOG=debug
./target/release/okx-api-client
```

## 注意事项

1. **API密钥安全**
   - 请妥善保管API密钥，不要提交到版本控制系统
   - 建议使用只读权限的API密钥

2. **限速限制**
   - OKX API限制：10次/2秒
   - 工具内置限速功能，会自动等待

3. **数据时效性**
   - 持仓ID有30天有效期
   - 过期后需要使用新的posId

4. **沙盒环境**
   - 建议先在沙盒环境测试
   - 设置 `OKX_SANDBOX=true` 使用沙盒环境

## 示例脚本

### 查询最近一周的历史持仓
```bash
#!/bin/bash
# 获取一周前的时间戳（毫秒）
week_ago=$(date -d '1 week ago' +%s)000

# 查询最近一周的历史持仓
./target/release/okx-api-client -a $week_ago -l 100
```

### 查询所有完全平仓的记录
```bash
#!/bin/bash
# 查询所有完全平仓的历史记录
./target/release/okx-api-client -c 2 -l 100
```

### 批量查询多个交易对
```bash
#!/bin/bash
# 查询多个交易对的历史持仓
for pair in "BTC-USD-SWAP" "ETH-USD-SWAP" "LTC-USD-SWAP"; do
    echo "查询 $pair 的历史持仓:"
    ./target/release/okx-api-client -i $pair -l 10
    echo "---"
done
``` 