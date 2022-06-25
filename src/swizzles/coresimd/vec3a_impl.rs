// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

#![allow(clippy::useless_conversion)]

use crate::{Vec2, Vec3A, Vec3Swizzles, Vec4};

use core::simd::*;

impl Vec3Swizzles for Vec3A {
    type Vec2 = Vec2;

    type Vec4 = Vec4;

    #[inline]
    fn xx(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn xz(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn yx(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn zx(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn zy(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn zz(self) -> Vec2 {
        Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn xxx(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [0, 0, 4, 4]).into())
    }

    #[inline]
    fn xxy(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [0, 0, 5, 4]).into())
    }

    #[inline]
    fn xxz(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [0, 0, 6, 4]).into())
    }

    #[inline]
    fn xyx(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [0, 1, 4, 4]).into())
    }

    #[inline]
    fn xyy(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [0, 1, 5, 4]).into())
    }

    #[inline]
    fn xyz(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [0, 1, 6, 4]).into())
    }

    #[inline]
    fn xzx(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [0, 2, 4, 4]).into())
    }

    #[inline]
    fn xzy(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [0, 2, 5, 4]).into())
    }

    #[inline]
    fn xzz(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [0, 2, 6, 4]).into())
    }

    #[inline]
    fn yxx(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [1, 0, 4, 4]).into())
    }

    #[inline]
    fn yxy(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [1, 0, 5, 4]).into())
    }

    #[inline]
    fn yxz(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [1, 0, 6, 4]).into())
    }

    #[inline]
    fn yyx(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [1, 1, 4, 4]).into())
    }

    #[inline]
    fn yyy(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [1, 1, 5, 4]).into())
    }

    #[inline]
    fn yyz(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [1, 1, 6, 4]).into())
    }

    #[inline]
    fn yzx(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [1, 2, 4, 4]).into())
    }

    #[inline]
    fn yzy(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [1, 2, 5, 4]).into())
    }

    #[inline]
    fn yzz(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [1, 2, 6, 4]).into())
    }

    #[inline]
    fn zxx(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [2, 0, 4, 4]).into())
    }

    #[inline]
    fn zxy(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [2, 0, 5, 4]).into())
    }

    #[inline]
    fn zxz(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [2, 0, 6, 4]).into())
    }

    #[inline]
    fn zyx(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [2, 1, 4, 4]).into())
    }

    #[inline]
    fn zyy(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [2, 1, 5, 4]).into())
    }

    #[inline]
    fn zyz(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [2, 1, 6, 4]).into())
    }

    #[inline]
    fn zzx(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [2, 2, 4, 4]).into())
    }

    #[inline]
    fn zzy(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [2, 2, 5, 4]).into())
    }

    #[inline]
    fn zzz(self) -> Vec3A {
        Vec3A(simd_swizzle!(self.0, [2, 2, 6, 4]).into())
    }

    #[inline]
    fn xxxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 4, 4]))
    }

    #[inline]
    fn xxxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 4, 5]))
    }

    #[inline]
    fn xxxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 4, 6]))
    }

    #[inline]
    fn xxyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 5, 4]))
    }

    #[inline]
    fn xxyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 5, 5]))
    }

    #[inline]
    fn xxyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 5, 6]))
    }

    #[inline]
    fn xxzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 6, 4]))
    }

    #[inline]
    fn xxzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 6, 5]))
    }

    #[inline]
    fn xxzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 0, 6, 6]))
    }

    #[inline]
    fn xyxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 4, 4]))
    }

    #[inline]
    fn xyxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 4, 5]))
    }

    #[inline]
    fn xyxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 4, 6]))
    }

    #[inline]
    fn xyyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 5, 4]))
    }

    #[inline]
    fn xyyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 5, 5]))
    }

    #[inline]
    fn xyyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 5, 6]))
    }

    #[inline]
    fn xyzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 6, 4]))
    }

    #[inline]
    fn xyzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 6, 5]))
    }

    #[inline]
    fn xyzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 1, 6, 6]))
    }

    #[inline]
    fn xzxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 4, 4]))
    }

    #[inline]
    fn xzxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 4, 5]))
    }

    #[inline]
    fn xzxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 4, 6]))
    }

    #[inline]
    fn xzyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 5, 4]))
    }

    #[inline]
    fn xzyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 5, 5]))
    }

    #[inline]
    fn xzyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 5, 6]))
    }

    #[inline]
    fn xzzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 6, 4]))
    }

    #[inline]
    fn xzzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 6, 5]))
    }

    #[inline]
    fn xzzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [0, 2, 6, 6]))
    }

    #[inline]
    fn yxxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 4, 4]))
    }

    #[inline]
    fn yxxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 4, 5]))
    }

    #[inline]
    fn yxxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 4, 6]))
    }

    #[inline]
    fn yxyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 5, 4]))
    }

    #[inline]
    fn yxyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 5, 5]))
    }

    #[inline]
    fn yxyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 5, 6]))
    }

    #[inline]
    fn yxzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 6, 4]))
    }

    #[inline]
    fn yxzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 6, 5]))
    }

    #[inline]
    fn yxzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 0, 6, 6]))
    }

    #[inline]
    fn yyxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 4, 4]))
    }

    #[inline]
    fn yyxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 4, 5]))
    }

    #[inline]
    fn yyxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 4, 6]))
    }

    #[inline]
    fn yyyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 5, 4]))
    }

    #[inline]
    fn yyyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 5, 5]))
    }

    #[inline]
    fn yyyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 5, 6]))
    }

    #[inline]
    fn yyzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 6, 4]))
    }

    #[inline]
    fn yyzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 6, 5]))
    }

    #[inline]
    fn yyzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 1, 6, 6]))
    }

    #[inline]
    fn yzxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 4, 4]))
    }

    #[inline]
    fn yzxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 4, 5]))
    }

    #[inline]
    fn yzxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 4, 6]))
    }

    #[inline]
    fn yzyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 5, 4]))
    }

    #[inline]
    fn yzyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 5, 5]))
    }

    #[inline]
    fn yzyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 5, 6]))
    }

    #[inline]
    fn yzzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 6, 4]))
    }

    #[inline]
    fn yzzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 6, 5]))
    }

    #[inline]
    fn yzzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [1, 2, 6, 6]))
    }

    #[inline]
    fn zxxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 4, 4]))
    }

    #[inline]
    fn zxxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 4, 5]))
    }

    #[inline]
    fn zxxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 4, 6]))
    }

    #[inline]
    fn zxyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 5, 4]))
    }

    #[inline]
    fn zxyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 5, 5]))
    }

    #[inline]
    fn zxyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 5, 6]))
    }

    #[inline]
    fn zxzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 6, 4]))
    }

    #[inline]
    fn zxzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 6, 5]))
    }

    #[inline]
    fn zxzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 0, 6, 6]))
    }

    #[inline]
    fn zyxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 4, 4]))
    }

    #[inline]
    fn zyxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 4, 5]))
    }

    #[inline]
    fn zyxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 4, 6]))
    }

    #[inline]
    fn zyyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 5, 4]))
    }

    #[inline]
    fn zyyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 5, 5]))
    }

    #[inline]
    fn zyyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 5, 6]))
    }

    #[inline]
    fn zyzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 6, 4]))
    }

    #[inline]
    fn zyzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 6, 5]))
    }

    #[inline]
    fn zyzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 1, 6, 6]))
    }

    #[inline]
    fn zzxx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 4, 4]))
    }

    #[inline]
    fn zzxy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 4, 5]))
    }

    #[inline]
    fn zzxz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 4, 6]))
    }

    #[inline]
    fn zzyx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 5, 4]))
    }

    #[inline]
    fn zzyy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 5, 5]))
    }

    #[inline]
    fn zzyz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 5, 6]))
    }

    #[inline]
    fn zzzx(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 6, 4]))
    }

    #[inline]
    fn zzzy(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 6, 5]))
    }

    #[inline]
    fn zzzz(self) -> Vec4 {
        Vec4(simd_swizzle!(self.0, [2, 2, 6, 6]))
    }
}
