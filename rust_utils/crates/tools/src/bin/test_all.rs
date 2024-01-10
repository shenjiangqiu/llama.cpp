use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use rust_utils_common::transform::default_map::DefaultTransform;
use rust_utils_common::transform::minus_map::MinusMap;
use rust_utils_common::transform::shift_map::ShiftMap;
use rust_utils_common::transform::sorted_map::{NoSortMap, SortedMap};
use rust_utils_common::translate::DefaultTranslator;
use rust_utils_tools::{run_all_data, test_all};
use rust_utils_tools::{test_all_schemes_parallel, AllData};
use rust_utils_tools::{test_width, FileResult};
use tracing::{info, info_span};
fn main() {
    let current_time = std::time::SystemTime::now();
    let next_step = |all_data: &AllData, path: &Path| {
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
            let file_path = path.join(file_name).with_extension("bin");
            let file_result = FileResult {
                file_path: path.to_owned(),
                results: result,
            };
            bincode::serialize_into(
                BufWriter::new(File::create(file_path).unwrap()),
                &file_result,
            )
            .unwrap();
        }
    };

    run_all_data(next_step);
    let eclapsed = current_time.elapsed().unwrap();
    info!("eclapsed: {:?}", eclapsed.as_secs());
    File::create("eclapsed_test_all.txt")
        .unwrap()
        .write_all(format!("{:?}", eclapsed.as_secs()).as_bytes())
        .unwrap();
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
