use std::mem;

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
#[repr(C)]
#[derive(Clone, Serialize, Deserialize)]
/// the Block type are created in lamma.cpp, use save_tensor in llama.cpp
pub struct BlockQ2K {
    pub scales: [u8; 16],
    #[serde(with = "BigArray")]
    pub qs: [u8; 64],
    pub d: u16,
    pub dmin: u16,
}
#[repr(C)]
#[derive(Clone, Serialize, Deserialize)]
/// the Block type are created in lamma.cpp, use save_tensor in llama.cpp
pub struct BlockQ3K {
    pub hmask: [u8; 32],
    #[serde(with = "BigArray")]
    pub qs: [u8; 64],
    pub scales: [u8; 12],
    pub d: u16,
}

#[repr(C)]
#[derive(Clone, Serialize, Deserialize)]
/// the Block type are created in lamma.cpp, use save_tensor in llama.cpp
pub struct BlockQ4K {
    pub d: u16,
    pub dmin: u16,
    pub scales: [u8; 12],
    #[serde(with = "BigArray")]
    pub qs: [u8; 128],
}

#[repr(C)]
#[derive(Clone, Serialize, Deserialize)]
/// the Block type are created in lamma.cpp, use save_tensor in llama.cpp
pub struct BlockQ5K {
    pub d: u16,
    pub dmin: u16,
    pub scales: [u8; 12],
    pub qh: [u8; 32],
    #[serde(with = "BigArray")]
    pub qs: [u8; 128],
}
#[repr(C)]
#[derive(Clone, Serialize, Deserialize)]
/// the Block type are created in lamma.cpp, use save_tensor in llama.cpp
pub struct BlockQ6K {
    #[serde(with = "BigArray")]
    pub ql: [u8; 128],
    #[serde(with = "BigArray")]
    pub qh: [u8; 64],
    pub scales: [u8; 16],
    pub d: u16,
}

#[repr(C)]
#[derive(Clone, Serialize, Deserialize)]
/// the Block type are created in lamma.cpp, use save_tensor in llama.cpp
pub struct BlockQ8K {
    pub d: f32,
    #[serde(with = "BigArray")]
    pub qs: [i8; 256],
    pub bsum: [i16; 16],
}
const _: () = {
    assert!(mem::size_of::<BlockQ2K>() == 84);
    assert!(mem::size_of::<BlockQ3K>() == 110);
    assert!(mem::size_of::<BlockQ4K>() == 2 * 2 + 12 + 256 / 2);
    assert!(mem::size_of::<BlockQ5K>() == 2 * 2 + 12 + 256 / 2 + 256 / 8);
    assert!(mem::size_of::<BlockQ6K>() == 2 + 256 / 16 + 3 * 256 / 4);
    assert!(mem::size_of::<BlockQ8K>() == (4 + 256 + 16 * 2));
};
