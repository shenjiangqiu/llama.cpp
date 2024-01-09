pub mod quants;
use std::sync::RwLock;
use std::{
    collections::BTreeMap,
    ffi::{c_char, CStr},
};

use tracing::level_filters::LevelFilter;

use tracing_subscriber::EnvFilter;
pub mod transform;
pub mod translate;
// return if all bits fron 0..TH is 1
pub fn is_all_1<const TH: usize>(data: u8) -> bool {
    for i in 0..TH {
        if data & (1 << i) == 0 {
            return false;
        }
    }
    return true;
}

pub fn save_data<BlockType: Clone>(
    name: *const c_char,
    size: usize,
    data: *const BlockType,
    store: &RwLock<BTreeMap<String, Vec<BlockType>>>,
    modified: &std::sync::atomic::AtomicBool,
) {
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap();
    assert!(name.len() < 100);

    let size = size / 256;
    let q2_data = unsafe { std::slice::from_raw_parts(data, size) };
    let q2_data = q2_data.to_vec();
    let mut all_data_q2 = store.write().unwrap();
    if all_data_q2.contains_key(name) {
        println!("the name is {:?} already exists", name);
        return;
    } else {
        println!("the name is {:?},insert!", name);
        let q2_data = q2_data.to_vec();

        all_data_q2.insert(name.to_owned(), q2_data);
        modified.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    println!("the name is {:?}", name);
    println!("the size is {:?}", size);
}

pub fn save_data_append<BlockType: Clone>(
    name: *const c_char,
    size: usize,
    data: *const BlockType,
    store: &RwLock<BTreeMap<String, Vec<BlockType>>>,
    modified: &std::sync::atomic::AtomicBool,
) {
    modified.store(true, std::sync::atomic::Ordering::SeqCst);
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap();
    assert!(name.len() < 100);
    assert!(size % 256 == 0);
    println!("the size is {:?}", size);
    let size = size / 256;
    println!("the blocks is {:?}", size);
    let q2_data = unsafe { std::slice::from_raw_parts(data, size) };
    let q2_data = q2_data.to_vec();
    let mut all_data_q2 = store.write().unwrap();
    if all_data_q2.contains_key(name) {
        println!("the name is {:?} already exists appending", name);
        let data = all_data_q2.get_mut(name).unwrap();
        data.extend_from_slice(&q2_data);
        println!("after appending the size is {:?}", data.len());
        return;
    } else {
        println!("the name is {:?},insert!", name);
        let q2_data = q2_data.to_vec();

        all_data_q2.insert(name.to_owned(), q2_data);
    }
    println!("the name is {:?}", name);
    println!("the size is {:?}", size);
}
pub fn init_logger_asni() {
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .try_init()
        .unwrap_or_default();
}
pub fn init_logger() {
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .try_init()
        .unwrap_or_default();
}
#[cfg(test)]
mod tests {
    use tracing::{info, info_span};

    use crate::init_logger_asni;

    #[test]
    fn test_size() {}

    #[test]
    fn test_tracing() {
        init_logger_asni();
        let span = info_span!("my_span", a = 10);
        let _enter = span.enter();
        info!("hello world");
    }
    #[test]
    fn test_tracing_mutithread() {
        init_logger_asni();
        let span = info_span!("my_span", a = 10);
        let _enter = span.enter();
        let mut handles = vec![];
        info!("hello world");

        for _i in 0..10 {
            handles.push(std::thread::spawn(|| {
                info!("hello world");
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
