/// 该程序作用是计算 disksim 格式的 trace 实际占用的硬盘空间
/// 
/// 这和 trace 表示的数据量大小不同。
/// 
/// 前者会包含重复的请求，故不会重复计算，且 trace 实际占用的空间
/// 
/// footprint 只计算写请求。
/// 
/// 执行方式：
/// 
/// - 无 cargo: rustc trace_foot_size.rs -o trace_foot_size
/// 
///             ./trace_foot_size [tracefile] 
/// 
/// - 有 cargo: cargo run --bin trace_foot_size
/// 
/// 
// use radixtree;
// use art;
use rand::{rngs::ThreadRng, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let offset_max = (18 as f64 * 1024 as f64 * 1024 as f64 * 256 as f64) as u64;
    let offset = rng.gen::<u64>() % offset_max;
    println!("{:?}", offset_max);
    println!("{:?}", offset * 8);

}