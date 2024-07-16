// 原始 trace 格式有 7 列，含义分别如下：
// Col 1: 时间戳(timestamp，单位为 100 ns)
// Col 2: 主机名(hostname)
// Col 3: 设备名称(devname)
// Col 4: 读写(rw)
// Col 5: 偏移量(offset，单位为字节)
// Col 6: 长度(length，单位为字节)
// Col 7: 响应时间(responsetime，单位为 100 ns)

// disksim 格式的 trace 各列含义如下：
// Col 1: 读写(RW)
// Col 2: Hit(暂时固定为 Hit)
// Col 3: 偏移量(offset，单位：扇区)
// Col 4: 长度(length，单位：块，扇区，即 512B)
// Col 5: 服务时间(servtime，即完成该次请求的总时间)
// Col 6: 时间戳(源码中的字段名为 nextinter)

use std::{
    fs::{remove_file, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use log::info;

use crate::error::HMSimError;

pub fn origin_to_sim(filename: &str, timestamp: bool) -> Result<(), HMSimError> {
    let file = File::open(filename)?;

    // 将文件后缀替换成 .trace
    let path = Path::new(filename);

    // parent 获取文件父目录，file_stem 获取不包含扩展名的文件名
    let new_filename = format!("{}.trace", path.file_stem().unwrap().to_string_lossy().to_string());

    remove_file(&new_filename)?;
    let mut output_file = OpenOptions::new()
        .append(true)
        .create(true)
        // .write(true)
        // .truncate(true)
        .open(new_filename)
        .unwrap();

    // 使用 BufReader 包装文件，以便按行读取
    let reader = BufReader::new(file);

    let rw = vec!["R", "W"];
    let mut nr = 1;
    let mut pre_timestamp;
    let mut next_timestamp = 0.0;

    // 遍历每一行并将其存储为 String
    for line in reader.lines() {
        let mut new_vec = vec![];

        // 每一行按照逗号(,)分隔，每一列含义见文件头注释
        let line = line?;
        let line: Vec<&str> = line.split(',').map(|item| item.trim()).collect();

        let rw_flag = if line[3].eq("Read") {
            0
        } else if line[3].eq("Write") {
            1
        } else {
            return Err(HMSimError::FileError);
        };

        // 模拟器 trace 第一个参数: 读写
        new_vec.push(rw[rw_flag]);

        // 模拟器 trace 第二个参数: Hit
        new_vec.push("Hit");

        // 模拟器 trace 第三个参数: 偏移量
        let offset = (line[4].parse::<u64>().unwrap() / 512).to_string();
        new_vec.push(offset.as_str());

        // 模拟器 trace 第四个参数: 长度
        let length = (line[5].parse::<u64>().unwrap() / 512).to_string();
        new_vec.push(length.as_str());

        // 模拟器 trace 第五个参数: 服务时间
        new_vec.push("0.000000");

        // 模拟器 trace 第六个参数: 时间戳
        let inter;

        if timestamp {
            if nr == 1 {
                pre_timestamp = line[0].parse::<f64>().unwrap();
                next_timestamp = line[0].parse::<f64>().unwrap();
                nr = 0;
            } else {
                pre_timestamp = next_timestamp;
                next_timestamp = line[0].parse::<f64>().unwrap();
            }

            inter = ((next_timestamp - pre_timestamp) / 10000 as f64).to_string();
        } else {
            inter = String::from("0.000000");
        }
        new_vec.push(inter.as_str());

        // 将转换后的结果写入新文件，如果文件存在则重新创建

        output_file.write_all(new_vec.join(" ").as_bytes()).unwrap();
        output_file.write_all("\n".as_bytes()).unwrap();
    }
    info!("origin_to_sim running done.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_filename() {
        let path = Path::new("test.csv");

        // let new_filename = format!("{}/{}.trace", path.parent().unwrap().to_string_lossy().to_string(), path.file_stem().unwrap().to_string_lossy().to_string());
        let new_filename = format!("{}.trace", path.file_stem().unwrap().to_string_lossy().to_string());

        println!("{}", new_filename);
    }
}
