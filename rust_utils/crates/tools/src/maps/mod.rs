use rust_utils_capi::quants::*;

pub mod default_map;
pub mod minus_map;
pub mod shift_map;
pub mod shift_sort_map;
pub mod sorted_map;

pub trait TransformMapping {
    fn map_q2<const WIDTH: u16>(data: &BlockQ2K) -> Vec<u8>;
    fn map_q3<const WIDTH: u16>(data: &BlockQ3K) -> Vec<u8>;
    fn map_q4<const WIDTH: u16>(data: &BlockQ4K) -> Vec<u8>;
    fn map_q5<const WIDTH: u16>(data: &BlockQ5K) -> Vec<u8>;
    fn map_q6<const WIDTH: u16>(data: &BlockQ6K) -> Vec<u8>;
    fn map_q8<const WIDTH: u16>(data: &BlockQ8K) -> Vec<u8>;
}
