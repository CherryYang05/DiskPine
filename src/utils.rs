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

pub fn rate_to_num(size: &str) -> Result<(f32, f32), HMSimError> {
    Ok(parse_colon(size)?)
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
    size: HMSimBlock,
    rwrate: (f32, f32),
    write_size: Option<SizePair>,
    read_size: Option<SizePair>,
    rwsize: Option<SizePair>,
    batch: Option<String>,
    batch_write_size: Option<SizePair>,
    batch_read_size: Option<SizePair>,
) -> Result<TapeTrace, HMSimError> {
    let mut tape_trace = TapeTrace::new();

    tape_trace.total_size = size.block;

    tape_trace.read_rate = rwrate.0;
    tape_trace.write_rate = rwrate.1;

    if let Some(size_pair) = write_size {
        tape_trace.write_size_start = size_pair.size_begin.block;
        tape_trace.write_size_end = size_pair.size_end.block;
        tape_trace.write_size_range = size_pair.size_end.block - size_pair.size_begin.block;
    } else {
        tape_trace.write_size_start = 0;
        tape_trace.write_size_end = 0;
        tape_trace.write_size_range = 0;
    }

    if let Some(size_pair) = read_size {
        tape_trace.read_size_start = size_pair.size_begin.block;
        tape_trace.read_size_end = size_pair.size_end.block;
        tape_trace.read_size_range = size_pair.size_end.block - size_pair.size_begin.block;
    } else {
        tape_trace.read_size_start = 0;
        tape_trace.read_size_end = 0;
        tape_trace.read_size_range = 0;
    }

    if let Some(size_pair) = rwsize {
        tape_trace.rwsize_start = size_pair.size_begin.block;
        tape_trace.rwsize_end = size_pair.size_end.block;
        tape_trace.rwsize_range = size_pair.size_end.block - size_pair.size_begin.block;
    } else {
        tape_trace.rwsize_start = 0;
        tape_trace.rwsize_end = 0;
        tape_trace.rwsize_range = 0;
    }

    if let Some(size_pair) = batch_write_size {
        tape_trace.batch_write_size_begin = size_pair.size_begin.block;
        tape_trace.batch_write_size_end = size_pair.size_end.block;
        tape_trace.batch_write_size_range = size_pair.size_end.block - size_pair.size_begin.block;
    } else {
        tape_trace.batch_write_size_begin = 0;
        tape_trace.batch_write_size_end = 0;
        tape_trace.batch_write_size_range = 0;
    }

    if let Some(size_pair) = batch_read_size {
        tape_trace.batch_read_size_begin = size_pair.size_begin.block;
        tape_trace.batch_read_size_end = size_pair.size_end.block;
        tape_trace.batch_read_size_range = size_pair.size_end.block - size_pair.size_begin.block;
    } else {
        tape_trace.batch_read_size_begin = 0;
        tape_trace.batch_read_size_end = 0;
        tape_trace.batch_read_size_range = 0;
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
        let size = "1M-2M";
        println!("{:?}", size_range_to_start_end(size));
    }

    // fn test_command_gen_tape_trace_to_tape_trace_struct() {
    //     let tape_trace = TapeTrace::new();
    // }
}
