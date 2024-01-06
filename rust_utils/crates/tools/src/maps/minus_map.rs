use rust_utils::translate::*;
use rust_utils_capi::quants::*;

use super::{shift_map::add_by_one, sorted_map::sort_by_most_zeros, TransformMapping};

pub fn minus_by_one(data: &mut Vec<u8>) {
    data.iter_mut().for_each(|x| {
        if *x & 1 == 1 {
            *x = *x - 1;
        }
    });
}

#[derive(Debug, Clone, Copy)]
pub struct MinusMap;

impl TransformMapping for MinusMap {
    fn map_q2<const WIDTH: u16>(data: &BlockQ2K) -> Vec<u8> {
        let mut data = new_from_q2_to_u8(&data.qs).to_vec();
        add_by_one(&mut data);
        minus_by_one(&mut data);
        sort_by_most_zeros(&mut data);
        data
    }
    fn map_q3<const WIDTH: u16>(data: &BlockQ3K) -> Vec<u8> {
        let mut data = new_from_q3_to_u8(&data.qs, &data.hmask).to_vec();
        add_by_one(&mut data);
        minus_by_one(&mut data);
        sort_by_most_zeros(&mut data);
        data
    }
    fn map_q4<const WIDTH: u16>(data: &BlockQ4K) -> Vec<u8> {
        let mut data = new_from_q4_to_u8(&data.qs).to_vec();
        add_by_one(&mut data);
        minus_by_one(&mut data);
        sort_by_most_zeros(&mut data);
        data
    }
    fn map_q5<const WIDTH: u16>(data: &BlockQ5K) -> Vec<u8> {
        let mut data = from_q5_to_u8(&data.qs, &data.qh).to_vec();
        add_by_one(&mut data);
        minus_by_one(&mut data);
        sort_by_most_zeros(&mut data);
        data
    }
    fn map_q6<const WIDTH: u16>(data: &BlockQ6K) -> Vec<u8> {
        let mut data = from_q6_to_u8(&data.ql, &data.qh).to_vec();
        add_by_one(&mut data);
        minus_by_one(&mut data);
        sort_by_most_zeros(&mut data);
        data
    }
    fn map_q8<const WIDTH: u16>(data: &BlockQ8K) -> Vec<u8> {
        data.qs.iter().map(|x| *x as u8).collect()
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_mao(){
        let mut data = vec![0b00000000, 0b00000001, 0b00000010, 0b00000011];
        minus_by_one(&mut data);
        assert_eq!(data, vec![0b00000000, 0b00000000, 0b00000010, 0b00000010]);
    }

    #[test]
    fn test_combo(){
        let mut data = vec![0b00000000, 0b00000001, 0b00000010, 0b00000011];
        add_by_one(&mut data);
        minus_by_one(&mut data);
        
        assert_eq!(data, vec![0b00000000, 0b00000000, 0b00000010, 0b00000100]);
    }
}