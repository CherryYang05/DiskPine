use std::{
    fs::{self, remove_file, OpenOptions},
    io::Write,
    vec,
};

use log::{debug, error, info};
use rand::{rngs::ThreadRng, Rng};
use rand_distr::Distribution;

/// 生成的 trace 包含以下参数：
///
/// size(读写操作的总大小): 例如 size=1T, 需要加上单位(忽略大小写)
///
/// rwrate(读写比例，读:写): a:b, 支持小数，若某一个为 0(例如 rwrate=0:5, 和 rwrate=0:1 等价)，则表示全为写操作
///
/// write_size(写操作大小范围): 例如 write_size=1G-32G, 需要加上单位(忽略大小写)
///
/// read_size(读操作大小范围): 例如 read_size=12M-1G, 需要加上单位(忽略大小写)
///
/// batch(批操作, 是否连续生成一系列相同操作的请求): 可选参数为 r, w, rw
///         batch=r, 表示连续生成若干个读请求，这若干个读请求成为一个 batch
///         如果不指定该参数，将不会连续生成若干个相同操作的请求
///
/// batch_size(设定每个 batch 的大小范围)=512M-8G, 表示每个 batch 大小为 1G
///
use crate::{error::HMSimError, Dist};
// #[warn(dead_code)]

/// 通过子命令参数转化成的 TapeTrace 结构体
///
/// 大小单位全部转化为 block(512B)
#[derive(Debug, Clone)]
pub struct TapeTrace {
    pub total_size: u64,

    // pub read_rate: f32,
    // pub write_rate: f32,
    pub block_size: u64,
    pub rw: String,
    pub read_order: String,
    pub write_order: String,

    pub write_offset: u64,

    pub write_size_start: u64,
    pub write_size_end: u64,
    pub write_size_range: u64,

    pub read_size_start: u64,
    pub read_size_end: u64,
    pub read_size_range: u64,

    pub rwsize_start: u64,
    pub rwsize_end: u64,
    pub rwsize_range: u64,

    pub batch: String,
    pub batch_iow_num_begin: u64,
    pub batch_iow_num_end: u64,
    pub batch_iow_num_range: u64,

    pub batch_ior_num_begin: u64,
    pub batch_ior_num_end: u64,
    pub batch_ior_num_range: u64,
    pub time_interval_dist: Dist,
    pub req_length_dist: Dist,
}

impl TapeTrace {
    pub fn new() -> TapeTrace {
        TapeTrace {
            total_size: 0,
            // read_rate: 0.0,
            // write_rate: 0.0,
            block_size: 0,
            rw: String::new(),
            read_order: String::new(),
            write_order: String::new(),
            write_offset: 0,
            write_size_start: 0,
            write_size_end: 0,
            write_size_range: 0,
            read_size_start: 0,
            read_size_end: 0,
            read_size_range: 0,
            rwsize_start: 0,
            rwsize_end: 0,
            rwsize_range: 0,
            batch: String::new(),
            batch_iow_num_begin: 0,
            batch_iow_num_end: 0,
            batch_iow_num_range: 0,
            batch_ior_num_begin: 0,
            batch_ior_num_end: 0,
            batch_ior_num_range: 0,
            time_interval_dist: Dist::None,
            req_length_dist: Dist::None
        }
    }

    /// 生成读写请求，返回 (op_num, return_size)
    fn operation(&self, rand: &mut ThreadRng, trace: &TapeTrace, rw: &str, cur_write_offset: &mut u64, cur_read_offset: &mut u64) -> (u64, u64) {
        let mut op_num = 0;
        if rw == "R" {
            if *cur_write_offset == 0 {
                return (0, 0);
            }
            if self.batch.contains("r") {
                // debug!("batch_ior_num_begin: {}, batch_ior_num_begin: {}", self.batch_ior_num_begin, self.batch_ior_num_end);
                // 随机生成一个 batch 大小
                let mut op_num_per_batch =
                    rand.gen_range(self.batch_ior_num_begin..=self.batch_ior_num_end);
                // debug!("op_num_per_batch: {}", op_num_per_batch);
                
                let mut return_size = 0;
                while op_num_per_batch > 0 {
                    let blocksize = self.generate_one(rand, trace, "R", cur_write_offset, cur_read_offset);
                    op_num += 1;
                    return_size += blocksize;
                    op_num_per_batch -= 1;
                }
                return (op_num, return_size);
            } else {
                return (1, self.generate_one(rand, trace, "R", cur_write_offset, cur_read_offset));
            }
        } else if rw == "W" {
            if self.batch.contains("w") {
                // 随机生成一个 batch 大小，按照 ALIEN 对齐
                let mut op_num_per_batch =
                rand.gen_range(self.batch_iow_num_begin..=self.batch_iow_num_end);
                // debug!("op_num_per_batch: {}", op_num_per_batch);
                
                let mut return_size = 0;
                while op_num_per_batch > 0 {
                    let blocksize = self.generate_one(rand, trace, "W", cur_write_offset, cur_read_offset);
                    op_num += 1;
                    return_size += blocksize;
                    op_num_per_batch -= 1;
                }
                // debug!("return size: {}", return_size);
                return (op_num, return_size);
            } else {
                return (1, self.generate_one(rand, trace, "W", cur_write_offset, cur_read_offset));
            }
        }
        (0, 0)
    }

    /// 生成一条请求的请求大小和请求偏移，返回生成的请求大小
    /// 首先看是读操作还是写操作
    /// 再看 read_order/write_order 是随机还是顺序
    /// 然后看是否有 batch
    fn generate_one(&self, rand: &mut ThreadRng, trace: &TapeTrace, rw: &str, cur_write_offset: &mut u64, cur_read_offset: &mut u64) -> u64 {
        if rw == "R" {
            let mut read_blocksize;
            match trace.req_length_dist {
                // 随机生成一个读请求大小
                Dist::None => {
                    read_blocksize = rand.gen_range(self.read_size_start..=self.read_size_end) * self.block_size;
                    // debug!("read_size: {}-{}", self.read_size_start, self.read_size_end);
                },
                // 根据数学分布生成请求大小
                _ => {
                    // info!("Genereate Exp req");
                    let exp = get_timeinteval_from_distribution(&trace.req_length_dist);
                    read_blocksize = exp as u64 / self.block_size * self.block_size;
                    if read_blocksize == 0 {
                        // error!("req_len can't be zero!!!");
                        return 0;
                    }
                }
            }

            // 要生成的地址的起始地址
            // let addr_begin = 263680000;
            let addr_begin = 0;

            // 根据读写的顺序参数(rand or seq)生成请求偏移量
            let mut read_offset = 0;
            // debug!("read_order: {}", trace.read_order);
            if trace.read_order.eq_ignore_ascii_case("rand") {
                read_offset = rand.gen_range(addr_begin..*cur_write_offset) / self.block_size * self.block_size;
            } else if trace.read_order.eq_ignore_ascii_case("seq") {
                read_offset = *cur_read_offset;
                *cur_read_offset = *cur_read_offset + read_blocksize;
            }

            // debug!("read_offset: {}", read_offset);
            if trace.read_order.eq_ignore_ascii_case("seq") {
                if read_blocksize + read_offset > trace.total_size {
                    // debug!("trace.total_size - read_offset: {}", trace.total_size - read_offset);
                    // debug!("trace.total_size: {}", trace.total_size);
                    // debug!("read_offset: {}", read_offset);
                    // debug!("read_blocksize: {}", read_blocksize);
                    read_blocksize = trace.total_size - read_offset;
                }
            } else if trace.read_order.eq_ignore_ascii_case("rand") {
                if read_blocksize + read_offset > *cur_write_offset {
                    read_blocksize = *cur_write_offset - read_offset;
                }
            }
            
            Self::write_to_file("R", read_offset, read_blocksize, trace);
            
            return read_blocksize;
        } else if rw == "W" {
            // 随机生成一个写请求大小
            let write_blocksize =
            rand.gen_range(self.write_size_start..=self.write_size_end) * self.block_size;
            // debug!("write_size: {}-{}", self.write_size_start, self.write_size_end);
            // debug!("block_size: {}", self.block_size);

                // 由于写请求太大，设置越大的请求生成概率越低
                // generate_weighted_random_number(rand, self.write_size_start, self.write_size_end) * self.block_size;

            // debug!("write_blocksize: {}", write_blocksize);

            Self::write_to_file("W", *cur_write_offset, write_blocksize, trace);

            *cur_write_offset += write_blocksize;
            // debug!("cur_offset: {}", cur_offset);

            return write_blocksize;
        }
        0
    }

    /// 将生成的请求写入 trace 文件，时间间隔(包含时间间隔的分布)在写文件时生成
    fn write_to_file(rw: &str, offset: u64, blocksize: u64, trace: &TapeTrace) {
        let mut output_file = OpenOptions::new()
            .append(true)
            .create(true)
            // .write(true)
            .open("tape.trace")
            .unwrap();

        let mut req = vec![];

        // 模拟器 trace 第一个参数: 读写
        req.push(rw);

        // 模拟器 trace 第二个参数: Hit
        req.push("Hit");

        // 模拟器 trace 第三个参数: 偏移量
        let tmp = offset.to_string();
        req.push(tmp.as_str());

        // 模拟器 trace 第四个参数: 长度
        let tmp = blocksize.to_string();
        req.push(tmp.as_str());

        // 模拟器 trace 第五个参数: 服务时间
        req.push("0.000000");

        // 模拟器 trace 第六个参数: 时间间隔
        let mut time_interval = String::from("0.000000");
        match trace.time_interval_dist {
            Dist::None => {
                req.push(time_interval.as_str());
            },
            _ => {
                time_interval = get_timeinteval_from_distribution(&trace.time_interval_dist).to_string()[0..=7].to_string();
                req.push(time_interval.as_str());
            }
        }

        output_file.write_all(req.join(" ").as_bytes()).unwrap();
        output_file.write_all("\n".as_bytes()).unwrap();
    }

    // /// 如果有 batch 操作，在考虑 batch 的情况下重新计算读写比
    // fn recalculate_rwrate(&mut self) {
    //     let avg_write_size = mean([self.write_size_start, self.write_size_end].as_slice()).unwrap();

    //     let avg_read_size = mean([self.read_size_start, self.read_size_end].as_slice()).unwrap();

    //     if self.batch.contains("w") {
    //         let avg_batch_write_size =
    //             mean([self.batch_write_size_begin, self.batch_write_size_end].as_slice()).unwrap();
    //         self.read_rate *= (avg_batch_write_size / avg_write_size) as f32;
    //     }

    //     if self.batch.contains("r") {
    //         let avg_batch_read_size =
    //             mean([self.batch_read_size_begin, self.batch_read_size_end].as_slice()).unwrap();
    //         // self.write_rate *= (avg_batch_read_size / avg_read_size) as f32 / 5f32 * 3.25;
    //         self.write_rate *= (avg_batch_read_size / avg_read_size) as f32;
    //     }
    // }

    // /// 将读写比归一化
    // fn rwrate_in_one(&mut self) {
    //     let min_num = self.write_rate.min(self.read_rate);
    //     if self.write_rate > 0.0 && self.read_rate > 0.0 {
    //         self.write_rate /= min_num;
    //         self.read_rate /= min_num;
    //     }
    //     if self.read_rate == 0.0 {
    //         self.write_rate = 1.0;
    //     } else if self.write_rate == 0.0 {
    //         self.read_rate = 1.0;
    //     }
    // }
}

// 地址对齐的单位，256KB
// static ALIEN: u64 = 512;

/// 对外暴露的函数
pub fn generate_tape_trace(trace: TapeTrace) -> Result<(), HMSimError> {

    // 记录顺序写请求已经写到的偏移量
    // 注意：只会在已经写过的地址内生成读请求，因此当只生成读请求时请确保该偏移量足够大
    // 515000 - 728000 (wrap2, 178GB)
    // 515000 - 940000 (wrap2-3, 230GB)
    // let mut cur_write_offset = 0;
    // let mut cur_write_offset = 481280000;
    let mut cur_write_offset = trace.write_offset;
    if trace.read_order.eq_ignore_ascii_case("seq") {
        cur_write_offset = trace.total_size;
    }

    // 只有顺序读时该参数才会被使用
    let mut cur_read_offset = 0;

    // // 如果有 batch 操作，重新计算读写比
    // trace.recalculate_rwrate();

    // info!("read_rate: {}, write_rate: {}", trace.read_rate, trace.write_rate);
    // // 将读写比归一化
    // trace.rwrate_in_one();
    // info!("after in one: read_rate: {}, write_rate: {}", trace.read_rate, trace.write_rate);

    // 通过读写比生成随机数，若随机数小于 1 则为读操作
    let mut rand = rand::thread_rng();

    // let rand_floor = trace.write_rate + trace.read_rate;

    let mut op_read = 0u64;
    let mut op_write = 0u64;

    let mut read_data = 0u64;
    let mut write_data = 0u64;

    if fs::metadata("tape.trace").is_ok() {
        remove_file("tape.trace").unwrap();
    }

    let mut loop_rw;
    let mut total_size = trace.total_size;
    while total_size > 0 {
        // let rand_num;
        // debug!("read: {}, write: {}", trace.read_size_end, trace.write_size_end);
        // if trace.read_size_end == 0 {
        //     rand_num = rand.gen_range(1..2);
        // } else if trace.write_size_end == 0 {
        //     rand_num = rand.gen_range(0..1);
        // } else {
        //     rand_num = rand.gen_range(0..2);
        // }

        if trace.rw.eq_ignore_ascii_case("r") {
            loop_rw = "R";
        } else if trace.rw.eq_ignore_ascii_case("w") {
            loop_rw = "W";
        } else if trace.rw.eq_ignore_ascii_case("rw") {
            let rand_num = rand.gen_range(0..2);
            // 随机数为 0 则为读操作，为 1 则为写操作
            if rand_num < 1 {
                loop_rw = "R";
            } else {
                loop_rw = "W";
            }
        } else {
            return Err(HMSimError::CommandError)
        }
        
        // debug!("rw: {}", rw);

        // 函数 operation 很重要，生成 trace 请求的所有操作都在该函数中
        // 生成 trace 请求，返回值是 (生成的请求数量, 生成的请求大小) tuple
        let (op_num, generate_size) = trace.operation(&mut rand, &trace, loop_rw, &mut cur_write_offset, &mut cur_read_offset);

        // debug!("op_num: {}, generate_size: {}", op_num, generate_size);

        // ============= 下面是统计信息 =============
        if loop_rw == "R" {
            op_read += op_num;
            read_data += generate_size;
        } else {
            op_write += op_num;
            write_data += generate_size;
        }

        if total_size <= generate_size {
            total_size = 0;
        } else {
            total_size -= generate_size;
        }
        // ============= 统计信息结束 =============
    }

    // ============= 打印统计信息日志 =============
    info!(
        "read_op:   {:<10}    write_op:   {:<10}    rate(w:r): {}",
        op_read,
        op_write,
        op_write as f32 / op_read as f32
    );
    info!(
        "read_data: {:<10}MB  write_data: {:<10}MB  rate(w:r): {}",
        read_data as f32 / 2048f32,
        write_data as f32 / 2048f32,
        write_data as f32 / read_data as f32
    );
    // ============= 打印统计信息日志结束 =============

    info!("generate_tape_trace running done.");
    Ok(())
}

/// 求平均数
fn mean(data: &[u64]) -> Option<f32> {
    let sum = data.iter().sum::<u64>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

/// 根据不同权重生成随机数
fn generate_weighted_random_number(rng: &mut ThreadRng, start: u64, end: u64) -> u64 {
    let rand_num: u64 = rng.gen_range(0..=10); // 生成一个在 0 到 10 之间的随机整数
    let range = vec![0, 2, 4, 7, 10];
    let probility = vec![4, 7, 9, 10];
    let piece = (end - start) / 10;
    let res;
    if rand_num < probility[0] {
        res = rng.gen_range(start + piece * range[0]..start + piece * range[1]);
    // 40% 的概率
    } else if rand_num < probility[1] {
        res = rng.gen_range(start + piece * range[1]..start + piece * range[2]);
    // 30% 的概率
    } else if rand_num < probility[2] {
        res = rng.gen_range(start + piece * range[2]..start + piece * range[3]);
    // 20% 的概率
    } else {
        res = rng.gen_range(start + piece * range[3]..=start + piece * range[4]);
        // 10% 的概率
    }
    return res;
}


/// 根据数学分布生成时间间隔
fn get_timeinteval_from_distribution(dist: &Dist) -> f64 {
    match *dist {
        Dist::Exponential(lambda) => {
            let exp = rand_distr::Exp::new(lambda).unwrap();
            exp.sample(&mut rand::thread_rng())
        },
        _ => 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_timeinteval_from_distribution() {
        let exp = rand_distr::Exp::new(2.0).unwrap();
        let v = exp.sample(&mut rand::thread_rng());
        println!("{} is from a Exp(2) distribution", v);
    }
}

