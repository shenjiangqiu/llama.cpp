pub mod default_map;
pub mod minus_map;
pub mod shift_map;
pub mod sorted_map;

pub trait TransformMapping {
    /// reorder the src_1 and src_2
    fn transform<const BITS: u8>(src_1: &mut [u8], src_2: &mut [u8], ne0: usize);
}

pub trait ReorderMapping {
    /// reorder the src_1 by row, return the new index for each row
    fn reorder<const BITS: u8>(src_1: &[u8], ne0: usize) -> Option<Vec<Vec<usize>>>;
}

macro_rules! gen_transform_mapping_impl {
    ($(($($ID:ident),+ $(,)?)),* $(,)?) => {
        $(
            impl<$($ID: TransformMapping),+> TransformMapping for ($($ID,)+) {
                fn transform<const BITS: u8>(src_1: &mut [u8], src_2: &mut [u8],ne0: usize) {
                    $(
                        $ID::transform::<BITS>(src_1, src_2, ne0);
                    )+
                }
            }
        )*
    };

}

gen_transform_mapping_impl! {
    (A),
    (A,B),
    (A,B,C),
    (A,B,C,D),
    (A,B,C,D,E),
    (A,B,C,D,E,F),
    (A,B,C,D,E,F,G),
    (A,B,C,D,E,F,G,H),
}
