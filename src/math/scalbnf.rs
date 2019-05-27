/* sf_scalbn.c -- float version of s_scalbn.c.
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
 */

/*
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 */

const OVERFLOW_INT: i32 = 50000;
const TWO25: f32 = 3.355_443_200_e+07; /* 0x4c000000 */
const TWOM25: f32 = 2.980_232_238_8e-08; /* 0x33000000 */
const HUGE: f32 = 1.0e+30;
const TINY: f32 = 1.0e-30;

#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn scalbnf(mut x: f32, n: i32) -> f32 {
    use super::copysignf;
    use super::fdlibm::{
        FLT_LARGEST_EXP, FLT_SMALLEST_EXP, FLT_UWORD_IS_FINITE, FLT_UWORD_IS_SUBNORMAL,
        FLT_UWORD_IS_ZERO,
    };
    let mut ix: u32 = x.to_bits();
    let hx: u32 = ix & 0x7fff_ffff; /*|x| */
    let mut k: i32 = (hx >> 23) as i32; /* extract exponent */

    if FLT_UWORD_IS_ZERO(hx) {
        return x;
    }
    if !FLT_UWORD_IS_FINITE(hx) {
        return x + x; /* NaN or Inf */
    }
    if FLT_UWORD_IS_SUBNORMAL(hx) {
        x *= TWO25;
        ix = x.to_bits();
        k = ((ix & 0x7f80_0000) >> 23) as i32 - 25;
        if n < -OVERFLOW_INT {
            return TINY * x; /*underflow*/
        }
    }
    k += n;
    if k as u32 > FLT_LARGEST_EXP {
        return HUGE * copysignf(HUGE, x); /* overflow  */
    }
    if k > 0 {
        /* normal result */
        return f32::from_bits((ix & 0x807f_ffff) | ((k as u32) << 23));
    }
    if k < FLT_SMALLEST_EXP {
        if n > OVERFLOW_INT {
            /* in case integer overflow in n+k */
            return HUGE * copysignf(HUGE, x); /*overflow*/
        } else {
            return TINY * copysignf(TINY, x); /*underflow*/
        }
    }
    k += 25; /* subnormal result */
    f32::from_bits((ix & 0x807f_ffff) | ((k as u32) << 23)) * TWOM25
}
