use crate::quants::*;

use tracing::info;

pub fn iter_quants<const N: usize>(data: &[u8; N]) -> impl IntoIterator<Item = u8> {
    let mut result = [0u8; 256];
    let mut result_index = 0;
    let parts = N / 32;
    let num_per_byte = 256 / N;
    let bits = 8 / num_per_byte;

    for i in 0..parts {
        let array_part = &data[i * 32..(i + 1) * 32];
        for num in 0..num_per_byte {
            let shift = num * bits;
            for j in 0..32 {
                let array_num: u8 = (array_part[j] >> shift) & (2u8.pow(bits as u32) - 1);
                result[result_index] = array_num;
                result_index += 1;
            }
        }
    }
    assert_eq!(result_index, 256);
    result
}
#[deprecated = "use new_from_q2_to_u8 instead"]
pub fn from_q2_to_u8(array: &[u8; 64]) -> [u8; 256] {
    info!("the q2 qs before trans is {:?}", array);
    let mut result = [0; 256];
    // there are two parts, first 128 and last 128, first from the first 32 array.each byte have 4 num, so it's 32*4 =128
    for i in 0..2 {
        let array_part = &array[i * 32..(i + 1) * 32];
        // the shift of the byts
        for k in 0..4 {
            let array_shift = 2 * k;
            // for each shift, get 32 num
            for j in 0..32 {
                let array_num = (array_part[j] >> array_shift) & 3;
                result[i * 128 + k * 32 + j] = array_num;
            }
        }
    }
    result
}

pub fn new_from_q2_to_u8(array: &[u8; 64]) -> [u8; 256] {
    info!("the q2 qs before trans is {:?}", array);
    let mut result = [0; 256];
    let mut result_index = 0;
    for ele in iter_quants(array) {
        result[result_index] = ele;
        result_index += 1;
    }
    assert_eq!(result_index, 256);
    result
}
#[deprecated = "use new_from_q3_to_u8 instead"]
pub fn from_q3_to_u8(array: &[u8; 64], hmask: &[u8; 32]) -> [u8; 256] {
    let mut result = [0u8; 256];
    // there are two parts, first 128 and last 128, first from the first 32 array.each byte have 4 num, so it's 32*4 =128
    for i in 0..2 {
        let array_part = &array[i * 32..(i + 1) * 32];
        // the shift of the byts
        for k in 0..4 {
            let array_shift = 2 * k;
            let hmask_shift = i * 4 + k;
            // for each shift, get 32 num
            for j in 0..32 {
                let array_num = (array_part[j] >> array_shift) & 3;
                let hmask_num = (hmask[j] >> hmask_shift) & 1;

                let result_num = (array_num) + (hmask_num << 2);
                result[i * 128 + k * 32 + j] = result_num;
            }
        }
    }
    result
}
pub fn new_from_q3_to_u8(array: &[u8; 64], hmask: &[u8; 32]) -> [u8; 256] {
    let mut result = [0u8; 256];
    let mut result_index = 0;
    let qs_bits = 8 / (256 / 64);
    // there are two parts, first 128 and last 128, first from the first 32 array.each byte have 4 num, so it's 32*4 =128
    for (s, h) in iter_quants(array).into_iter().zip(iter_quants(hmask)) {
        let num = s + (h << qs_bits);
        result[result_index] = num;
        result_index += 1;
    }
    assert_eq!(result_index, 256);
    result
}
#[deprecated = "use new_from_q4_to_u8 instead"]
pub fn from_q4_to_u8(array: &[u8; 128]) -> [u8; 256] {
    let mut result = [0u8; 256];
    // there are two parts, first 128 and last 128, first from the first 32 array.each byte have 4 num, so it's 32*4 =128
    let bits = 4;
    let num_per_byte = 8 / bits;
    let parts = 256 / num_per_byte / 32;
    let mut result_index = 0;
    assert_eq!(parts, 128 / 32);
    for i in 0..parts {
        let array_part = &array[i * 32..(i + 1) * 32];
        // the shift of the byts
        for k in 0..num_per_byte {
            let array_shift = bits * k;
            // for each shift, get 32 num
            for j in 0..32 {
                let array_num: u8 = (array_part[j] >> array_shift) & (2u8.pow(bits as u32) - 1);
                result[result_index] = array_num;
                result_index += 1;
            }
        }
    }
    assert_eq!(result_index, 256);
    result
}

pub fn new_from_q4_to_u8(array: &[u8; 128]) -> [u8; 256] {
    let mut result = [0u8; 256];
    let mut result_index = 0;
    for s in iter_quants(array).into_iter() {
        let num = s;
        result[result_index] = num;
        result_index += 1;
    }
    assert_eq!(result_index, 256);
    result
}

pub fn from_q5_to_u8(qs: &[u8; 128], qh: &[u8; 32]) -> [u8; 256] {
    let mut result = [0u8; 256];

    let mut result_index = 0;
    let s_bits = 8 / (256 / 128);
    assert_eq!(s_bits, 4);

    for (s, h) in iter_quants(qs).into_iter().zip(iter_quants(qh)) {
        let num = s + (h << s_bits);
        result[result_index] = num;
        result_index += 1;
    }
    assert_eq!(result_index, 256);
    result
}

pub fn from_q6_to_u8(qs: &[u8; 128], qh: &[u8; 64]) -> [u8; 256] {
    let mut result = [0u8; 256];

    let mut result_index = 0;
    let s_bits = 8 / (256 / 128);
    assert_eq!(s_bits, 4);

    for (s, h) in iter_quants(qs).into_iter().zip(iter_quants(qh)) {
        let num = s + (h << s_bits);
        result[result_index] = num;
        result_index += 1;
    }
    assert_eq!(result_index, 256);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    #[allow(deprecated)]
    fn test_new_method() {
        let mut rng = rand::thread_rng();

        let mut fake_q2_data: [u8; 64] = [0; 64];
        for i in 0..64 {
            fake_q2_data[i] = rng.gen();
        }
        let result_old = from_q2_to_u8(&fake_q2_data);
        let new_result = new_from_q2_to_u8(&fake_q2_data);
        assert_eq!(result_old, new_result);

        let mut fake_q3_data_qs: [u8; 64] = [0; 64];
        let mut fake_q3_data_qh: [u8; 32] = [0; 32];
        for i in 0..64 {
            fake_q3_data_qs[i] = rng.gen();
        }
        for i in 0..32 {
            fake_q3_data_qh[i] = rng.gen();
        }
        let result_old = from_q3_to_u8(&fake_q3_data_qs, &fake_q3_data_qh);
        let new_result = new_from_q3_to_u8(&fake_q3_data_qs, &fake_q3_data_qh);
        assert_eq!(result_old, new_result);

        let mut fake_q4_data: [u8; 128] = [0; 128];
        for i in 0..128 {
            fake_q4_data[i] = rng.gen();
        }
        let result_old = from_q4_to_u8(&fake_q4_data);
        let new_result = new_from_q4_to_u8(&fake_q4_data);

        assert_eq!(result_old, new_result);
    }
}

pub trait TranslateMapping {
    fn map_q2(data: &[BlockQ2K]) -> Vec<u8>;
    fn map_q3(data: &[BlockQ3K]) -> Vec<u8>;
    fn map_q4(data: &[BlockQ4K]) -> Vec<u8>;
    fn map_q5(data: &[BlockQ5K]) -> Vec<u8>;
    fn map_q6(data: &[BlockQ6K]) -> Vec<u8>;
    fn map_q8(data: &[BlockQ8K]) -> Vec<u8>;
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultTranslator;
impl TranslateMapping for DefaultTranslator {
    fn map_q2(data: &[BlockQ2K]) -> Vec<u8> {
        data.iter()
            .map(|x| new_from_q2_to_u8(&x.qs))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(&x);
                acc
            })
    }

    fn map_q3(data: &[BlockQ3K]) -> Vec<u8> {
        data.iter()
            .map(|x| new_from_q3_to_u8(&x.qs, &x.hmask))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(&x);
                acc
            })
    }

    fn map_q4(data: &[BlockQ4K]) -> Vec<u8> {
        data.iter()
            .map(|x| new_from_q4_to_u8(&x.qs))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(&x);
                acc
            })
    }

    fn map_q5(data: &[BlockQ5K]) -> Vec<u8> {
        data.iter()
            .map(|x| from_q5_to_u8(&x.qs, &x.qh))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(&x);
                acc
            })
    }

    fn map_q6(data: &[BlockQ6K]) -> Vec<u8> {
        data.iter()
            .map(|x| from_q6_to_u8(&x.ql, &x.qh))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend_from_slice(&x);
                acc
            })
    }

    fn map_q8(data: &[BlockQ8K]) -> Vec<u8> {
        data.iter()
            .map(|x| x.qs.iter().map(|x| *x as u8))
            .fold(Vec::new(), |mut acc, x| {
                acc.extend(x);
                acc
            })
    }
}
