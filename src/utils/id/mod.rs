use std::sync::{Arc, Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

/*定义相关常量*/
// 开始时间戳（2022-08-01）
const START_TIME: u128 = 1659283200000;
// 工作id所占位数
const WORKER_ID_BITS: u64 = 10;
// 序列号所占的位数
const SEQUENCE_BITS: u64 = 12;
// 工作节点标识ID向左移12位
const WORKER_ID_SHIFT: u64 = SEQUENCE_BITS;
// 时间戳向左移动22位（12位序列号+5位工作节点+5位数据节点）
const TIMESTAMP_LEFT_SHIFT: u64 = SEQUENCE_BITS + WORKER_ID_BITS;
// 生成的序列掩码，这里是4095
const SEQUENCE_MASK: u64 = (-1 ^ (-1 << SEQUENCE_BITS)) as u64;

/**
对外暴露的解库，用于通过雪花算法生成唯一性id
*/
pub fn generate_id() -> i64 {
    SNOW.get().unwrap().clone().next_id() as i64
}

static SNOW: OnceLock<SnowflakeIdWorker> = OnceLock::new();
pub fn init_snow() {
    let _ = SNOW.set(SnowflakeIdWorker::new(20));
}

#[derive(Clone)]
struct SnowflakeIdWorker(Arc<Mutex<SnowflakeIdWorkerInner>>);

impl SnowflakeIdWorker {
    pub fn new(worker_id: u64) -> SnowflakeIdWorker {
        Self(Arc::new(Mutex::new(SnowflakeIdWorkerInner::new(worker_id))))
    }

    pub fn next_id(&self) -> u64 {
        match self.0.lock() {
            Ok(mut it) => it.next_id(),
            Err(e) => {
                panic!("there is some wrong in get id,wrong msg:{}", e)
            }
        }
    }
}

// 这是一个内部结构体，只在这个mod里面使用
struct SnowflakeIdWorkerInner {
    // 工作节点ID
    worker_id: u64,
    // 序列号
    sequence: u64,
    // 上一次时间戳
    last_timestamp: u128,
}

impl SnowflakeIdWorkerInner {
    fn new(worker_id: u64) -> SnowflakeIdWorkerInner {
        // 创建SnowflakeIdWorkerInner对象
        SnowflakeIdWorkerInner {
            worker_id,
            sequence: 0,
            last_timestamp: 0,
        }
    }

    // 获取下一个id
    fn next_id(&mut self) -> u64 {
        // 获取当前时间戳
        let mut timestamp = Self::get_time();
        // 如果当前时间戳小于上一次的时间戳，那么跑异常
        if timestamp < self.last_timestamp {
            panic!(
                "{}",
                format!(
                    "Clock moved backwards.  Refusing to generate id for {} milliseconds",
                    self.last_timestamp - timestamp
                )
            );
        }
        // 如果当前时间戳等于上一次的时间戳，那么计算出序列号目前是第几位
        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & SEQUENCE_MASK;
            // 如果计算出来的序列号等于0，那么重新获取当前时间戳
            if self.sequence == 0 {
                timestamp = Self::til_next_mills(self.last_timestamp);
            }
        } else {
            // 如果当前时间戳大于上一次的时间戳，序列号置为0。因为又开始了新的毫秒，所以序列号要从0开始。
            self.sequence = 0;
        }
        // 把当前时间戳赋值给last_timestamp，以便下一次计算next_id
        self.last_timestamp = timestamp;
        // 把上面计算得到的对应数值按要求移位拼接起来
        ((((timestamp - START_TIME) << TIMESTAMP_LEFT_SHIFT) as u64)
            | (self.worker_id << WORKER_ID_SHIFT)
            | self.sequence)
            & (!(1u64 << 63))
    }
    // 计算一个大于上一次时间戳的时间戳
    fn til_next_mills(last_timestamp: u128) -> u128 {
        // 获取当前时间戳
        let mut timestamp = Self::get_time();
        // 如果当前时间戳一直小于上次时间戳，那么一直循环获取，直至当前时间戳大于上次获取的时间戳
        while timestamp <= last_timestamp {
            timestamp = Self::get_time();
        }
        // 返回满足要求的时间戳
        timestamp
    }

    // 获取当前时间戳
    fn get_time() -> u128 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(s) => s.as_millis(),
            Err(e) => {
                panic!("there is some wrong in get time in id,wrong msg:{}", e)
            }
        }
    }
}
