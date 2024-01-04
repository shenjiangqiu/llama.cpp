use std::{fs::File, io::BufReader, path::Path};

use itertools::izip;
use rust_utils_tools::{real_sim::Report, FileResult, MergeAll};

fn load_result(path: &Path) -> FileResult {
    let file = File::open(path).expect(path.to_str().unwrap());

    bincode::deserialize_from(BufReader::new(file)).unwrap()
}
fn get_report(result: FileResult) -> Vec<(u16, Report)> {
    result
        .results
        .into_iter()
        .map(|x| (x.real_sim_width, x.all_results.merge_all()))
        .collect()
}
fn main() {
    for p in ["./q3data", "./q5data", "./q6data"] {
        let real = load_result(&Path::new(p).join("default_real_sim.bin"));
        let sorted = load_result(&Path::new(p).join("default_real_sim_sorted.bin"));
        let shifted = load_result(&Path::new(p).join("default_real_sim_shifted.bin"));
        let shifted_sorted = load_result(&Path::new(p).join("default_real_sim_shifted_sorted.bin"));
        let real_report: Vec<_> = get_report(real);
        let sorted_report: Vec<_> = get_report(sorted);
        let shifted_report = get_report(shifted);
        let shifted_sorted_report = get_report(shifted_sorted);
        println!("file: {}", p);
        for (real_report, sorted_report, shifted_report, shifted_sorted_report) in izip!(
            &real_report,
            &sorted_report,
            &shifted_report,
            &shifted_sorted_report
        ) {
            assert_eq!(real_report.1.max_steps, sorted_report.1.max_steps);
            assert_eq!(real_report.1.max_steps, shifted_report.1.max_steps);
            assert_eq!(real_report.1.max_steps, shifted_sorted_report.1.max_steps);
            println!("SIM_WIDTH: {}", real_report.0);
            println!(
                "{}: max_steps_shifted_sorted: {}",
                p, shifted_sorted_report.1.max_steps
            );
            println!(
                "{}: max_steps_real speedup {:.2}",
                p,
                real_report.1.max_steps as f64 / real_report.1.all_steps as f64
            );
            println!(
                "{}: max_steps_sorted speedup {:.2}",
                p,
                sorted_report.1.max_steps as f64 / sorted_report.1.all_steps as f64
            );
            println!(
                "{}: max_steps_shifted speedup {:.2}",
                p,
                shifted_report.1.max_steps as f64 / shifted_report.1.all_steps as f64
            );
            println!(
                "{}: max_steps_shifted_sorted speedup {:.2}",
                p,
                shifted_sorted_report.1.max_steps as f64 / shifted_sorted_report.1.all_steps as f64
            );
        }
    }
}
