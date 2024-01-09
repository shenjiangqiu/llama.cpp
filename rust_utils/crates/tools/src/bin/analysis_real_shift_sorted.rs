use rust_utils_common::{
    transform::{shift_map::ShiftMap, sorted_map::SortedMap},
    translate::DefaultTranslator,
};
fn main() {
    rust_utils_tools::run_main::<DefaultTranslator, ShiftMap, SortedMap>(
        "default_real_sim_shifted_sorted.bin",
    );
}

#[cfg(test)]
mod tests {}
