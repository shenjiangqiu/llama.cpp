use rust_utils::translate::*;
use rust_utils_capi::quants::*;

fn sort_by_most_zeros(data: &mut Vec<u8>) {
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

pub fn map_q2(data: &BlockQ2K) -> Vec<u8> {
    let mut data = new_from_q2_to_u8(&data.qs).to_vec();
    sort_by_most_zeros(&mut data);
    data
}
pub fn map_q3(data: &BlockQ3K) -> Vec<u8> {
    let mut data = new_from_q3_to_u8(&data.qs, &data.hmask).to_vec();
    sort_by_most_zeros(&mut data);
    data
}
pub fn map_q4(data: &BlockQ4K) -> Vec<u8> {
    let mut data = new_from_q4_to_u8(&data.qs).to_vec();
    sort_by_most_zeros(&mut data);
    data
}
pub fn map_q5(data: &BlockQ5K) -> Vec<u8> {
    let mut data = from_q5_to_u8(&data.qs, &data.qh).to_vec();
    sort_by_most_zeros(&mut data);
    data
}
pub fn map_q6(data: &BlockQ6K) -> Vec<u8> {
    let mut data = from_q6_to_u8(&data.ql, &data.qh).to_vec();
    sort_by_most_zeros(&mut data);
    data
}
pub fn map_q8(data: &BlockQ8K) -> Vec<u8> {
    data.qs.iter().map(|x| *x as u8).collect()
}
fn main() {
    rust_utils_tools::run_main(
        map_q2,
        map_q3,
        map_q4,
        map_q5,
        map_q6,
        map_q8,
        "default_real_sim_sorted.bin",
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sort_by_most_bits() {
        let mut data = vec![
            0b1111_1111,
            0b0000_1111,
            0b0001_1111,
            0b0011_1111,
            0b0111_1111,
        ];

        let ones_count: Vec<_> = (0..8)
            .map(|bit| {
                data.iter()
                    .map(|x| ((*x >> bit) & 1) as usize)
                    .sum::<usize>()
            })
            .collect();
        println!("ones_count is {:?}", ones_count);
        let mut sorted: Vec<_> = ones_count.into_iter().enumerate().collect();
        println!("sorted is {:?}", sorted);
        sorted.sort_by_key(|x| x.1);
        println!("sorted is {:?}", sorted);
        let bit_index = sorted.into_iter().map(|x| x.0).collect::<Vec<_>>();
        println!("bit_index is {:?}", bit_index);
        data.sort_by_key(|x| {
            (
                *x & (1 << bit_index[0]),
                *x & (1 << bit_index[1]),
                *x & (1 << bit_index[2]),
                *x & (1 << bit_index[3]),
            )
        });
        println!("data is {:?}", data);
        assert_eq!(
            data,
            vec![
                0b0000_1111,
                0b0001_1111,
                0b0011_1111,
                0b0111_1111,
                0b1111_1111
            ]
        )
    }
}
