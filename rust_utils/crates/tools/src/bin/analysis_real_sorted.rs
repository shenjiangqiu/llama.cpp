use rust_utils_common::{
    transform::{default_map::DefaultTransform, sorted_map::SortedMap},
    translate::DefaultTranslator,
};

fn main() {
    rust_utils_tools::run_main::<DefaultTranslator, DefaultTransform, SortedMap>(
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
