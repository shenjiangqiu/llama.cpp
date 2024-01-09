use rust_utils::translate::{
    from_q5_to_u8, from_q6_to_u8, new_from_q2_to_u8, new_from_q3_to_u8, new_from_q4_to_u8,
};
use rust_utils_capi::quants::*;

pub trait TranslateMapping {
    fn map_q2(data: &[BlockQ2K]) -> Vec<u8>;
    fn map_q3(data: &[BlockQ3K]) -> Vec<u8>;
    fn map_q4(data: &[BlockQ4K]) -> Vec<u8>;
    fn map_q5(data: &[BlockQ5K]) -> Vec<u8>;
    fn map_q6(data: &[BlockQ6K]) -> Vec<u8>;
    fn map_q8(data: &[BlockQ8K]) -> Vec<u8>;
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultTranslator;
impl TranslateMapping for DefaultTranslator {
    fn map_q2(data: &[BlockQ2K]) -> Vec<u8> {
        data.iter()
            .map(|x| new_from_q2_to_u8(&x.qs))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(&x);
                acc
            })
    }

    fn map_q3(data: &[BlockQ3K]) -> Vec<u8> {
        data.iter()
            .map(|x| new_from_q3_to_u8(&x.qs, &x.hmask))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(&x);
                acc
            })
    }

    fn map_q4(data: &[BlockQ4K]) -> Vec<u8> {
        data.iter()
            .map(|x| new_from_q4_to_u8(&x.qs))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(&x);
                acc
            })
    }

    fn map_q5(data: &[BlockQ5K]) -> Vec<u8> {
        data.iter()
            .map(|x| from_q5_to_u8(&x.qs, &x.qh))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(&x);
                acc
            })
    }

    fn map_q6(data: &[BlockQ6K]) -> Vec<u8> {
        data.iter()
            .map(|x| from_q6_to_u8(&x.ql, &x.qh))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(&x);
                acc
            })
    }

    fn map_q8(data: &[BlockQ8K]) -> Vec<u8> {
        data.iter()
            .map(|x| x.qs.iter().map(|x| *x as u8))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend(x);
                acc
            })
    }
}
