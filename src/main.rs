use std::arch::asm;
use std::time::{Duration, Instant};

const SIZE_1G: u64 = 1024 * 1024 * 1024;

fn main() {
    println!("Hello, world!");
    let block_size: u64 = SIZE_1G;
    let round = 1;
    let res = bench_bandwidth(block_size, round);
    res.desc();
}

#[derive(Default)]
struct BenchResult {
    du: Duration,
    bytes: u64,
}

impl BenchResult {
    fn new() -> Self {
        BenchResult::default()
    }

    fn desc(&self) {
        let bandwidth = (self.bytes / SIZE_1G) as f64 / self.du.as_secs_f64();
        println!(
            "time: {:?}, bytes: {}, bandwidth: {}",
            self.du, self.bytes, bandwidth
        );
    }
}

fn bench_bandwidth(block_size: u64, round: usize) -> BenchResult {
    if block_size % 64 != 0 {
        panic!("block size 必须是 64 的整数倍")
    }

    let u64_count = block_size as usize / std::mem::size_of::<u64>();
    let d: Vec<u64> = vec![1; u64_count];
    let mut res = BenchResult::new();
    let mut _read: u64 = 4;
    for _ in 0..round {
        let range = d.as_ptr_range();
        let mut cursor = range.start;
        let time = Instant::now();
        while cursor != range.end {
            unsafe {
                asm! {
                   "ld {}, 0({})",
                   out(reg) _read,
                   in(reg) cursor
                }
                cursor = cursor.offset(8);
            }
        }
        res.bytes += block_size;
        res.du += time.elapsed();
    }
    res
}
