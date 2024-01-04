use rust_utils::translate::*;
use rust_utils_capi::quants::*;

pub fn map_q2(data: &BlockQ2K) -> Vec<u8> {
    new_from_q2_to_u8(&data.qs).to_vec()
}
pub fn map_q3(data: &BlockQ3K) -> Vec<u8> {
    new_from_q3_to_u8(&data.qs, &data.hmask).to_vec()
}
pub fn map_q4(data: &BlockQ4K) -> Vec<u8> {
    new_from_q4_to_u8(&data.qs).to_vec()
}
pub fn map_q5(data: &BlockQ5K) -> Vec<u8> {
    from_q5_to_u8(&data.qs, &data.qh).to_vec()
}
pub fn map_q6(data: &BlockQ6K) -> Vec<u8> {
    from_q6_to_u8(&data.ql, &data.qh).to_vec()
}
pub fn map_q8(data: &BlockQ8K) -> Vec<u8> {
    data.qs.iter().map(|x| *x as u8).collect()
}
