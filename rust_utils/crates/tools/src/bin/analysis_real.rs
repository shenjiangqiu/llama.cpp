use rust_utils_common::{
    transform::{default_map::DefaultTransform, sorted_map::NoSortMap},
    translate::DefaultTranslator,
};
use tracing::info_span;

fn main() {
    let span = info_span!("main", t = "default");
    let _enter = span.enter();
    rust_utils_tools::run_main::<DefaultTranslator, DefaultTransform, NoSortMap>(
        "default_real_sim.bin",
    );
}
