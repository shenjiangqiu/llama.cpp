use rust_utils_common::{
    transform::{shift_map::ShiftMap, sorted_map::NoSortMap},
    translate::DefaultTranslator,
};

fn main() {
    rust_utils_tools::run_main::<DefaultTranslator, ShiftMap, NoSortMap>(
        "default_real_sim_shifted.bin",
    );
}

#[cfg(test)]
mod tests {}
