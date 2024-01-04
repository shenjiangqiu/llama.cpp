//!
//! the ideas:
//! 1. use add one
//! 2. change the middle
//! 3. ignore some bits
//! 3. sort?
//!

pub mod chage_mid;
pub mod ignore;
pub mod real_sim;

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use real_sim::{RealSim, Report};
use rust_utils::translate::{
    from_q5_to_u8, from_q6_to_u8, new_from_q2_to_u8, new_from_q3_to_u8, new_from_q4_to_u8,
};
use rust_utils_capi::{quants::*, MulMatRegister};
use serde::{Deserialize, Serialize};
use tracing::info;

macro_rules! test_all {
    ($test_fn:ident,$all_data:ident,$mb2:expr,$mb3:expr,$mb4:expr,$mb5:expr,$mb6:expr,$mb8:expr;$($size:literal),* $(,)?) => {
        {
            let mut results = vec![];
            $(
                let _r=$test_fn::<$size>(&$all_data,$mb2,$mb3,$mb4,$mb5,$mb6,$mb8);
                results.push(_r);
            )*
            results
        }
    };
}
pub struct AllData {
    pub q2: BTreeMap<String, Vec<BlockQ2K>>,
    pub q3: BTreeMap<String, Vec<BlockQ3K>>,
    pub q4: BTreeMap<String, Vec<BlockQ4K>>,
    pub q5: BTreeMap<String, Vec<BlockQ5K>>,
    pub q6: BTreeMap<String, Vec<BlockQ6K>>,
    pub q8: BTreeMap<String, Vec<BlockQ8K>>,
    pub mul_mat_register: Vec<MulMatRegister>,
}

impl AllData {
    pub fn print_names(&self) {
        let all_computes = self
            .mul_mat_register
            .iter()
            .map(|r| {
                (
                    r.src_0_bits,
                    r.src_1_bits,
                    r.src_0_ne0,
                    r.src_0_name.as_str(),
                    r.src_1_name.as_str(),
                )
            })
            .collect::<BTreeSet<_>>();
        for (src_0_bits, src_1_bits, _ne0, src_0_name, src_1_name) in all_computes {
            assert!(src_1_bits == 8);
            // assert!(ne0 % 4096 == 0, "the ne0 is {:?}", ne0);
            info!("src0:{},src1:{}", src_0_bits, src_1_bits);
            match (src_0_bits, src_1_bits) {
                (2, 8) => {
                    let data_0 = (self.q2).get(src_0_name).unwrap();
                    let data_1 = (self.q8).get(src_1_name).unwrap();

                    info!("{}x{}", src_0_name, src_1_name);
                    info!("{}x{}", data_0.len(), data_1.len());
                }
                (3, 8) => {
                    let data_0 = (self.q3).get(src_0_name).unwrap();
                    let data_1 = (self.q8).get(src_1_name).unwrap();
                    info!("{}x{}", src_0_name, src_1_name);
                    info!("{}x{}", data_0.len(), data_1.len());
                }
                (4, 8) => {
                    let data_0 = (self.q4).get(src_0_name).unwrap();
                    let data_1 = (self.q8).get(src_1_name).unwrap();
                    info!("{}x{}", src_0_name, src_1_name);
                    info!("{}x{}", data_0.len(), data_1.len());
                }
                (5, 8) => {
                    let data_0 = (self.q5).get(src_0_name).unwrap();
                    let data_1 = (self.q8).get(src_1_name).unwrap();
                    info!("{}x{}", src_0_name, src_1_name);
                    info!("{}x{}", data_0.len(), data_1.len());
                }
                (6, 8) => {
                    let data_0 = (self.q6).get(src_0_name).unwrap();
                    let data_1 = (self.q8).get(src_1_name).unwrap();
                    info!("{}x{}", src_0_name, src_1_name);
                    info!("{}x{}", data_0.len(), data_1.len());
                }
                _ => {
                    unreachable!()
                }
            }
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct FileResult {
    file_path: String,
    results: Vec<Result>,
}

pub fn default_map_q2(data: &BlockQ2K) -> Vec<u8> {
    new_from_q2_to_u8(&data.qs).to_vec()
}
pub fn default_map_q3(data: &BlockQ3K) -> Vec<u8> {
    new_from_q3_to_u8(&data.qs, &data.hmask).to_vec()
}
pub fn default_map_q4(data: &BlockQ4K) -> Vec<u8> {
    new_from_q4_to_u8(&data.qs).to_vec()
}
pub fn default_map_q5(data: &BlockQ5K) -> Vec<u8> {
    from_q5_to_u8(&data.qs, &data.qh).to_vec()
}
pub fn default_map_q6(data: &BlockQ6K) -> Vec<u8> {
    from_q6_to_u8(&data.ql, &data.qh).to_vec()
}
pub fn default_map_q8(data: &BlockQ8K) -> Vec<u8> {
    data.qs.iter().map(|x| *x as u8).collect()
}
pub fn is_all_1<const TH: usize>(data: u8) -> bool {
    for i in 0..TH {
        if data & (1 << i) == 0 {
            return false;
        }
    }
    return true;
}
pub fn add_by_one(data: &mut Vec<u8>) {
    data.iter_mut().for_each(|x| {
        if is_all_1::<2>(*x) {
            *x = *x + 1;
        }
    });
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

pub fn run_main(
    map_q2: impl Fn(&BlockQ2K) -> Vec<u8> + Sync + Send + Clone + Copy,
    map_q3: impl Fn(&BlockQ3K) -> Vec<u8> + Sync + Send + Clone + Copy,
    map_q4: impl Fn(&BlockQ4K) -> Vec<u8> + Sync + Send + Clone + Copy,
    map_q5: impl Fn(&BlockQ5K) -> Vec<u8> + Sync + Send + Clone + Copy,
    map_q6: impl Fn(&BlockQ6K) -> Vec<u8> + Sync + Send + Clone + Copy,
    map_q8: impl Fn(&BlockQ8K) -> Vec<u8> + Sync + Send + Clone + Copy,
    result_file_name: &str,
) {
    rust_utils::init_logger_asni();
    let paths = ["./q3data", "./q5data", "./q6data"];
    for p in paths {
        let folder = Path::new(p);
        let q2: BTreeMap<String, Vec<BlockQ2K>> = bincode::deserialize_from(BufReader::new(
            File::open(folder.join("ALL_DATA_Q2.bin")).unwrap(),
        ))
        .unwrap();
        let q3: BTreeMap<String, Vec<BlockQ3K>> = bincode::deserialize_from(BufReader::new(
            File::open(folder.join("ALL_DATA_Q3.bin")).unwrap(),
        ))
        .unwrap();
        let q4: BTreeMap<String, Vec<BlockQ4K>> = bincode::deserialize_from(BufReader::new(
            File::open(folder.join("ALL_DATA_Q4.bin")).unwrap(),
        ))
        .unwrap();
        let q5: BTreeMap<String, Vec<BlockQ5K>> = bincode::deserialize_from(BufReader::new(
            File::open(folder.join("ALL_DATA_Q5.bin")).unwrap(),
        ))
        .unwrap();
        let q6: BTreeMap<String, Vec<BlockQ6K>> = bincode::deserialize_from(BufReader::new(
            File::open(folder.join("ALL_DATA_Q6.bin")).unwrap(),
        ))
        .unwrap();
        let q8: BTreeMap<String, Vec<BlockQ8K>> = bincode::deserialize_from(BufReader::new(
            File::open(folder.join("ALL_DATA_Q8.bin")).unwrap(),
        ))
        .unwrap();
        let registrys: Vec<MulMatRegister> = bincode::deserialize_from(BufReader::new(
            File::open(folder.join("mul_mat_register.bin")).unwrap(),
        ))
        .unwrap();
        let all_data = AllData {
            q2,
            q3,
            q4,
            q5,
            q6,
            q8,
            mul_mat_register: registrys,
        };
        let results = test_all!(test_width,all_data,map_q2,map_q3,map_q4,map_q5,map_q6,map_q8;32,64,128,256,512,1024);
        let file_result = FileResult {
            file_path: p.to_owned(),
            results,
        };
        bincode::serialize_into(
            BufWriter::new(File::create(folder.join(result_file_name)).unwrap()),
            &file_result,
        )
        .unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SingleResult {
    src_0_bits: u8,
    src_1_bits: u8,
    src_0_name: String,
    src_1_name: String,
    report: Report,
}
#[derive(Debug, Serialize, Deserialize)]
struct Result {
    all_results: Vec<SingleResult>,
    real_sim_width: u16,
}
fn test_width<const WIDTH: u16>(
    all_data: &AllData,
    map_q2: impl Fn(&BlockQ2K) -> Vec<u8> + Sync + Send + Clone + Copy,
    map_q3: impl Fn(&BlockQ3K) -> Vec<u8> + Sync + Send + Clone + Copy,
    map_q4: impl Fn(&BlockQ4K) -> Vec<u8> + Sync + Send + Clone + Copy,
    map_q5: impl Fn(&BlockQ5K) -> Vec<u8> + Sync + Send + Clone + Copy,
    map_q6: impl Fn(&BlockQ6K) -> Vec<u8> + Sync + Send + Clone + Copy,
    map_q8: impl Fn(&BlockQ8K) -> Vec<u8> + Sync + Send + Clone + Copy,
) -> Result {
    all_data.print_names();
    let all_computes = all_data
        .mul_mat_register
        .iter()
        .map(|r| {
            (
                r.src_0_bits,
                r.src_1_bits,
                r.src_0_ne0,
                r.src_0_name.as_str(),
                r.src_1_name.as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    let real_sim = RealSim::<WIDTH>::new();

    let result = all_computes
        .into_par_iter()
        .map(
            move |(src_0_bits, src_1_bits, ne0, src_0_name, src_1_name)| match (
                src_0_bits, src_1_bits,
            ) {
                (2, 8) => {
                    let data_0 = (all_data.q2).get(src_0_name).unwrap();
                    let data_1 = (all_data.q8).get(src_1_name).unwrap();
                    get_single_result(
                        ne0, data_0, data_1, &real_sim, src_0_bits, src_1_bits, src_0_name,
                        src_1_name, map_q2, map_q8,
                    )
                }
                (3, 8) => {
                    let data_0 = (all_data.q3).get(src_0_name).unwrap();
                    let data_1 = (all_data.q8).get(src_1_name).unwrap();
                    get_single_result(
                        ne0, data_0, data_1, &real_sim, src_0_bits, src_1_bits, src_0_name,
                        src_1_name, map_q3, map_q8,
                    )
                }
                (4, 8) => {
                    let data_0 = (all_data.q4).get(src_0_name).unwrap();
                    let data_1 = (all_data.q8).get(src_1_name).unwrap();
                    get_single_result(
                        ne0, data_0, data_1, &real_sim, src_0_bits, src_1_bits, src_0_name,
                        src_1_name, map_q4, map_q8,
                    )
                }
                (5, 8) => {
                    let data_0 = (all_data.q5).get(src_0_name).unwrap();
                    let data_1 = (all_data.q8).get(src_1_name).unwrap();
                    get_single_result(
                        ne0, data_0, data_1, &real_sim, src_0_bits, src_1_bits, src_0_name,
                        src_1_name, map_q5, map_q8,
                    )
                }
                (6, 8) => {
                    let data_0 = (all_data.q6).get(src_0_name).unwrap();
                    let data_1 = (all_data.q8).get(src_1_name).unwrap();
                    get_single_result(
                        ne0, data_0, data_1, &real_sim, src_0_bits, src_1_bits, src_0_name,
                        src_1_name, map_q6, map_q8,
                    )
                }
                _ => {
                    unreachable!()
                }
            },
        )
        .collect::<Vec<_>>();
    Result {
        all_results: result,
        real_sim_width: WIDTH,
    }
}

fn get_single_result<const WIDTH: u16, T1, T2>(
    ne0: usize,
    data_0: &Vec<T1>,
    data_1: &Vec<T2>,
    real_sim: &RealSim<WIDTH>,
    src_0_bits: u8,
    src_1_bits: u8,
    src_0_name: &str,
    src_1_name: &str,
    map_data_0_to_vec: impl FnMut(&T1) -> Vec<u8>,
    map_data_1_to_vec: impl FnMut(&T2) -> Vec<u8>,
) -> SingleResult {
    let data_0_u8 = data_0.iter().map(map_data_0_to_vec).fold(
        Vec::with_capacity(data_0.len() * 256),
        |mut acc, v| {
            acc.extend_from_slice(&v);
            acc
        },
    );
    let data_1_u8 = data_1.iter().map(map_data_1_to_vec).fold(
        Vec::with_capacity(data_1.len() * 256),
        |mut acc, v| {
            acc.extend_from_slice(&v);
            acc
        },
    );
    assert!(data_0_u8.len() % ne0 == 0);
    assert!(data_1_u8.len() % ne0 == 0);
    let data_0_rows = data_0_u8.len() / ne0;
    let data_1_rows = data_1_u8.len() / ne0;
    let mut report = Report::default();
    for r0 in 0..data_0_rows {
        for r1 in 0..data_1_rows {
            let data_0_row_data = &data_0_u8[r0 * ne0..(r0 + 1) * ne0];
            let data_1_row_data = &data_1_u8[r1 * ne0..(r1 + 1) * ne0];
            let t_report = real_sim.run_2_8(data_0_row_data, data_1_row_data);
            report.all_steps += t_report.all_steps;
            report.max_steps += t_report.max_steps;
        }
    }
    let single_result = SingleResult {
        src_0_bits,
        src_1_bits,
        src_0_name: src_0_name.to_owned(),
        src_1_name: src_1_name.to_owned(),
        report,
    };
    single_result
}

#[cfg(test)]
mod tests {}
