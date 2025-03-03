mod bvec2;
mod bvec3;
mod bvec4;

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
mod sse2;

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
mod wasm32;

pub use bvec2::BVec2;
pub use bvec3::BVec3;
pub use bvec4::BVec4;

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
pub use sse2::bvec3a::BVec3A;
#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
pub use sse2::bvec4a::BVec4A;

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
pub use wasm32::bvec3a::BVec3A;
#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
pub use wasm32::bvec4a::BVec4A;

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
pub type BVec3A = BVec3;

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
pub type BVec4A = BVec4;

mod const_test_bvec2 {
    const_assert_eq!(1, core::mem::align_of::<super::BVec2>());
    const_assert_eq!(2, core::mem::size_of::<super::BVec2>());
}

mod const_test_bvec3 {
    const_assert_eq!(1, core::mem::align_of::<super::BVec3>());
    const_assert_eq!(3, core::mem::size_of::<super::BVec3>());
}

mod const_test_bvec4 {
    const_assert_eq!(1, core::mem::align_of::<super::BVec4>());
    const_assert_eq!(4, core::mem::size_of::<super::BVec4>());
}

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
mod const_test_bvec3a {
    const_assert_eq!(16, core::mem::align_of::<super::BVec3A>());
    const_assert_eq!(16, core::mem::size_of::<super::BVec3A>());
}

#[cfg(all(
    any(target_feature = "sse2", target_feature = "simd128"),
    not(feature = "scalar-math")
))]
mod const_test_bvec4a {
    const_assert_eq!(16, core::mem::align_of::<super::BVec4A>());
    const_assert_eq!(16, core::mem::size_of::<super::BVec4A>());
}
