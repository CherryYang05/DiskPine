pub mod error;
pub mod log;
pub mod commands;
pub mod utils;

/// 模拟器的块结构体，每个块大小是一个扇区(512B)

#[derive(Debug, Clone)]
pub struct HMSimBlock {
    pub size_in_string: String,
    pub byte: u64,
    pub block: u64
}

#[derive(Debug, Clone)]
pub struct SizePair {
    pub size_begin: HMSimBlock,
    pub size_end: HMSimBlock
}

impl HMSimBlock {
    pub fn new() -> HMSimBlock{
        HMSimBlock { size_in_string: String::new(), byte: 0, block: 0 }
    }
}

impl SizePair {
    pub fn new() -> SizePair {
        SizePair { size_begin: HMSimBlock::new(), size_end: HMSimBlock::new() }
    }
}