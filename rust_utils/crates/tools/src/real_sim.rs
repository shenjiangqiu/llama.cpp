//! the real simulation

use std::ops::Add;

use serde::{Deserialize, Serialize};

/// return the merged bits. that is , if all number of that bit are 0, then set the result bit to 1. else, set the result bit to 0.
fn merge_bits<const SRC_BITS: u8>(src: &[u8]) -> u8 {
    // return the merged bits. that is , if all number of that bit are 0, then set the result bit to 1. else, set the result bit to 0.
    let mut result = 0u8;
    for i in 0..SRC_BITS {
        if src.iter().map(|x| (x >> i) & 1).any(|x| x == 1) {
            result |= 1 << i;
        }
    }
    result
}

fn count_continues_steps<const SRC_A_BITS: u8, const SRC_B_BITS: u8>(
    src_a: &[u8],
    src_b: &[u8],
) -> u64 {
    let a_merged_bits = merge_bits::<SRC_A_BITS>(src_a);
    let b_merged_bits = merge_bits::<SRC_B_BITS>(src_b);
    return a_merged_bits.count_ones() as u64 * b_merged_bits.count_ones() as u64;
}

pub struct RealSim<const WIDTH: u16>;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct Report {
    pub all_steps: u64,
    pub max_steps: u64,
}

impl Add for Report {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            all_steps: self.all_steps + other.all_steps,
            max_steps: self.max_steps + other.max_steps,
        }
    }
}

impl<const WIDTH: u16> RealSim<WIDTH> {
    pub fn new() -> Self {
        Self
    }
    pub fn run<const SRC_A_BITS: u8, const SRC_B_BITS: u8>(
        &self,
        src_a: &[u8],
        src_b: &[u8],
    ) -> Report {
        assert_eq!(src_a.len(), src_b.len());
        assert!(src_a.len() > 0);
        let steps = (src_a.len() + WIDTH as usize - 1) / WIDTH as usize;
        let chunk_a = src_a.chunks(WIDTH as usize);
        let chunk_b = src_b.chunks(WIDTH as usize);
        let all_steps = chunk_a
            .zip(chunk_b)
            .map(|(c_a, c_b)| count_continues_steps::<SRC_A_BITS, SRC_B_BITS>(c_a, c_b))
            .sum::<u64>();
        let max_steps = (SRC_A_BITS * SRC_B_BITS) as u64;
        let max_steps = max_steps * steps as u64;
        return Report {
            all_steps,
            max_steps,
        };
    }
}

macro_rules! impl_real_sim {
    ($name:ident;$($width:literal,$new_func:ident $(,)?);*  $(;)?) => {
        $(
            impl $name<$width> {
                pub fn $new_func() -> Self {
                    Self::new()
                }
            }
        )*
    };
}
macro_rules! impl_real_sim_run {
    ($name:ident,$report:ident;$($src_a_bits:literal,$src_b_bits:literal,$run_func:ident $(,)?);*  $(;)?) => {
        $(
            impl<const WIDTH: u16> $name<WIDTH> {

                /// Run the real simulation with the given source bit widths.
                pub fn $run_func(&self, src_a: &[u8], src_b: &[u8]) ->$report{
                    self.run::<$src_a_bits,$src_b_bits>(src_a,src_b)
                }
            }
        )*
    };
}

impl_real_sim!(RealSim;
    1,new_1;
    2,new_2;
    4,new_4;
    8,new_8;
    16,new_16;
    32,new_32;
    64,new_64;
    128,new_128;
    256,new_256;
    512,new_512;
    1024,new_1024;
    2048,new_2048;
);

impl_real_sim_run!(RealSim,Report;
    2,8,run_2_8;
    3,8,run_3_8;
    4,8,run_4_8;
    5,8,run_5_8;
    6,8,run_6_8;
    7,8,run_7_8;
    8,8,run_8_8;
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_real_sim() {
        let real_sim = RealSim::new_32();
        let src_a = [0b0100_0111u8; 4096];
        let src_b = [0b1111_0101u8; 4096];
        let report = real_sim.run_3_8(&src_a, &src_b);
        assert_eq!(report.all_steps, 3 * 6 * 4096 / 32);
        assert_eq!(report.max_steps, 3 * 8 * 4096 / 32);
    }
    #[test]
    #[should_panic]
    fn test_real_sim_panic() {
        let real_sim = RealSim::new_32();
        let src_a = [1u8; 4096];
        let src_b = [2u8; 4097];
        real_sim.run_3_8(&src_a, &src_b);
    }

    #[test]
    fn test_merge_bits() {
        let src = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000];
        let result = merge_bits::<8>(&src);
        assert_eq!(result, 0b0000_1111);
    }

    #[test]
    fn test_run() {
        let src = [0b0000_0001, 0b0000_0010, 0b0000_0100, 0b0000_1000];
        let real_sim = RealSim::new_2();
        let report = real_sim.run_5_8(&src, &src);
        println!("report is {:?}", report);
    }
}
