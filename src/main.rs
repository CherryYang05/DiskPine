use ::log::{debug, info};
use clap::{arg, Parser, Subcommand};
use diskpine::{
    commands::Pine, error::HMSimError, log, utils::{self, dist_analyze, range_to_num, size_range_to_start_end, string_to_hmsim_block}, Dist, HMSimBlock, SizePair
};
use dotenv::dotenv;

#[warn(dead_code)]
#[derive(Parser, Debug)]
#[command(author = "Cherry")]
#[command(version)]
#[command(about = "HMSim Command Tool", long_about = "HMSim Command Tool")]
#[command(help_template = "{about}
Author: {author}
Version: {version}
{usage-heading} {usage}
{all-args} {tab}")]
struct Args {
    /// 子命令
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // /// 生成 trace，可以指定随机或顺序，读写比例，请求大小
    // GenerateTrace {
    // /// 指定 trace 的起始地址，若不指定该字段，则随机从一个地址开始
    // #[arg(short, long)]
    // addr_start: Option<u64>,

    // /// 通过指定数据量确定 trace(单位: B, KB, MB, GB, TB 等)
    // #[arg(short, long)]
    // #[clap(value_parser = string_to_hmsim_block)]
    // size_data: Option<HMSimBlock>,

    // /// 通过指定请求数量确定 trace(单位: 条数)
    // #[arg(short, long)]
    // num_request: Option<u64>,

    // /// 指定每个请求的大小，若不指定，则通过指定随机变化的范围随机变化
    // /// (单位: B, KB, MB, GB, TB 等)
    // #[arg(short, long)]
    // #[clap(value_parser = string_to_hmsim_block)]
    //length_request: Option<HMSimBlock>,
    // },
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

        /// 是否保留时间戳
        #[arg(short, long)]
        timestamp: bool,
    },

    /// 生成适用于 Tape 的 trace
    GenerateTapeTrace {
        /// 读写操作的总大小
        #[arg(name = "size", long)]
        #[clap(value_parser = string_to_hmsim_block)]
        total_size: HMSimBlock,

        /// 生成的请求粒度，即块大小
        #[arg(name = "blk_size", long)]
        #[clap(value_parser = string_to_hmsim_block)]
        block_size: HMSimBlock,

        /// 指定生成的请求类型 [支持参数为 r, w, rw]
        #[arg(long)]
        rw: Option<String>,

        /// 单个写请求大小范围，若没有写请求则设置为 0-0
        #[arg(name = "wsize", long)]
        #[clap(value_parser = range_to_num)]
        write_size: Option<(u64, u64)>,

        /// 单个读请求大小范围，若没有读请求则设置为 0-0
        #[arg(name = "rsize", long)]
        #[clap(value_parser = range_to_num)]
        read_size: Option<(u64, u64)>,

        /// 单个请求大小范围(该参数当 wsize 和 rsize 均为 None 时有效)
        #[arg(long)]
        // #[clap(requires_if_all(&["write_size", "read_size"], &["None", "None"]))]
        #[clap(value_parser = range_to_num)]
        rwsize: Option<(u64, u64)>,

        /// 设置读写操作的 batch [支持参数为 r, w, rw]
        #[arg(long)]
        batch: Option<String>,

        /// 每个 write batch 的大小范围(单位为 blk_size，该参数当 batch 包含 'w' 时有效)
        #[arg(name = "batch_IOw_num", long)]
        #[clap(value_parser = range_to_num)]
        // #[clap(requires_if("batch", "Some"))] // 设置该参数依赖于 batch
        // batch_write_size: Option<SizePair>,
        batch_iow_num: Option<(u64, u64)>,

        /// 每个 read batch 的大小范围(单位为 blk_size，该参数当 batch 包含 'r' 时有效)
        #[arg(name = "batch_IOr_num", long)]
        #[clap(value_parser = range_to_num)]
        // #[clap(requires_if("batch", "Some"))] // 设置该参数依赖于 batch
        // batch_read_size: Option<SizePair>,
        batch_ior_num: Option<(u64, u64)>,

        /// 生成的时间间隔满足的数学分布[支持的参数：exp:lambda(指数分布:lambda)，uni(均匀分布)，poi(泊松分布:lambda)]
        #[arg(name = "time_dist", long)]
        #[clap(value_parser = dist_analyze)]
        time_interval_distribution: Option<Dist>,

        /// 生成的请求大小满足的数学分布[支持的参数：exp:lambda(指数分布:lambda)，uni(均匀分布)，poi(泊松分布:lambda)]
        #[arg(name = "req_dist", long)]
        #[clap(value_parser = dist_analyze)]
        req_length_distribution: Option<Dist>,
    },
}

fn main() -> Result<(), HMSimError> {
    // 从 .env 文件中读取 LOG_LEVEL 环境变量
    dotenv().ok();
    log::log_init();
    info!("Diskpine is running...");

    let args = Args::parse();

    match args.command {
        // Commands::GenerateTrace {
        //     addr_start,
        //     size_data,
        //     num_request,
        //     length_request,
        // } => Pine.generate_trace(),
        Commands::TraceFootSize { file } => Pine.trace_foot_size(file.as_str()),

        Commands::OriginToSim { file, timestamp } => Pine.origin_to_sim(file.as_str(), timestamp),

        Commands::GenerateTapeTrace {
            total_size,
            block_size,
            rw,
            write_size,
            read_size,
            rwsize,
            batch,
            batch_iow_num,
            batch_ior_num,
            time_interval_distribution,
            req_length_distribution
        } => {
            let tape_trace_struct = utils::command_gen_tape_trace_to_tape_trace_struct(
                total_size,
                block_size,
                rw,
                write_size,
                read_size,
                rwsize,
                batch,
                batch_iow_num,
                batch_ior_num,
                time_interval_distribution,
                req_length_distribution
            );

            // debug!("{:#?}", tape_trace_struct);
            Pine.generate_tape_trace(tape_trace_struct?)
        }

        _ => Err(HMSimError::CommandError),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_generate_tape_trace() {}
}
