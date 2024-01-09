use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use rust_utils_capi::MulMatRegister;
use rust_utils_common::quants::*;
use rust_utils_common::transform::default_map::DefaultTransform;
use rust_utils_common::transform::minus_map::MinusMap;
use rust_utils_common::transform::shift_map::ShiftMap;
use rust_utils_common::transform::sorted_map::{NoSortMap, SortedMap};
use rust_utils_common::translate::DefaultTranslator;
use rust_utils_tools::test_all;
use rust_utils_tools::{test_all_schemes_parallel, AllData};
use rust_utils_tools::{test_width, FileResult};
use tracing::info_span;
fn main() {
    rust_utils_common::init_logger_asni();
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
        let all_result = test_all_schemes_parallel!(
            (test_width,all_data);
            (
                (DefaultTranslator,DefaultTransform,NoSortMap),
                (DefaultTranslator,DefaultTransform,SortedMap),
                (DefaultTranslator,ShiftMap,NoSortMap),
                (DefaultTranslator,ShiftMap,SortedMap),
                (DefaultTranslator,(ShiftMap,MinusMap),SortedMap),
                (DefaultTranslator,(ShiftMap,MinusMap),NoSortMap),
            );
            (128, 256, 512, 1024)
        );
        let file_names = [
            "all_default",
            "all_sort",
            "all_shift",
            "all_shift_sort",
            "all_shift_minus_sort",
            "all_shift_minus_nosort",
        ];
        for (result, file_name) in all_result.into_iter().zip(file_names) {
            let file_path = Path::new(p).join(file_name).with_extension("bin");
            let file_result = FileResult {
                file_path: Path::new(p).to_owned(),
                results: result,
            };
            bincode::serialize_into(
                BufWriter::new(File::create(file_path).unwrap()),
                &file_result,
            )
            .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    macro_rules! test_tt {
        ($a:tt;$b:tt) => {
            println!("{} {}", stringify!($a), stringify!($b));
            test_sub!($a;$b);
        };
    }
    macro_rules! test_sub {
        (($a:tt,$b:tt);[$c:tt,$d:tt]) => {
            println!(
                "{} {} {} {}",
                stringify!($a),
                stringify!($b),
                stringify!($c),
                stringify!($d)
            );
        };
    }
    #[test]
    fn test_tt() {
        test_tt!((a,b);[b,c]);
    }

    #[test]
    fn test_fn() {
        let mut all_fn: Vec<Box<dyn Fn() + Send>> = vec![];
        all_fn.push(Box::new(|| {
            println!("hello");
        }));
        all_fn.push(Box::new(|| {
            println!("world");
        }));
        all_fn.into_par_iter().for_each(|f| f());
    }
}
