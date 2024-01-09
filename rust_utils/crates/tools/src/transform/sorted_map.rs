use itertools::Itertools;

use super::ReorderMapping;

#[derive(Debug, Clone, Copy)]
pub struct SortedMap;
#[derive(Debug, Clone, Copy)]
pub struct NoSortMap;
pub fn apply_index(src: &mut [u8], index: &[usize]) {
    assert_eq!(src.len(), index.len());
    let tmp = src.to_vec();
    for (i, &x) in index.iter().enumerate() {
        src[i] = tmp[x];
    }
}
impl ReorderMapping for SortedMap {
    fn reorder<const BITS: u8>(src_1: &[u8], ne0: usize) -> Option<Vec<Vec<usize>>> {
        assert!(src_1.len() % ne0 == 0);
        let result = src_1.chunks(ne0).map(sort_by_most_zeros::<BITS>).collect();
        Some(result)
    }
}
impl ReorderMapping for NoSortMap {
    fn reorder<const BITS: u8>(_src_1: &[u8], _ne0: usize) -> Option<Vec<Vec<usize>>> {
        None
    }
}
/// return the index after sort
pub fn sort_by_most_zeros<const BITS: u8>(data: &[u8]) -> Vec<usize> {
    let ones_count: Vec<_> = (0..BITS)
        .map(|bit| {
            data.iter()
                .map(|x| ((*x >> bit) & 1) as usize)
                .sum::<usize>()
        })
        .collect();

    let mut sorted: Vec<_> = ones_count.into_iter().enumerate().collect();
    // sort by the number of ones.
    sorted.sort_by_key(|x| x.1);

    // bit index contains the bit of most zeros. because the onese from least to most. so the zeros from most to least.
    let bit_index = sorted.into_iter().map(|x| x.0).collect::<Vec<_>>();

    // the index of the original data
    let mut index = (0..data.len()).collect_vec();

    // return the index after sort(each index is the original position)
    index.sort_by(|x, y| {
        // sorted by the bit of most zeros
        let x = data[*x];
        let y = data[*y];
        for &bit in bit_index.iter() {
            let x_bit = (x >> bit) & 1;
            let y_bit = (y >> bit) & 1;
            if x_bit != y_bit {
                return x_bit.cmp(&y_bit);
            }
        }
        std::cmp::Ordering::Equal
    });
    index
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_sort() {
        let data = vec![0b0000_1111, 0b0000_0111, 0b0000_0011, 0b0000_0001];
        let index = super::sort_by_most_zeros::<4>(&data);
        assert_eq!(index, vec![3, 2, 1, 0]);
    }
    #[test]
    fn test_sort_wrong_bits() {
        let data = vec![0b0000_1111, 0b0000_0111, 0b0000_0011, 0b0000_0001];
        // if only use 3 bit , the first 2 are the same
        let index = super::sort_by_most_zeros::<3>(&data);
        assert_eq!(index, vec![3, 2, 0, 1]);
    }
    #[test]
    fn test_map() {
        let mut data = vec![0b0000_1111, 0b0000_0111, 0b0000_0011, 0b0000_0001];
        let index = super::sort_by_most_zeros::<4>(&data);
        assert_eq!(index, vec![3, 2, 1, 0]);
        super::apply_index(&mut data, &index);
        assert_eq!(
            data,
            vec![0b0000_0001, 0b0000_0011, 0b0000_0111, 0b0000_1111]
        );
    }

    #[test]
    fn test_reorder() {
        let data = vec![0b0000_1111, 0b0000_0111, 0b0000_0011, 0b0000_0001];
        let index = super::SortedMap::reorder::<4>(&data, 2);
        assert_eq!(index, Some(vec![vec![1, 0], vec![1, 0]]));
    }

    #[test]
    fn test_apply() {
        let mut data = vec![0b0000_1111, 0b0000_0111, 0b0000_0011, 0b0000_0001];
        let index = super::SortedMap::reorder::<4>(&data, 2);
        assert_eq!(index, Some(vec![vec![1, 0], vec![1, 0]]));
        let index = index.unwrap();
        data.chunks_mut(2).zip(index).for_each(|(c, i)| {
            apply_index(c, &i);
        });
        assert_eq!(
            data,
            vec![0b0000_0111, 0b0000_1111, 0b0000_0001, 0b0000_0011]
        );
    }
}
