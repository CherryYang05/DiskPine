/// 该程序作用是计算 disksim 格式的 trace 实际占用的硬盘空间
///
/// 这和 trace 表示的数据量大小不同。
///
/// 前者会包含重复的请求，故不会重复计算
///
/// trace 实际占用的空间 footprint 只计算写请求。
///


use std::{fs::File, io::BufRead, io::BufReader};
use crate::utils::{bitmap::*, randomness::{Entropy, AJD}};


use crate::error::HMSimError;

/// 计算 trace 的数据量、落盘量，以及平均跳跃距离和归一化熵
pub fn trace_property(filename: &str) -> Result<(String, String, f64, f64), HMSimError> {
    // 位图
    let mut bitmap_write = BitMap::new();

    let mut jump = AJD::new();
    let mut entropy = Entropy::new();

    let buf = BufReader::new(File::open(filename).unwrap());

    let mut max_index: u64 = 0;
    let mut min_index: u64 = u64::MAX;
    let mut max_index_length = 0;

    let mut volume: u64 = 0;
    let mut footprint: u64 = 0;

    // 从 trace 中解析读写、长度以及偏移量字段
    for (_index, line) in buf.lines().enumerate() {
        // if index % 10000 == 0 {
        //     info!("{}", index);
        // }
        let line = line?;

        // 存储每一行 trace
        let data: Vec<&str> = line.split(' ').collect();

        update_min_max(&mut min_index, &mut max_index, &mut max_index_length, data[2], data[3]);

        let mut real: u64 = 0;
        let offset: u64 = data[2].parse::<u64>().unwrap();
        let len: u64 = data[3].parse::<u64>().unwrap();

        jump.jumping(data[0], offset, len);
        entropy.block_req_count(data[0], offset, len, 512);

        if data[0].eq("W") {
            for i in offset..offset + len {
                // 如果该位没有设置，real++，然后将该位设置为 1
                if bitmap_write.test_bit(i) == false {
                    real += 1;
                    bitmap_write.set_bit(i);
                }
            }
            footprint += real;
        }

        volume += len;

    }
    let (footprint, volume) = convert_to_str(footprint, volume);

    entropy.max_offset = (max_index + max_index_length) as u32 / 512;

    jump.get_jumping();
    entropy.get_entropy();

    // info!("entropy: {:.4}", entropy.normalized_entropy);

    Ok((footprint, volume, jump.ajd_r, entropy.nor_entropy))
}

fn update_min_max(min_index: &mut u64, max_index: &mut u64, max_index_length: &mut u64, offset: &str, length: &str) {
    let offset = offset.parse::<u64>().unwrap();
    let length: u64 = length.parse().unwrap();
    if offset < *min_index {
        *min_index = offset;
    }

    if offset > *max_index {
        *max_index = offset;
        *max_index_length = length;
    }
}

/// 判断容量是否超限，如果超限则扩容
// fn overflow<F>(mut bitmap: F, max_index: &mut u64, length: &str) -> bool
// where
//     F: BitOperation,
// {
//     let length = length.parse::<u64>().unwrap();
//     let count = (*max_index + length) / 64 + 1;
//     if count >= bitmap.get_bitmap().capacity() as u64 {
//         bitmap.get_bitmap().resize(count as usize, 0);
//     }
//     true
// }

/// 将容量转化为以 KB、MB、GB、TB 易读的形式
fn convert_to_str(footprint: u64, volume: u64) -> (String, String) {
    let suffix = vec!["KB", "MB", "GB", "TB", "PB"];

    // 单位是块(512B)
    let mut footprint = footprint as f64 / 2 as f64;
    let mut volume = volume as f64 / 2 as f64;

    let footprint_str;
    let volume_str;

    let mut cnt = 0;

    loop {
        if footprint < 512 as f64 {
            // 保留两位小数
            footprint_str = format!("{:.2}{}", footprint, suffix[cnt]);
            break;
        }
        footprint /= 1024 as f64;
        cnt += 1;
    }

    cnt = 0;

    loop {
        if volume < 512 as f64 {
            // 保留两位小数
            volume_str = format!("{:.2}{}", volume, suffix[cnt]);
            break;
        }
        volume /= 1024 as f64;
        cnt += 1;
    }

    (footprint_str, volume_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let res = trace_property("ts_0_validate.trace").unwrap();
        println!("{:?}", res);
    }

    #[test]
    fn test_convert_to_str() {
        let footprint = 1424000;
        let volume = 208777;
        println!("{:?}", convert_to_str(footprint, volume));
    }
}
