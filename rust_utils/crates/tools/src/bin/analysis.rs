use std::{collections::BTreeMap, fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use clap_derive::Parser;
use rust_utils_capi::quants::*;
use rust_utils_tools::is_all_1;

#[derive(Parser)]
struct Cli {
    /// the path to store the bin data
    folder: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let q2: BTreeMap<String, Vec<BlockQ2K>> = bincode::deserialize_from(BufReader::new(
        File::open(cli.folder.join("ALL_DATA_Q2.bin")).unwrap(),
    ))
    .unwrap();
    let q3: BTreeMap<String, Vec<BlockQ3K>> = bincode::deserialize_from(BufReader::new(
        File::open(cli.folder.join("ALL_DATA_Q3.bin")).unwrap(),
    ))
    .unwrap();
    let q4: BTreeMap<String, Vec<BlockQ4K>> = bincode::deserialize_from(BufReader::new(
        File::open(cli.folder.join("ALL_DATA_Q4.bin")).unwrap(),
    ))
    .unwrap();
    let q5: BTreeMap<String, Vec<BlockQ5K>> = bincode::deserialize_from(BufReader::new(
        File::open(cli.folder.join("ALL_DATA_Q5.bin")).unwrap(),
    ))
    .unwrap();
    let q6: BTreeMap<String, Vec<BlockQ6K>> = bincode::deserialize_from(BufReader::new(
        File::open(cli.folder.join("ALL_DATA_Q6.bin")).unwrap(),
    ))
    .unwrap();
    let q8: BTreeMap<String, Vec<BlockQ8K>> = bincode::deserialize_from(BufReader::new(
        File::open(cli.folder.join("ALL_DATA_Q8.bin")).unwrap(),
    ))
    .unwrap();

    for (name, data) in q2 {
        let total_count = data.len() * 256;
        let (count, mg_count) = data
            .into_iter()
            .map(|x| {
                let data_8 = rust_utils::translate::new_from_q2_to_u8(&x.qs);
                (analysis_zeros::<2>(&data_8), analysis_zeros::<2>(&data_8))
            })
            .reduce(|mut x, y| {
                x.0.iter_mut().zip(y.0.iter()).for_each(|(a, b)| *a += b);
                x.1.iter_mut().zip(y.1.iter()).for_each(|(a, b)| *a += b);
                x
            })
            .unwrap();
        println!("q2 {name} count: {:?}", count);
        let count_percent = count
            .iter()
            .map(|&x| x as f64 / total_count as f64)
            .collect::<Vec<_>>();
        println!("total count: {}", total_count);
        println!("q2 {name} count percent: {:?}", count_percent);
        let mg_count_percent_to_count = mg_count
            .iter()
            .zip(count)
            .map(|(&x, y)| x as f64 / y as f64)
            .collect::<Vec<_>>();
        println!(
            "q2 {name} mg count percent to count: {:?}",
            mg_count_percent_to_count
        );
    }
    for (name, data) in q3 {
        let total_count = data.len() * 256;
        let (count, mg_count) = data
            .into_iter()
            .map(|x| {
                let data_8 = rust_utils::translate::new_from_q3_to_u8(&x.qs, &x.hmask);
                (
                    analysis_zeros::<3>(&data_8),
                    analysis_zero_add_bit::<2, 3>(&data_8),
                )
            })
            .reduce(|mut x, y| {
                x.0.iter_mut().zip(y.0.iter()).for_each(|(a, b)| *a += b);
                x.1.iter_mut().zip(y.1.iter()).for_each(|(a, b)| *a += b);
                x
            })
            .unwrap();
        println!("q3 {name} count: {:?}", count);
        let count_percent = count
            .iter()
            .map(|&x| x as f64 / total_count as f64)
            .collect::<Vec<_>>();
        println!("total count: {}", total_count);
        println!("q3 {name} count percent: {:?}", count_percent);
        let mg_count_percent_to_count = mg_count
            .iter()
            .zip(count)
            .map(|(&x, y)| x as f64 / y as f64)
            .collect::<Vec<_>>();
        println!(
            "q2 {name} mg count percent to count: {:?}",
            mg_count_percent_to_count
        );
    }
    //q4
    for (name, data) in q4 {
        let total_count = data.len() * 256;
        let (count, mg_count) = data
            .into_iter()
            .map(|x| {
                let data_8 = rust_utils::translate::new_from_q4_to_u8(&x.qs);
                (
                    analysis_zeros::<4>(&data_8),
                    analysis_zero_add_bit::<2, 4>(&data_8),
                )
            })
            .reduce(|mut x, y| {
                x.0.iter_mut().zip(y.0.iter()).for_each(|(a, b)| *a += b);
                x.1.iter_mut().zip(y.1.iter()).for_each(|(a, b)| *a += b);
                x
            })
            .unwrap();
        println!("q4 {name} count: {:?}", count);
        let count_percent = count
            .iter()
            .map(|&x| x as f64 / total_count as f64)
            .collect::<Vec<_>>();
        println!("total count: {}", total_count);
        println!("q4 {name} count percent: {:?}", count_percent);
        let mg_count_percent_to_count = mg_count
            .iter()
            .zip(count)
            .map(|(&x, y)| x as f64 / y as f64)
            .collect::<Vec<_>>();
        println!(
            "q2 {name} mg count percent to count: {:?}",
            mg_count_percent_to_count
        );
    }
    //q5
    for (name, data) in q5 {
        let total_count = data.len() * 256;
        let (count, mg_count) = data
            .into_iter()
            .map(|x| {
                let x = rust_utils::translate::from_q5_to_u8(&x.qs, &x.qh);

                (analysis_zeros::<5>(&x), analysis_zero_add_bit::<2, 5>(&x))
            })
            .reduce(|mut x, y| {
                x.0.iter_mut().zip(y.0.iter()).for_each(|(a, b)| *a += b);
                x.1.iter_mut().zip(y.1.iter()).for_each(|(a, b)| *a += b);
                x
            })
            .unwrap();
        println!("q5 {name} count: {:?}", count);
        let count_percent = count
            .iter()
            .map(|&x| x as f64 / total_count as f64)
            .collect::<Vec<_>>();
        println!("total count: {}", total_count);
        println!("q5 {name} count percent: {:?}", count_percent);
        let mg_count_percent_to_count = mg_count
            .iter()
            .zip(count)
            .map(|(&x, y)| x as f64 / y as f64)
            .collect::<Vec<_>>();
        println!(
            "q2 {name} mg count percent to count: {:?}",
            mg_count_percent_to_count
        );
    }
    //q6
    for (name, data) in q6 {
        let total_count = data.len() * 256;
        let (count, mg_count) = data
            .into_iter()
            .map(|x| {
                let x = rust_utils::translate::from_q6_to_u8(&x.ql, &x.qh);

                (analysis_zeros::<6>(&x), analysis_zero_add_bit::<2, 6>(&x))
            })
            .reduce(|mut x, y| {
                x.0.iter_mut().zip(y.0.iter()).for_each(|(a, b)| *a += b);
                x.1.iter_mut().zip(y.1.iter()).for_each(|(a, b)| *a += b);
                x
            })
            .unwrap();
        println!("q6 {name} count: {:?}", count);
        let count_percent = count
            .iter()
            .map(|&x| x as f64 / total_count as f64)
            .collect::<Vec<_>>();
        println!("total count: {}", total_count);
        println!("q6 {name} count percent: {:?}", count_percent);
        let mg_count_percent_to_count = mg_count
            .iter()
            .zip(count)
            .map(|(&x, y)| x as f64 / y as f64)
            .collect::<Vec<_>>();
        println!(
            "q2 {name} mg count percent to count: {:?}",
            mg_count_percent_to_count
        );
    }
    //q8
    for (name, data) in q8 {
        let total_count = data.len() * 256;
        let (count, mg_count) = data
            .into_iter()
            .map(|x| {
                let u8_data = x.qs.iter().map(|x| *x as u8).collect::<Vec<_>>();
                (
                    analysis_zeros::<8>(&u8_data),
                    analysis_zero_add_bit::<2, 8>(&u8_data),
                )
            })
            .reduce(|mut x, y| {
                x.0.iter_mut().zip(y.0.iter()).for_each(|(a, b)| *a += b);
                x.1.iter_mut().zip(y.1.iter()).for_each(|(a, b)| *a += b);
                x
            })
            .unwrap();
        println!("q8 {name} count: {:?}", count);
        let count_percent = count
            .iter()
            .map(|&x| x as f64 / total_count as f64)
            .collect::<Vec<_>>();
        println!("total count: {}", total_count);
        println!("q8 {name} count percent: {:?}", count_percent);
        let mg_count_percent_to_count = mg_count
            .iter()
            .zip(count)
            .map(|(&x, y)| x as f64 / y as f64)
            .collect::<Vec<_>>();
        println!(
            "q2 {name} mg count percent to count: {:?}",
            mg_count_percent_to_count
        );
    }
}
fn analysis_zeros<const WIDTH: usize>(data: &[u8]) -> [usize; WIDTH] {
    let mut count = [0; WIDTH];
    data.iter().for_each(|&x| {
        for i in 0..WIDTH {
            if x & (1 << i) == 0 {
                count[i] += 1;
            }
        }
    });
    count
}


fn analysis_zero_add_bit<const TH: usize, const WIDTH: usize>(data: &[u8]) -> [usize; WIDTH] {
    let mut count = [0; WIDTH];
    data.iter().for_each(|&x| {
        let mut x = x;
        if is_all_1::<TH>(x) {
            x += 1;
        }
        for i in 0..WIDTH {
            if x & (1 << i) == 0 {
                count[i] += 1;
            }
        }
    });
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_analysis_zeros() {
        let data = [0b0000_0000, 0b0000_0001, 0b0000_0010, 0b0000_0011];
        let count = analysis_zeros::<8>(&data);
        assert_eq!(count, [2, 2, 4, 4, 4, 4, 4, 4]);
        let count = analysis_zeros::<2>(&data);
        assert_eq!(count, [2, 2,]);
    }
    #[test]
    #[ignore = "this test is too slow"]
    fn test_deserialization() {
        let data: BTreeMap<String, Vec<BlockQ2K>> =
            bincode::deserialize_from(BufReader::new(File::open("../../q2data/q2.bin").unwrap())).unwrap();
        for (k, v) in data {
            println!("{}: {}", k, v.len());
            for b in v {
                let result: [_; 3] =
                    analysis_zeros(&rust_utils::translate::new_from_q2_to_u8(&b.qs));
                assert_eq!(result[2], 256);
            }
        }
    }
    #[test]
    fn test_all_1() {
        let r = is_all_1::<3>(7);
        assert!(r);
        let r = is_all_1::<3>(6);
        assert!(!r);
    }
}
