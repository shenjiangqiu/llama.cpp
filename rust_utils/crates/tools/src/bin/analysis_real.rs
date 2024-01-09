use rust_utils_common::{
    transform::{default_map::DefaultTransform, sorted_map::NoSortMap},
    translate::DefaultTranslator,
};

fn main() {
    rust_utils_tools::run_main::<DefaultTranslator, DefaultTransform, NoSortMap>(
        "default_real_sim.bin",
    );
}
