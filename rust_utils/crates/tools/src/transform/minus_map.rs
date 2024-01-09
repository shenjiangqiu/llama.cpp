use super::TransformMapping;

pub fn minus_by_one(data: &mut [u8]) {
    data.iter_mut().for_each(|x| {
        if *x & 1 == 1 {
            *x = *x - 1;
        }
    });
}

#[derive(Debug, Clone, Copy)]
pub struct MinusMap;

impl TransformMapping for MinusMap {
    fn transform<const BITS:u8>(src_1: &mut [u8], src_2: &mut [u8],_:usize) {
        minus_by_one(src_1);
        minus_by_one(src_2);
    }
}

#[cfg(test)]
mod tests {
    use crate::transform::shift_map::add_by_one;

    use super::*;
    #[test]
    fn test_mao() {
        let mut data = vec![0b00000000, 0b00000001, 0b00000010, 0b00000011];
        minus_by_one(&mut data);
        assert_eq!(data, vec![0b00000000, 0b00000000, 0b00000010, 0b00000010]);
    }

    #[test]
    fn test_combo() {
        let mut data = vec![0b00000000, 0b00000001, 0b00000010, 0b00000011];
        add_by_one(&mut data);
        minus_by_one(&mut data);

        assert_eq!(data, vec![0b00000000, 0b00000000, 0b00000010, 0b00000100]);
    }
}
