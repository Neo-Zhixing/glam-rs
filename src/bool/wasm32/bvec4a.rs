// Generated from vec_mask.rs.tera template. Edit the template, not the generated file.

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{hash, ops::*};

use core::arch::wasm32::*;

/// A 4-dimensional SIMD vector mask.
///
/// This type is 16 byte aligned and is backed by a SIMD vector. If SIMD is not available
/// `BVec4A` will be a type alias for `BVec4`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BVec4A(pub(crate) v128);

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

const FALSE: BVec4A = BVec4A::new(false, false, false, false);

impl BVec4A {
    /// Creates a new vector mask.
    #[inline(always)]
    pub const fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self(u32x4(
            MASK[x as usize],
            MASK[y as usize],
            MASK[z as usize],
            MASK[w as usize],
        ))
    }

    /// Returns a bitmask with the lowest two bits set from the elements of `self`.
    ///
    /// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
    /// into the first lowest bit, element `y` into the second, etc.
    #[inline]
    pub fn bitmask(self) -> u32 {
        u32x4_bitmask(self.0) as u32
    }

    /// Returns true if any of the elements are true, false otherwise.
    #[inline]
    pub fn any(self) -> bool {
        u32x4_bitmask(self.0) != 0
    }

    /// Returns true if all the elements are true, false otherwise.
    #[inline]
    pub fn all(self) -> bool {
        u32x4_bitmask(self.0) == 0xf
    }

    #[inline]
    fn into_bool_array(self) -> [bool; 4] {
        let bitmask = self.bitmask();
        [
            (bitmask & 1) != 0,
            (bitmask & 2) != 0,
            (bitmask & 4) != 0,
            (bitmask & 8) != 0,
        ]
    }

    #[inline]
    fn into_u32_array(self) -> [u32; 4] {
        let bitmask = self.bitmask();
        [
            MASK[(bitmask & 1) as usize],
            MASK[((bitmask >> 1) & 1) as usize],
            MASK[((bitmask >> 2) & 1) as usize],
            MASK[((bitmask >> 3) & 1) as usize],
        ]
    }
}

impl Default for BVec4A {
    #[inline]
    fn default() -> Self {
        FALSE
    }
}

impl PartialEq for BVec4A {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.bitmask().eq(&rhs.bitmask())
    }
}

impl Eq for BVec4A {}

impl hash::Hash for BVec4A {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.bitmask().hash(state);
    }
}

impl BitAnd for BVec4A {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Self(v128_and(self.0, rhs.0))
    }
}

impl BitAndAssign for BVec4A {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs);
    }
}

impl BitOr for BVec4A {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(v128_or(self.0, rhs.0))
    }
}

impl BitOrAssign for BVec4A {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs);
    }
}

impl BitXor for BVec4A {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Self(v128_xor(self.0, rhs.0))
    }
}

impl BitXorAssign for BVec4A {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs);
    }
}
impl Not for BVec4A {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        Self(v128_not(self.0))
    }
}

impl From<BVec4A> for v128 {
    #[inline]
    fn from(t: BVec4A) -> Self {
        t.0
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for BVec4A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_u32_array();
        write!(
            f,
            "{}({:#x}, {:#x}, {:#x}, {:#x})",
            stringify!(BVec4A),
            arr[0],
            arr[1],
            arr[2],
            arr[3]
        )
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for BVec4A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arr = self.into_bool_array();
        write!(f, "[{}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3])
    }
}

impl From<BVec4A> for [bool; 4] {
    #[inline]
    fn from(mask: BVec4A) -> Self {
        mask.into_bool_array()
    }
}

impl From<BVec4A> for [u32; 4] {
    #[inline]
    fn from(mask: BVec4A) -> Self {
        mask.into_u32_array()
    }
}
