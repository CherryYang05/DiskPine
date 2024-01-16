pub mod error;
pub mod log;
pub mod commands;

/// 模拟器的块结构体，每个块大小是一个扇区(512B)

#[derive(Debug, Clone)]
pub struct HMSimBlock {
    pub byte: u64,
    pub block: u64
}

impl HMSimBlock {
    pub fn new() -> HMSimBlock{
        HMSimBlock { byte: 0, block: 0 }
    }
}
