use clap::{App, Arg, SubCommand};
use okx_api_client::{
    config::Config,
    positions::get_positions,
    positions_history::{get_positions_history, PositionsHistoryParams},
    types::PositionsParams,
};

fn main() -> anyhow::Result<()> {
    // 创建运行时
    let mut rt = tokio::runtime::Runtime::new()?;
    
    // 在运行时中执行异步代码
    rt.block_on(async {
        // 加载环境变量
        dotenv::dotenv().ok();

        let matches = App::new("OKX API Client")
            .version("1.0")
            .about("OKX API 持仓查询工具")
            .subcommand(
                SubCommand::with_name("history")
                    .about("查询历史持仓信息")
                    .arg(
                        Arg::new("inst_type")
                            .short('t')
                            .long("inst-type")
                            .help("产品类型 (MARGIN, SWAP, FUTURES, OPTION)")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("inst_id")
                            .short('i')
                            .long("inst-id")
                            .help("交易产品ID，如：BTC-USD-SWAP")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("mgn_mode")
                            .short('m')
                            .long("mgn-mode")
                            .help("保证金模式 (cross, isolated)")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("close_type")
                            .short('c')
                            .long("close-type")
                            .help("最近一次平仓的类型 (1-5)")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("pos_id")
                            .short('p')
                            .long("pos-id")
                            .help("持仓ID")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("before")
                            .short('b')
                            .long("before")
                            .help("查询仓位更新之前的时间戳 (毫秒)")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("after")
                            .short('a')
                            .long("after")
                            .help("查询仓位更新之后的时间戳 (毫秒)")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("limit")
                            .short('l')
                            .long("limit")
                            .help("分页返回结果的数量，最大100")
                            .default_value("100")
                            .takes_value(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("positions")
                    .about("查询当前持仓信息")
                    .arg(
                        Arg::new("inst_type")
                            .short('t')
                            .long("inst-type")
                            .help("产品类型 (MARGIN, SWAP, FUTURES, OPTION)")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("inst_id")
                            .short('i')
                            .long("inst-id")
                            .help("交易产品ID，如：BTC-USD-SWAP")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::new("pos_id")
                            .short('p')
                            .long("pos-id")
                            .help("持仓ID")
                            .takes_value(true),
                    ),
            )
            .get_matches();

        // 加载配置
        let config = Config::from_env()?;

        match matches.subcommand() {
            Some(("history", sub_matches)) => {
                println!("开始查询历史持仓信息...");

                // 构建查询参数
                let params = PositionsHistoryParams {
                    inst_type: sub_matches.value_of("inst_type").map(|s| s.to_string()),
                    inst_id: sub_matches.value_of("inst_id").map(|s| s.to_string()),
                    mgn_mode: sub_matches.value_of("mgn_mode").map(|s| s.to_string()),
                    close_type: sub_matches.value_of("close_type").map(|s| s.to_string()),
                    pos_id: sub_matches.value_of("pos_id").map(|s| s.to_string()),
                    before: sub_matches.value_of("before").map(|s| s.to_string()),
                    after: sub_matches.value_of("after").map(|s| s.to_string()),
                    limit: sub_matches.value_of("limit").map(|s| s.to_string()),
                };

                // 调用API
                match get_positions_history(&config, &params).await {
                    Ok(response) => {
                        println!("查询成功！");
                        println!("响应数据: {}", serde_json::to_string_pretty(&response)?);
                    }
                    Err(e) => {
                        eprintln!("查询失败: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Some(("positions", sub_matches)) => {
                println!("开始查询当前持仓信息...");

                // 构建查询参数
                let params = PositionsParams {
                    inst_type: sub_matches.value_of("inst_type").map(|s| s.to_string()),
                    inst_id: sub_matches.value_of("inst_id").map(|s| s.to_string()),
                    pos_id: sub_matches.value_of("pos_id").map(|s| s.to_string()),
                };

                // 调用API
                match get_positions(&config, &params).await {
                    Ok(response) => {
                        println!("查询成功！");
                        println!("响应数据: {}", serde_json::to_string_pretty(&response)?);
                    }
                    Err(e) => {
                        eprintln!("查询失败: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                println!("请指定要执行的命令:");
                println!("  history    - 查询历史持仓信息");
                println!("  positions  - 查询当前持仓信息");
                println!("\n使用 --help 查看详细帮助信息");
            }
        }

        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
} 