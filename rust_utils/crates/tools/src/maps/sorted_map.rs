use rust_utils::translate::*;
use rust_utils_capi::quants::*;

use super::TransformMapping;

#[derive(Debug, Clone, Copy)]
pub struct SortedMap;

impl TransformMapping for SortedMap {
    fn map_q2<const WIDTH: u16>(data: &BlockQ2K) -> Vec<u8> {
        let mut data = new_from_q2_to_u8(&data.qs).to_vec();
        sort_by_most_zeros(&mut data);
        data
    }
    fn map_q3<const WIDTH: u16>(data: &BlockQ3K) -> Vec<u8> {
        let mut data = new_from_q3_to_u8(&data.qs, &data.hmask).to_vec();
        sort_by_most_zeros(&mut data);
        data
    }
    fn map_q4<const WIDTH: u16>(data: &BlockQ4K) -> Vec<u8> {
        let mut data = new_from_q4_to_u8(&data.qs).to_vec();
        sort_by_most_zeros(&mut data);
        data
    }
    fn map_q5<const WIDTH: u16>(data: &BlockQ5K) -> Vec<u8> {
        let mut data = from_q5_to_u8(&data.qs, &data.qh).to_vec();
        sort_by_most_zeros(&mut data);
        data
    }
    fn map_q6<const WIDTH: u16>(data: &BlockQ6K) -> Vec<u8> {
        let mut data = from_q6_to_u8(&data.ql, &data.qh).to_vec();
        sort_by_most_zeros(&mut data);
        data
    }
    fn map_q8<const WIDTH: u16>(data: &BlockQ8K) -> Vec<u8> {
        data.qs.iter().map(|x| *x as u8).collect()
    }
}

pub fn sort_by_most_zeros(data: &mut Vec<u8>) {
    let ones_count: Vec<_> = (0..8)
        .map(|bit| {
            data.iter()
                .map(|x| ((*x >> bit) & 1) as usize)
                .sum::<usize>()
        })
        .collect();
    let mut sorted: Vec<_> = ones_count.into_iter().enumerate().collect();
    sorted.sort_by_key(|x| x.1);
    let bit_index = sorted.into_iter().map(|x| x.0).collect::<Vec<_>>();
    data.sort_by_key(|x| {
        (
            *x & (1 << bit_index[0]),
            *x & (1 << bit_index[1]),
            *x & (1 << bit_index[2]),
            *x & (1 << bit_index[3]),
        )
    })
}
