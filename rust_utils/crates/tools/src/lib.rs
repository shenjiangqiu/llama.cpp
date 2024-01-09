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
    ops::Add,
    path::Path,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use real_sim::{RealSim, Report};
use transform::{ReorderMapping, TransformMapping};

use rust_utils_capi::{quants::*, MulMatRegister};
use serde::{Deserialize, Serialize};
use tracing::{error, info, info_span};
use translate::TranslateMapping;

use crate::transform::sorted_map;

pub mod transform;
pub mod translate;
macro_rules! test_all {
    ($test_fn:ident,$all_data:ident,$translate_mapping:ty,$transform_mapping:ty,$sort_mapping:ty;$($size:literal),* $(,)?) => {
        {
            let mut results = vec![];
            $(
                let _r=$test_fn::<$size,$translate_mapping,$transform_mapping,$sort_mapping>(&$all_data);
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
pub struct FileResult {
    pub file_path: String,
    pub results: Vec<Result>,
}

// return if all bits fron 0..TH is 1
pub fn is_all_1<const TH: usize>(data: u8) -> bool {
    for i in 0..TH {
        if data & (1 << i) == 0 {
            return false;
        }
    }
    return true;
}

pub fn run_main<
    TransLate: TranslateMapping,
    TransForm: TransformMapping,
    Reorder: ReorderMapping,
>(
    result_file_name: &str,
) {
    rust_utils::init_logger_asni();
    let paths = ["./q5data", "./q6data"];
    for p in paths {
        let span = info_span!("run_main", path = p);
        let _enter = span.enter();
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
        let results = test_all!(test_width,all_data,TransLate,TransForm,Reorder;128,256,512,1024);
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
pub struct SingleResult {
    src_0_bits: u8,
    src_1_bits: u8,
    src_0_name: String,
    src_1_name: String,
    report: Report,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub all_results: Vec<SingleResult>,
    pub real_sim_width: u16,
}
pub trait MergeAll<Merged> {
    fn merge_all(self) -> Merged;
}

impl<T, In, I> MergeAll<I> for T
where
    T: IntoIterator<Item = In>,
    In: MergeAll<I>,
    I: Add<Output = I> + Default,
{
    fn merge_all(self) -> I {
        self.into_iter()
            .map(|x| x.merge_all())
            .fold(I::default(), |acc, x| acc + x)
    }
}
impl MergeAll<Report> for SingleResult {
    fn merge_all(self) -> Report {
        self.report
    }
}
impl MergeAll<Report> for Result {
    fn merge_all(self) -> Report {
        self.all_results.merge_all()
    }
}
impl MergeAll<Report> for FileResult {
    fn merge_all(self) -> Report {
        self.results.merge_all()
    }
}

/// select the right function to run according to the bits
macro_rules! match_bit_size {
    ($src_0_bits:expr,$src_1_bits:expr,$all_data:ident,$src_0_name:ident,$src_1_name:ident,$ne0:ident,$real_sim:ident,$width:ident;
        $($a:literal,$b:literal,$name_a:ident,$name_b:ident,$map_a:ident,$map_b:ident);* $(;)?) => {
        match ($src_0_bits,$src_1_bits){
            $(
                ($a, $b) => {
                    let data_0 = ($all_data.$name_a).get($src_0_name).unwrap();
                    let data_1 = ($all_data.$name_b).get($src_1_name).unwrap();
                    get_single_result::<$a,$b,$width, _, _, TransForm, Reorder>(
                        $ne0,
                        data_0,
                        data_1,
                        &$real_sim,
                        $src_0_name,
                        $src_1_name,
                        TransLate::$map_a,
                        TransLate::$map_b,
                    )
                }
            )*
            _=>{
                unreachable!()
            }
        }
    };
}

fn test_width<
    const WIDTH: u16,
    TransLate: TranslateMapping,
    TransForm: TransformMapping,
    Reorder: ReorderMapping,
>(
    all_data: &AllData,
) -> Result {
    info!("start testting width {:?}", WIDTH);
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
    let num_tasks = all_computes.len();
    let finished = std::sync::atomic::AtomicUsize::new(0);
    let real_sim = RealSim::<WIDTH>::new();
    let span = info_span!("test_width", width = WIDTH);
    info!("the num_tasks is {:?}", num_tasks);
    let result = all_computes
        .into_par_iter()
        .map(
            move |(src_0_bits, src_1_bits, ne0, src_0_name, src_1_name)| {
                let _enter = span.enter();
                let current_finished = finished.load(std::sync::atomic::Ordering::Relaxed);
                info!("start testting :{}/{}", current_finished, num_tasks,);
                let result = match_bit_size!(
                    src_0_bits,src_1_bits,all_data,src_0_name,src_1_name,ne0,real_sim,WIDTH;
                    2,8,q2,q8,map_q2,map_q8;
                    3,8,q3,q8,map_q3,map_q8;
                    4,8,q4,q8,map_q4,map_q8;
                    5,8,q5,q8,map_q5,map_q8;
                    6,8,q6,q8,map_q6,map_q8;);
                info!(
                    "finish testting :{}/{}",
                    finished.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1,
                    num_tasks,
                );
                result
            },
        )
        .collect::<Vec<_>>();
    Result {
        all_results: result,
        real_sim_width: WIDTH,
    }
}
/// return the report
fn get_single_result<
    const SRC_A_BITS: u8,
    const SRC_B_BITS: u8,
    const WIDTH: u16,
    T1,
    T2,
    TransForm: TransformMapping,
    Reorder: ReorderMapping,
>(
    ne0: usize,
    data_0: &Vec<T1>,
    data_1: &Vec<T2>,
    real_sim: &RealSim<WIDTH>,
    src_0_name: &str,
    src_1_name: &str,
    map_data_0_to_vec: impl FnOnce(&[T1]) -> Vec<u8>,
    map_data_1_to_vec: impl FnOnce(&[T2]) -> Vec<u8>,
) -> SingleResult {
    // the data_0 and data_1 might not have the same length
    let mut data_0_u8 = map_data_0_to_vec(data_0);

    let mut data_1_u8 = map_data_1_to_vec(data_1);

    // todo!("cannot transform here, it doesn't have the same length, so the sort is wrong");
    TransForm::transform::<SRC_A_BITS>(&mut data_0_u8, &mut data_1_u8, ne0);
    let reorder = Reorder::reorder::<SRC_A_BITS>(&data_0_u8, ne0);
    assert!(
        data_0_u8.len() % ne0 == 0,
        "the data_0_u8 is {:?}",
        data_0_u8.len()
    );

    assert!(
        data_1_u8.len() % ne0 == 0,
        "the data_1_u8 is {:?}",
        data_1_u8.len()
    );
    assert!(data_0_u8.len() % ne0 == 0);
    assert!(data_1_u8.len() % ne0 == 0);

    let fold_fn = |mut report: Report, r: Report| {
        report.all_steps = report.all_steps.checked_add(r.all_steps).unwrap();
        report.max_steps = report.max_steps.checked_add(r.max_steps).unwrap();
        report
    };
    let all_result = match reorder {
        Some(order) => {
            let data_0_rows = data_0_u8.chunks_exact_mut(ne0);

            // if the order is not none, then for each src_0_row, sort it, and sort all src_1_row
            assert_eq!(data_0_rows.len(), order.len());
            let all_result = data_0_rows
                .zip(order)
                .map(|(data_0_row, order)| {
                    assert!(data_0_row.len() == ne0);
                    assert!(order.len() == ne0);
                    // sort the wright bits
                    sorted_map::apply_index(data_0_row, &order);
                    let mut data_1_rows = data_1_u8.to_owned();
                    let data_1_rows = data_1_rows.chunks_exact_mut(ne0);
                    // for each src_1_row, sort it by the same order
                    data_1_rows
                        .map(|row| {
                            assert!(row.len() == ne0);
                            sorted_map::apply_index(row, &order);
                            real_sim.run::<SRC_A_BITS, SRC_B_BITS>(data_0_row, row)
                        })
                        .fold(Report::default(), fold_fn)
                })
                .fold(Report::default(), fold_fn);
            all_result
        }
        None => {
            let all_result = data_0_u8
                .chunks_exact(ne0)
                .map(|data_0_row| {
                    data_1_u8
                        .chunks_exact(ne0)
                        .map(|data_1_row| {
                            assert!(data_0_row.len() == ne0);
                            assert!(data_1_row.len() == ne0);
                            real_sim.run::<SRC_A_BITS, SRC_B_BITS>(data_0_row, data_1_row)
                        })
                        .fold(Report::default(), fold_fn)
                })
                .fold(Report::default(), fold_fn);
            all_result
        }
    };

    let single_result = SingleResult {
        src_0_bits: SRC_A_BITS,
        src_1_bits: SRC_B_BITS,
        src_0_name: src_0_name.to_owned(),
        src_1_name: src_1_name.to_owned(),
        report: all_result,
    };
    if single_result.report.all_steps > single_result.report.max_steps {
        error!("the single_result is {:?}", single_result);
        error!("the data_0_u8 is {:?}", &data_0_u8[0..100]);
        error!("the data_1_u8 is {:?}", &data_1_u8[0..100]);
        error!("the data_0_bits is {:?}", SRC_A_BITS);
        error!("the data_1_bits is {:?}", SRC_B_BITS);
        panic!("the single_result is {:?}", single_result);
    }
    single_result
}

#[cfg(test)]
mod tests {
    use crate::{
        transform::{
            default_map::{self, DefaultTransform},
            shift_map::{self, add_by_one, ShiftMap},
            sorted_map::{self, sort_by_most_zeros},
        },
        translate::DefaultTranslator,
    };

    use super::*;
    #[test]
    fn test_shift_and_sort() {
        let mut data = vec![
            0b0111_1111,
            0b0000_1111,
            0b0001_1111,
            0b0011_1111,
            0b0111_1111,
        ];
        sort_by_most_zeros::<6>(&mut data);
        println!("data is {:?}", data);
        add_by_one(&mut data);
        println!("data is {:?}", data);
    }

    #[test]
    fn test_test_width() {
        let all_data = init_test_data();
        let r1 = {
            test_width::<32, DefaultTranslator, DefaultTransform, sorted_map::NoSortMap>(&all_data)
        };

        let r2 = {
            test_width::<32, DefaultTranslator, DefaultTransform, sorted_map::SortedMap>(&all_data)
        };
        println!("r1 is {:?}", r1);
        let r1 = r1.merge_all();
        println!("r1 is {:?}", r1);

        println!("r2 is {:?}", r2);
        let r2 = r2.merge_all();
        println!("r2 is {:?}", r2);
        assert_eq!(r1.all_steps, r2.all_steps);
        assert_eq!(r1.max_steps, r2.max_steps);
        let r3 =
            { test_width::<32, DefaultTranslator, ShiftMap, sorted_map::SortedMap>(&all_data) };
        println!("r3 is {:?}", r3);
        let r3 = r3.merge_all();
        println!("r3 is {:?}", r3);
        assert_eq!(r1.max_steps, r3.max_steps);
        assert!(r3.all_steps < r1.all_steps);
        assert!(r3.all_steps < r2.all_steps);
        assert!(r3.all_steps < r3.max_steps);
    }

    fn init_test_data() -> AllData {
        let all_data = AllData {
            q2: [(
                "test".to_owned(),
                vec![
                    BlockQ2K {
                        scales: [0; 16],
                        qs: [15; 64],
                        d: 0,
                        dmin: 0,
                    };
                    16
                ],
            )]
            .into(),
            q3: [(
                "test".to_owned(),
                vec![BlockQ3K {
                    scales: [0; 12],
                    qs: [15; 64],
                    hmask: [0; 32],
                    d: 0,
                }],
            )]
            .into(),
            q4: [(
                "test".to_owned(),
                vec![BlockQ4K {
                    scales: [0; 12],
                    qs: [15; 128],
                    d: 0,
                    dmin: 0,
                }],
            )]
            .into(),
            q5: [(
                "test".to_owned(),
                vec![BlockQ5K {
                    scales: [0; 12],
                    qs: [15; 128],
                    qh: [0; 32],
                    d: 0,
                    dmin: 0,
                }],
            )]
            .into(),
            q6: [(
                "test".to_owned(),
                vec![BlockQ6K {
                    scales: [0; 16],
                    ql: [15; 128],
                    qh: [15; 64],
                    d: 0,
                }],
            )]
            .into(),
            q8: [(
                "test".to_owned(),
                vec![
                    BlockQ8K {
                        qs: [15; 256],
                        d: 0.,
                        bsum: Default::default(),
                    };
                    16
                ],
            )]
            .into(),
            mul_mat_register: vec![MulMatRegister {
                src_0_bits: 2,
                src_1_bits: 8,
                src_0_ne0: 4096,
                src_0_ne1: 1,
                src_1_ne0: 4096,
                src_1_ne1: 1,
                src_0_name: "test".to_owned(),
                src_1_name: "test".to_owned(),
            }],
        };
        all_data
    }

    #[test]
    fn test_test_all() {
        let data = init_test_data();
        use sorted_map::*;
        use translate::DefaultTranslator;
        let result_default = {
            use default_map::*;
            test_all!(test_width, data, DefaultTranslator,DefaultTransform,NoSortMap; 32, 64, 128, 256, 512, 1024)
        };
        let result_sorted = {
            use sorted_map::*;
            test_all!(
                test_width,
                data,DefaultTranslator,
                DefaultTransform,SortedMap;
                32, 64, 128, 256, 512, 1024
            )
        };
        let result_shift = {
            use shift_map::*;
            test_all!(
                test_width,
                data,DefaultTranslator,
                ShiftMap, NoSortMap;
                32, 64, 128, 256, 512, 1024
            )
        };
        let result_shift_sort = {
            test_all!(
                test_width,
                data,DefaultTranslator,
                ShiftMap, SortedMap;
                32, 64, 128, 256, 512, 1024
            )
        };
        let result_default = result_default.merge_all();
        let result_sorted = result_sorted.merge_all();
        let result_shift = result_shift.merge_all();
        let result_shift_sort = result_shift_sort.merge_all();
        println!("result_default is {:?}", result_default);
        println!("result_sorted is {:?}", result_sorted);
        println!("result_shift is {:?}", result_shift);
        println!("result_shift_sort is {:?}", result_shift_sort);
        assert_eq!(result_default.max_steps, result_sorted.max_steps);
        assert_eq!(result_default.max_steps, result_shift.max_steps);
        assert_eq!(result_default.max_steps, result_shift_sort.max_steps);
    }

    #[test]
    fn test_u32_max() {
        println!("u32::MAX is {:?}", u32::MAX);
    }
}
