use rust_utils_common::{
    transform::{minus_map::MinusMap, shift_map::ShiftMap, sorted_map::SortedMap},
    translate,
};
use tracing::info_span;

fn main() {
    let span = info_span!("main", t = "shift_minus_sorted");
    let _enter = span.enter();
    rust_utils_tools::run_main::<translate::DefaultTranslator, (ShiftMap, MinusMap), SortedMap>(
        "default_real_shift_minus_sorted.bin",
    );
}
