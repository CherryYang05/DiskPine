/// 该程序作用是计算 disksim 格式的 trace 实际占用的硬盘空间
///
/// 这和 trace 表示的数据量大小不同。
///
/// 前者会包含重复的请求，故不会重复计算
///
/// trace 实际占用的空间 footprint 只计算写请求。
///


use std::{fs::File, io::BufRead, io::BufReader};

use log::info;

use crate::error::HMSimError;

// 位图数据结构
struct BitMapRead {
    map: Vec<u64>,
}

struct BitMapWrite {
    map: Vec<u64>,
}

// BitMapRead 和 BitMapWrite 需要实现这个 trait
trait BitOperation {
    
    // 获取底层位图数据
    fn get_bitmap(&mut self) -> &mut Vec<u64>;

    // 设置某一位(置为 1)
    fn set_bit(&mut self, index: u64) -> bool {
        let block = index / 64;
        let bit = index % 64;
        self.get_bitmap()[block as usize] |= 0x01 << bit;
        true
    }

    // 重置某一位(置为 0)
    fn reset_bit(&mut self, index: u64) -> bool {
        let block = index / 64;
        let bit = index % 64;
        self.get_bitmap()[block as usize] &= 0x00 << bit;
        true
    }

    // 判断某一位是否被设置
    fn test_bit(&mut self, index: u64) -> bool {
        let block = index / 64;
        let bit = index % 64;
        if (self.get_bitmap()[block as usize] & (0x01 << bit)) != 0 {
            true
        } else {
            false
        }
    }
}

impl BitOperation for BitMapRead {
    fn get_bitmap(&mut self) -> &mut Vec<u64> {
        &mut self.map
    }
}

impl BitOperation for BitMapWrite {
    fn get_bitmap(&mut self) -> &mut Vec<u64> {
        &mut self.map
    }
}

// impl BitMapRead {
//     fn new() -> Self {
//         BitMapRead {
//             map: vec![0; u32::MAX as usize],
//         }
//     }
// }

impl BitMapWrite {
    fn new() -> Self {
        BitMapWrite {
            map: vec![0; u32::MAX as usize],
        }
    }
}

/// 计算 trace 的数据量及落盘量
pub fn trace_foot_size(filename: &str) -> Result<(String, String), HMSimError> {
    // 位图
    // let mut bitmap_read = BitMapRead::new();
    let mut bitmap_write = BitMapWrite::new();

    let buf = BufReader::new(File::open(filename).unwrap());

    let mut max_index: u64 = 0;
    let mut min_index: u64 = u64::MAX;

    let mut volume: u64 = 0;
    let mut footprint: u64 = 0;

    // 从 trace 中解析读写、长度以及偏移量字段
    for (index, line) in buf.lines().enumerate() {
        // if index % 10000 == 0 {
        //     info!("{}", index);
        // }
        let line = line?;

        // 存储每一行 trace
        let data: Vec<&str> = line.split(' ').collect();

        update_min_max(&mut min_index, &mut max_index, data[2]);

        let mut real: u64 = 0;
        let offset: u64 = data[2].parse::<u64>().unwrap();
        let len: u64 = data[3].parse::<u64>().unwrap();

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
    Ok((footprint, volume))
}

fn update_min_max(min_index: &mut u64, max_index: &mut u64, offset: &str) {
    let offset = offset.parse::<u64>().unwrap();
    if offset < *min_index {
        *min_index = offset;
    }

    if offset > *max_index {
        *max_index = offset;
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
        let res = trace_foot_size("ts_0_validate.trace").unwrap();
        println!("{:?}", res);
    }

    #[test]
    fn test_convert_to_str() {
        let footprint = 1424000;
        let volume = 208777;
        println!("{:?}", convert_to_str(footprint, volume));
    }
}
