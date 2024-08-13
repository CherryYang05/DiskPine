/// 用平均跳跃距离和归一化熵来度量工作负载的随机性

/// 平均跳跃距离结构体
#[derive(Debug, Clone)]
pub struct AJD {
    // 总跳跃距离(total_jumping_distance)
    pub tjd: f64,

    // 读请求的平均跳跃距离(avg_jumping_distance_read)
    pub ajd_r: f64,

    // 读请求个数
    pub read_count: u64,

    // 前一个请求的读写标志
    // pub prev_rw: String,

    // 前一个请求的终止地址(offset + length)
    pub prev_end: u64,
}

impl AJD {
    pub fn new() -> AJD {
        AJD {
            tjd: 0.0,
            ajd_r: 0.0,
            read_count: 0,
            // prev_rw: String::from(""),
            prev_end: 0,
        }
    }

    /// 计算平均跳跃距离，仅和读操作的前一条请求相关
    pub fn jumping(&mut self, rw: &str, offset: u64, length: u64) {
        if rw.eq_ignore_ascii_case("r") {
            let delta = offset as i64 - self.prev_end as i64;
            self.tjd = delta.abs() as f64;
            self.read_count += 1;
        }
        self.prev_end = offset + length;
    }

    // 计算平均跳跃距离
    pub fn get_jumping(&mut self) {
        self.ajd_r = self.tjd / self.read_count as f64;
    }
}

/// 负载的熵
#[derive(Debug, Clone)]
pub struct Entropy {
    // bitmap: BitMap,
    pub count: Vec<u64>,
    pub nor_entropy: f64,       // normalized_entropy 归一化熵
    pub max_offset: u32,
    pub total_req: u64
}

impl Entropy {
    pub fn new() -> Self {
        Entropy {
            // bitmap: BitMap::new(),
            count: vec![0; u32::MAX as usize],
            nor_entropy: 0.0,
            max_offset: 0,
            total_req: 0
        }
    }

    // 统计块被请求的次数，为了计算负载的熵
    pub fn block_req_count(&mut self, rw: &str, offset: u64, length: u64, block_size: u64) {
        let offset = offset / block_size;
        let length = length / block_size;

        if rw.eq_ignore_ascii_case("r") {
            for i in offset..offset + length {
                self.count[i as usize] += 1;
                self.total_req += 1;
            }
        }
    }

    // 计算负载的熵
    pub fn get_entropy(&mut self) {
        // debug!("max_offset = {}", n);
        let mut entropy: f64 = 0.0;
        for i in 0..self.max_offset {
            let cnt = self.count[i as usize] as f64;
            if cnt != 0.0 {
                // 计算访问比例
                let p = self.count[i as usize] as f64 / self.total_req as f64;
                entropy += p * p.log(2.0);
                // debug!("p = {}", p);
                // debug!("entropy = {}", entropy);
            }
        }
        
        // 返回归一化熵
        self.nor_entropy =  -entropy / (self.total_req as f64).log(2.0);
        // self.nor_entropy =  -entropy;
    }
}