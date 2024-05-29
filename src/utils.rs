use regex::Regex;

use crate::{commands::generate_tape_trace::TapeTrace, error::HMSimError, HMSimBlock, SizePair};

/// 将以 KB, MB 为单位的字符串转化成 HMSimBlock 结构体(即以扇区为单位)
pub fn string_to_hmsim_block(size: &str) -> Result<HMSimBlock, HMSimError> {
    Ok(unit_parse(size)?)
}

/// 将字符串表示的范围大小转化成 SizePair 结构体
pub fn size_range_to_start_end(size: &str) -> Result<SizePair, HMSimError> {
    Ok(parse_dash(size)?)
}

/// 将形如 a:b 的形式转化为 (f32, f32)
pub fn rate_to_num(size: &str) -> Result<(f32, f32), HMSimError> {
    Ok(parse_colon(size)?)
}

/// 将形如 a-b 的形式转化为 (u64, u64)
pub fn range_to_num(size: &str) -> Result<(u64, u64), HMSimError> {
    Ok(parse_dash_num(size)?)
}

/// 把用横杠(-)分隔的两个字符转化成两个 HMSimBlock 结构体
fn parse_dash(size: &str) -> Result<SizePair, HMSimError> {
    let regex = Regex::new(r"(\d+[A-Za-z]+)-(\d+[A-Za-z]+)").unwrap();

    let mut size_parse_pair = SizePair::new();

    if let Some(captures) = regex.captures(size) {
        size_parse_pair.size_begin = unit_parse(&captures[1])?;
        size_parse_pair.size_end = unit_parse(&captures[2])?;
        Ok(size_parse_pair)
    } else {
        return Err(HMSimError::ParseError);
    }
}

/// 把用横杠(-)分隔的两个字符转化成两个 HMSimBlock 结构体
fn parse_dash_num(size: &str) -> Result<(u64, u64), HMSimError> {
    let regex = Regex::new(r"(\d+)-(\d+)").unwrap();

    if let Some(captures) = regex.captures(size) {
        let first = captures[1].parse::<u64>().unwrap();
        let second = captures[2].parse::<u64>().unwrap();
        Ok((first, second))
    } else {
        return Err(HMSimError::ParseError);
    }
}

/// 把用冒号分隔的两个字符转化成两个数字
fn parse_colon(size: &str) -> Result<(f32, f32), HMSimError> {
    let regex = Regex::new(r"(\d+):(\d+)").unwrap();

    if let Some(captures) = regex.captures(size) {
        let first = captures[1].parse::<f32>().unwrap();
        let second = captures[2].parse::<f32>().unwrap();
        Ok((first, second))
    } else {
        return Err(HMSimError::ParseError);
    }
}

/// 对 KB, MB 为单位的字符串进行正则匹配
fn unit_parse(size: &str) -> Result<HMSimBlock, HMSimError> {
    let regex = Regex::new(r"(\d+)([A-Za-z]+)").unwrap();

    let mut hmsim_block = HMSimBlock::new();

    hmsim_block.size_in_string = size.to_string();

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
            return Err(HMSimError::ParseError);
        }

        hmsim_block.block = hmsim_block.byte / 512;
    } else {
        return Err(HMSimError::ParseError);
    }

    Ok(hmsim_block)
}

/// 将子命令参数转化为 TapeTrace 结构体
pub fn command_gen_tape_trace_to_tape_trace_struct(
    total_size: HMSimBlock,
    // rwrate: (f32, f32),
    block_size: HMSimBlock,
    write_size: Option<(u64, u64)>,
    read_size: Option<(u64, u64)>,
    rwsize: Option<(u64, u64)>,
    batch: Option<String>,
    batch_iow_num: Option<(u64, u64)>,
    batch_ior_num: Option<(u64, u64)>,
) -> Result<TapeTrace, HMSimError> {
    let mut tape_trace = TapeTrace::new();

    tape_trace.total_size = total_size.block;

    tape_trace.block_size = block_size.block;

    // tape_trace.read_rate = rwrate.0;
    // tape_trace.write_rate = rwrate.1;

    if let Some(size_pair) = write_size {
        tape_trace.write_size_start = size_pair.0;
        tape_trace.write_size_end = size_pair.1;
        tape_trace.write_size_range = size_pair.1 - size_pair.0;
    } else {
        tape_trace.write_size_start = 0;
        tape_trace.write_size_end = 0;
        tape_trace.write_size_range = 0;
    }

    if let Some(size_pair) = read_size {
        tape_trace.read_size_start = size_pair.0;
        tape_trace.read_size_end = size_pair.1;
        tape_trace.read_size_range = size_pair.1 - size_pair.0;
    } else {
        tape_trace.read_size_start = 0;
        tape_trace.read_size_end = 0;
        tape_trace.read_size_range = 0;
    }

    if let Some(size_pair) = rwsize {
        tape_trace.rwsize_start = size_pair.0;
        tape_trace.rwsize_end = size_pair.1;
        tape_trace.rwsize_range = size_pair.1 - size_pair.0;
    } else {
        tape_trace.rwsize_start = 0;
        tape_trace.rwsize_end = 0;
        tape_trace.rwsize_range = 0;
    }

    if let Some(size_pair) = batch_iow_num {
        tape_trace.batch_iow_num_begin = size_pair.0;
        tape_trace.batch_iow_num_end = size_pair.1;
        tape_trace.batch_iow_num_range = size_pair.1 - size_pair.0;
    } else {
        tape_trace.batch_iow_num_begin = 0;
        tape_trace.batch_iow_num_end = 0;
        tape_trace.batch_iow_num_range = 0;
    }

    if let Some(size_pair) = batch_ior_num {
        tape_trace.batch_ior_num_begin = size_pair.0;
        tape_trace.batch_ior_num_end = size_pair.1;
        tape_trace.batch_ior_num_range = size_pair.1 - size_pair.0;
    } else {
        tape_trace.batch_iow_num_begin = 0;
        tape_trace.batch_iow_num_end = 0;
        tape_trace.batch_iow_num_range = 0;
    }

    if let Some(batch) = batch {
        tape_trace.batch = batch;
    } else {
        tape_trace.batch = String::new();
    }

    Ok(tape_trace)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_unit_parse() {
        let size = "5-15";
        println!("{:?}", range_to_num(size));
    }
    
    #[test]
    fn test_block_size() {
        let size = "12M";
        println!("{:?}", string_to_hmsim_block(size));
    }
    // fn test_command_gen_tape_trace_to_tape_trace_struct() {
    //     let tape_trace = TapeTrace::new();
    // }
}
