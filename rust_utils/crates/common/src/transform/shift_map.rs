use crate::is_all_1;

use super::TransformMapping;

pub fn add_by_one(data: &mut [u8]) {
    data.iter_mut().for_each(|x| {
        if is_all_1::<2>(*x) {
            *x = *x + 1;
        }
    });
}
#[derive(Debug, Clone, Copy)]
pub struct ShiftMap;

impl TransformMapping for ShiftMap {
    fn transform<const BITS: u8>(src_1: &mut [u8], src_2: &mut [u8], _: usize) {
        add_by_one(src_1);
        add_by_one(src_2);
    }
}
