use super::TransformMapping;

#[derive(Debug, Clone, Copy)]
pub struct DefaultTransform;
impl TransformMapping for DefaultTransform {
    fn transform<const BITS:u8>(_src_1: &mut [u8], _src_2: &mut [u8],_:usize) {}
}
