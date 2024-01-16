use clap::{arg, Parser, Subcommand};
use diskpine::{error::HMSimError, log, block::HMSimBlock};
use ::log::info;
use regex::Regex;
use dotenv::dotenv;


#[derive(Parser, Debug)]
#[command(author = "Cherry")]
#[command(version)]
#[command(about = "HMSim", long_about = "HMSim")]
#[command(
    help_template = "{about}
Author: {author}
Version: {version}
{usage-heading} {usage}
{all-args} {tab}"
)]
struct Args {
    // /// 姓名
    // #[arg(short, long, default_value = "B")]
    // name: String,

    // /// 年龄
    // #[arg(short, long, default_value_t = 25)]
    // age: u32,

    // /// 昵称
    // #[arg(long, default_value = "cherry")]
    // nick: String,

    /// 子命令
    #[command(subcommand)]
    command: Commands,

}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 生成 trace，可以指定随机或顺序，读写比例，请求大小
    GenerateTrace {
        /// 指定 trace 的起始地址，若不指定该字段，则随机从一个地址开始
        #[arg(short, long)]
        addr_start: Option<u64>,
        
        /// 通过指定数据量确定 trace(单位: B, KB, MB, GB, TB 等)
        #[arg(short, long)]
        #[clap(value_parser = string_to_hmsim_block)]
        size_data: Option<HMSimBlock>,
        
        /// 通过指定请求数量确定 trace(单位: 条数)
        #[arg(short, long)]
        num_request: Option<u64>,

        /// 指定每个请求的大小，若不指定，则通过指定随机变化的范围随机变化
        /// (单位: B, KB, MB, GB, TB 等)
        #[arg(short, long)]
        #[clap(value_parser = string_to_hmsim_block)]
        length_request: Option<HMSimBlock>,
    },

    /// 计算 trace 数据量及落盘量
    TraceFootSize {
        /// trace 文件名
        #[arg(short, long)]
        file: String,
    },

    /// 将微软原始 trace 格式转化为 HMSim 格式的 trace，修改后的文件与其同名
    OriginToSim {
        /// 原始 trace 文件名
        #[arg(short, long)]
        file: String,
    },
}

fn main() {
    // 从 .env 文件中读取 LOG_LEVEL 环境变量
    dotenv().ok();
    log::log_init();
    info!("Diskpine is running...");

    let args = Args::parse();
    match args.command {
        
        Commands::GenerateTrace { addr_start, size_data, num_request, length_request } => {
            println!("{:?}", addr_start.unwrap())
        },

        Commands::TraceFootSize { file } => {

        },

        Commands::OriginToSim { file } => {

        }
    }
}


/// 将以 KB, MB 为单位的字符串转化成以扇区为单位
fn string_to_hmsim_block(size: &str) -> Result<HMSimBlock, HMSimError> {

    let regex = Regex::new(r"(\d+)([A-Za-z]+)").unwrap();

    let mut hmsim_block = HMSimBlock::new();

    if let Some(captures) = regex.captures(size) {
        // 提取数字部分
        let number = captures[1].parse::<u64>().unwrap();

        // 提取单位部分
        let unit = &captures[2];
        if unit.eq_ignore_ascii_case("b") {
            hmsim_block.byte = number;
        } else if unit.eq_ignore_ascii_case("k") || unit.eq_ignore_ascii_case("kb") {
            hmsim_block.byte = number * 1024;
        } else if unit.eq_ignore_ascii_case("m") || unit.eq_ignore_ascii_case("mb") {
            hmsim_block.byte = number * 1024 * 1024;
        } else if unit.eq_ignore_ascii_case("g") || unit.eq_ignore_ascii_case("gb") {
            hmsim_block.byte = number * 1024 * 1024 * 1024;
        } else if unit.eq_ignore_ascii_case("t") || unit.eq_ignore_ascii_case("tb") {
            hmsim_block.byte = number * 1024 * 1024 * 1024 * 1024;
        } else {
            return Err(HMSimError::ParseError)
        }

        hmsim_block.block = hmsim_block.byte / 512;
        
        // 打印结果
        // println!("byte: {}", hmsim_block.byte);
        // println!("block: {}", hmsim_block.block);
    } else {
        return Err(HMSimError::ParseError)
    }

    Ok(hmsim_block)
}