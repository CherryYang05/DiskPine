/// 该程序作用是生成大容量 trace，第一部分请求为顺序请求，用来填充缓存，
///
/// 第二部分请求是真正的请求，需要计算平均时延，和命中率
///
/// 程序有两个参数，第一个参数是设定的缓存大小(单位 GB)，每个请求大小为 1MB，
///
/// 表示第一部分的请求数量，第二个参数是第二部分的请求所表示的数据量(单位 TB)
///
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
};

use rand::{rngs::ThreadRng, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let argv: Vec<_> = env::args().collect();
    let cache_size = argv[1].clone();
    let trace_size = argv[2].clone();
    let cache_size = cache_size.parse::<f64>().unwrap();
    let trace_size = trace_size.parse::<f64>().unwrap();

    let filename = format!("{}TB-{}GB.trace", trace_size, cache_size);
    let rand_filename = format!("rand-{}TB.trace", trace_size);

    let mut tracefile = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();

    let mut rand_tracefile = if fs::metadata(&rand_filename).is_err() {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(rand_filename)
            .unwrap();

        generate_rand_trace(&mut file, &mut rng, trace_size);

        file.flush().unwrap();

        file
    } else {
        OpenOptions::new().read(true).open(rand_filename).unwrap()
    };

    generate_seq_trace(&mut tracefile, 0, cache_size);

    append_trace(&mut rand_tracefile, &mut tracefile);
    println!("Done");
}

/// 每个请求大小为 1MB，offset 按照 4KB 对齐，偏移量单位为 512B(扇区)，size 单位为 GB
fn generate_seq_trace(file: &mut File, start: u64, size: f64) {
    let step = 2048;
    let req_num = (size * 1024 as f64) as u64;

    for i in 0..req_num {
        let req = format!("W Hit {} {} 0.000000 0.000000\n", start + i * step, step);
        file.write_all(req.as_bytes()).unwrap();
    }
}

/// 18TB Tape 容量，请求大小为 4KB-1MB 随机，读写比例 1:1，
///
/// offset 按照 4KB(8 个扇区)对齐，size 单位为 TB
fn generate_rand_trace(file: &mut File, rng: &mut ThreadRng, size: f64) {
    // 18T => 18 * 1024 * 1024 * 1024 * 1024 / 512 / 8
    // let offset_max = (18 as f64 * 1024 as f64 * 1024 as f64 * 256 as f64) as u64;
    
    // 100G =>100 * 1024 * 1024 * 1024 / 512 / 8
    let offset_max = (100 as f64 * 1024 as f64 * 256 as f64) as u64;

    // size * 1024 * 1024 * 1024 * 1024 / 512
    let mut total_sector = (size * 1024 as f64 * 1024 as f64 * 1024 as f64 * 2 as f64) as i64;

    let blockv: Vec<u64> = (1..=256).collect();
    let rw: Vec<String> = vec!["R".to_string(), "W".to_string()];

    while total_sector > 0 {
        let offset = rng.gen::<u64>() % offset_max;
        let rw_index = rng.gen_range(0..=1);
        let block_index = rng.gen_range(0..256);
        let req = format!(
            "{} Hit {} {} 0.000000 0.000000\n",
            rw[rw_index],
            offset * 8,
            blockv[block_index] * 8
        );
        if offset * 8 + blockv[block_index] * 8 < offset_max * 8 {
            file.write_all(req.as_bytes()).unwrap();
            total_sector = total_sector - blockv[block_index] as i64 * 8;
        }
    }
    println!("Generate OK");
}

/// 将随机的 trace 和 顺序的 trace 拼接起来
fn append_trace(rand_file: &mut File, seq_file: &mut File) {
    rand_file.seek(SeekFrom::Start(0)).unwrap();
    let buf = BufReader::new(rand_file.try_clone().unwrap());
    for line in buf.lines() {
        seq_file.write_all(line.unwrap().as_bytes()).unwrap();
        seq_file.write_all("\n".as_bytes()).unwrap();
    }
}
