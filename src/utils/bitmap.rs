// 位图数据结构
#[derive(Debug, Clone)]
pub struct BitMap {
    map: Vec<u64>,
}

// BitMapRead 和 BitMapWrite 需要实现这个 trait
pub trait BitOperation {
    
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

impl BitOperation for BitMap {
    fn get_bitmap(&mut self) -> &mut Vec<u64> {
        &mut self.map
    }
}

impl BitMap {
    pub fn new() -> Self {
        BitMap {
            map: vec![0; u32::MAX as usize],
        }
    }
}