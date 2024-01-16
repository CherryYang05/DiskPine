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
