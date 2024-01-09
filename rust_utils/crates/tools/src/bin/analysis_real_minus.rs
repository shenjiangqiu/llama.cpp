use rust_utils_common::{
    transform::{minus_map::MinusMap, shift_map::ShiftMap, sorted_map::SortedMap},
    translate,
};

fn main() {
    rust_utils_tools::run_main::<translate::DefaultTranslator, (ShiftMap, MinusMap), SortedMap>(
        "default_real_shift_minus_sorted.bin",
    );
}
