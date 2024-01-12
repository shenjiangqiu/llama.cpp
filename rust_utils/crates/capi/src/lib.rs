use rust_utils_common::transform::minus_map;
use rust_utils_common::translate;
use rust_utils_common::{quants::*, transform::shift_map};
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_char, CStr},
    ops::Deref,
    sync::atomic::AtomicBool,
};
use tracing::info;
static ENABLE_SAVE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MulMatRegister {
    pub src_0_bits: u8,
    pub src_1_bits: u8,
    pub src_0_ne0: usize,
    pub src_0_ne1: usize,
    pub src_1_ne0: usize,
    pub src_1_ne1: usize,
    pub src_0_name: String,
    pub src_1_name: String,
}
static MUL_MAT_REGISTER: std::sync::RwLock<Vec<MulMatRegister>> = std::sync::RwLock::new(vec![]);

#[macro_export]
macro_rules! generate_static {
    ($($name:ident: $typ:ty,$boolname:ident, $func:ident,$append:literal $(,)? );* $(;)?) => {
        $(
            pub static $name: std::sync::RwLock<std::collections::BTreeMap<String, Vec<$typ>>> = std::sync::RwLock::new(std::collections::BTreeMap::new());
            pub static $boolname: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);
            #[no_mangle]
            /// save the data into the global map
            pub extern fn $func(data:*const $typ,size:usize,name:*const core::ffi::c_char){
                if !ENABLE_SAVE.load(std::sync::atomic::Ordering::SeqCst){
                    return;
                }
                if $append{
                    rust_utils_common::save_data_append(name,size,data,&$name,&$boolname);
                }else{
                    rust_utils_common::save_data(name,size,data,&$name,&$boolname);
                }
            }
        )*
        #[no_mangle]
        pub extern fn save_file(){
            {
                $(
                    if $boolname.load(std::sync::atomic::Ordering::SeqCst){
                        println!("saving file {:?}",stringify!($name));
                        let all_data = $name.read().unwrap();
                        let all_data = all_data.deref();

                        let  file = std::fs::File::create(format!("{}.bin",stringify!($name))).unwrap();
                        let mut buffer_writer = std::io::BufWriter::new(file);
                        bincode::serialize_into(&mut buffer_writer,&*all_data).unwrap();
                        $boolname.store(false,std::sync::atomic::Ordering::SeqCst);
                    }else{
                        println!("no need to save file {:?}",stringify!($name));
                    }

                )*
            }
            {
                let mul_mat_register = MUL_MAT_REGISTER.read().unwrap();
                let mul_mat_register = mul_mat_register.deref();
                let file = std::fs::File::create("mul_mat_register.bin").unwrap();
                let mut buffer_writer = std::io::BufWriter::new(file);
                bincode::serialize_into(&mut buffer_writer, &*mul_mat_register).unwrap();
            }

            println!("save file done,press any key to continue,press N to exit");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "N"{
                std::process::exit(0);
            }
        }
    };
}
generate_static!(
    ALL_DATA_Q2: BlockQ2K,ALL_DATA_Q2_MODIFIED,save_tensor_continouse_q2,false;
    ALL_DATA_Q3: BlockQ3K,ALL_DATA_Q3_MODIFIED,save_tensor_continouse_q3,false;
    ALL_DATA_Q4: BlockQ4K,ALL_DATA_Q4_MODIFIED,save_tensor_continouse_q4,false;
    ALL_DATA_Q5: BlockQ5K,ALL_DATA_Q5_MODIFIED,save_tensor_continouse_q5,false;
    ALL_DATA_Q6: BlockQ6K,ALL_DATA_Q6_MODIFIED,save_tensor_continouse_q6,false;
    ALL_DATA_Q8: BlockQ8K,ALL_DATA_Q8_MODIFIED,save_tensor_continouse_q8,true;
);
#[no_mangle]
/// create a new name by concating two names, remember to call `free_name` to free the memory
pub extern "C" fn creat_concated_name(name_1: *const c_char, name_2: *const c_char) -> *mut c_char {
    let name1 = unsafe { CStr::from_ptr(name_1) }.to_str().unwrap();
    let name2 = unsafe { CStr::from_ptr(name_2) }.to_str().unwrap();
    let name = format!("{}_{}", name1, name2);
    let name = std::ffi::CString::new(name).unwrap();
    name.into_raw()
}

#[no_mangle]
/// free the memory created by `creat_concated_name`
pub extern "C" fn free_name(name: *mut c_char) {
    unsafe {
        let _ = std::ffi::CString::from_raw(name);
    }
}

#[no_mangle]
pub extern "C" fn register_mul_mat(
    src_0_bits: u8,
    src_1_bits: u8,
    src_0_ne0: usize,
    src_0_ne1: usize,
    src_1_ne0: usize,
    src_1_ne1: usize,
    src_0_name: *const c_char,
    src_1_name: *const c_char,
) {
    assert_ne!(src_0_bits, 0);
    let src_0_name = unsafe { CStr::from_ptr(src_0_name) }
        .to_str()
        .unwrap()
        .to_string();
    let src_1_name = unsafe { CStr::from_ptr(src_1_name) }
        .to_str()
        .unwrap()
        .to_string();
    let register = MulMatRegister {
        src_0_bits,
        src_1_bits,
        src_0_ne0,
        src_0_ne1,
        src_1_ne0,
        src_1_ne1,
        src_0_name,
        src_1_name,
    };
    MUL_MAT_REGISTER.write().unwrap().push(register);
}
#[no_mangle]
pub extern "C" fn init_logger_asni() {
    rust_utils_common::init_logger_asni();
}
#[no_mangle]
pub extern "C" fn init_logger() {
    rust_utils_common::init_logger();
}

#[no_mangle]
pub extern "C" fn set_enable_save(enable: bool) {
    ENABLE_SAVE.store(enable, std::sync::atomic::Ordering::SeqCst);
}
#[no_mangle]
pub extern "C" fn get_enable_save() -> bool {
    ENABLE_SAVE.load(std::sync::atomic::Ordering::SeqCst)
}

#[no_mangle]
pub extern "C" fn enable_save() {
    set_enable_save(true);
}
#[no_mangle]
pub extern "C" fn disable_save() {
    set_enable_save(false);
}

#[no_mangle]
pub extern "C" fn print_vector_q2_raw(data: *const c_char, size: usize) {
    info!("printing q2 raw;the size is {:?}", size);
    let nb = size / 256;
    let data: &[BlockQ2K] = unsafe { std::slice::from_raw_parts(data as *const BlockQ2K, nb) };
    let data: Vec<_> = data
        .iter()
        .map(|x| translate::new_from_q2_to_u8(&x.qs))
        .collect();
    for d in data {
        info!("the data is {:?}", d);
    }
}
#[no_mangle]
pub extern "C" fn print_vector_q3_raw(data: *const c_char, size: usize) {
    info!("printing q3 raw;the size is {:?}", size);
    let nb = size / 256;
    let data: &[BlockQ3K] = unsafe { std::slice::from_raw_parts(data as *const BlockQ3K, nb) };
    let data: Vec<_> = data
        .iter()
        .map(|x| translate::new_from_q3_to_u8(&x.qs, &x.hmask))
        .collect();
    for d in data {
        info!("the data is {:?}", d);
    }
}
#[no_mangle]
pub extern "C" fn print_vector_q8_raw(data: *const c_char, size: usize) {
    info!("printing q8 raw;the size is {:?}", size);
    let nb = size / 256;
    let data: &[BlockQ8K] = unsafe { std::slice::from_raw_parts(data as *const BlockQ8K, nb) };
    let data: Vec<_> = data.iter().map(|x| x.qs).collect();
    for d in data {
        info!("the data is {:?}", d);
    }
}

#[no_mangle]
pub extern "C" fn log_info(data: *const c_char) {
    let data = unsafe { CStr::from_ptr(data) }.to_str().unwrap();
    tracing::info!("{}", data);
}
#[no_mangle]
pub extern "C" fn log_debug(data: *const c_char) {
    let data = unsafe { CStr::from_ptr(data) }.to_str().unwrap();
    tracing::debug!("{}", data);
}

#[no_mangle]
pub extern "C" fn print_vector_q2(array: &[u8; 64]) {
    let result = translate::new_from_q2_to_u8(array);
    println!("the_rust_array_q2");
    println!("{:?}", result);
}

#[no_mangle]
pub extern "C" fn print_vector_q3(array: &[u8; 64], hmask: &[u8; 32]) {
    let result = translate::new_from_q3_to_u8(array, hmask);
    println!("the rust array");
    println!("{:?}", result);
}
#[no_mangle]
pub extern "C" fn print_vector_q8(array: &[i8; 64]) {
    println!("the rust array");
    println!("{:?}", array);
}

#[no_mangle]
pub extern "C" fn print_vec(array: &[i8; 256]) {
    println!("the quants array");
    println!("{:?}", array);
}
#[no_mangle]
pub extern "C" fn print_vec_u8(array: &[u8; 256]) {
    println!("the quants array u8");
    println!("{:?}", array);
}

#[no_mangle]
/// update the data, policys:
/// - 0: shift
/// - 1: minus
/// - 2: shift and minus
pub extern "C" fn update_data(array: &mut [u8; 256], policy: u8) {
    match policy {
        0 => shift_map::add_by_one(array),
        1 => minus_map::minus_by_one(array),
        2 => {
            shift_map::add_by_one(array);
            minus_map::minus_by_one(array);
        }
        _ => panic!("the policy is not supported"),
    }
}

#[repr(C)]
pub struct AttentionContext {}
#[no_mangle]
pub extern "C" fn save_attention_result(context: &mut AttentionContext,seq_num:usize,block_num:usize,data:*const f32) {}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn test_read_line() {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        println!("the input is {:?}", buffer);
    }
}
